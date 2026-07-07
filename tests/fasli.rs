use lunar_rs::{CalendarDay, CalendarSpan, EventKind, EventQuery, Fasli, FasliMonth, FasliYear, Solar};

#[test]
fn fasli_round_trip_core_examples() {
    let solar = Solar::from_ymd(2000, 3, 21).unwrap();
    let fasli = solar.fasli().unwrap();
    assert_eq!(fasli.to_string(), "法斯里1370年1月1日");
    assert_eq!(fasli.solar().to_ymd(), "2000-03-21");

    assert_eq!(Solar::from_ymd(2001, 3, 20).unwrap().fasli().unwrap().to_string(), "法斯里1370年13月5日");
    assert_eq!(Fasli::from_ymd(1370, 1, 1).unwrap().solar().to_ymd(), "2000-03-21");
}

#[test]
fn fasli_year_and_month_layers_are_available() {
    let year = FasliYear::from_year(1370).unwrap();
    let month = FasliMonth::from_ym(1370, 13).unwrap();

    assert_eq!(year.solar_year(), 2000);
    assert!(!year.is_leap());
    assert_eq!(year.day_count(), 365);
    assert_eq!(year.first_month().to_string(), "法斯里1370年1月");
    assert_eq!(year.last_month().to_string(), "法斯里1370年13月");
    assert_eq!(year.first_day().to_string(), "法斯里1370年1月1日");
    assert_eq!(year.last_day().to_string(), "法斯里1370年13月5日");
    assert_eq!(year.months().len(), 13);
    assert!(year.contains_solar(Solar::from_ymd(2000, 8, 1).unwrap()));

    assert_eq!(month.day_count(), 5);
    assert_eq!(month.first_solar_day().to_ymd(), "2001-03-16");
    assert_eq!(month.last_solar_day().to_ymd(), "2001-03-20");
    assert_eq!(month.days().len(), 5);
}

#[test]
fn fasli_next_and_subtract_follow_solar_days() {
    let a = Fasli::from_ymd(1370, 1, 1).unwrap();
    let b = a.next(10).unwrap();
    assert_eq!(b.to_string(), "法斯里1370年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = FasliMonth::from_ym(1370, 13).unwrap();
    assert_eq!(month.next(1).unwrap().to_string(), "法斯里1371年1月");
}

#[test]
fn solar_can_resolve_fasli_companions() {
    let solar = Solar::from_ymd(2000, 3, 21).unwrap();
    assert_eq!(solar.fasli().unwrap().to_string(), "法斯里1370年1月1日");
    assert_eq!(solar.fasli_year().unwrap().year(), 1370);
    assert_eq!(solar.fasli_month().unwrap().month(), 1);
}

#[test]
fn fasli_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().fasli().unwrap();
    let month = day.fasli_month();
    let year = day.fasli_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 1).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
