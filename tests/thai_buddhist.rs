use lunar_rs::{
    CalendarDay, CalendarSpan, EventKind, EventQuery, Solar, ThaiBuddhist, ThaiBuddhistMonth, ThaiBuddhistYear,
};

#[test]
fn thai_buddhist_round_trip_core_examples() {
    assert_eq!(Solar::from_ymd(1912, 4, 1).unwrap().thai_buddhist().unwrap().to_string(), "泰佛历2455年4月1日");
    assert_eq!(Solar::from_ymd(1913, 3, 31).unwrap().thai_buddhist().unwrap().to_string(), "泰佛历2455年3月31日");
    assert_eq!(ThaiBuddhist::from_ymd(2455, 4, 1).unwrap().solar().to_ymd(), "1912-04-01");
    assert_eq!(ThaiBuddhist::from_ymd(2455, 3, 31).unwrap().solar().to_ymd(), "1913-03-31");
}

#[test]
fn thai_buddhist_year_and_month_layers_are_available() {
    let year = ThaiBuddhistYear::from_year(2455).unwrap();
    let month = ThaiBuddhistMonth::from_ym(2455, 4).unwrap();

    assert_eq!(year.first_day().solar().to_ymd(), "1912-04-01");
    assert_eq!(year.last_day().solar().to_ymd(), "1913-03-31");
    assert_eq!(year.months().len(), 12);
    assert!(year.contains_solar(Solar::from_ymd(1912, 12, 31).unwrap()));

    assert_eq!(month.solar_year(), 1912);
    assert_eq!(month.day_count(), 30);
    assert_eq!(month.first_solar_day().to_ymd(), "1912-04-01");
    assert_eq!(month.last_solar_day().to_ymd(), "1912-04-30");
    assert_eq!(month.days().len(), 30);
}

#[test]
fn thai_buddhist_next_and_subtract_follow_solar_days_and_year_boundary() {
    let last = Solar::from_ymd(1913, 3, 31).unwrap().thai_buddhist().unwrap();
    let next = last.next(1).unwrap();
    assert_eq!(next.to_string(), "泰佛历2456年4月1日");
    assert_eq!(next.subtract(last), 1);

    let march = ThaiBuddhistMonth::from_ym(2455, 3).unwrap();
    assert_eq!(march.next(1).unwrap().to_string(), "泰佛历2456年4月");
}

#[test]
fn solar_can_resolve_thai_buddhist_companions() {
    let solar = Solar::from_ymd(1912, 4, 1).unwrap();
    assert_eq!(solar.thai_buddhist().unwrap().to_string(), "泰佛历2455年4月1日");
    assert_eq!(solar.thai_buddhist_year().unwrap().year(), 2455);
    assert_eq!(solar.thai_buddhist_month().unwrap().month(), 4);
}

#[test]
fn thai_buddhist_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().thai_buddhist().unwrap();
    let month = day.thai_buddhist_month();
    let year = day.thai_buddhist_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
