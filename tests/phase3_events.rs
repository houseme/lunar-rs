use lunar_rs::{EventKind, Lunar, Solar};

#[test]
fn solar_events_include_festivals_and_jieqi() {
    let solar = Solar::from_ymd(2022, 3, 28).unwrap();
    let events = solar.events();

    assert!(events.iter().any(|event| matches!(event.kind(), EventKind::SolarFestival) && event.name() == "全国中小学生安全教育日"));

    let solar = Solar::from_ymd(2021, 12, 21).unwrap();
    let events = solar.events();
    assert!(events.iter().any(|event| matches!(event.kind(), EventKind::JieQi) && event.name() == "冬至"));
}

#[test]
fn lunar_events_include_lunar_festivals_and_other_festivals() {
    let lunar = Lunar::from_ymd(2021, 12, 29).unwrap();
    let events = lunar.events();
    assert!(events.iter().any(|event| matches!(event.kind(), EventKind::LunarFestival) && event.name() == "除夕"));

    let lunar = Solar::from_ymd(1722, 9, 25).unwrap().lunar();
    let events = lunar.events();
    assert!(events.iter().any(|event| matches!(event.kind(), EventKind::LunarOtherFestival) && event.name() == "秋社"));
}

#[test]
fn holiday_events_include_detail() {
    let solar = Solar::from_ymd(2024, 1, 1).unwrap();
    let events = solar.events();
    let holiday = events
        .iter()
        .find(|event| matches!(event.kind(), EventKind::Holiday))
        .expect("expected at least one holiday event");

    assert_eq!(holiday.solar(), solar);
    assert!(holiday.detail().is_some());
}
