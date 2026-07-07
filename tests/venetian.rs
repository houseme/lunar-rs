use lunar_rs::{CalendarDay, CalendarSpan, EventKind, EventQuery, Solar, Venetian, VenetianMonth, VenetianYear};

#[test]
fn venetian_round_trip_core_examples() {
    assert_eq!(Solar::from_ymd(2024, 2, 29).unwrap().venetian().unwrap().to_string(), "威尼斯2023年2月29日");
    assert_eq!(Solar::from_ymd(2024, 3, 1).unwrap().venetian().unwrap().to_string(), "威尼斯2024年3月1日");
    assert_eq!(Venetian::from_ymd(2023, 2, 29).unwrap().solar().to_ymd(), "2024-02-29");
    assert_eq!(Venetian::from_ymd(2024, 3, 1).unwrap().solar().to_ymd(), "2024-03-01");
}

#[test]
fn venetian_year_and_month_layers_are_available() {
    let year = VenetianYear::from_year(2024).unwrap();
    let month = VenetianMonth::from_ym(2023, 2).unwrap();

    assert_eq!(year.first_day().solar().to_ymd(), "2024-03-01");
    assert_eq!(year.last_day().solar().to_ymd(), "2025-02-28");
    assert_eq!(year.months().len(), 12);
    assert!(year.contains_solar(Solar::from_ymd(2024, 12, 31).unwrap()));

    assert_eq!(month.solar_year(), 2024);
    assert_eq!(month.day_count(), 29);
    assert_eq!(month.first_solar_day().to_ymd(), "2024-02-01");
    assert_eq!(month.last_solar_day().to_ymd(), "2024-02-29");
    assert_eq!(month.days().len(), 29);
}

#[test]
fn venetian_next_and_subtract_follow_solar_days_and_year_boundary() {
    let last = Solar::from_ymd(2024, 2, 29).unwrap().venetian().unwrap();
    let next = last.next(1).unwrap();
    assert_eq!(next.to_string(), "威尼斯2024年3月1日");
    assert_eq!(next.subtract(last), 1);

    let february = VenetianMonth::from_ym(2023, 2).unwrap();
    assert_eq!(february.next(1).unwrap().to_string(), "威尼斯2024年3月");
}

#[test]
fn solar_can_resolve_venetian_companions() {
    let solar = Solar::from_ymd(2024, 2, 29).unwrap();
    assert_eq!(solar.venetian().unwrap().to_string(), "威尼斯2023年2月29日");
    assert_eq!(solar.venetian_year().unwrap().year(), 2023);
    assert_eq!(solar.venetian_month().unwrap().month(), 2);
}

#[test]
fn venetian_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 5, 1).unwrap().venetian().unwrap();
    let month = day.venetian_month();
    let year = day.venetian_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-05-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 5, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
