//! Solar / conversion focused tests migrated from the reference implementations.

mod common;

use lunar_rs::{CycleItem, Solar, lunar_util, solar_util};

use common::norm;

#[test]
fn solar_to_lunar_basic() {
    assert_eq!(Solar::from_ymd(2020, 5, 24).unwrap().lunar().to_string(), "二〇二〇年闰四月初二");
    assert_eq!(Solar::from_ymd(2020, 3, 1).unwrap().lunar().to_string(), "二〇二〇年二月初八");
    assert_eq!(Solar::from_ymd(11, 1, 1).unwrap().lunar().to_string(), "一〇年腊月初八");
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
    assert_eq!(lunar_rs::Lunar::from_ymd(1582, 9, 18).unwrap().solar().to_ymd(), "1582-10-04");
    assert_eq!(lunar_rs::Lunar::from_ymd(1582, 9, 19).unwrap().solar().to_ymd(), "1582-10-15");
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
    assert_eq!(solar_util::days_between(1582, 1, 1, 1583, 1, 1), 355);
    assert_eq!(solar_util::days_between(1582, 10, 4, 1582, 11, 1), 18);
    assert_eq!(solar_util::days_between(1582, 10, 4, 1582, 10, 15), 1);
}

#[test]
fn solar_week_info_wraps_legacy_week_getters() {
    let solar = Solar::from_ymd(2024, 4, 22).unwrap();
    let week = solar.week_info();

    assert_eq!(week.index() as i32, solar.week());
    assert_eq!(week.name(), solar.week_in_chinese());
    assert_eq!(week.next(1).steps_back_to(week.index()), -1);
    assert_eq!(lunar_rs::Week::from_name(week.name()).unwrap(), week);
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
    let solar = Solar::from_ymd(2020, 1, 10).unwrap();
    let lunar = solar.lunar();
    for i in -500..=500 {
        let expected = norm(&solar.next_day(i).lunar().to_full_string());
        let got = norm(&lunar.next(i).to_full_string());
        assert_eq!(expected, got, "mismatch at offset {i}");
    }
}
