use lunar_rs::{EventKind, EventQuery, RabByungDay, RabByungElement, RabByungMonth, RabByungYear, Solar};

#[test]
fn rab_byung_year_core_examples_match_reference_cases() {
    assert_eq!(RabByungYear::from_year(1027).unwrap().to_string(), "第一饶迥火兔年");
    assert_eq!(RabByungYear::from_year(2010).unwrap().to_string(), "第十七饶迥铁虎年");
    assert_eq!(RabByungYear::from_year(1961).unwrap().to_string(), "第十六饶迥铁牛年");
}

#[test]
fn rab_byung_year_leap_months_match_reference_cases() {
    assert_eq!(RabByungYear::from_year(2043).unwrap().leap_month(), 5);
    assert_eq!(RabByungYear::from_year(2044).unwrap().leap_month(), 0);
}

#[test]
fn solar_can_resolve_rab_byung_year() {
    let year = Solar::from_ymd(2010, 1, 1).unwrap().rab_byung_year().unwrap();
    assert_eq!(year.year(), 2010);
    assert_eq!(year.element().name(), "铁");
    assert_eq!(year.zodiac().name(), "虎");
}

#[test]
fn rab_byung_element_supports_name_lookup() {
    let element = RabByungElement::from_name("铁").unwrap();
    assert_eq!(element.index(), 3);
    assert_eq!(element.to_string(), "铁");
}

#[test]
fn rab_byung_month_and_day_core_examples_match_reference_cases() {
    let month = RabByungMonth::from_ym(1950, 12).unwrap();
    assert_eq!(month.to_string(), "第十六饶迥铁虎年十二月");
    assert_eq!(month.alias(), "满意月");
    assert_eq!(month.first_day().to_string(), "第十六饶迥铁虎年十二月初一");

    assert_eq!(Solar::from_ymd(1951, 1, 8).unwrap().rab_byung_day().unwrap().to_string(), "第十六饶迥铁虎年十二月初一");
    assert_eq!(RabByungDay::from_ymd(1950, 12, 1).unwrap().solar().to_ymd(), "1951-01-08");

    assert_eq!(
        Solar::from_ymd(2051, 2, 11).unwrap().rab_byung_day().unwrap().to_string(),
        "第十八饶迥铁马年十二月三十"
    );
    assert_eq!(RabByungDay::from_ymd(2050, 12, 30).unwrap().solar().to_ymd(), "2051-02-11");

    assert_eq!(Solar::from_ymd(2025, 4, 23).unwrap().rab_byung_day().unwrap().to_string(), "第十七饶迥木蛇年二月廿五");
    assert_eq!(RabByungDay::from_ymd(2025, 2, 25).unwrap().solar().to_ymd(), "2025-04-23");
}

#[test]
fn rab_byung_leap_and_special_days_are_supported() {
    let leap_day = Solar::from_ymd(1951, 1, 24).unwrap().rab_byung_day().unwrap();
    assert_eq!(leap_day.to_string(), "第十六饶迥铁虎年十二月闰十六");
    assert!(leap_day.is_leap());
    assert_eq!(RabByungDay::from_ymd(1950, 12, -16).unwrap().solar().to_ymd(), "1951-01-24");

    let regular_day = Solar::from_ymd(1952, 2, 23).unwrap().rab_byung_day().unwrap();
    assert_eq!(regular_day.to_string(), "第十六饶迥铁兔年十二月廿八");
    assert_eq!(RabByungDay::from_ymd(1951, 12, 28).unwrap().solar().to_ymd(), "1952-02-23");

    let month = RabByungMonth::from_ym(1950, 12).unwrap();
    assert!(month.contains_solar(Solar::from_ymd(1951, 1, 24).unwrap()));
    assert!(month.contains_day(leap_day));
    assert!(month.leap_days().contains(&16));
}

#[test]
fn rab_byung_day_reuses_phase3_event_queries() {
    let day = Solar::from_ymd(2024, 1, 1).unwrap().rab_byung_day().unwrap();
    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));
}

#[test]
fn rab_byung_subtract_tracks_solar_day_distance() {
    let a = Solar::from_ymd(2025, 4, 26).unwrap().rab_byung_day().unwrap();
    let b = Solar::from_ymd(2025, 4, 23).unwrap().rab_byung_day().unwrap();
    assert_eq!(a.subtract(b), 3);
    assert!(a.is_after(b));
    assert!(b.is_before(a));
}

#[test]
fn rab_byung_year_and_month_proxy_events_and_ranges() {
    let year = RabByungYear::from_year(2024).unwrap();
    let month = Solar::from_ymd(2024, 1, 1).unwrap().rab_byung_month().unwrap();
    assert!(!year.all_events().is_empty());
    assert!(!month.all_events().is_empty());
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
    assert!(!month.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
    assert!(!month.events_until(Solar::from_ymd(2024, 1, 3).unwrap()).is_empty());
}
