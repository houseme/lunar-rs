use lunar_rs::{Armenian, ArmenianMonth, ArmenianYear, CalendarDay, CalendarSpan, EventKind, EventQuery, Solar};

#[test]
fn armenian_round_trip_core_examples() {
    let solar = Solar::from_ymd(2024, 7, 24).unwrap();
    let armenian = solar.armenian();
    assert_eq!(armenian.to_string(), "亚美尼亚1474年1月1日");
    assert_eq!(armenian.solar().to_ymd(), "2024-07-24");

    assert_eq!(Solar::from_ymd(2024, 7, 23).unwrap().armenian().to_string(), "亚美尼亚1473年13月5日");
    assert_eq!(Armenian::from_ymd(1474, 1, 1).unwrap().solar().to_ymd(), "2024-07-24");
}

#[test]
fn armenian_year_and_month_layers_are_available() {
    let year = ArmenianYear::from_year(1474).unwrap();
    let month = ArmenianMonth::from_ym(1474, 13).unwrap();

    assert!(!year.is_leap());
    assert_eq!(year.day_count(), 365);
    assert_eq!(year.first_month().to_string(), "亚美尼亚1474年1月");
    assert_eq!(year.last_month().to_string(), "亚美尼亚1474年13月");
    assert_eq!(year.first_day().to_string(), "亚美尼亚1474年1月1日");
    assert_eq!(year.months().len(), 13);
    assert!(year.contains_solar(Solar::from_ymd(2025, 1, 1).unwrap()));

    assert_eq!(month.day_count(), 5);
    assert_eq!(month.first_day().to_string(), "亚美尼亚1474年13月1日");
    assert_eq!(month.last_day().to_string(), "亚美尼亚1474年13月5日");
    assert_eq!(month.days().len(), 5);
}

#[test]
fn armenian_next_and_subtract_follow_solar_days() {
    let a = Armenian::from_ymd(1474, 1, 1).unwrap();
    let b = a.next(10);
    assert_eq!(b.to_string(), "亚美尼亚1474年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = ArmenianMonth::from_ym(1474, 13).unwrap();
    assert_eq!(month.next(1).unwrap().to_string(), "亚美尼亚1475年1月");
}

#[test]
fn solar_can_resolve_armenian_companions() {
    let solar = Solar::from_ymd(2024, 7, 24).unwrap();
    assert_eq!(solar.armenian().to_string(), "亚美尼亚1474年1月1日");
    assert_eq!(solar.armenian_year().year(), 1474);
    assert_eq!(solar.armenian_month().month(), 1);
}

#[test]
fn armenian_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().armenian();
    let month = day.armenian_month();
    let year = day.armenian_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 1).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
