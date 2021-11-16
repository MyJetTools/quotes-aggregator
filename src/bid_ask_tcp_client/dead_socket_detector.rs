use std::sync::atomic::{AtomicI64, Ordering};

use std::time::{SystemTime, UNIX_EPOCH};

pub enum SocketTimeoutStatus {
    Ok,
    Timeout(i64),
}

pub struct DeadSocketDetector {
    last_success_mess_unix_ms: AtomicI64,
    timeout_sec: i8,
}

impl DeadSocketDetector {
    pub fn new(timeout: i8) -> DeadSocketDetector {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as i64;

        DeadSocketDetector {
            last_success_mess_unix_ms: AtomicI64::new(timestamp),
            timeout_sec: timeout,
        }
    }

    pub fn track_event(&self) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as i64;

        self.last_success_mess_unix_ms
            .store(timestamp, Ordering::SeqCst);
    }

    pub fn is_timeout(&self) -> SocketTimeoutStatus {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as i64;

        let timeout_sec = (timestamp - self.last_success_mess_unix_ms.load(Ordering::SeqCst)) / 100;
        if timeout_sec > self.timeout_sec.into() {
            return SocketTimeoutStatus::Timeout(timeout_sec);
        }

        return SocketTimeoutStatus::Ok;
    }
}