//! JieQi focused tests migrated from the reference implementations.

use lunar_rs::{Lunar, Solar};

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
