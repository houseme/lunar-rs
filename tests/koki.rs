use lunar_rs::{CalendarDay, CalendarSpan, EventKind, EventQuery, Koki, KokiMonth, KokiYear, Solar};

#[test]
fn koki_round_trip_core_examples() {
    let solar = Solar::from_ymd(2026, 1, 1).unwrap();
    let koki = solar.koki().unwrap();
    assert_eq!(koki.to_string(), "皇纪2686年1月1日");
    assert_eq!(koki.solar().to_ymd(), "2026-01-01");

    assert_eq!(Solar::from_ymd(1940, 1, 1).unwrap().koki().unwrap().to_string(), "皇纪2600年1月1日");
    assert_eq!(Koki::from_ymd(2686, 1, 1).unwrap().solar().to_ymd(), "2026-01-01");
}

#[test]
fn koki_year_and_month_layers_are_available() {
    let year = KokiYear::from_year(2686).unwrap();
    let month = KokiMonth::from_ym(2686, 2).unwrap();

    assert_eq!(year.solar_year(), 2026);
    assert!(!year.is_leap());
    assert_eq!(year.day_count(), 365);
    assert_eq!(year.first_month().to_string(), "皇纪2686年1月");
    assert_eq!(year.last_month().to_string(), "皇纪2686年12月");
    assert_eq!(year.first_day().to_string(), "皇纪2686年1月1日");
    assert_eq!(year.last_day().to_string(), "皇纪2686年12月31日");
    assert_eq!(year.months().len(), 12);
    assert!(year.contains_solar(Solar::from_ymd(2026, 6, 1).unwrap()));

    assert_eq!(month.solar_year(), 2026);
    assert_eq!(month.day_count(), 28);
    assert_eq!(month.first_solar_day().to_ymd(), "2026-02-01");
    assert_eq!(month.last_solar_day().to_ymd(), "2026-02-28");
    assert_eq!(month.days().len(), 28);
}

#[test]
fn koki_next_and_subtract_follow_solar_days() {
    let a = Koki::from_ymd(2686, 1, 1).unwrap();
    let b = a.next(10).unwrap();
    assert_eq!(b.to_string(), "皇纪2686年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = KokiMonth::from_ym(2686, 12).unwrap();
    assert_eq!(month.next(1).unwrap().to_string(), "皇纪2687年1月");
}

#[test]
fn solar_can_resolve_koki_companions() {
    let solar = Solar::from_ymd(2026, 2, 11).unwrap();
    assert_eq!(solar.koki().unwrap().to_string(), "皇纪2686年2月11日");
    assert_eq!(solar.koki_year().unwrap().year(), 2686);
    assert_eq!(solar.koki_month().unwrap().month(), 2);
}

#[test]
fn koki_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().koki().unwrap();
    let month = day.koki_month();
    let year = day.koki_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
