use lunar_rs::{EventKind, EventQuery, Solar, TaoMonth, TaoYear};

#[test]
fn tao_year_and_month_template_layers_are_available() {
    let lunar = Solar::from_ymd(2021, 12, 21).unwrap().lunar();
    let tao = lunar.tao();
    let year = tao.tao_year();
    let month = tao.tao_month();

    assert_eq!(year.year(), 4718);
    assert_eq!(year.lunar_year(), 2021);
    assert!(year.contains_solar(Solar::from_ymd(2021, 12, 21).unwrap()));
    assert!(!year.months().is_empty());

    assert_eq!(month.year(), 4718);
    assert_eq!(month.month(), 11);
    assert!(!month.is_leap());
    assert_eq!(month.day_count(), 30);
    assert!(month.contains_solar(Solar::from_ymd(2021, 12, 21).unwrap()));
}

#[test]
fn tao_template_layers_reuse_phase3_event_queries() {
    let year = TaoYear::from_year(4718);
    let lunar = Solar::from_ymd(2021, 12, 21).unwrap().lunar();
    let month = lunar.tao().tao_month();

    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::TaoFestival)).is_empty());
    assert!(!month.find_events(&EventQuery::new().with_kind(EventKind::TaoFestival)).is_empty());
}

#[test]
fn tao_month_next_tracks_underlying_lunar_month() {
    let lunar = Solar::from_ymd(2021, 12, 21).unwrap().lunar();
    let month = lunar.tao().tao_month();
    let next = month.next(1).unwrap();
    assert_eq!(next.year(), 4718);
    assert_eq!(next.month(), 12);
    assert_eq!(TaoMonth::from_lunar_month(lunar_rs::LunarMonth::from_ym(2021, 12).unwrap()).month(), 12);
}
