#![cfg(feature = "i18n")]

use lunar_rs::{
    AnnoLucis, Armenian, Assyrian, Auc, Bengali, Byzantine, Coptic, Dangi, Ethiopian, Fasli, HispanicEra, Holocene,
    Japanese, Juche, Koki, Language, Lunar, Minguo, Nanakshahi, NineStar, Rattanakosin, Rumi, Saka, Seleucid, Solar,
    ThaiBuddhist, ThaiSolar, Venetian,
};

#[test]
fn solar_i18n_helpers_translate_week_and_constellation() {
    let solar = Solar::from_ymd(2024, 4, 22).unwrap();
    assert_eq!(solar.to_string_in_lang(Language::ZhCn), "2024-04-22");
    assert_eq!(solar.to_string_in_lang(Language::En), "2024-04-22");
    assert_eq!(solar.week_in_lang(Language::ZhCn), "一");
    assert_eq!(solar.week_in_lang(Language::En), "Mon");
    assert_eq!(solar.xing_zuo_in_lang(Language::ZhCn), "金牛");
    assert_eq!(solar.xing_zuo_in_lang(Language::En), "Taurus");
    let full = solar.to_full_string_in_lang(Language::En);
    assert!(full.contains("Weekday Mon"));
    assert!(full.contains("Taurus Sign"));

    let new_year = Solar::from_ymd(2024, 1, 1).unwrap().to_full_string_in_lang(Language::En);
    assert!(new_year.contains("New Year's Day"));
}

#[test]
fn lunar_i18n_helpers_translate_ganzhi_zodiac_and_jieqi() {
    let lunar = Solar::from_ymd(2024, 4, 22).unwrap().lunar();
    assert_eq!(lunar.to_string_in_lang(Language::En), "Lunar 2024-03-14");
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
    assert!(full.contains("Covered Lamp Fire"));
    assert!(full.contains("White Tiger"));
    assert!(full.contains("Auspicious"));
    assert!(full.contains("Southwest"));
    assert!(full.contains("(Geng Xu)Dog"));
    assert!(full.contains("Sha [South]"));
    assert!(!full.contains("覆灯火"));
    assert!(!full.contains("玄武"));
    assert!(!full.contains("西南"));

    let winter_solstice = Solar::from_ymd(2021, 12, 21).unwrap().lunar();
    assert_eq!(winter_solstice.jie_qi_in_lang(Language::En), "Winter Solstice");

    let eve = Lunar::from_ymd(2021, 12, 29).unwrap().to_full_string_in_lang(Language::En);
    assert!(eve.contains("Chinese New Year's Eve"));
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

    assert_eq!(foto.to_string_in_lang(Language::En), "Buddhist 2568-04-08");
    assert!(foto.to_string_in_lang(Language::En).starts_with("Buddhist "));
    assert!(foto.to_full_string_in_lang(Language::En).contains("Buddhist "));
    assert_eq!(tao.to_string_in_lang(Language::En), "Taoist 4721-04-08");
    assert!(tao.to_string_in_lang(Language::En).starts_with("Taoist "));
    assert!(tao.to_full_string_in_lang(Language::En).contains("Taoist "));
    assert!(tao.to_full_string_in_lang(Language::En).contains("TianYun"));
    assert!(tao.to_full_string_in_lang(Language::En).contains("Hour"));

    let nine_star = NineStar::from_index(0);
    assert_eq!(nine_star.color_in_lang(Language::En), "White");
    assert_eq!(nine_star.wu_xing_in_lang(Language::En), "Water");
    assert_eq!(nine_star.name_in_bei_dou_in_lang(Language::En), "Tian Shu");
    assert_eq!(nine_star.to_string_in_lang(Language::En), "一 White Water Tian Shu");
}

#[test]
fn minguo_i18n_outputs_are_available() {
    let minguo = Minguo::from_ymd(113, 1, 1).unwrap();

    assert_eq!(minguo.to_string_in_lang(Language::ZhCn), "民国113年1月1日");
    assert_eq!(minguo.to_string_in_lang(Language::En), "Minguo 113-01-01");

    let zh_full = minguo.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-01-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = minguo.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Minguo 113 Year"));
    assert!(en_full.contains("Solar 2024-01-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn thai_solar_i18n_outputs_are_available() {
    let thai = ThaiSolar::from_ymd(2567, 1, 1).unwrap();

    assert_eq!(thai.to_string_in_lang(Language::ZhCn), "泰历2567年1月1日");
    assert_eq!(thai.to_string_in_lang(Language::En), "ThaiSolar 2567-01-01");

    let zh_full = thai.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-01-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = thai.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("ThaiSolar 2567 Year"));
    assert!(en_full.contains("Solar 2024-01-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn japanese_i18n_outputs_are_available() {
    let japanese = Japanese::from_solar(Solar::from_ymd(2024, 1, 1).unwrap()).unwrap();

    assert_eq!(japanese.to_string_in_lang(Language::ZhCn), "令和6年1月1日");
    assert_eq!(japanese.to_string_in_lang(Language::En), "Japanese 令和 6-01-01");

    let zh_full = japanese.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-01-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = japanese.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Japanese 令和 6 Year"));
    assert!(en_full.contains("Solar 2024-01-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn juche_i18n_outputs_are_available() {
    let juche = Juche::from_ymd(113, 1, 1).unwrap();

    assert_eq!(juche.to_string_in_lang(Language::ZhCn), "主体113年1月1日");
    assert_eq!(juche.to_string_in_lang(Language::En), "Juche 113-01-01");

    let zh_full = juche.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-01-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = juche.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Juche 113 Year"));
    assert!(en_full.contains("Solar 2024-01-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn dangi_i18n_outputs_are_available() {
    let dangi = Dangi::from_ymd(4357, 1, 1).unwrap();

    assert_eq!(dangi.to_string_in_lang(Language::ZhCn), "檀纪4357年1月1日");
    assert_eq!(dangi.to_string_in_lang(Language::En), "Dangi 4357-01-01");

    let zh_full = dangi.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-01-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = dangi.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Dangi 4357 Year"));
    assert!(en_full.contains("Solar 2024-01-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn holocene_i18n_outputs_are_available() {
    let holocene = Holocene::from_ymd(12024, 1, 1).unwrap();

    assert_eq!(holocene.to_string_in_lang(Language::ZhCn), "全新世纪12024年1月1日");
    assert_eq!(holocene.to_string_in_lang(Language::En), "Holocene 12024-01-01");

    let zh_full = holocene.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-01-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = holocene.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Holocene 12024 Year"));
    assert!(en_full.contains("Solar 2024-01-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn byzantine_i18n_outputs_are_available() {
    let byzantine = Byzantine::from_solar(Solar::from_ymd(2024, 1, 1).unwrap());

    assert_eq!(byzantine.to_string_in_lang(Language::ZhCn), "拜占庭7532年1月1日");
    assert_eq!(byzantine.to_string_in_lang(Language::En), "Byzantine 7532-01-01");

    let zh_full = byzantine.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-01-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = byzantine.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Byzantine 7532 Year"));
    assert!(en_full.contains("Solar 2024-01-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn coptic_i18n_outputs_are_available() {
    let coptic = Coptic::from_ymd(1741, 1, 1).unwrap();

    assert_eq!(coptic.to_string_in_lang(Language::ZhCn), "科普特1741年1月1日");
    assert_eq!(coptic.to_string_in_lang(Language::En), "Coptic 1741-01-01");

    let zh_full = coptic.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-09-11"));
    assert!(zh_full.contains("星期三"));

    let en_full = coptic.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Coptic 1741 Year"));
    assert!(en_full.contains("Solar 2024-09-11"));
    assert!(en_full.contains("Weekday Wed"));
}

#[test]
fn armenian_i18n_outputs_are_available() {
    let armenian = Armenian::from_ymd(1474, 1, 1).unwrap();

    assert_eq!(armenian.to_string_in_lang(Language::ZhCn), "亚美尼亚1474年1月1日");
    assert_eq!(armenian.to_string_in_lang(Language::En), "Armenian 1474-01-01");

    let zh_full = armenian.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-07-24"));
    assert!(zh_full.contains("星期三"));

    let en_full = armenian.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Armenian 1474 Year"));
    assert!(en_full.contains("Solar 2024-07-24"));
    assert!(en_full.contains("Weekday Wed"));
}

#[test]
fn anno_lucis_i18n_outputs_are_available() {
    let anno_lucis = AnnoLucis::from_ymd(6024, 1, 1).unwrap();

    assert_eq!(anno_lucis.to_string_in_lang(Language::ZhCn), "光明纪年6024年1月1日");
    assert_eq!(anno_lucis.to_string_in_lang(Language::En), "AnnoLucis 6024-01-01");

    let zh_full = anno_lucis.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-01-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = anno_lucis.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("AnnoLucis 6024 Year"));
    assert!(en_full.contains("Solar 2024-01-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn auc_i18n_outputs_are_available() {
    let auc = Auc::from_ymd(2777, 1, 1).unwrap();

    assert_eq!(auc.to_string_in_lang(Language::ZhCn), "建城纪年2777年1月1日");
    assert_eq!(auc.to_string_in_lang(Language::En), "AUC 2777-01-01");

    let zh_full = auc.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-01-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = auc.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("AUC 2777 Year"));
    assert!(en_full.contains("Solar 2024-01-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn assyrian_i18n_outputs_are_available() {
    let assyrian = Assyrian::from_ymd(6774, 4, 1).unwrap();

    assert_eq!(assyrian.to_string_in_lang(Language::ZhCn), "亚述6774年4月1日");
    assert_eq!(assyrian.to_string_in_lang(Language::En), "Assyrian 6774-04-01");

    let zh_full = assyrian.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-04-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = assyrian.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Assyrian 6774 Year"));
    assert!(en_full.contains("Solar 2024-04-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn hispanic_era_i18n_outputs_are_available() {
    let era = HispanicEra::from_ymd(2062, 1, 1).unwrap();

    assert_eq!(era.to_string_in_lang(Language::ZhCn), "西班牙纪元2062年1月1日");
    assert_eq!(era.to_string_in_lang(Language::En), "HispanicEra 2062-01-01");

    let zh_full = era.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-01-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = era.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("HispanicEra 2062 Year"));
    assert!(en_full.contains("Solar 2024-01-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn saka_i18n_outputs_are_available() {
    let saka = Saka::from_ymd(1946, 1, 1).unwrap();

    assert_eq!(saka.to_string_in_lang(Language::ZhCn), "萨卡1946年1月1日");
    assert_eq!(saka.to_string_in_lang(Language::En), "Saka 1946-01-01");

    let zh_full = saka.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-03-21"));
    assert!(zh_full.contains("星期四"));

    let en_full = saka.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Saka 1946 Year"));
    assert!(en_full.contains("Solar 2024-03-21"));
    assert!(en_full.contains("Weekday Thu"));
}

#[test]
fn bengali_i18n_outputs_are_available() {
    let bengali = Bengali::from_ymd(1431, 1, 1).unwrap();

    assert_eq!(bengali.to_string_in_lang(Language::ZhCn), "孟加拉1431年1月1日");
    assert_eq!(bengali.to_string_in_lang(Language::En), "Bengali 1431-01-01");

    let zh_full = bengali.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-04-14"));
    assert!(zh_full.contains("星期日"));

    let en_full = bengali.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Bengali 1431 Year"));
    assert!(en_full.contains("Solar 2024-04-14"));
    assert!(en_full.contains("Weekday Sun"));
}

#[test]
fn koki_i18n_outputs_are_available() {
    let koki = Koki::from_ymd(2686, 1, 1).unwrap();

    assert_eq!(koki.to_string_in_lang(Language::ZhCn), "皇纪2686年1月1日");
    assert_eq!(koki.to_string_in_lang(Language::En), "Koki 2686-01-01");

    let zh_full = koki.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2026-01-01"));
    assert!(zh_full.contains("星期四"));

    let en_full = koki.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Koki 2686 Year"));
    assert!(en_full.contains("Solar 2026-01-01"));
    assert!(en_full.contains("Weekday Thu"));
}

#[test]
fn thai_buddhist_i18n_outputs_are_available() {
    let thai_buddhist = ThaiBuddhist::from_ymd(2455, 4, 1).unwrap();

    assert_eq!(thai_buddhist.to_string_in_lang(Language::ZhCn), "泰佛历2455年4月1日");
    assert_eq!(thai_buddhist.to_string_in_lang(Language::En), "ThaiBuddhist 2455-04-01");

    let zh_full = thai_buddhist.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历1912-04-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = thai_buddhist.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("ThaiBuddhist 2455 Year"));
    assert!(en_full.contains("Solar 1912-04-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn fasli_i18n_outputs_are_available() {
    let fasli = Fasli::from_ymd(1370, 1, 1).unwrap();

    assert_eq!(fasli.to_string_in_lang(Language::ZhCn), "法斯里1370年1月1日");
    assert_eq!(fasli.to_string_in_lang(Language::En), "Fasli 1370-01-01");

    let zh_full = fasli.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2000-03-21"));
    assert!(zh_full.contains("星期二"));

    let en_full = fasli.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Fasli 1370 Year"));
    assert!(en_full.contains("Solar 2000-03-21"));
    assert!(en_full.contains("Weekday Tue"));
}

#[test]
fn nanakshahi_i18n_outputs_are_available() {
    let nanakshahi = Nanakshahi::from_ymd(556, 1, 1).unwrap();

    assert_eq!(nanakshahi.to_string_in_lang(Language::ZhCn), "纳纳克沙希556年1月1日");
    assert_eq!(nanakshahi.to_string_in_lang(Language::En), "Nanakshahi 556-01-01");

    let zh_full = nanakshahi.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-03-14"));
    assert!(zh_full.contains("星期四"));

    let en_full = nanakshahi.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Nanakshahi 556 Year"));
    assert!(en_full.contains("Solar 2024-03-14"));
    assert!(en_full.contains("Weekday Thu"));
}

#[test]
fn seleucid_i18n_outputs_are_available() {
    let seleucid = Seleucid::from_ymd(2336, 10, 1).unwrap();

    assert_eq!(seleucid.to_string_in_lang(Language::ZhCn), "塞琉古2336年10月1日");
    assert_eq!(seleucid.to_string_in_lang(Language::En), "Seleucid 2336-10-01");

    let zh_full = seleucid.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-10-01"));
    assert!(zh_full.contains("星期二"));

    let en_full = seleucid.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Seleucid 2336 Year"));
    assert!(en_full.contains("Solar 2024-10-01"));
    assert!(en_full.contains("Weekday Tue"));
}

#[test]
fn rattanakosin_i18n_outputs_are_available() {
    let rattanakosin = Rattanakosin::from_ymd(243, 4, 1).unwrap();

    assert_eq!(rattanakosin.to_string_in_lang(Language::ZhCn), "拉达那哥欣243年4月1日");
    assert_eq!(rattanakosin.to_string_in_lang(Language::En), "Rattanakosin 243-04-01");

    let zh_full = rattanakosin.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-04-01"));
    assert!(zh_full.contains("星期一"));

    let en_full = rattanakosin.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Rattanakosin 243 Year"));
    assert!(en_full.contains("Solar 2024-04-01"));
    assert!(en_full.contains("Weekday Mon"));
}

#[test]
fn venetian_i18n_outputs_are_available() {
    let venetian = Venetian::from_ymd(2024, 3, 1).unwrap();

    assert_eq!(venetian.to_string_in_lang(Language::ZhCn), "威尼斯2024年3月1日");
    assert_eq!(venetian.to_string_in_lang(Language::En), "Venetian 2024-03-01");

    let zh_full = venetian.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-03-01"));
    assert!(zh_full.contains("星期五"));

    let en_full = venetian.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Venetian 2024 Year"));
    assert!(en_full.contains("Solar 2024-03-01"));
    assert!(en_full.contains("Weekday Fri"));
}

#[test]
fn rumi_i18n_outputs_are_available() {
    let rumi = Rumi::from_ymd(1333, 3, 1).unwrap();

    assert_eq!(rumi.to_string_in_lang(Language::ZhCn), "鲁米历1333年3月1日");
    assert_eq!(rumi.to_string_in_lang(Language::En), "Rumi 1333-03-01");

    let zh_full = rumi.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历1917-03-01"));
    assert!(zh_full.contains("星期四"));

    let en_full = rumi.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Rumi 1333 Year"));
    assert!(en_full.contains("Solar 1917-03-01"));
    assert!(en_full.contains("Weekday Thu"));
}

#[test]
fn ethiopian_i18n_outputs_are_available() {
    let ethiopian = Ethiopian::from_ymd(2017, 1, 1).unwrap();

    assert_eq!(ethiopian.to_string_in_lang(Language::ZhCn), "埃塞2017年1月1日");
    assert_eq!(ethiopian.to_string_in_lang(Language::En), "Ethiopian 2017-01-01");

    let zh_full = ethiopian.to_full_string_in_lang(Language::ZhCn);
    assert!(zh_full.contains("公历2024-09-11"));
    assert!(zh_full.contains("星期三"));

    let en_full = ethiopian.to_full_string_in_lang(Language::En);
    assert!(en_full.contains("Ethiopian 2017 Year"));
    assert!(en_full.contains("Solar 2024-09-11"));
    assert!(en_full.contains("Weekday Wed"));
}

#[test]
fn typed_culture_helpers_translate_direction_duty_phase_and_cycle() {
    let lunar = Solar::from_ymd(2024, 4, 22).unwrap().lunar();

    assert_eq!(lunar.year_heaven_stem().name_in_lang(Language::En), "Jia");
    assert_eq!(lunar.year_earth_branch().name_in_lang(Language::En), "Chen");
    assert_eq!(lunar.year_sixty_cycle().name_in_lang(Language::En), "Jia Chen");
    assert_eq!(lunar.year_zodiac().name_in_lang(Language::En), "Dragon");
    assert_eq!(lunar.day_position_xi_direction().name_in_lang(Language::En), "Kun");
    assert_eq!(lunar.duty().name_in_lang(Language::En), "Establish");
    assert_eq!(lunar.phase().name_in_lang(Language::En), "Near Full Moon");
    assert_eq!(lunar.year_nayin_in_lang(Language::En), "Covered Lamp Fire");
    assert_eq!(lunar.day_position_xi_desc_in_lang(Language::En), "Southwest");
    assert_eq!(lunar.xiu_in_lang(Language::En), "Bi");
    assert_eq!(lunar.xiu_luck_in_lang(Language::En), "Auspicious");
    assert_eq!(lunar.shou_in_lang(Language::En), "White Tiger");
    assert!(lunar.peng_zu_gan_in_lang(Language::En).contains("avoid repairing stoves"));
    assert_eq!(lunar.day_chong_desc_in_lang(Language::En), "(Geng Xu)Dog");
    assert_eq!(lunar.day_sha_in_lang(Language::En), "South");
}

#[test]
fn leap_lunar_base_string_uses_shared_locale_templates() {
    let lunar = Lunar::from_ymd(2020, -4, 2).unwrap();
    assert_eq!(lunar.to_string_in_lang(Language::En), "Lunar 2020-Leap04-02");
}
