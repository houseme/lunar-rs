//! Lunar domain tests migrated from the reference implementations.

mod common;

use lunar_rs::{Lunar, LunarYear, Solar};

use common::norm;

#[test]
fn lunar_full_string() {
    let lunar = Lunar::from_ymd_hms(2019, 3, 27, 0, 0, 0).unwrap();
    assert_eq!(lunar.to_string(), "二〇一九年三月廿七");
    assert_eq!(lunar.year_in_gan_zhi(), "己亥");
    assert_eq!(lunar.year_sheng_xiao(), "猪");
    assert_eq!(lunar.month_in_gan_zhi(), "戊辰");
    assert_eq!(lunar.day_in_gan_zhi(), "戊戌");
    assert_eq!(lunar.time_in_gan_zhi(), "壬子");
    assert_eq!(lunar.year_nayin(), "平地木");
    assert_eq!(lunar.month_nayin(), "大林木");
    assert_eq!(lunar.day_nayin(), "平地木");
    assert_eq!(lunar.time_nayin(), "桑柘木");
    assert_eq!(lunar.week_in_chinese(), "三");
    assert_eq!(lunar.gong(), "西");
    assert_eq!(lunar.shou(), "白虎");
    assert_eq!(lunar.xiu(), "参");
    assert_eq!(lunar.zheng(), "水");
    assert_eq!(lunar.animal(), "猿");
    assert_eq!(lunar.xiu_luck(), "吉");
    assert_eq!(lunar.peng_zu_gan(), "戊不受田田主不祥");
    assert_eq!(lunar.peng_zu_zhi(), "戌不吃犬作怪上床");
    assert_eq!(lunar.day_position_xi(), "巽");
    assert_eq!(lunar.day_position_xi_desc(), "东南");
    assert_eq!(lunar.day_position_yang_gui_desc(), "东北");
    assert_eq!(lunar.day_position_yin_gui_desc(), "西南");
    assert_eq!(lunar.day_position_fu_desc(), "东北");
    assert_eq!(lunar.day_position_cai_desc(), "正北");
    assert_eq!(lunar.day_chong_desc(), "(壬辰)龙");
    assert_eq!(lunar.day_sha(), "北");
    assert_eq!(
        norm(&lunar.to_full_string()),
        norm(
            "二〇一九年三月廿七 己亥(猪)年 戊辰(龙)月 戊戌(狗)日 子(鼠)时 \
             纳音[平地木 大林木 平地木 桑柘木] 星期三 西方白虎 \
             星宿[参水猿](吉) 彭祖百忌[戊不受田田主不祥 戌不吃犬作怪上床] \
             喜神方位[巽](东南) 阳贵神方位[艮](东北) 阴贵神方位[坤](西南) \
             福神方位[艮](东北) 财神方位[坎](正北) 冲[(壬辰)龙] 煞[北]"
        )
    );

    let solar = lunar.solar();
    assert_eq!(solar.to_string(), "2019-05-01");
    assert_eq!(norm(&solar.to_full_string()), norm("2019-05-01 00:00:00 星期三 (劳动节) 金牛座"));
}

#[test]
fn lunar_to_solar_round_trip() {
    assert_eq!(Solar::from_ymd(2020, 1, 10).unwrap().lunar().next(0).solar().to_ymd(), "2020-01-10");
    assert_eq!(Lunar::from_ymd(2019, 3, 27).unwrap().solar().to_ymd(), "2019-05-01");
    assert_eq!(Lunar::from_ymd(2020, 12, 10).unwrap().solar().to_ymd(), "2021-01-22");
    assert_eq!(Lunar::from_ymd(1500, 1, 1).unwrap().solar().to_ymd(), "1500-01-31");
    assert_eq!(Lunar::from_ymd(1500, 12, 29).unwrap().solar().to_ymd(), "1501-01-18");
}

#[test]
fn leap_months() {
    assert_eq!(Solar::from_ymd(2033, 12, 22).unwrap().lunar().to_string(), "二〇三三年闰冬月初一");
    assert_eq!(Lunar::from_ymd(2033, -11, 1).unwrap().solar().to_ymd(), "2033-12-22");
    assert_eq!(Lunar::from_ymd(2025, -6, 1).unwrap().solar().to_ymd(), "2025-07-25");
    assert_eq!(Solar::from_ymd(917, 12, 1).unwrap().lunar().to_string(), "九一七年闰十月十四");
    assert_eq!(Lunar::from_ymd(37, -12, 1).unwrap().month_in_chinese(), "闰腊");
}

#[test]
fn ganzhi_variants() {
    assert_eq!(Solar::from_ymd(1990, 10, 8).unwrap().lunar().month_in_gan_zhi_exact(), "乙酉");
    assert_eq!(Solar::from_ymd(1990, 10, 9).unwrap().lunar().month_in_gan_zhi_exact(), "丙戌");
    assert_eq!(Solar::from_ymd(1991, 2, 5).unwrap().lunar().month_in_gan_zhi(), "庚寅");
    assert_eq!(Solar::from_ymd(2022, 4, 5).unwrap().lunar().month_in_gan_zhi(), "甲辰");
}

#[test]
fn positions_and_misc() {
    let lunar = Solar::from_ymd(2021, 11, 13).unwrap().lunar();
    assert_eq!(lunar.day_position_tai(), "碓磨厕 外东南");
    assert_eq!(lunar.day_position_fu_desc(), "西南");
    assert_eq!(Solar::from_ymd(2021, 11, 12).unwrap().lunar().day_position_fu_desc(), "正北");
    assert_eq!(Solar::from_ymd(2011, 11, 12).unwrap().lunar().day_position_tai(), "厨灶厕 外西南");
    assert_eq!(Solar::from_ymd(2017, 2, 15).unwrap().lunar().day_lu(), "子命互禄 辛命进禄");
    assert_eq!(Solar::from_ymd(2017, 2, 16).unwrap().lunar().day_lu(), "寅命互禄");
}

#[test]
fn festivals() {
    assert_eq!(Lunar::from_ymd(2021, 12, 29).unwrap().festivals().first().copied(), Some("除夕"));
    assert_eq!(Lunar::from_ymd(2020, 12, 30).unwrap().festivals().first().copied(), Some("除夕"));
    assert_eq!(Lunar::from_ymd(2020, 12, 29).unwrap().festivals().len(), 0);
    assert_eq!(Solar::from_ymd(1722, 9, 25).unwrap().lunar().other_festivals().first().copied(), Some("秋社"));
    assert_eq!(Solar::from_ymd(2022, 3, 16).unwrap().lunar().other_festivals().first().copied(), Some("春社"));
    assert_eq!(Solar::from_ymd(2022, 3, 28).unwrap().festivals().first().copied(), Some("全国中小学生安全教育日"));
}

#[test]
fn lunar_year_misc() {
    let y = LunarYear::from_year(2020);
    assert_eq!(y.gan_zhi(), "庚子");
    assert_eq!(y.leap_month(), 4);
    assert_eq!(y.yuan(), "下元");
    assert!(y.zhi_shui().contains("龙治水"));
    assert!(y.geng_tian().contains("牛耕田"));
}
