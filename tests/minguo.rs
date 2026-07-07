use lunar_rs::{CalendarDay, CalendarSpan, EventKind, EventQuery, Minguo, MinguoMonth, MinguoYear, Solar};

#[test]
fn minguo_round_trip_core_examples() {
    let solar = Solar::from_ymd(2024, 1, 1).unwrap();
    let minguo = solar.minguo();
    assert_eq!(minguo.to_string(), "民国113年1月1日");
    assert_eq!(minguo.solar().to_ymd(), "2024-01-01");

    let pre_epoch = Solar::from_ymd(1911, 10, 10).unwrap().minguo();
    assert_eq!(pre_epoch.to_string(), "民国0年10月10日");
    assert_eq!(Minguo::from_ymd(0, 10, 10).unwrap().solar().to_ymd(), "1911-10-10");
}

#[test]
fn minguo_year_and_month_layers_are_available() {
    let year = MinguoYear::from_year(113);
    let month = MinguoMonth::from_ym(113, 2).unwrap();

    assert_eq!(year.solar_year(), 2024);
    assert!(year.is_leap());
    assert_eq!(year.day_count(), 366);
    assert_eq!(year.first_month().to_string(), "民国113年1月");
    assert_eq!(year.last_month().to_string(), "民国113年12月");
    assert_eq!(year.first_day().to_string(), "民国113年1月1日");
    assert_eq!(year.last_day().to_string(), "民国113年12月31日");
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
fn minguo_next_and_subtract_follow_solar_days() {
    let a = Minguo::from_ymd(113, 1, 1).unwrap();
    let b = a.next(10);
    assert_eq!(b.to_string(), "民国113年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = MinguoMonth::from_ym(113, 12).unwrap();
    assert_eq!(month.next(1).to_string(), "民国114年1月");
}

#[test]
fn solar_can_resolve_minguo_companions() {
    let solar = Solar::from_ymd(2024, 2, 10).unwrap();
    assert_eq!(solar.minguo().to_string(), "民国113年2月10日");
    assert_eq!(solar.minguo_year().year(), 113);
    assert_eq!(solar.minguo_month().month(), 2);
}

#[test]
fn minguo_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().minguo();
    let month = MinguoMonth::from_ym(113, 1).unwrap();
    let year = MinguoYear::from_year(113);

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert!(CalendarDay::solar(&day).to_ymd() == "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
