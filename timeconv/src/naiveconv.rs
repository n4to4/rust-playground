#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    #[test]
    fn test_from_naive() {
        let t = NaiveDate::from_ymd(2022, 4, 1).and_hms(0, 0, 0);

        let utc = Utc.from_utc_datetime(&t);
        assert_eq!("2022-04-01T00:00:00+00:00", &utc.to_rfc3339());

        let local_local = Local.from_local_datetime(&t).single().unwrap();
        assert_eq!("2022-04-01T00:00:00+09:00", &local_local.to_rfc3339());

        let local_utc = Local.from_utc_datetime(&t);
        assert_eq!("2022-04-01T09:00:00+09:00", &local_utc.to_rfc3339());
    }

    #[test]
    fn test_to_naive() {
        let t = 1648771200;

        let naive = Utc.timestamp(t, 0).naive_utc();
        assert_eq!(naive, NaiveDate::from_ymd(2022, 4, 1).and_hms(0, 0, 0));

        let naive = Utc.timestamp(t, 0).naive_local();
        assert_eq!(naive, NaiveDate::from_ymd(2022, 4, 1).and_hms(0, 0, 0));

        let naive = Local.timestamp(t, 0).naive_utc();
        assert_eq!(naive, NaiveDate::from_ymd(2022, 4, 1).and_hms(0, 0, 0));

        let naive = Local.timestamp(t, 0).naive_local();
        assert_eq!(naive, NaiveDate::from_ymd(2022, 4, 1).and_hms(9, 0, 0));
    }
}
