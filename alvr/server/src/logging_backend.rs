use alvr_common::logging::*;
use fern::Dispatch;
use log::LevelFilter;
use std::fs;
use tokio::sync::broadcast::Sender;

// Define logging modes, create crash log and (re)create session log
pub fn init_logging(log_sender: Sender<String>) {
    let mut log_dispatch = Dispatch::new().format(move |out, message, record| {
        let log_line = format!(
            "{} [{}] {}",
            chrono::Local::now().format("%H:%M:%S.%f"),
            record.level(),
            message
        );
        log_sender.send(log_line.clone()).ok();
        out.finish(format_args!("{}", log_line));
    });

    if cfg!(debug_assertions) {
        log_dispatch = log_dispatch
            .level(LevelFilter::Debug)
            .chain(std::io::stdout());
    } else {
        log_dispatch = log_dispatch.level(LevelFilter::Info);
    }

    log_dispatch
        .chain(
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(SESSION_LOG_FNAME)
                .unwrap(),
        )
        .chain(
            Dispatch::new()
                .level(LevelFilter::Error)
                .chain(fern::log_file(CRASH_LOG_FNAME).unwrap()),
        )
        .apply()
        .unwrap();

    crate::logging::set_panic_hook();
}
