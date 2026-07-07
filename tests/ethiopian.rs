use lunar_rs::{CalendarDay, CalendarSpan, Ethiopian, EthiopianMonth, EthiopianYear, EventKind, EventQuery, Solar};

#[test]
fn ethiopian_round_trip_core_examples() {
    let solar = Solar::from_ymd(2024, 9, 11).unwrap();
    let ethiopian = solar.ethiopian();
    assert_eq!(ethiopian.to_string(), "埃塞2017年1月1日");
    assert_eq!(ethiopian.solar().to_ymd(), "2024-09-11");

    assert_eq!(Solar::from_ymd(2024, 9, 10).unwrap().ethiopian().to_string(), "埃塞2016年13月5日");
    assert_eq!(Ethiopian::from_ymd(2017, 1, 1).unwrap().solar().to_ymd(), "2024-09-11");
}

#[test]
fn ethiopian_year_and_month_layers_are_available() {
    let year = EthiopianYear::from_year(2017).unwrap();
    let month = EthiopianMonth::from_ym(2017, 13).unwrap();

    assert!(!year.is_leap());
    assert_eq!(year.day_count(), 365);
    assert_eq!(year.first_month().to_string(), "埃塞2017年1月");
    assert_eq!(year.last_month().to_string(), "埃塞2017年13月");
    assert_eq!(year.first_day().to_string(), "埃塞2017年1月1日");
    assert_eq!(year.months().len(), 13);
    assert!(year.contains_solar(Solar::from_ymd(2025, 1, 1).unwrap()));

    assert_eq!(month.day_count(), 5);
    assert_eq!(month.first_day().to_string(), "埃塞2017年13月1日");
    assert_eq!(month.last_day().to_string(), "埃塞2017年13月5日");
    assert_eq!(month.days().len(), 5);
}

#[test]
fn ethiopian_next_and_subtract_follow_solar_days() {
    let a = Ethiopian::from_ymd(2017, 1, 1).unwrap();
    let b = a.next(10);
    assert_eq!(b.to_string(), "埃塞2017年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = EthiopianMonth::from_ym(2017, 13).unwrap();
    assert_eq!(month.next(1).unwrap().to_string(), "埃塞2018年1月");
}

#[test]
fn solar_can_resolve_ethiopian_companions() {
    let solar = Solar::from_ymd(2024, 9, 11).unwrap();
    assert_eq!(solar.ethiopian().to_string(), "埃塞2017年1月1日");
    assert_eq!(solar.ethiopian_year().year(), 2017);
    assert_eq!(solar.ethiopian_month().month(), 1);
}

#[test]
fn ethiopian_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().ethiopian();
    let month = day.ethiopian_month();
    let year = day.ethiopian_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 1).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
