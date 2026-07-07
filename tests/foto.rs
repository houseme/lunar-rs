use lunar_rs::{EventKind, EventQuery, FotoMonth, FotoYear, Solar};

#[test]
fn foto_year_and_month_template_layers_are_available() {
    let lunar = Solar::from_ymd(2024, 5, 15).unwrap().lunar();
    let foto = lunar.foto();
    let year = foto.foto_year();
    let month = foto.foto_month();

    assert_eq!(year.year(), 2568);
    assert_eq!(foto.get_year(), 2568);
    assert_eq!(foto.get_month(), 4);
    assert_eq!(foto.get_day(), 8);
    assert_eq!(year.get_year(), 2568);
    assert_eq!(year.lunar_year(), 2024);
    assert_eq!(year.get_lunar_year(), 2024);
    assert!(year.contains_solar(Solar::from_ymd(2024, 5, 15).unwrap()));
    assert_eq!(year.first_month().year(), 2568);
    assert_eq!(year.get_first_month().year(), 2568);
    assert_eq!(year.get_last_month().year(), 2568);
    assert!(!year.months().is_empty());
    assert_eq!(year.get_months().len(), year.months().len());
    assert_eq!(year.get_first_solar_day().to_ymd(), year.first_solar_day().to_ymd());
    assert_eq!(year.get_last_solar_day().to_ymd(), year.last_solar_day().to_ymd());
    assert_eq!(foto.get_foto_year().year(), 2568);
    assert_eq!(foto.get_foto_month().month(), 4);
    assert_eq!(foto.get_year_in_chinese(), foto.year_in_chinese());
    assert_eq!(foto.get_month_in_chinese(), foto.month_in_chinese());
    assert_eq!(foto.get_day_in_chinese(), foto.day_in_chinese());
    assert_eq!(foto.get_lunar().solar().to_ymd(), "2024-05-15");

    assert_eq!(month.year(), 2568);
    assert_eq!(month.get_year(), 2568);
    assert_eq!(month.month(), 4);
    assert_eq!(month.get_month(), 4);
    assert!(!month.is_leap());
    assert_eq!(month.day_count(), 29);
    assert_eq!(month.get_day_count(), 29);
    assert_eq!(month.get_index(), month.index());
    assert_eq!(month.get_name(), month.name());
    assert_eq!(month.get_first_solar_day().to_ymd(), month.first_solar_day().to_ymd());
    assert_eq!(month.get_last_solar_day().to_ymd(), month.last_solar_day().to_ymd());
    assert!(month.contains_solar(Solar::from_ymd(2024, 5, 15).unwrap()));
}

#[test]
fn foto_template_layers_reuse_phase3_event_queries() {
    let year = FotoYear::from_year(2568);
    let lunar = Solar::from_ymd(2024, 5, 15).unwrap().lunar();
    let month = lunar.foto().foto_month();
    let foto = lunar.foto();

    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::FotoOtherFestival)).is_empty());
    assert!(!month.find_events(&EventQuery::new().with_kind(EventKind::FotoOtherFestival)).is_empty());
    assert_eq!(foto.get_festivals().len(), foto.festivals().len());
    assert_eq!(foto.get_other_festivals().len(), foto.other_festivals().len());
    assert_eq!(foto.get_is_month_zhai(), foto.is_month_zhai());
    assert_eq!(foto.get_is_day_yang_gong(), foto.is_day_yang_gong());
    assert_eq!(foto.get_is_day_zhai_shuo_wang(), foto.is_day_zhai_shuo_wang());
    assert_eq!(foto.get_is_day_zhai_six(), foto.is_day_zhai_six());
    assert_eq!(foto.get_is_day_zhai_ten(), foto.is_day_zhai_ten());
    assert_eq!(foto.get_is_day_zhai_guan_yin(), foto.is_day_zhai_guan_yin());
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

#[test]
fn foto_wrapper_can_outlive_intermediate_lunar_binding() {
    let foto = Solar::from_ymd(2024, 5, 15).unwrap().lunar().foto();
    assert_eq!(foto.year(), 2568);
    assert_eq!(foto.lunar().solar().to_ymd(), "2024-05-15");
    assert_eq!(foto.get_xiu(), foto.xiu());
    assert_eq!(foto.get_xiu_luck(), foto.xiu_luck());
    assert_eq!(foto.get_xiu_song(), foto.xiu_song());
    assert_eq!(foto.get_zheng(), foto.zheng());
    assert_eq!(foto.get_animal(), foto.animal());
    assert_eq!(foto.get_gong(), foto.gong());
    assert_eq!(foto.get_shou(), foto.shou());
}
