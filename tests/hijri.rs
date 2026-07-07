use lunar_rs::{EventKind, EventQuery, Hijri, HijriMonth, HijriYear, Solar};

#[test]
fn hijri_round_trip_core_examples() {
    assert_eq!(Solar::from_ymd(622, 7, 16).unwrap().hijri().to_string(), "1年穆哈兰姆月1日");
    assert_eq!(Solar::from_ymd(2026, 5, 13).unwrap().hijri().to_string(), "1447年都尔喀尔德月26日");
    assert_eq!(Hijri::from_ymd(1447, 11, 26).unwrap().solar().to_ymd(), "2026-05-13");
}

#[test]
fn hijri_supports_pre_epoch_and_far_future_dates() {
    assert_eq!(Solar::from_ymd(100, 7, 8).unwrap().hijri().to_string(), "-538年都尔黑哲月12日");
    assert_eq!(Hijri::from_ymd(-538, 12, 12).unwrap().solar().to_ymd(), "0100-07-08");

    assert_eq!(Solar::from_ymd(622, 7, 15).unwrap().hijri().to_string(), "0年都尔黑哲月29日");
    assert_eq!(Hijri::from_ymd(0, 12, 29).unwrap().solar().to_ymd(), "0622-07-15");

    assert_eq!(Solar::from_ymd(1, 1, 1).unwrap().hijri().to_string(), "-640年主马达·敖外鲁月16日");
    assert_eq!(Hijri::from_ymd(-640, 5, 16).unwrap().solar().to_ymd(), "0001-01-01");

    assert_eq!(Solar::from_ymd(9999, 12, 31).unwrap().hijri().to_string(), "9666年赖比尔·阿色尼月2日");
    assert_eq!(Hijri::from_ymd(9666, 4, 2).unwrap().solar().to_ymd(), "9999-12-31");
}

#[test]
fn hijri_leap_rules_and_day_counts_match_tabular_cycle() {
    assert!(!Hijri::from_ymd(1, 1, 1).unwrap().is_leap_year());
    assert!(Hijri::from_ymd(2, 1, 1).unwrap().is_leap_year());
    assert!(!Hijri::from_ymd(0, 1, 1).unwrap().is_leap_year());
    assert!(Hijri::from_ymd(-1, 1, 1).unwrap().is_leap_year());

    assert_eq!(Hijri::from_ymd(1447, 12, 1).unwrap().day_count_of_month(), 30);
    assert_eq!(Hijri::from_ymd(1448, 12, 1).unwrap().day_count_of_month(), 29);
}

#[test]
fn hijri_next_moves_by_solar_days() {
    let hijri = Hijri::from_ymd(1447, 11, 26).unwrap();
    assert_eq!(hijri.next(1).solar().to_ymd(), "2026-05-14");
    assert_eq!(hijri.next(-1).solar().to_ymd(), "2026-05-12");
}

#[test]
fn hijri_year_and_month_layers_are_available() {
    let year = HijriYear::from_year(1447);
    assert_eq!(year.year(), 1447);
    assert_eq!(year.day_count(), 355);
    assert!(year.is_leap());
    assert_eq!(year.first_month().to_string(), "1447年穆哈兰姆月");
    assert_eq!(year.months().len(), 12);

    let month = HijriMonth::from_ym(1447, 11).unwrap();
    assert_eq!(month.hijri_year().year(), 1447);
    assert_eq!(month.month_name(), "都尔喀尔德月");
    assert_eq!(month.day_count(), 30);
    assert_eq!(month.first_day().to_string(), "1447年都尔喀尔德月1日");
    assert_eq!(month.last_day().to_string(), "1447年都尔喀尔德月30日");
    assert_eq!(month.days().len(), 30);
    assert!(year.contains_solar(Solar::from_ymd(2026, 5, 13).unwrap()));
    assert!(month.contains_solar(Solar::from_ymd(2026, 5, 13).unwrap()));
    assert!(year.contains_hijri(Hijri::from_ymd(1447, 11, 26).unwrap()));
    assert!(month.contains_hijri(Hijri::from_ymd(1447, 11, 26).unwrap()));
    assert_eq!(year.first_day().to_string(), "1447年穆哈兰姆月1日");
    assert_eq!(year.last_day().to_string(), "1447年都尔黑哲月30日");
}

#[test]
fn hijri_reuses_phase3_event_queries() {
    let hijri = Solar::from_ymd(2024, 1, 1).unwrap().hijri();
    let holidays = hijri.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));
}

#[test]
fn hijri_subtract_tracks_solar_day_distance() {
    let a = Hijri::from_ymd(1447, 11, 26).unwrap();
    let b = Hijri::from_ymd(1447, 11, 20).unwrap();
    assert_eq!(a.subtract(b), 6);
    assert!(a.is_after(b));
    assert!(b.is_before(a));
}

#[test]
fn solar_can_resolve_hijri_year_and_month() {
    let solar = Solar::from_ymd(2026, 5, 13).unwrap();
    assert_eq!(solar.hijri_year().year(), 1447);
    assert_eq!(solar.hijri_month().month(), 11);
}
