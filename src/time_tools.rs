use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct TimeUtils;

pub enum TimeUnit {
    MILLIS,
    NANOS,
    MICROS,
    SEC,
}


impl TimeUtils {
    fn duration() -> Duration {
        let current_time = SystemTime::now();
        current_time.duration_since(UNIX_EPOCH).expect("Time went backwards")
    }

    pub fn now() -> u128 {
        let since_epoch = Self::duration();
        since_epoch.as_millis()
    }

    pub fn now_as_timeunit(time_unit: TimeUnit) -> u128 {
        let since_epoch = Self::duration();
        match time_unit {
            TimeUnit::MILLIS => since_epoch.as_millis(),
            TimeUnit::NANOS => since_epoch.as_nanos(),
            TimeUnit::MICROS => since_epoch.as_micros(),
            TimeUnit::SEC => since_epoch.as_secs() as u128,
        }
    }
}

