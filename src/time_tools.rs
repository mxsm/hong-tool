use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

/// Utility for working with time.
pub struct TimeUtils;

/// Enumeration representing different time units.
pub enum TimeUnit {
    MILLIS,
    NANOS,
    MICROS,
    SEC,
}

impl TimeUtils {
    /// Get the duration since the UNIX epoch.
    fn duration() -> Result<Duration, SystemTimeError> {
        let current_time = SystemTime::now();
        current_time.duration_since(UNIX_EPOCH)
    }

    /// Get the current time in milliseconds since the UNIX epoch.
    ///
    /// # Returns
    ///
    /// Returns the current time in milliseconds as a `Result<u128, SystemTimeError>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hong_tool::time_tools::{TimeUtils};
    ///
    /// fn main() {
    ///     match TimeUtils::now() {
    ///         Ok(milliseconds) => println!("Current time in milliseconds: {}", milliseconds),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    pub fn now() -> Result<u128, SystemTimeError> {
        let since_epoch = Self::duration();
        Ok(since_epoch?.as_millis())
    }

    /// Get the current time in the specified time unit.
    ///
    /// # Arguments
    ///
    /// * `time_unit` - The time unit to use for the result.
    ///
    /// # Returns
    ///
    /// Returns the current time in the specified time unit as a `Result<u128, SystemTimeError>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hong_tool::time_tools::{TimeUtils, TimeUnit};
    ///
    /// fn main() {
    ///     match TimeUtils::now_as_timeunit(TimeUnit::MILLIS) {
    ///         Ok(milliseconds) => println!("Current time in milliseconds: {}", milliseconds),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    pub fn now_as_timeunit(time_unit: TimeUnit) -> Result<u128, SystemTimeError> {
        let since_epoch = Self::duration();
        match time_unit {
            TimeUnit::MILLIS => Ok(since_epoch?.as_millis()),
            TimeUnit::NANOS => Ok(since_epoch?.as_nanos()),
            TimeUnit::MICROS => Ok(since_epoch?.as_micros()),
            TimeUnit::SEC => Ok(since_epoch?.as_secs() as u128),
        }
    }
}

