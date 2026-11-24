//! Timestamped event trace for resize diagnosis, enabled by `RTTY_TRACE=1`.

use std::sync::OnceLock;
use std::time::Instant;

static START: OnceLock<Option<Instant>> = OnceLock::new();

pub fn enabled() -> bool {
    START
        .get_or_init(|| std::env::var_os("RTTY_TRACE").map(|_| Instant::now()))
        .is_some()
}

pub fn log(message: std::fmt::Arguments) {
    if let Some(Some(start)) = START.get() {
        eprintln!("TR {:>10}us {message}", start.elapsed().as_micros());
    }
}

/// Sync experiment selected by `RTTY_SYNC`: `off`, `gtk`, `jump`, or unset.
pub fn sync_mode() -> &'static str {
    static MODE: OnceLock<String> = OnceLock::new();
    MODE.get_or_init(|| std::env::var("RTTY_SYNC").unwrap_or_default())
        .as_str()
}
