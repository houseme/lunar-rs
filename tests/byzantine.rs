use lunar_rs::{Byzantine, ByzantineMonth, ByzantineYear, CalendarDay, CalendarSpan, EventKind, EventQuery, Solar};

#[test]
fn byzantine_round_trip_core_examples() {
    assert_eq!(Solar::from_ymd(2024, 1, 1).unwrap().byzantine().unwrap().to_string(), "拜占庭7532年1月1日");
    assert_eq!(Solar::from_ymd(2024, 9, 1).unwrap().byzantine().unwrap().to_string(), "拜占庭7533年9月1日");
    assert_eq!(Byzantine::from_ymd(7532, 1, 1).unwrap().solar().to_ymd(), "2024-01-01");
    assert_eq!(Byzantine::from_ymd(7533, 9, 1).unwrap().solar().to_ymd(), "2024-09-01");
}

#[test]
fn byzantine_year_and_month_layers_are_available() {
    let year = ByzantineYear::from_year(7533).unwrap();
    let month = ByzantineMonth::from_ym(7533, 9).unwrap();

    assert_eq!(year.first_day().solar().to_ymd(), "2024-09-01");
    assert_eq!(year.last_day().solar().to_ymd(), "2025-08-31");
    assert_eq!(year.months().len(), 12);
    assert!(year.contains_solar(Solar::from_ymd(2025, 3, 1).unwrap()));

    assert_eq!(month.solar_year(), 2024);
    assert_eq!(month.day_count(), 30);
    assert_eq!(month.first_solar_day().to_ymd(), "2024-09-01");
    assert_eq!(month.last_solar_day().to_ymd(), "2024-09-30");
    assert_eq!(month.days().len(), 30);
}

#[test]
fn byzantine_next_and_subtract_follow_solar_days_and_year_boundary() {
    let last = Solar::from_ymd(2024, 8, 31).unwrap().byzantine().unwrap();
    let next = last.next(1);
    assert_eq!(next.to_string(), "拜占庭7533年9月1日");
    assert_eq!(next.subtract(last), 1);

    let august = ByzantineMonth::from_ym(7532, 8).unwrap();
    assert_eq!(august.next(1).unwrap().to_string(), "拜占庭7533年9月");
}

#[test]
fn solar_can_resolve_byzantine_companions() {
    let solar = Solar::from_ymd(2024, 9, 1).unwrap();
    assert_eq!(solar.byzantine().unwrap().to_string(), "拜占庭7533年9月1日");
    assert_eq!(solar.byzantine_year().unwrap().year(), 7533);
    assert_eq!(solar.byzantine_month().unwrap().month(), 9);
}

#[test]
fn byzantine_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().byzantine().unwrap();
    let month = Solar::from_ymd(2024, 1, 1).unwrap().byzantine_month().unwrap();
    let year = Solar::from_ymd(2024, 1, 1).unwrap().byzantine_year().unwrap();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
