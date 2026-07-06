use lunar_rs::{CalendarKind, Event, EventKind, EventQuery, EventSource, Lunar, Solar};

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

#[test]
fn all_events_aggregate_multiple_calendar_contexts() {
    let solar = Solar::from_ymd(2021, 12, 21).unwrap();
    let solar_events = solar.all_events();
    assert!(solar_events.iter().any(|event| matches!(event.kind(), EventKind::JieQi)));
    assert!(solar_events.iter().any(|event| matches!(event.kind(), EventKind::TaoFestival)));

    let lunar = solar.lunar();
    let lunar_events = lunar.all_events();
    assert!(lunar_events.iter().any(|event| matches!(event.kind(), EventKind::JieQi)));
    assert!(lunar_events.iter().any(|event| matches!(event.kind(), EventKind::TaoFestival)));
}

#[test]
fn event_display_text_and_dedup_sort_are_stable() {
    let solar = Solar::from_ymd(2024, 1, 1).unwrap();
    let holiday = solar
        .events()
        .into_iter()
        .find(|event| matches!(event.kind(), EventKind::Holiday))
        .expect("expected holiday event");
    assert!(holiday.display_text().contains("holiday target"));

    let mut events = vec![
        Event::with_detail(
            EventKind::SolarFestival,
            CalendarKind::Solar,
            EventSource::BuiltInFestival,
            "B",
            solar,
            "detail",
        ),
        Event::new(EventKind::SolarFestival, CalendarKind::Solar, EventSource::BuiltInFestival, "A", solar),
        Event::new(EventKind::SolarFestival, CalendarKind::Solar, EventSource::BuiltInFestival, "A", solar),
    ];

    lunar_rs::dedup_events(&mut events);

    assert_eq!(events.len(), 2);
    assert_eq!(events[0].name(), "A");
    assert_eq!(events[1].display_text(), "B (detail)");
}

#[test]
fn event_query_filters_by_kind_and_source() {
    let solar = Solar::from_ymd(2021, 12, 21).unwrap();

    let jieqi_events = solar.find_events(&EventQuery::new().with_kind(EventKind::JieQi));
    assert_eq!(jieqi_events.len(), 1);
    assert_eq!(jieqi_events[0].name(), "冬至");

    let holiday_events = Solar::from_ymd(2024, 1, 1)
        .unwrap()
        .find_events(&EventQuery::new().with_source(EventSource::HolidayData));
    assert!(!holiday_events.is_empty());
    assert!(holiday_events.iter().all(|event| matches!(event.source(), EventSource::HolidayData)));
}

#[test]
fn event_query_filters_by_calendar_and_detail() {
    let solar = Solar::from_ymd(2021, 12, 21).unwrap();
    let tao_events = solar.find_events(&EventQuery::new().with_calendar_kind(CalendarKind::Tao));
    assert!(tao_events.iter().all(|event| matches!(event.calendar_kind(), CalendarKind::Tao)));

    let lunar = Solar::from_ymd(2024, 5, 15).unwrap().lunar();
    let foto = lunar.foto();
    let detail_events = foto.find_events(&EventQuery::new().with_detail_contains("犯者"));
    assert!(detail_events.iter().all(|event| event.detail().is_some_and(|detail| detail.contains("犯者"))));
}
