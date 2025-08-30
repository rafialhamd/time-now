//! # time-now
//!
//! Standard (std) library based time library. Safe and single functions to get current time as duration or seconds or milli/micro/nano seconds since unix epoch time.
//!
//! # Examples:
//! ```
//! use time_now;
//!
//! fn main() {
//!     println!("time_now::now_as_secs: {:?}", time_now::now_as_secs());
//!     println!("time_now::now_as_millis: {:?}", time_now::now_as_millis());
//!     println!("time_now::now_as_micros: {:?}", time_now::now_as_micros());
//!     println!("time_now::now_as_nanos: {:?}", time_now::now_as_nanos());
//!     println!(
//!         "time_now::duration_since_epoch: {:?}",
//!         time_now::duration_since_epoch()
//!     );
//! }
//! ```

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Returns [`std::time::Duration`] since [`UNIX_EPOCH`]. Uses: [`std::time`].
/// The returned `Duration` is _safe_ unwrap of `Result` returned by `SystemTime::now().duration_since(UNIX_EPOCH)`. The _unwrap is safe_ because the `UNIX_EPOCH` is _earlier than now_.
pub fn duration_since_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("This shouldn't happen since UNIX_EPOCH is EARLIER than NOW")
}

/// Returns the number of _whole_ microseconds since [`UNIX_EPOCH`]. Uses: [`std::time`].
pub fn now_as_micros() -> u128 {
    duration_since_epoch().as_micros()
}

/// Returns the number of _whole_ milliseconds since [`UNIX_EPOCH`]. Uses: [`std::time`].
pub fn now_as_millis() -> u128 {
    duration_since_epoch().as_millis()
}

/// Returns the number of nanoseconds since [`UNIX_EPOCH`]. Uses: [`std::time`].
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
