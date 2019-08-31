use std::time::{Duration, SystemTime};

use log::{Metadata, Record, SetLoggerError, Log};
pub use log::LevelFilter;

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let elapsed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            - unsafe { EPOCH };
        println!(
            "[{}.{:03}] {} {}",
            elapsed.as_secs(),
            elapsed.subsec_millis(),
            record.level(),
            record.args()
        );
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;
static mut EPOCH: Duration = Duration::from_secs(0);

pub fn set_level(lv: LevelFilter) {
    log::set_max_level(lv);
}

pub fn init() -> Result<(), SetLoggerError> {
    unsafe {
        EPOCH = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
    }
    log::set_logger(&LOGGER).map(|()| set_level(LevelFilter::Info))
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    #[test]
    fn test_log() {
        use std::thread::sleep;

        init().unwrap();

        info!("info");
        set_level(LevelFilter::Error);
        info!("info 0");
        set_level(LevelFilter::Info);
        info!("info 1");
        sleep(Duration::from_secs(1));
        info!("info 2");
    }
}
