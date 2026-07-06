//! 核心测试：移植自 lunar-go 的 Lunar_test.go / Solar_test.go / JieQi_test.go。
//!
//! 注：项目 CJK 排版工具会在 CJK/拉丁边界插入空格，故合成串（to_full_string）
//! 采用去空白后比较；分量值（纯 CJK / 纯 ASCII）直接逐字比较。

use lunar_rs::{Lunar, Solar, lunar_util};

fn norm(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn lunar_full_string() {
    let lunar = Lunar::from_ymd_hms(2019, 3, 27, 0, 0, 0).unwrap();
    assert_eq!(lunar.to_string(), "二〇一九年三月廿七");
    // 分量值（纯 CJK，稳定）
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
    // 合成串去空白后与 Go 参考一致
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
fn solar_to_lunar_basic() {
    assert_eq!(Solar::from_ymd(2020, 5, 24).unwrap().lunar().to_string(), "二〇二〇年闰四月初二");
    assert_eq!(Solar::from_ymd(2020, 3, 1).unwrap().lunar().to_string(), "二〇二〇年二月初八");
    assert_eq!(Solar::from_ymd(11, 1, 1).unwrap().lunar().to_string(), "一〇年腊月初八");
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
fn solar_next_day_1582() {
    assert_eq!(Solar::from_ymd(1582, 10, 4).unwrap().next_day(1).to_ymd(), "1582-10-15");
    assert_eq!(Solar::from_ymd(1582, 10, 15).unwrap().next_day(-1).to_ymd(), "1582-10-04");
    assert_eq!(Solar::from_ymd(1582, 10, 4).unwrap().next_day(18).to_ymd(), "1582-11-01");
}

#[test]
fn solar_1582_lunar() {
    assert_eq!(Solar::from_ymd(1582, 10, 4).unwrap().lunar().to_string(), "一五八二年九月十八");
    assert_eq!(Solar::from_ymd(1582, 10, 15).unwrap().lunar().to_string(), "一五八二年九月十九");
    assert_eq!(Lunar::from_ymd(1582, 9, 18).unwrap().solar().to_ymd(), "1582-10-04");
    assert_eq!(Lunar::from_ymd(1582, 9, 19).unwrap().solar().to_ymd(), "1582-10-15");
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
fn jieqi_times() {
    let lunar = Lunar::from_ymd(2012, 9, 1).unwrap();
    assert_eq!(lunar.jie_qi_table().get("白露").unwrap().to_ymd_hms(), "2012-09-07 13:29:01");
    let lunar = Lunar::from_ymd(2050, 12, 1).unwrap();
    assert_eq!(lunar.jie_qi_table().get("DA_XUE").unwrap().to_ymd_hms(), "2050-12-07 06:41:54");
}

#[test]
fn jieqi_current() {
    let lunar = Solar::from_ymd(2021, 12, 21).unwrap().lunar();
    assert_eq!(lunar.jie_qi(), "冬至");
    assert_eq!(lunar.jie(), "");
    assert_eq!(lunar.qi(), "冬至");
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
fn solar_traversal() {
    assert_eq!(Solar::from_ymd(2022, 1, 1).unwrap().next_day(1).to_ymd(), "2022-01-02");
    assert_eq!(Solar::from_ymd(2022, 1, 31).unwrap().next_day(1).to_ymd(), "2022-02-01");
    assert_eq!(Solar::from_ymd(2022, 1, 1).unwrap().next_day(365).to_ymd(), "2023-01-01");
    assert_eq!(Solar::from_ymd(2023, 8, 31).unwrap().next_month(1).to_ymd(), "2023-09-30");
    assert_eq!(Solar::from_ymd(2023, 8, 31).unwrap().next_month(2).to_ymd(), "2023-10-31");
    assert_eq!(Solar::from_ymd(2023, 8, 31).unwrap().next_year(2).to_ymd(), "2025-08-31");
    assert_eq!(Solar::from_ymd(2023, 8, 31).unwrap().next_month(6).to_ymd(), "2024-02-29");
}

#[test]
fn days_between_1582() {
    use lunar_rs::solar_util;
    assert_eq!(solar_util::days_between(1582, 1, 1, 1583, 1, 1), 355);
    assert_eq!(solar_util::days_between(1582, 10, 4, 1582, 11, 1), 18);
    assert_eq!(solar_util::days_between(1582, 10, 4, 1582, 10, 15), 1);
}

#[test]
fn time_zhi_index_boundaries() {
    assert_eq!(lunar_util::get_time_zhi_index("00:00"), 0);
    assert_eq!(lunar_util::get_time_zhi_index("00:59"), 0);
    assert_eq!(lunar_util::get_time_zhi_index("01:00"), 1);
    assert_eq!(lunar_util::get_time_zhi_index("02:59"), 1);
    assert_eq!(lunar_util::get_time_zhi_index("03:00"), 2);
    assert_eq!(lunar_util::get_time_zhi_index("21:00"), 11);
    assert_eq!(lunar_util::get_time_zhi_index("22:59"), 11);
    assert_eq!(lunar_util::get_time_zhi_index("23:00"), 0);
    assert_eq!(lunar_util::get_time_zhi_index("23:59:59"), 0);
    assert_eq!(lunar_util::get_time_zhi_index(""), 0);
    assert_eq!(lunar_util::get_time_zhi_index("1:00"), 0);
    assert_eq!(lunar_util::get_time_zhi_index("24:00"), 0);
}

#[test]
fn lunar_next_matches_solar_next() {
    // 对 ±500 天，Solar.next_day(i).lunar() 应等于 Lunar.next(i)（去空白比较）
    let solar = Solar::from_ymd(2020, 1, 10).unwrap();
    let lunar = solar.lunar();
    for i in -500..=500 {
        let expected = norm(&solar.next_day(i).lunar().to_full_string());
        let got = norm(&lunar.next(i).to_full_string());
        assert_eq!(expected, got, "mismatch at offset {i}");
    }
}

#[test]
fn eight_char_bazi() {
    // 与 lunar-go EightChar_test.go::TestEightChar1 完全对齐
    let lunar = Lunar::from_ymd_hms(2019, 12, 12, 11, 22, 0).unwrap();
    let ec = lunar.eight_char();
    assert_eq!(ec.year(), "己亥");
    assert_eq!(ec.month(), "丁丑");
    assert_eq!(ec.day(), "戊申");
    assert_eq!(ec.time(), "戊午");
}

#[test]
fn lunar_year_misc() {
    use lunar_rs::LunarYear;
    let y = LunarYear::from_year(2020);
    assert_eq!(y.gan_zhi(), "庚子");
    assert_eq!(y.leap_month(), 4);
    assert_eq!(y.yuan(), "下元");
    // 杂占
    assert!(y.zhi_shui().contains("龙治水"));
    assert!(y.geng_tian().contains("牛耕田"));
}
