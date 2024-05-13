const LOGS_DIR: &str = "./logs";
const LOGS_FILE_PREFIX: &str = "logs.log";

pub fn init() {
    // setup non blocking file appender
    let file_appender = tracing_appender::rolling::never(LOGS_DIR, LOGS_FILE_PREFIX);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // forget the guard so it is never dropped; otherwise logs aren't being written
    std::mem::forget(guard);

    tracing_subscriber::fmt()
        .json()
        .with_target(false)
        .with_current_span(false)
        .with_ansi(false)
        .with_writer(non_blocking)
        .init();
}
