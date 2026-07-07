use lunar_rs::{Assyrian, AssyrianMonth, AssyrianYear, CalendarDay, CalendarSpan, EventKind, EventQuery, Solar};

#[test]
fn assyrian_round_trip_core_examples() {
    assert_eq!(Solar::from_ymd(2024, 3, 31).unwrap().assyrian().unwrap().to_string(), "亚述6773年3月31日");
    assert_eq!(Solar::from_ymd(2024, 4, 1).unwrap().assyrian().unwrap().to_string(), "亚述6774年4月1日");
    assert_eq!(Assyrian::from_ymd(6773, 3, 31).unwrap().solar().to_ymd(), "2024-03-31");
    assert_eq!(Assyrian::from_ymd(6774, 4, 1).unwrap().solar().to_ymd(), "2024-04-01");
}

#[test]
fn assyrian_year_and_month_layers_are_available() {
    let year = AssyrianYear::from_year(6774).unwrap();
    let month = AssyrianMonth::from_ym(6774, 4).unwrap();

    assert_eq!(year.first_day().solar().to_ymd(), "2024-04-01");
    assert_eq!(year.last_day().solar().to_ymd(), "2025-03-31");
    assert_eq!(year.months().len(), 12);
    assert!(year.contains_solar(Solar::from_ymd(2025, 1, 1).unwrap()));

    assert_eq!(month.solar_year(), 2024);
    assert_eq!(month.day_count(), 30);
    assert_eq!(month.first_solar_day().to_ymd(), "2024-04-01");
    assert_eq!(month.last_solar_day().to_ymd(), "2024-04-30");
    assert_eq!(month.days().len(), 30);
}

#[test]
fn assyrian_next_and_subtract_follow_solar_days_and_year_boundary() {
    let last = Solar::from_ymd(2024, 3, 31).unwrap().assyrian().unwrap();
    let next = last.next(1).unwrap();
    assert_eq!(next.to_string(), "亚述6774年4月1日");
    assert_eq!(next.subtract(last), 1);

    let march = AssyrianMonth::from_ym(6773, 3).unwrap();
    assert_eq!(march.next(1).unwrap().to_string(), "亚述6774年4月");
}

#[test]
fn solar_can_resolve_assyrian_companions() {
    let solar = Solar::from_ymd(2024, 4, 1).unwrap();
    assert_eq!(solar.assyrian().unwrap().to_string(), "亚述6774年4月1日");
    assert_eq!(solar.assyrian_year().unwrap().year(), 6774);
    assert_eq!(solar.assyrian_month().unwrap().month(), 4);
}

#[test]
fn assyrian_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().assyrian().unwrap();
    let month = Solar::from_ymd(2024, 1, 1).unwrap().assyrian_month().unwrap();
    let year = Solar::from_ymd(2024, 1, 1).unwrap().assyrian_year().unwrap();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
