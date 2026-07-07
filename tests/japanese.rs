use lunar_rs::{
    CalendarDay, CalendarSpan, EventKind, EventQuery, Japanese, JapaneseEra, JapaneseMonth, JapaneseYear, Solar,
};

#[test]
fn japanese_round_trip_core_examples_match_modern_era_boundaries() {
    assert_eq!(Solar::from_ymd(2019, 4, 30).unwrap().japanese().unwrap().to_string(), "平成31年4月30日");
    assert_eq!(Solar::from_ymd(2019, 5, 1).unwrap().japanese().unwrap().to_string(), "令和1年5月1日");
    assert_eq!(Solar::from_ymd(1989, 1, 7).unwrap().japanese().unwrap().to_string(), "昭和64年1月7日");
    assert_eq!(Solar::from_ymd(1989, 1, 8).unwrap().japanese().unwrap().to_string(), "平成1年1月8日");

    assert_eq!(Japanese::from_eymd(JapaneseEra::Reiwa, 1, 5, 1).unwrap().solar().to_ymd(), "2019-05-01");
}

#[test]
fn japanese_year_and_month_layers_capture_partial_era_ranges() {
    let reiwa1 = JapaneseYear::from_era_year(JapaneseEra::Reiwa, 1).unwrap();
    assert_eq!(reiwa1.solar_year(), 2019);
    assert_eq!(reiwa1.first_month().month(), 5);
    assert_eq!(reiwa1.last_month().month(), 12);
    assert_eq!(reiwa1.months().len(), 8);
    assert_eq!(reiwa1.first_day().solar().to_ymd(), "2019-05-01");
    assert_eq!(reiwa1.last_day().solar().to_ymd(), "2019-12-31");
    assert!(reiwa1.contains_solar(Solar::from_ymd(2019, 8, 10).unwrap()));

    let heisei31 = JapaneseYear::from_era_year(JapaneseEra::Heisei, 31).unwrap();
    assert_eq!(heisei31.first_month().month(), 1);
    assert_eq!(heisei31.last_month().month(), 4);
    assert_eq!(heisei31.last_day().solar().to_ymd(), "2019-04-30");

    let showa64_january = JapaneseMonth::from_eym(JapaneseEra::Showa, 64, 1).unwrap();
    assert_eq!(showa64_january.day_count(), 7);
    assert_eq!(showa64_january.first_day().solar().to_ymd(), "1989-01-01");
    assert_eq!(showa64_january.last_day().solar().to_ymd(), "1989-01-07");
}

#[test]
fn japanese_next_and_subtract_follow_solar_days_and_era_switches() {
    let heisei_last = Solar::from_ymd(2019, 4, 30).unwrap().japanese().unwrap();
    let reiwa_first = heisei_last.next(1).unwrap();
    assert_eq!(reiwa_first.to_string(), "令和1年5月1日");
    assert_eq!(reiwa_first.subtract(heisei_last), 1);
    assert!(reiwa_first.is_after(heisei_last));

    let april = JapaneseMonth::from_eym(JapaneseEra::Heisei, 31, 4).unwrap();
    assert_eq!(april.next(1).unwrap().to_string(), "令和1年5月");

    let heisei31 = JapaneseYear::from_era_year(JapaneseEra::Heisei, 31).unwrap();
    assert_eq!(heisei31.next(1).unwrap().to_string(), "令和2年");
}

#[test]
fn solar_can_resolve_japanese_companions() {
    let solar = Solar::from_ymd(2019, 5, 1).unwrap();
    assert_eq!(solar.japanese().unwrap().to_string(), "令和1年5月1日");
    assert_eq!(solar.japanese_year().unwrap().to_string(), "令和1年");
    assert_eq!(solar.japanese_month().unwrap().to_string(), "令和1年5月");
}

#[test]
fn japanese_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().japanese().unwrap();
    let month = Solar::from_ymd(2024, 1, 1).unwrap().japanese_month().unwrap();
    let year = Solar::from_ymd(2024, 1, 1).unwrap().japanese_year().unwrap();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
