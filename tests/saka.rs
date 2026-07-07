use lunar_rs::{CalendarDay, CalendarSpan, EventKind, EventQuery, Saka, SakaMonth, SakaYear, Solar};

#[test]
fn saka_round_trip_core_examples() {
    let solar = Solar::from_ymd(2024, 3, 21).unwrap();
    let saka = solar.saka().unwrap();
    assert_eq!(saka.to_string(), "萨卡1946年1月1日");
    assert_eq!(saka.solar().to_ymd(), "2024-03-21");

    assert_eq!(Solar::from_ymd(2024, 3, 20).unwrap().saka().unwrap().to_string(), "萨卡1945年12月30日");
    assert_eq!(Saka::from_ymd(1946, 1, 1).unwrap().solar().to_ymd(), "2024-03-21");
}

#[test]
fn saka_year_and_month_layers_are_available() {
    let year = SakaYear::from_year(1946).unwrap();
    let month = SakaMonth::from_ym(1946, 1).unwrap();

    assert_eq!(year.solar_year(), 2024);
    assert!(year.is_leap());
    assert_eq!(year.day_count(), 366);
    assert_eq!(year.first_month().to_string(), "萨卡1946年1月");
    assert_eq!(year.last_month().to_string(), "萨卡1946年12月");
    assert_eq!(year.first_day().to_string(), "萨卡1946年1月1日");
    assert_eq!(year.last_day().to_string(), "萨卡1946年12月30日");
    assert_eq!(year.months().len(), 12);
    assert!(year.contains_solar(Solar::from_ymd(2024, 8, 23).unwrap()));

    assert_eq!(month.solar_year(), 2024);
    assert_eq!(month.day_count(), 31);
    assert_eq!(month.first_solar_day().to_ymd(), "2024-03-21");
    assert_eq!(month.last_solar_day().to_ymd(), "2024-04-20");
    assert_eq!(month.days().len(), 31);
}

#[test]
fn saka_next_and_subtract_follow_solar_days() {
    let a = Saka::from_ymd(1946, 1, 1).unwrap();
    let b = a.next(10).unwrap();
    assert_eq!(b.to_string(), "萨卡1946年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = SakaMonth::from_ym(1946, 12).unwrap();
    assert_eq!(month.next(1).unwrap().to_string(), "萨卡1947年1月");
}

#[test]
fn solar_can_resolve_saka_companions() {
    let solar = Solar::from_ymd(2024, 3, 21).unwrap();
    assert_eq!(solar.saka().unwrap().to_string(), "萨卡1946年1月1日");
    assert_eq!(solar.saka_year().unwrap().year(), 1946);
    assert_eq!(solar.saka_month().unwrap().month(), 1);
}

#[test]
fn saka_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().saka().unwrap();
    let month = day.saka_month();
    let year = day.saka_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
