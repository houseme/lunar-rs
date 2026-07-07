use lunar_rs::{CalendarDay, CalendarSpan, EventKind, EventQuery, Seleucid, SeleucidMonth, SeleucidYear, Solar};

#[test]
fn seleucid_round_trip_core_examples() {
    assert_eq!(Solar::from_ymd(2024, 9, 30).unwrap().seleucid().unwrap().to_string(), "塞琉古2335年9月30日");
    assert_eq!(Solar::from_ymd(2024, 10, 1).unwrap().seleucid().unwrap().to_string(), "塞琉古2336年10月1日");
    assert_eq!(Seleucid::from_ymd(2335, 9, 30).unwrap().solar().to_ymd(), "2024-09-30");
    assert_eq!(Seleucid::from_ymd(2336, 10, 1).unwrap().solar().to_ymd(), "2024-10-01");
}

#[test]
fn seleucid_year_and_month_layers_are_available() {
    let year = SeleucidYear::from_year(2336).unwrap();
    let month = SeleucidMonth::from_ym(2336, 10).unwrap();

    assert_eq!(year.first_day().solar().to_ymd(), "2024-10-01");
    assert_eq!(year.last_day().solar().to_ymd(), "2025-09-30");
    assert_eq!(year.months().len(), 12);
    assert!(year.contains_solar(Solar::from_ymd(2025, 4, 1).unwrap()));

    assert_eq!(month.solar_year(), 2024);
    assert_eq!(month.day_count(), 31);
    assert_eq!(month.first_solar_day().to_ymd(), "2024-10-01");
    assert_eq!(month.last_solar_day().to_ymd(), "2024-10-31");
    assert_eq!(month.days().len(), 31);
}

#[test]
fn seleucid_next_and_subtract_follow_solar_days_and_year_boundary() {
    let last = Solar::from_ymd(2024, 9, 30).unwrap().seleucid().unwrap();
    let next = last.next(1).unwrap();
    assert_eq!(next.to_string(), "塞琉古2336年10月1日");
    assert_eq!(next.subtract(last), 1);

    let september = SeleucidMonth::from_ym(2335, 9).unwrap();
    assert_eq!(september.next(1).unwrap().to_string(), "塞琉古2336年10月");
}

#[test]
fn solar_can_resolve_seleucid_companions() {
    let solar = Solar::from_ymd(2024, 10, 1).unwrap();
    assert_eq!(solar.seleucid().unwrap().to_string(), "塞琉古2336年10月1日");
    assert_eq!(solar.seleucid_year().unwrap().year(), 2336);
    assert_eq!(solar.seleucid_month().unwrap().month(), 10);
}

#[test]
fn seleucid_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().seleucid().unwrap();
    let month = day.seleucid_month();
    let year = day.seleucid_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
