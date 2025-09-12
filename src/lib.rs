//! # time-now
//!
//! Get _current time_ as seconds or milli/micro/nano seconds, since unix epoch time, using _single function_ call.
//!
//! Pure Rust `std` library based. No dependencies on any other crate.
//!
//! Safe and Lightweight (~2 kb).
//!
//! # Examples:
//! ```
//!use time_now;
//!
//!fn main() {
//!    println!("time_now::now_as_secs    : {:?}", time_now::now_as_secs());
//!    println!(
//!        "time_now::now_as_secs_f32: {:?}",
//!        time_now::now_as_secs_f32()
//!    );
//!    println!(
//!        "time_now::now_as_secs_f64: {:?}",
//!        time_now::now_as_secs_f64()
//!    );
//!    println!("time_now::now_as_millis  : {:?}", time_now::now_as_millis());
//!    println!("time_now::now_as_micros  : {:?}", time_now::now_as_micros());
//!    println!("time_now::now_as_nanos   : {:?}", time_now::now_as_nanos());
//!    println!(
//!        "time_now::duration_since_epoch: {:?}",
//!        time_now::duration_since_epoch()
//!    );
//!
//!    println!("NOTE: Each function calls std::time::SystemTime::now(),
//!      hence each subsequent call will be ahead of time compare to previous call");
//!}
//! ```
//! # Stability
//! Only stable functions have been included.
//! The _Nightly-only experimental APIs_ (`Duration::as_millis_f32()` and `Duration::as_millis_f64()`
//! have not been included. They'll be included in future, once they become stable.
//!

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

/// Returns the number of seconds since [`UNIX_EPOCH`], as `f32`. Uses: [`std::time`].
///
/// As per [std docs](https://doc.rust-lang.org/1.88.0/core/time/struct.Duration.html#method.as_secs_f32): _The returned value includes the fractional (nanosecond) part of the duration_.
pub fn now_as_secs_f32() -> f32 {
    duration_since_epoch().as_secs_f32()
}

/// Returns the number of seconds since [`UNIX_EPOCH`], as `f64`. Uses: [`std::time`].
///
/// As per [std docs](https://doc.rust-lang.org/1.88.0/core/time/struct.Duration.html#method.as_secs_f64): _The returned value includes the fractional (nanosecond) part of the duration_.
pub fn now_as_secs_f64() -> f64 {
    duration_since_epoch().as_secs_f64()
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

    #[test]
    fn now_as_secs_f32_works() {
        let _result = now_as_secs_f32();
    }

    #[test]
    fn now_as_secs_f64_works() {
        let _result = now_as_secs_f64();
    }
}
