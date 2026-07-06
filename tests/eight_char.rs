//! EightChar / Yun focused tests migrated from the reference implementations.

use lunar_rs::Lunar;

#[test]
fn eight_char_bazi() {
    let lunar = Lunar::from_ymd_hms(2019, 12, 12, 11, 22, 0).unwrap();
    let ec = lunar.eight_char();
    assert_eq!(ec.year(), "己亥");
    assert_eq!(ec.month(), "丁丑");
    assert_eq!(ec.day(), "戊申");
    assert_eq!(ec.time(), "戊午");
}
