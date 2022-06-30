//! ## 時刻を表すための方式
//!
//! `DateTime<Tz: TimeZone>`
//!
//! - `DateTime<Utc>`
//! - `DateTime<Local>`
//! - `DateTime<FixxedOffset>`
//!
//! とか色々あってややこしい
//!
//! `DateTIme<Tz>`を作るには`TimeZone`のメソッドから作る
//!
//! - `Utc::now()` -> `DateTime<Utc>` とか、
//! - `Local.ymd(2022, 04, 01).and_hms(...)` -> `DateTime<Local>`

use chrono::prelude::*;

/// ## `DateTime<Tz>`型からの変換
///
/// `DateTime<Tz>` -> UNIX時間
pub fn time2unix<Tz: TimeZone>(t: DateTime<Tz>) -> i64 {
    t.timestamp()
}

/// `DateTime<Tz>` -> 文字列
pub fn time2str<Tz>(t: DateTime<Tz>) -> String
where
    Tz: TimeZone,
    Tz::Offset: std::fmt::Display,
{
    //t.format("%Y-%m-%dT%H:%M:%S%:z").to_string()
    t.to_rfc3339()
}

/// ## UNIX時間からの変換
///
/// UNIX時間 -> `DateTime<Tz>`型
pub fn unix2time(t: i64) -> DateTime<Local> {
    Local.timestamp(t, 0)
}

/// UNIX時間 -> 文字列
pub fn unix2str(t: i64) -> String {
    time2str(Local.timestamp(t, 0))
}

/// ## 文字列からの変換
///
/// 文字列 -> `DateTime<Tz>`型
pub fn str2time(t: &str) -> Result<DateTime<Local>, chrono::ParseError> {
    Ok(DateTime::parse_from_rfc3339(t)?.with_timezone(&Local))
}

/// 文字列 -> UNIX時間
pub fn str2unix(t: &str) -> Result<i64, chrono::ParseError> {
    Ok(DateTime::parse_from_rfc3339(t)?.timestamp())
}

#[cfg(test)]
mod tests {
    use super::*;

    //
    // DateTime to X
    //

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

    //
    // Unixtime to X
    //

    #[test]
    fn test_unix2time() {
        let t = 1648771200;
        let t = unix2time(t);
        assert_eq!("2022-04-01T09:00:00+09:00", &time2str(t));
    }

    #[test]
    fn test_unix2str() {
        let t = 1648771200;
        assert_eq!("2022-04-01T09:00:00+09:00", &unix2str(t));
    }

    //
    // String to X
    //

    #[test]
    fn test_str2time() {
        let t = str2time("2022-04-01T09:00:00+09:00").unwrap();
        assert_eq!(1648771200, time2unix(t));
    }

    #[test]
    fn test_str2unix() {
        assert_eq!(1648771200, str2unix("2022-04-01T09:00:00+09:00").unwrap());
    }
}
