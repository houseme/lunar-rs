use lunar_rs::{CalendarDay, CalendarSpan, EventKind, EventQuery, Juche, JucheMonth, JucheYear, Solar};

#[test]
fn juche_round_trip_core_examples_and_epoch_boundary() {
    let solar = Solar::from_ymd(2024, 1, 1).unwrap();
    let juche = solar.juche().unwrap();
    assert_eq!(juche.to_string(), "主体113年1月1日");
    assert_eq!(juche.solar().to_ymd(), "2024-01-01");

    assert_eq!(Solar::from_ymd(1912, 1, 1).unwrap().juche().unwrap().to_string(), "主体1年1月1日");
    assert_eq!(Juche::from_ymd(1, 1, 1).unwrap().solar().to_ymd(), "1912-01-01");
    assert!(Solar::from_ymd(1911, 12, 31).unwrap().juche().is_err());
}

#[test]
fn juche_year_and_month_layers_are_available() {
    let year = JucheYear::from_year(113).unwrap();
    let month = JucheMonth::from_ym(113, 2).unwrap();

    assert_eq!(year.solar_year(), 2024);
    assert!(year.is_leap());
    assert_eq!(year.day_count(), 366);
    assert_eq!(year.first_month().to_string(), "主体113年1月");
    assert_eq!(year.last_month().to_string(), "主体113年12月");
    assert_eq!(year.first_day().to_string(), "主体113年1月1日");
    assert_eq!(year.last_day().to_string(), "主体113年12月31日");
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
fn juche_next_and_subtract_follow_solar_days() {
    let a = Juche::from_ymd(113, 1, 1).unwrap();
    let b = a.next(10).unwrap();
    assert_eq!(b.to_string(), "主体113年1月11日");
    assert_eq!(b.subtract(a), 10);
    assert!(b.is_after(a));
    assert!(a.is_before(b));

    let month = JucheMonth::from_ym(113, 12).unwrap();
    assert_eq!(month.next(1).unwrap().to_string(), "主体114年1月");
}

#[test]
fn solar_can_resolve_juche_companions() {
    let solar = Solar::from_ymd(2024, 2, 10).unwrap();
    assert_eq!(solar.juche().unwrap().to_string(), "主体113年2月10日");
    assert_eq!(solar.juche_year().unwrap().year(), 113);
    assert_eq!(solar.juche_month().unwrap().month(), 2);
}

#[test]
fn juche_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().juche().unwrap();
    let month = JucheMonth::from_ym(113, 1).unwrap();
    let year = JucheYear::from_year(113).unwrap();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-01-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 1, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
