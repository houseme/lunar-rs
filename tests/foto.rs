use lunar_rs::{EventKind, EventQuery, FotoMonth, FotoYear, Solar};

#[test]
fn foto_year_and_month_template_layers_are_available() {
    let lunar = Solar::from_ymd(2024, 5, 15).unwrap().lunar();
    let foto = lunar.foto();
    let year = foto.foto_year();
    let month = foto.foto_month();

    assert_eq!(year.year(), 2568);
    assert_eq!(year.lunar_year(), 2024);
    assert!(year.contains_solar(Solar::from_ymd(2024, 5, 15).unwrap()));
    assert_eq!(year.first_month().year(), 2568);
    assert!(!year.months().is_empty());

    assert_eq!(month.year(), 2568);
    assert_eq!(month.month(), 4);
    assert!(!month.is_leap());
    assert_eq!(month.day_count(), 29);
    assert!(month.contains_solar(Solar::from_ymd(2024, 5, 15).unwrap()));
}

#[test]
fn foto_template_layers_reuse_phase3_event_queries() {
    let year = FotoYear::from_year(2568);
    let lunar = Solar::from_ymd(2024, 5, 15).unwrap().lunar();
    let month = lunar.foto().foto_month();

    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::FotoOtherFestival)).is_empty());
    assert!(!month.find_events(&EventQuery::new().with_kind(EventKind::FotoOtherFestival)).is_empty());
}

#[test]
fn foto_month_next_tracks_underlying_lunar_month() {
    let lunar = Solar::from_ymd(2024, 5, 15).unwrap().lunar();
    let month = lunar.foto().foto_month();
    let next = month.next(1).unwrap();
    assert_eq!(next.year(), 2568);
    assert_eq!(next.month(), 5);
    assert_eq!(FotoMonth::from_lunar_month(lunar_rs::LunarMonth::from_ym(2024, 5).unwrap()).month(), 5);
}
