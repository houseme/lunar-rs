use lunar_rs::{CalendarDay, CalendarSpan, EventKind, EventQuery, Julian, JulianMonth, JulianYear, Solar};

#[test]
fn julian_round_trip_core_examples() {
    let solar = Solar::from_ymd(2024, 1, 1).unwrap();
    let julian = solar.julian_calendar();
    assert_eq!(julian.to_string(), "儒略历2023年12月19日");
    assert_eq!(julian.solar().to_ymd(), "2024-01-01");

    let reform = Solar::from_ymd(1582, 10, 15).unwrap().julian_calendar();
    assert_eq!(reform.to_string(), "儒略历1582年10月5日");
    assert_eq!(Julian::from_ymd(1582, 10, 5).unwrap().solar().to_ymd(), "1582-10-15");
}

#[test]
fn julian_year_and_month_layers_are_available() {
    let year = JulianYear::from_year(2024);
    let month = JulianMonth::from_ym(2024, 2).unwrap();

    assert!(year.is_leap());
    assert_eq!(year.day_count(), 366);
    assert_eq!(year.first_month().to_string(), "儒略历2024年1月");
    assert_eq!(year.last_month().to_string(), "儒略历2024年12月");
    assert_eq!(year.first_day().to_string(), "儒略历2024年1月1日");
    assert_eq!(year.last_day().to_string(), "儒略历2024年12月31日");
    assert_eq!(year.months().len(), 12);

    assert_eq!(month.day_count(), 29);
    assert_eq!(month.first_day().to_string(), "儒略历2024年2月1日");
    assert_eq!(month.last_day().to_string(), "儒略历2024年2月29日");
    assert_eq!(month.days().len(), 29);
}

#[test]
fn julian_next_and_subtract_follow_solar_days() {
    let a = Julian::from_ymd(2023, 12, 19).unwrap();
    let b = a.next(1);
    assert_eq!(b.to_string(), "儒略历2023年12月20日");
    assert_eq!(b.subtract(a), 1);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = JulianMonth::from_ym(2023, 12).unwrap();
    assert_eq!(month.next(1).to_string(), "儒略历2024年1月");
}

#[test]
fn solar_can_resolve_julian_companions() {
    let solar = Solar::from_ymd(2024, 2, 10).unwrap();
    assert_eq!(solar.julian_calendar().to_string(), "儒略历2024年1月28日");
    assert_eq!(solar.julian_calendar_year().year(), 2024);
    assert_eq!(solar.julian_calendar_month().month(), 1);
}

#[test]
fn julian_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().julian_calendar();
    let month = Solar::from_ymd(2024, 1, 1).unwrap().julian_calendar_month();
    let year = Solar::from_ymd(2024, 1, 1).unwrap().julian_calendar_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 10).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
