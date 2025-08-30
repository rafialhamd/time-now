use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Returns [`std::time::Duration`] since [`UNIX_EPOCH`]. Uses: [`std::time`].
/// The returned `Duration` is unwrap of `Result` returned by `SystemTime::now().duration_since(UNIX_EPOCH)`, as `UNIX_EPOCH` is _earlier than now_.
pub fn duration_since_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("This shouldn't happen since UNIX_EPOCH is EARLIER than NOW")
}

/// Returns the total number of whole microseconds since [`UNIX_EPOCH`]. Uses: [`std::time`].
pub fn now_as_micros() -> u128 {
    duration_since_epoch().as_micros()
}

/// Returns the total number of whole milliseconds since [`UNIX_EPOCH`]. Uses: [`std::time`].
pub fn now_as_millis() -> u128 {
    duration_since_epoch().as_millis()
}

/// Returns the total number of nanoseconds since [`UNIX_EPOCH`]. Uses: [`std::time`].
pub fn now_as_nanos() -> u128 {
    duration_since_epoch().as_nanos()
}

/// Returns the number of _whole_ seconds since [`UNIX_EPOCH`]. Uses: [`std::time`].
///
/// As per [std docs](https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs): _The returned value does not include the fractional (nanosecond) part of the duration, which can be obtained using subsec_nanos_.
pub fn now_as_secs() -> u64 {
    duration_since_epoch().as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_since_epoch_works() {
        let _result = duration_since_epoch();
    }

    #[test]
    fn now_as_micros_works() {
        let _result = now_as_micros();
    }

    #[test]
    fn now_as_millis_works() {
        let _result = now_as_millis();
    }

    #[test]
    fn now_as_nanos_works() {
        let _result = now_as_nanos();
    }

    #[test]
    fn now_as_secs_works() {
        let _result = now_as_secs();
    }
}
