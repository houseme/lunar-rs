use lunar_rs::{
    CalendarDay, CalendarSpan, EventKind, EventQuery, HispanicEra, HispanicEraMonth, HispanicEraYear, Solar,
};

#[test]
fn hispanic_era_round_trip_core_examples() {
    let solar = Solar::from_ymd(2024, 1, 1).unwrap();
    let era = solar.hispanic_era().unwrap();
    assert_eq!(era.to_string(), "西班牙纪元2062年1月1日");
    assert_eq!(era.solar().to_ymd(), "2024-01-01");

    let start = Solar::from_ymd(1, 1, 1).unwrap().hispanic_era().unwrap();
    assert_eq!(start.to_string(), "西班牙纪元39年1月1日");
    assert_eq!(HispanicEra::from_ymd(39, 1, 1).unwrap().solar().to_ymd(), "0001-01-01");
}

#[test]
fn hispanic_era_year_and_month_layers_are_available() {
    let year = HispanicEraYear::from_year(2062).unwrap();
    let month = HispanicEraMonth::from_ym(2062, 2).unwrap();

    assert_eq!(year.solar_year(), 2024);
    assert!(year.is_leap());
    assert_eq!(year.day_count(), 366);
    assert_eq!(year.first_month().to_string(), "西班牙纪元2062年1月");
    assert_eq!(year.last_month().to_string(), "西班牙纪元2062年12月");
    assert_eq!(year.first_day().to_string(), "西班牙纪元2062年1月1日");
    assert_eq!(year.last_day().to_string(), "西班牙纪元2062年12月31日");
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
fn hispanic_era_next_and_subtract_follow_solar_days() {
    let a = HispanicEra::from_ymd(2062, 1, 1).unwrap();
    let b = a.next(10).unwrap();
    assert_eq!(b.to_string(), "西班牙纪元2062年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = HispanicEraMonth::from_ym(2062, 12).unwrap();
    assert_eq!(month.next(1).unwrap().to_string(), "西班牙纪元2063年1月");
}

#[test]
fn solar_can_resolve_hispanic_era_companions() {
    let solar = Solar::from_ymd(2024, 2, 10).unwrap();
    assert_eq!(solar.hispanic_era().unwrap().to_string(), "西班牙纪元2062年2月10日");
    assert_eq!(solar.hispanic_era_year().unwrap().year(), 2062);
    assert_eq!(solar.hispanic_era_month().unwrap().month(), 2);
}

#[test]
fn hispanic_era_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().hispanic_era().unwrap();
    let month = HispanicEraMonth::from_ym(2062, 1).unwrap();
    let year = HispanicEraYear::from_year(2062).unwrap();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
