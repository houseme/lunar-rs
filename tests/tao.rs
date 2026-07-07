use lunar_rs::{EventKind, EventQuery, Solar, TaoMonth, TaoYear};

#[test]
fn tao_year_and_month_template_layers_are_available() {
    let lunar = Solar::from_ymd(2021, 12, 21).unwrap().lunar();
    let tao = lunar.tao();
    let year = tao.tao_year();
    let month = tao.tao_month();

    assert_eq!(year.year(), 4718);
    assert_eq!(tao.get_year(), 4718);
    assert_eq!(tao.get_month(), 11);
    assert_eq!(tao.get_day(), 18);
    assert_eq!(year.lunar_year(), 2021);
    assert!(year.contains_solar(Solar::from_ymd(2021, 12, 21).unwrap()));
    assert!(!year.months().is_empty());
    assert_eq!(tao.get_tao_year().year(), 4718);
    assert_eq!(tao.get_tao_month().month(), 11);
    assert_eq!(tao.get_year_in_chinese(), tao.year_in_chinese());
    assert_eq!(tao.get_month_in_chinese(), tao.month_in_chinese());
    assert_eq!(tao.get_day_in_chinese(), tao.day_in_chinese());
    assert_eq!(tao.get_lunar().solar().to_ymd(), "2021-12-21");

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
    let tao = lunar.tao();

    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::TaoFestival)).is_empty());
    assert!(!month.find_events(&EventQuery::new().with_kind(EventKind::TaoFestival)).is_empty());
    assert_eq!(tao.get_festivals().len(), tao.festivals().len());
    assert_eq!(tao.get_is_day_san_hui(), tao.is_day_san_hui());
    assert_eq!(tao.get_is_day_san_yuan(), tao.is_day_san_yuan());
    assert_eq!(tao.get_is_day_wu_la(), tao.is_day_wu_la());
    assert_eq!(tao.get_is_day_ba_jie(), tao.is_day_ba_jie());
    assert_eq!(tao.get_is_day_ba_hui(), tao.is_day_ba_hui());
    assert_eq!(tao.get_is_day_ming_wu(), tao.is_day_ming_wu());
    assert_eq!(tao.get_is_day_an_wu(), tao.is_day_an_wu());
    assert_eq!(tao.get_is_day_wu(), tao.is_day_wu());
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

#[test]
fn tao_wrapper_can_outlive_intermediate_lunar_binding() {
    let tao = Solar::from_ymd(2021, 12, 21).unwrap().lunar().tao();
    assert_eq!(tao.year(), 4718);
    assert_eq!(tao.lunar().solar().to_ymd(), "2021-12-21");
}
