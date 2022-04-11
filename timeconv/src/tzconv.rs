pub mod utc {
    use chrono::format::ParseError;
    use chrono::prelude::*;

    pub fn unix2time(t: i64) -> DateTime<Utc> {
        Utc.timestamp(t, 0)
    }

    pub fn str2time(t: &str) -> Result<DateTime<FixedOffset>, ParseError> {
        DateTime::parse_from_rfc3339(t)
    }
}

pub mod local {
    use chrono::prelude::*;
    pub fn unix2time(t: i64) -> DateTime<Local> {
        Local.timestamp(t, 0)
    }
}

pub mod newyork {
    use chrono::prelude::*;
    use once_cell::sync::Lazy;

    static NEW_YORK: Lazy<FixedOffset> = Lazy::new(|| FixedOffset::west(4 * 3600));

    pub fn unix2time(t: i64) -> DateTime<FixedOffset> {
        NEW_YORK.timestamp(t, 0)
    }
}

pub mod newyork2 {
    use chrono::prelude::*;
    use chrono_tz::America::New_York;

    pub fn unix2time(t: i64) -> DateTime<chrono_tz::Tz> {
        New_York.timestamp(t, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utc() {
        let dt = utc::unix2time(1648771200);
        assert_eq!(&dt.to_rfc3339(), "2022-04-01T00:00:00+00:00");
    }

    #[test]
    fn test_local() {
        let dt = local::unix2time(1648771200);
        assert_eq!(&dt.to_rfc3339(), "2022-04-01T09:00:00+09:00");
    }

    #[test]
    fn test_newyork() {
        let dt = newyork::unix2time(1648771200);
        assert_eq!(&dt.to_rfc3339(), "2022-03-31T20:00:00-04:00");
    }

    #[test]
    fn test_newyork2() {
        let dt = newyork2::unix2time(1648771200);
        assert_eq!(&dt.to_rfc3339(), "2022-03-31T20:00:00-04:00");
    }
}
