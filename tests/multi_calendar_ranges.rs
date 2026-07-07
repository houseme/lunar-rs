use lunar_rs::{FotoYear, HijriMonth, HijriYear, RabByungYear, Solar, TaoYear};

#[test]
fn hijri_range_helpers_expose_consistent_solar_boundaries() {
    let year = HijriYear::from_year(1447);
    let month = HijriMonth::from_ym(1447, 11).unwrap();

    assert_eq!(year.first_solar_day(), year.first_day().solar());
    assert_eq!(year.last_solar_day(), year.last_day().solar());
    assert!(year.contains_solar(year.first_solar_day()));
    assert!(year.contains_solar(year.last_solar_day()));
    assert!(!year.contains_solar(year.first_solar_day().next_day(-1)));

    assert_eq!(month.first_solar_day(), month.first_day().solar());
    assert_eq!(month.last_solar_day(), month.last_day().solar());
    assert!(month.contains_solar(month.first_solar_day()));
    assert!(month.contains_solar(month.last_solar_day()));
    assert!(!month.contains_solar(month.last_solar_day().next_day(1)));
}

#[test]
fn rab_byung_range_helpers_cover_month_and_year_boundaries() {
    let year = RabByungYear::from_year(2024).unwrap();
    let month = Solar::from_ymd(2024, 1, 1).unwrap().rab_byung_month().unwrap();

    assert_eq!(year.first_solar_day(), year.first_day().solar());
    assert_eq!(year.last_solar_day(), year.last_day().solar());

    assert_eq!(month.first_solar_day(), month.first_day().solar());
    assert_eq!(month.last_solar_day(), month.last_day().solar());
    assert!(month.contains_solar(month.first_solar_day()));
    assert!(month.contains_solar(month.last_solar_day()));
    assert!(!month.contains_solar(month.first_solar_day().next_day(-1)));
}

#[test]
fn foto_and_tao_year_ranges_reuse_common_span_behavior() {
    let foto = FotoYear::from_year(2568);
    let tao = TaoYear::from_year(4718);

    assert!(foto.contains_solar(foto.first_solar_day()));
    assert!(foto.contains_solar(foto.last_solar_day()));
    assert!(!foto.contains_solar(foto.first_solar_day().next_day(-1)));

    assert!(tao.contains_solar(tao.first_solar_day()));
    assert!(tao.contains_solar(tao.last_solar_day()));
    assert!(!tao.contains_solar(tao.last_solar_day().next_day(1)));
}
