use lunar_rs::{CalendarDay, CalendarSpan, Coptic, CopticMonth, CopticYear, EventKind, EventQuery, Solar};

#[test]
fn coptic_round_trip_core_examples() {
    let solar = Solar::from_ymd(2024, 9, 11).unwrap();
    let coptic = solar.coptic();
    assert_eq!(coptic.to_string(), "科普特1741年1月1日");
    assert_eq!(coptic.solar().to_ymd(), "2024-09-11");

    assert_eq!(Solar::from_ymd(2024, 9, 10).unwrap().coptic().to_string(), "科普特1740年13月5日");
    assert_eq!(Coptic::from_ymd(1741, 1, 1).unwrap().solar().to_ymd(), "2024-09-11");
}

#[test]
fn coptic_year_and_month_layers_are_available() {
    let year = CopticYear::from_year(1741).unwrap();
    let month = CopticMonth::from_ym(1741, 13).unwrap();

    assert!(!year.is_leap());
    assert_eq!(year.day_count(), 365);
    assert_eq!(year.first_month().to_string(), "科普特1741年1月");
    assert_eq!(year.last_month().to_string(), "科普特1741年13月");
    assert_eq!(year.first_day().to_string(), "科普特1741年1月1日");
    assert_eq!(year.months().len(), 13);
    assert!(year.contains_solar(Solar::from_ymd(2025, 1, 1).unwrap()));

    assert_eq!(month.day_count(), 5);
    assert_eq!(month.first_day().to_string(), "科普特1741年13月1日");
    assert_eq!(month.last_day().to_string(), "科普特1741年13月5日");
    assert_eq!(month.days().len(), 5);
}

#[test]
fn coptic_next_and_subtract_follow_solar_days() {
    let a = Coptic::from_ymd(1741, 1, 1).unwrap();
    let b = a.next(10);
    assert_eq!(b.to_string(), "科普特1741年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = CopticMonth::from_ym(1741, 13).unwrap();
    assert_eq!(month.next(1).unwrap().to_string(), "科普特1742年1月");
}

#[test]
fn solar_can_resolve_coptic_companions() {
    let solar = Solar::from_ymd(2024, 9, 11).unwrap();
    assert_eq!(solar.coptic().to_string(), "科普特1741年1月1日");
    assert_eq!(solar.coptic_year().year(), 1741);
    assert_eq!(solar.coptic_month().month(), 1);
}

#[test]
fn coptic_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().coptic();
    let month = day.coptic_month();
    let year = day.coptic_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 1).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
