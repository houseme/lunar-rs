#![cfg(feature = "i18n")]

use lunar_rs::{Language, NineStar, Solar};

#[test]
fn solar_i18n_helpers_translate_week_and_constellation() {
    let solar = Solar::from_ymd(2024, 4, 22).unwrap();
    assert_eq!(solar.week_in_lang(Language::ZhCn), "一");
    assert_eq!(solar.week_in_lang(Language::En), "Mon");
    assert_eq!(solar.xing_zuo_in_lang(Language::ZhCn), "金牛");
    assert_eq!(solar.xing_zuo_in_lang(Language::En), "Taurus");
    let full = solar.to_full_string_in_lang(Language::En);
    assert!(full.contains("Weekday Mon"));
    assert!(full.contains("Taurus Sign"));
}

#[test]
fn lunar_i18n_helpers_translate_ganzhi_zodiac_and_jieqi() {
    let lunar = Solar::from_ymd(2024, 4, 22).unwrap().lunar();
    assert_eq!(lunar.year_sheng_xiao_in_lang(Language::En), "Dragon");
    assert_eq!(lunar.day_sheng_xiao_in_lang(Language::En), "Dragon");
    assert_eq!(lunar.year_in_gan_zhi_in_lang(Language::En), "Jia Chen");
    assert_eq!(lunar.day_in_gan_zhi_in_lang(Language::En), "Bing Chen");
    assert_eq!(lunar.time_in_gan_zhi_in_lang(Language::En), "Wu Zi");
    assert_eq!(lunar.jie_qi_in_lang(Language::En), "");
    let full = lunar.to_full_string_in_lang(Language::En);
    assert!(full.contains("Jia Chen(Dragon) Year"));
    assert!(full.contains("Weekday Mon"));
    assert!(full.contains("Direction"));

    let winter_solstice = Solar::from_ymd(2021, 12, 21).unwrap().lunar();
    assert_eq!(winter_solstice.jie_qi_in_lang(Language::En), "Winter Solstice");
}

#[test]
fn jieqi_name_in_lang_translates_known_terms() {
    let jieqi = Solar::from_ymd(2021, 12, 21).unwrap().lunar().current_jie_qi().unwrap();
    assert_eq!(jieqi.name_in_lang(Language::ZhCn), "冬至");
    assert_eq!(jieqi.name_in_lang(Language::En), "Winter Solstice");
}

#[test]
fn foto_tao_and_nine_star_i18n_outputs_are_available() {
    let lunar = Solar::from_ymd(2024, 5, 15).unwrap().lunar();
    let foto = lunar.foto();
    let tao = lunar.tao();

    assert!(foto.to_string_in_lang(Language::En).starts_with("Buddhist "));
    assert!(foto.to_full_string_in_lang(Language::En).contains("Buddhist "));

    assert!(tao.to_string_in_lang(Language::En).starts_with("Taoist "));
    assert!(tao.to_full_string_in_lang(Language::En).contains("Taoist "));

    let nine_star = NineStar::from_index(0);
    assert_eq!(nine_star.color_in_lang(Language::En), "White");
    assert_eq!(nine_star.wu_xing_in_lang(Language::En), "Water");
    assert_eq!(nine_star.name_in_bei_dou_in_lang(Language::En), "Tian Shu");
    assert_eq!(nine_star.to_string_in_lang(Language::En), "一 White Water Tian Shu");
}
