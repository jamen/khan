use log::{Level, LevelFilter, Log, Metadata, Record};

struct UciLogger;

impl Log for UciLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("info string [{}] {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

pub fn init(max_level: LevelFilter) {
    log::set_logger(&UciLogger).expect("failed to initialize logger (already initialized?)");
    log::set_max_level(max_level);
}
