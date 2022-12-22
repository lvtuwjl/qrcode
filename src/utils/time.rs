use std::{
    thread,
    time::{Duration, SystemTime},
};

// 睡眠dur秒 sleep second
pub fn sleep(dur: u64) {
    thread::sleep(Duration::from_secs(dur));
}

// 多少秒 second
pub fn secs() -> u64 {
    let dur = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    dur.as_secs()
}

// 多少毫秒 millisecond
pub fn millis() -> u128 {
    let dur = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    dur.as_millis()
}

// 多少微秒 microsecond
pub fn micros() -> u128 {
    let dur = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    dur.as_micros()
}

// 多少纳秒 nanosecond
pub fn nanos() -> u128 {
    let dur = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    dur.as_nanos()
}
