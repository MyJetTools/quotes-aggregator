use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug)]
pub struct DateTimeAsMicroseconds {
    pub unix_microseconds: i64,
}

impl DateTimeAsMicroseconds {
    pub fn new(unix_microseconds: i64) -> Self {
        Self { unix_microseconds }
    }

    pub fn now() -> Self {
        let unix_microseconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as i64;

        Self { unix_microseconds }
    }

    pub fn parse_iso_string(iso_string: &str) -> Option<Self> {
        let result = super::utils::parse_iso_string(iso_string.as_bytes())?;
        return Some(Self::new(result));
    }

    pub fn to_chrono_utc(&self) -> DateTime<Utc> {
        let d = UNIX_EPOCH + Duration::from_micros(self.unix_microseconds as u64);
        return DateTime::<Utc>::from(d);
    }

    pub fn seconds_before(&self, before: DateTimeAsMicroseconds) -> i64 {
        (self.unix_microseconds - before.unix_microseconds) / 1000000
    }

    pub fn duration_since(&self, before: DateTimeAsMicroseconds) -> Duration {
        let dur = self.unix_microseconds - before.unix_microseconds;

        if dur < 0 {
            return Duration::from_micros(0);
        }

        Duration::from_micros(dur as u64)
    }

    pub fn to_rfc3339(&self) -> String {
        let chrono = self.to_chrono_utc();
        return chrono.to_rfc3339();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds_between() {
        let now = DateTimeAsMicroseconds::parse_iso_string("2021-04-25T17:30:03.000Z").unwrap();
        let before = DateTimeAsMicroseconds::parse_iso_string("2021-04-25T17:30:00.000Z").unwrap();
        assert_eq!(3, now.seconds_before(before));
    }
}