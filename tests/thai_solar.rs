use lunar_rs::{CalendarDay, CalendarSpan, EventKind, EventQuery, Solar, ThaiSolar, ThaiSolarMonth, ThaiSolarYear};

#[test]
fn thai_solar_round_trip_core_examples() {
    let solar = Solar::from_ymd(2024, 1, 1).unwrap();
    let thai = solar.thai_solar();
    assert_eq!(thai.to_string(), "泰历2567年1月1日");
    assert_eq!(thai.solar().to_ymd(), "2024-01-01");

    let old = Solar::from_ymd(1, 1, 1).unwrap().thai_solar();
    assert_eq!(old.to_string(), "泰历544年1月1日");
    assert_eq!(ThaiSolar::from_ymd(544, 1, 1).unwrap().solar().to_ymd(), "0001-01-01");
}

#[test]
fn thai_solar_year_and_month_layers_are_available() {
    let year = ThaiSolarYear::from_year(2567);
    let month = ThaiSolarMonth::from_ym(2567, 2).unwrap();

    assert_eq!(year.solar_year(), 2024);
    assert!(year.is_leap());
    assert_eq!(year.day_count(), 366);
    assert_eq!(year.first_month().to_string(), "泰历2567年1月");
    assert_eq!(year.last_month().to_string(), "泰历2567年12月");
    assert_eq!(year.first_day().to_string(), "泰历2567年1月1日");
    assert_eq!(year.last_day().to_string(), "泰历2567年12月31日");
    assert_eq!(year.months().len(), 12);
    assert!(year.contains_solar(Solar::from_ymd(2024, 2, 10).unwrap()));

    assert_eq!(month.solar_year(), 2024);
    assert_eq!(month.day_count(), 29);
    assert_eq!(month.first_solar_day().to_ymd(), "2024-02-01");
    assert_eq!(month.last_solar_day().to_ymd(), "2024-02-29");
    assert_eq!(month.days().len(), 29);
    assert!(month.contains_solar(Solar::from_ymd(2024, 2, 10).unwrap()));
}

#[test]
fn thai_solar_next_and_subtract_follow_solar_days() {
    let a = ThaiSolar::from_ymd(2567, 1, 1).unwrap();
    let b = a.next(10);
    assert_eq!(b.to_string(), "泰历2567年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = ThaiSolarMonth::from_ym(2567, 12).unwrap();
    assert_eq!(month.next(1).to_string(), "泰历2568年1月");
}

#[test]
fn solar_can_resolve_thai_solar_companions() {
    let solar = Solar::from_ymd(2024, 2, 10).unwrap();
    assert_eq!(solar.thai_solar().to_string(), "泰历2567年2月10日");
    assert_eq!(solar.thai_solar_year().year(), 2567);
    assert_eq!(solar.thai_solar_month().month(), 2);
}

#[test]
fn thai_solar_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().thai_solar();
    let month = ThaiSolarMonth::from_ym(2567, 1).unwrap();
    let year = ThaiSolarYear::from_year(2567);

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
