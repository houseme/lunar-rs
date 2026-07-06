#![cfg(feature = "i18n")]

use lunar_rs::{Language, Solar};

#[test]
fn solar_i18n_helpers_translate_week_and_constellation() {
    let solar = Solar::from_ymd(2024, 4, 22).unwrap();
    assert_eq!(solar.week_in_lang(Language::ZhCn), "一");
    assert_eq!(solar.week_in_lang(Language::En), "Mon");
    assert_eq!(solar.xing_zuo_in_lang(Language::ZhCn), "金牛");
    assert_eq!(solar.xing_zuo_in_lang(Language::En), "Taurus");
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

    let winter_solstice = Solar::from_ymd(2021, 12, 21).unwrap().lunar();
    assert_eq!(winter_solstice.jie_qi_in_lang(Language::En), "Winter Solstice");
}

#[test]
fn jieqi_name_in_lang_translates_known_terms() {
    let jieqi = Solar::from_ymd(2021, 12, 21).unwrap().lunar().current_jie_qi().unwrap();
    assert_eq!(jieqi.name_in_lang(Language::ZhCn), "冬至");
    assert_eq!(jieqi.name_in_lang(Language::En), "Winter Solstice");
}
