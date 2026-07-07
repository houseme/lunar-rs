use lunar_rs::{CalendarDay, CalendarSpan, EventKind, EventQuery, Nanakshahi, NanakshahiMonth, NanakshahiYear, Solar};

#[test]
fn nanakshahi_round_trip_core_examples() {
    let solar = Solar::from_ymd(2024, 3, 14).unwrap();
    let date = solar.nanakshahi().unwrap();
    assert_eq!(date.to_string(), "纳纳克沙希556年1月1日");
    assert_eq!(date.solar().to_ymd(), "2024-03-14");

    assert_eq!(Solar::from_ymd(2024, 3, 13).unwrap().nanakshahi().unwrap().to_string(), "纳纳克沙希555年12月31日");
    assert_eq!(Nanakshahi::from_ymd(556, 1, 1).unwrap().solar().to_ymd(), "2024-03-14");
}

#[test]
fn nanakshahi_year_and_month_layers_are_available() {
    let year = NanakshahiYear::from_year(556).unwrap();
    let month = NanakshahiMonth::from_ym(556, 12).unwrap();

    assert_eq!(year.solar_year(), 2024);
    assert!(!year.is_leap());
    assert_eq!(year.day_count(), 365);
    assert_eq!(year.first_month().to_string(), "纳纳克沙希556年1月");
    assert_eq!(year.last_month().to_string(), "纳纳克沙希556年12月");
    assert_eq!(year.first_day().to_string(), "纳纳克沙希556年1月1日");
    assert_eq!(year.last_day().to_string(), "纳纳克沙希556年12月30日");
    assert_eq!(year.months().len(), 12);
    assert!(year.contains_solar(Solar::from_ymd(2024, 8, 1).unwrap()));

    assert_eq!(month.day_count(), 30);
    assert_eq!(month.first_solar_day().to_ymd(), "2025-02-12");
    assert_eq!(month.last_solar_day().to_ymd(), "2025-03-13");
    assert_eq!(month.days().len(), 30);
}

#[test]
fn nanakshahi_next_and_subtract_follow_solar_days() {
    let a = Nanakshahi::from_ymd(556, 1, 1).unwrap();
    let b = a.next(10).unwrap();
    assert_eq!(b.to_string(), "纳纳克沙希556年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = NanakshahiMonth::from_ym(556, 12).unwrap();
    assert_eq!(month.next(1).unwrap().to_string(), "纳纳克沙希557年1月");
}

#[test]
fn solar_can_resolve_nanakshahi_companions() {
    let solar = Solar::from_ymd(2024, 3, 14).unwrap();
    assert_eq!(solar.nanakshahi().unwrap().to_string(), "纳纳克沙希556年1月1日");
    assert_eq!(solar.nanakshahi_year().unwrap().year(), 556);
    assert_eq!(solar.nanakshahi_month().unwrap().month(), 1);
}

#[test]
fn nanakshahi_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().nanakshahi().unwrap();
    let month = day.nanakshahi_month();
    let year = day.nanakshahi_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 1).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
