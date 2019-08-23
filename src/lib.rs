use std::time::{Duration, SystemTime};

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let elapsed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            - *EPOCH;
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
static mut _EPOCH: Duration = Duration::from_secs(0);
static EPOCH: &Duration = unsafe { &_EPOCH };

pub fn set_level(lv: log::LevelFilter) {
    log::set_max_level(lv);
}

pub fn init() -> Result<(), log::SetLoggerError> {
    unsafe {
        _EPOCH = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
    }
    log::set_logger(&LOGGER).map(|()| set_level(log::LevelFilter::Info))
}
