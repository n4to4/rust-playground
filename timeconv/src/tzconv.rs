pub mod utc {
    use chrono::prelude::*;

    /// ## UNIX時間から`DateTime<Utc>`型への変換
    ///
    /// Goのは`In`メソッドでタイムゾーンを後から指定するが
    /// Rustのは`TimeZone`を実装した型(`Utc`)のメソッドを使う
    pub fn unix2time(t: i64) -> DateTime<Utc> {
        Utc.timestamp(t, 0)

        //Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(t, 0))
    }

    /// ## 文字列から`DateTime<Utc>`型への変換
    ///
    /// `parse_from_rfc3339`はタイムゾーン情報を解釈して`DateTime<FixedOffset>`を返す
    /// それを`DateTime<Utc>`に変換するために`with_timezone`を使う
    ///
    /// 文字列にタイムゾーンがなかったりするとエラー
    pub fn str2time(t: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        Ok(DateTime::parse_from_rfc3339(t)?.with_timezone(&Utc))
    }
}

pub mod local {
    use chrono::prelude::*;

    /// `TimeZone` = `Local`の場合
    pub fn unix2time(t: i64) -> DateTime<Local> {
        Local.timestamp(t, 0)
    }

    pub fn str2time(t: &str) -> Result<DateTime<Local>, chrono::ParseError> {
        Ok(DateTime::parse_from_rfc3339(t)?.with_timezone(&Local))
    }
}

pub mod newyork {
    use chrono::prelude::*;
    use once_cell::sync::Lazy;

    static NEW_YORK: Lazy<FixedOffset> = Lazy::new(|| FixedOffset::west(4 * 3600));

    /// `TimeZone` = `NEW_YORK`の場合
    ///
    /// `FixedOffset`のメソッドから任意のタイムゾーンを作れる
    /// `FixedOffset`は`TimeZone`を実装しているので`Utc`や`Local`のように使える
    pub fn unix2time(t: i64) -> DateTime<FixedOffset> {
        NEW_YORK.timestamp(t, 0)
    }

    pub fn str2time(t: &str) -> Result<DateTime<FixedOffset>, chrono::ParseError> {
        DateTime::parse_from_rfc3339(t)
    }
}

pub mod newyork2 {
    use chrono::prelude::*;
    use chrono_tz::America::New_York;

    /// `TimeZone` = `NEW_YORK`の場合(2)
    ///
    /// `chrono_tz`には名前付きの`TimeZone`が定義されているのでこれを使っても良い
    pub fn unix2time(t: i64) -> DateTime<chrono_tz::Tz> {
        New_York.timestamp(t, 0)
    }

    pub fn str2time(t: &str) -> Result<DateTime<chrono_tz::Tz>, chrono::ParseError> {
        Ok(DateTime::parse_from_rfc3339(t)?.with_timezone(&New_York))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //
    // unix2time
    //

    #[test]
    fn test_utc_unix2time() {
        let dt = utc::unix2time(1648771200);
        assert_eq!(&dt.to_rfc3339(), "2022-04-01T00:00:00+00:00");
    }

    #[test]
    fn test_local_unix2time() {
        let dt = local::unix2time(1648771200);
        assert_eq!(&dt.to_rfc3339(), "2022-04-01T09:00:00+09:00");
    }

    #[test]
    fn test_newyork_unix2time() {
        let dt = newyork::unix2time(1648771200);
        assert_eq!(&dt.to_rfc3339(), "2022-03-31T20:00:00-04:00");
    }

    #[test]
    fn test_newyork2_unix2time() {
        let dt = newyork2::unix2time(1648771200);
        assert_eq!(&dt.to_rfc3339(), "2022-03-31T20:00:00-04:00");
    }

    //
    // str2time
    //

    #[test]
    fn test_utc_str2time() {
        assert_eq!(
            1648771200,
            utc::str2time("2022-03-31T20:00:00-04:00")
                .unwrap()
                .timestamp()
        );
    }

    #[test]
    fn test_local_str2time() {
        assert_eq!(
            1648771200,
            local::str2time("2022-03-31T20:00:00-04:00")
                .unwrap()
                .timestamp()
        );
    }

    #[test]
    fn test_newyork_str2time() {
        assert_eq!(
            1648771200,
            newyork::str2time("2022-03-31T20:00:00-04:00")
                .unwrap()
                .timestamp()
        );
    }

    #[test]
    fn test_newyork2_str2time() {
        assert_eq!(
            1648771200,
            newyork2::str2time("2022-03-31T20:00:00-04:00")
                .unwrap()
                .timestamp()
        );
    }

    //
    // fail to parse time string
    //

    #[test]
    fn fail_local_str2time() {
        let t = "2022-03-31T20:00:00"; // no timezone
        assert!(local::str2time(t).is_err());
    }
}
