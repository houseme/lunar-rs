use lunar_rs::{CalendarKind, EventKind, EventSource, Lunar, Solar};

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
    assert!(matches!(holiday.calendar_kind(), CalendarKind::Solar));
    assert!(matches!(holiday.source(), EventSource::HolidayData));
    assert_eq!(holiday.calendar_label(), "solar");
    assert_eq!(holiday.source_label(), "holiday_data");
    assert!(holiday.detail().is_some());
}

#[test]
fn foto_events_are_exposed_through_unified_model() {
    let lunar = Solar::from_ymd(2024, 5, 15).unwrap().lunar();
    let foto = lunar.foto();
    let events = foto.events();

    if let Some(event) = events.first() {
        assert!(matches!(event.kind(), EventKind::FotoFestival | EventKind::FotoOtherFestival));
        assert!(matches!(event.calendar_kind(), CalendarKind::Foto));
        assert_eq!(event.category_label().starts_with("foto_"), true);
    }
}

#[test]
fn tao_events_are_exposed_through_unified_model() {
    let lunar = Solar::from_ymd(2021, 12, 21).unwrap().lunar();
    let tao = lunar.tao();
    let events = tao.events();

    assert!(events.iter().any(|event| {
        matches!(event.kind(), EventKind::TaoFestival)
            && matches!(event.calendar_kind(), CalendarKind::Tao)
            && matches!(event.source(), EventSource::BuiltInFestival)
    }));
}
