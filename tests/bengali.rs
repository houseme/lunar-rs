use lunar_rs::{Bengali, BengaliMonth, BengaliYear, CalendarDay, CalendarSpan, EventKind, EventQuery, Solar};

#[test]
fn bengali_round_trip_core_examples() {
    let solar = Solar::from_ymd(2024, 4, 14).unwrap();
    let bengali = solar.bengali().unwrap();
    assert_eq!(bengali.to_string(), "孟加拉1431年1月1日");
    assert_eq!(bengali.solar().to_ymd(), "2024-04-14");

    assert_eq!(Solar::from_ymd(2024, 4, 13).unwrap().bengali().unwrap().to_string(), "孟加拉1430年12月30日");
    assert_eq!(Bengali::from_ymd(1431, 1, 1).unwrap().solar().to_ymd(), "2024-04-14");
}

#[test]
fn bengali_year_and_month_layers_are_available() {
    let year = BengaliYear::from_year(1431).unwrap();
    let month = BengaliMonth::from_ym(1431, 1).unwrap();

    assert_eq!(year.solar_year(), 2024);
    assert_eq!(year.day_count(), 365);
    assert_eq!(year.first_month().to_string(), "孟加拉1431年1月");
    assert_eq!(year.last_month().to_string(), "孟加拉1431年12月");
    assert_eq!(year.first_day().to_string(), "孟加拉1431年1月1日");
    assert_eq!(year.last_day().to_string(), "孟加拉1431年12月30日");
    assert_eq!(year.months().len(), 12);
    assert!(year.contains_solar(Solar::from_ymd(2024, 8, 1).unwrap()));

    assert_eq!(month.solar_year(), 2024);
    assert_eq!(month.day_count(), 31);
    assert_eq!(month.first_solar_day().to_ymd(), "2024-04-14");
    assert_eq!(month.last_solar_day().to_ymd(), "2024-05-14");
    assert_eq!(month.days().len(), 31);
}

#[test]
fn bengali_next_and_subtract_follow_solar_days() {
    let a = Bengali::from_ymd(1431, 1, 1).unwrap();
    let b = a.next(10).unwrap();
    assert_eq!(b.to_string(), "孟加拉1431年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = BengaliMonth::from_ym(1431, 12).unwrap();
    assert_eq!(month.next(1).unwrap().to_string(), "孟加拉1432年1月");
}

#[test]
fn solar_can_resolve_bengali_companions() {
    let solar = Solar::from_ymd(2024, 4, 14).unwrap();
    assert_eq!(solar.bengali().unwrap().to_string(), "孟加拉1431年1月1日");
    assert_eq!(solar.bengali_year().unwrap().year(), 1431);
    assert_eq!(solar.bengali_month().unwrap().month(), 1);
}

#[test]
fn bengali_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().bengali().unwrap();
    let month = day.bengali_month();
    let year = day.bengali_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 1).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
