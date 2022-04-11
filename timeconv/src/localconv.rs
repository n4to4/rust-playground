use chrono::prelude::*;

pub fn time2unix<Tz: TimeZone>(t: DateTime<Tz>) -> i64 {
    t.timestamp()
}

pub fn time2str<Tz>(t: DateTime<Tz>) -> String
where
    Tz: TimeZone,
    Tz::Offset: std::fmt::Display,
{
    // t.to_rfc3339()
    t.format("%Y-%m-%dT%H:%M:%S%:z").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time2unix() {
        let t = Local.ymd(2022, 4, 1).and_hms(9, 0, 0);
        assert_eq!(1648771200, time2unix(t));
    }

    #[test]
    fn test_time2str() {
        let t = Local.ymd(2022, 4, 1).and_hms(9, 0, 0);
        assert_eq!("2022-04-01T09:00:00+09:00", &time2str(t));
    }
}
