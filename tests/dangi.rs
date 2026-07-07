use lunar_rs::{CalendarDay, CalendarSpan, Dangi, DangiMonth, DangiYear, EventKind, EventQuery, Solar};

#[test]
fn dangi_round_trip_core_examples() {
    let solar = Solar::from_ymd(2024, 1, 1).unwrap();
    let dangi = solar.dangi().unwrap();
    assert_eq!(dangi.to_string(), "檀纪4357年1月1日");
    assert_eq!(dangi.solar().to_ymd(), "2024-01-01");

    let ancient = Solar::from_ymd(1, 1, 1).unwrap().dangi().unwrap();
    assert_eq!(ancient.to_string(), "檀纪2334年1月1日");
    assert_eq!(Dangi::from_ymd(2334, 1, 1).unwrap().solar().to_ymd(), "0001-01-01");
}

#[test]
fn dangi_year_and_month_layers_are_available() {
    let year = DangiYear::from_year(4357).unwrap();
    let month = DangiMonth::from_ym(4357, 2).unwrap();

    assert_eq!(year.solar_year(), 2024);
    assert!(year.is_leap());
    assert_eq!(year.day_count(), 366);
    assert_eq!(year.first_month().to_string(), "檀纪4357年1月");
    assert_eq!(year.last_month().to_string(), "檀纪4357年12月");
    assert_eq!(year.first_day().to_string(), "檀纪4357年1月1日");
    assert_eq!(year.last_day().to_string(), "檀纪4357年12月31日");
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
fn dangi_next_and_subtract_follow_solar_days() {
    let a = Dangi::from_ymd(4357, 1, 1).unwrap();
    let b = a.next(10).unwrap();
    assert_eq!(b.to_string(), "檀纪4357年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = DangiMonth::from_ym(4357, 12).unwrap();
    assert_eq!(month.next(1).unwrap().to_string(), "檀纪4358年1月");
}

#[test]
fn solar_can_resolve_dangi_companions() {
    let solar = Solar::from_ymd(2024, 2, 10).unwrap();
    assert_eq!(solar.dangi().unwrap().to_string(), "檀纪4357年2月10日");
    assert_eq!(solar.dangi_year().unwrap().year(), 4357);
    assert_eq!(solar.dangi_month().unwrap().month(), 2);
}

#[test]
fn dangi_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().dangi().unwrap();
    let month = DangiMonth::from_ym(4357, 1).unwrap();
    let year = DangiYear::from_year(4357).unwrap();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
