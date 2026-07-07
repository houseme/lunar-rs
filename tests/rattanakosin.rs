use lunar_rs::{
    CalendarDay, CalendarSpan, EventKind, EventQuery, Rattanakosin, RattanakosinMonth, RattanakosinYear, Solar,
};

#[test]
fn rattanakosin_round_trip_core_examples() {
    assert_eq!(Solar::from_ymd(2024, 3, 31).unwrap().rattanakosin().unwrap().to_string(), "拉达那哥欣242年3月31日");
    assert_eq!(Solar::from_ymd(2024, 4, 1).unwrap().rattanakosin().unwrap().to_string(), "拉达那哥欣243年4月1日");
    assert_eq!(Rattanakosin::from_ymd(243, 4, 1).unwrap().solar().to_ymd(), "2024-04-01");
    assert_eq!(Rattanakosin::from_ymd(242, 3, 31).unwrap().solar().to_ymd(), "2024-03-31");
}

#[test]
fn rattanakosin_year_and_month_layers_are_available() {
    let year = RattanakosinYear::from_year(243).unwrap();
    let month = RattanakosinMonth::from_ym(243, 4).unwrap();

    assert_eq!(year.first_day().solar().to_ymd(), "2024-04-01");
    assert_eq!(year.last_day().solar().to_ymd(), "2025-03-31");
    assert_eq!(year.months().len(), 12);
    assert!(year.contains_solar(Solar::from_ymd(2024, 12, 31).unwrap()));

    assert_eq!(month.solar_year(), 2024);
    assert_eq!(month.day_count(), 30);
    assert_eq!(month.first_solar_day().to_ymd(), "2024-04-01");
    assert_eq!(month.last_solar_day().to_ymd(), "2024-04-30");
    assert_eq!(month.days().len(), 30);
}

#[test]
fn rattanakosin_next_and_subtract_follow_solar_days_and_year_boundary() {
    let last = Solar::from_ymd(2024, 3, 31).unwrap().rattanakosin().unwrap();
    let next = last.next(1).unwrap();
    assert_eq!(next.to_string(), "拉达那哥欣243年4月1日");
    assert_eq!(next.subtract(last), 1);

    let march = RattanakosinMonth::from_ym(242, 3).unwrap();
    assert_eq!(march.next(1).unwrap().to_string(), "拉达那哥欣243年4月");
}

#[test]
fn solar_can_resolve_rattanakosin_companions() {
    let solar = Solar::from_ymd(2024, 4, 1).unwrap();
    assert_eq!(solar.rattanakosin().unwrap().to_string(), "拉达那哥欣243年4月1日");
    assert_eq!(solar.rattanakosin_year().unwrap().year(), 243);
    assert_eq!(solar.rattanakosin_month().unwrap().month(), 4);
}

#[test]
fn rattanakosin_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 5, 1).unwrap().rattanakosin().unwrap();
    let month = day.rattanakosin_month();
    let year = day.rattanakosin_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-05-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 5, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
