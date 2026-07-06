use lunar_rs::{
    CalendarKind, Event, EventKind, EventQuery, EventSource, Lunar, Solar, SolarMonth, SolarWeek,
    group_event_days_by_week, group_events_by_day, holiday_util, scan_event_days_in_range, scan_event_weeks_in_range,
    scan_events_in_range, scan_events_in_range_filtered,
};

#[test]
fn solar_events_include_festivals_and_jieqi() {
    let solar = Solar::from_ymd(2022, 3, 28).unwrap();
    let events = solar.events();

    assert!(
        events
            .iter()
            .any(|event| matches!(event.kind(), EventKind::SolarFestival) && event.name() == "全国中小学生安全教育日")
    );
    let festival = events
        .iter()
        .find(|event| matches!(event.kind(), EventKind::SolarFestival) && event.name() == "全国中小学生安全教育日")
        .expect("expected solar festival event");
    assert_eq!(festival.priority(), 30);
    assert_eq!(festival.source_id(), Some("solar-festival:2022-03-28:全国中小学生安全教育日"));
    assert!(festival.is_observed());
    assert!(festival.is_primary());
    assert!(festival.has_tag("festival"));

    let solar = Solar::from_ymd(2021, 12, 21).unwrap();
    let events = solar.events();
    let jieqi = events
        .iter()
        .find(|event| matches!(event.kind(), EventKind::JieQi) && event.name() == "冬至")
        .expect("expected jieqi event");
    assert!(jieqi.detail().is_some_and(|detail| detail.starts_with("at=2021-12-21")));
    assert_eq!(jieqi.priority(), 10);
    assert!(jieqi.is_observed());
    assert!(jieqi.is_primary());
    assert!(jieqi.has_tag("seasonal"));
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
    assert_eq!(holiday.detail(), Some("work=false target=2024-01-01"));
    assert_eq!(holiday.priority(), 20);
    assert_eq!(holiday.source_id(), Some("holiday:2024-01-01:元旦节"));
    assert!(holiday.is_observed());
    assert!(holiday.is_primary());
    assert!(holiday.has_tag("day_off"));
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
    if let Some(event) = events.iter().find(|event| matches!(event.kind(), EventKind::FotoFestival)) {
        assert!(event.detail().is_some_and(|detail| detail.contains("result=")));
        assert_eq!(event.priority(), 70);
        assert!(event.source_id().is_some());
        assert!(event.is_observed());
        assert!(event.is_primary());
        assert!(event.has_tag("foto"));
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
    if let Some(event) =
        events.iter().find(|event| matches!(event.kind(), EventKind::TaoFestival) && event.has_detail())
    {
        assert!(event.detail().is_some_and(|detail| detail.starts_with("remark=")));
        assert_eq!(event.priority(), 90);
        assert!(event.source_id().is_some());
        assert!(event.is_observed());
        assert!(event.is_primary());
        assert!(event.has_tag("tao"));
    }
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
    assert!(holiday.display_text().contains("work=false target=2024-01-01"));

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

    let holiday_events =
        Solar::from_ymd(2024, 1, 1).unwrap().find_events(&EventQuery::new().with_source(EventSource::HolidayData));
    assert!(!holiday_events.is_empty());
    assert!(holiday_events.iter().all(|event| matches!(event.source(), EventSource::HolidayData)));

    let primary_events = solar.find_events(&EventQuery::new().with_is_primary(true));
    assert!(primary_events.iter().all(|event| event.is_primary()));
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

    let tagged_events = foto.find_events(&EventQuery::new().with_tag("festival"));
    assert!(tagged_events.iter().all(|event| event.has_tag("festival")));
}

#[test]
fn range_scan_collects_and_sorts_events() {
    let start = Solar::from_ymd(2021, 12, 20).unwrap();
    let end = Solar::from_ymd(2021, 12, 22).unwrap();

    let events = scan_events_in_range(start, end);
    assert!(!events.is_empty());
    assert!(events.iter().any(|event| matches!(event.kind(), EventKind::JieQi) && event.name() == "冬至"));

    let mut previous = None::<String>;
    for event in &events {
        let current = format!("{}:{}:{}", event.solar().to_ymd_hms(), event.priority(), event.name());
        if let Some(prev) = &previous {
            assert!(prev <= &current);
        }
        previous = Some(current);
    }
}

#[test]
fn range_scan_filtered_supports_event_query() {
    let start = Solar::from_ymd(2021, 12, 20).unwrap();
    let end = Solar::from_ymd(2021, 12, 22).unwrap();

    let jieqi_events = scan_events_in_range_filtered(start, end, &EventQuery::new().with_kind(EventKind::JieQi));
    assert_eq!(jieqi_events.len(), 1);
    assert_eq!(jieqi_events[0].name(), "冬至");

    let holiday_events = Solar::from_ymd(2024, 1, 1).unwrap().events_until(Solar::from_ymd(2024, 1, 3).unwrap());
    assert!(holiday_events.iter().any(|event| matches!(event.kind(), EventKind::Holiday)));
}

#[test]
fn grouped_event_days_keep_day_boundaries() {
    let start = Solar::from_ymd(2021, 12, 20).unwrap();
    let end = Solar::from_ymd(2021, 12, 22).unwrap();

    let groups = scan_event_days_in_range(start, end);
    assert!(!groups.is_empty());
    assert!(groups.iter().any(|group| group.solar().to_ymd() == "2021-12-21"));
    assert!(
        groups
            .iter()
            .all(|group| { group.events().iter().all(|event| event.solar().to_ymd() == group.solar().to_ymd()) })
    );
}

#[test]
fn group_events_by_day_dedups_and_sorts() {
    let solar = Solar::from_ymd(2024, 1, 1).unwrap();
    let mut events = solar.events();
    events.extend(solar.events());

    let groups = group_events_by_day(events);
    assert_eq!(groups.len(), 1);
    let events = groups[0].events();
    let holiday_count = events.iter().filter(|event| matches!(event.kind(), EventKind::Holiday)).count();
    assert_eq!(holiday_count, 1);
}

#[test]
fn solar_month_event_days_support_query() {
    let month = SolarMonth::from_ym(2021, 12);
    let groups = month.find_event_days(&EventQuery::new().with_kind(EventKind::JieQi));

    assert_eq!(groups.len(), 2);
    assert_eq!(groups[0].solar().to_ymd(), "2021-12-07");
    assert_eq!(groups[1].solar().to_ymd(), "2021-12-21");
    assert!(groups.iter().all(|group| group.events().iter().all(|event| matches!(event.kind(), EventKind::JieQi))));
}

#[test]
fn week_views_group_events_into_one_week_bucket() {
    let start = Solar::from_ymd(2021, 12, 20).unwrap();
    let end = Solar::from_ymd(2021, 12, 22).unwrap();
    let weeks = scan_event_weeks_in_range(start, end, 0);

    assert_eq!(weeks.len(), 1);
    assert_eq!(weeks[0].start().to_ymd(), "2021-12-19");
    assert_eq!(weeks[0].end().to_ymd(), "2021-12-25");
    assert!(!weeks[0].days().is_empty());

    let regrouped = group_event_days_by_week(scan_event_days_in_range(start, end), 0);
    assert_eq!(regrouped.len(), 1);
    assert_eq!(regrouped[0].start().to_ymd(), "2021-12-19");
}

#[test]
fn solar_week_view_supports_query() {
    let week = SolarWeek::from_ymd(2021, 12, 21, 0);
    let weeks = week.find_event_weeks(&EventQuery::new().with_kind(EventKind::JieQi));

    assert_eq!(weeks.len(), 1);
    assert!(
        weeks[0].days().iter().all(|group| group.events().iter().all(|event| matches!(event.kind(), EventKind::JieQi)))
    );
}

#[test]
fn holiday_override_invalidates_event_cache() {
    struct HolidayDataReset(String);

    impl Drop for HolidayDataReset {
        fn drop(&mut self) {
            holiday_util::set_holiday_data(self.0.clone()).unwrap();
        }
    }

    let original_data = holiday_util::holiday_data();
    let _reset = HolidayDataReset(original_data.clone());
    let solar = Solar::from_ymd(2099, 1, 2).unwrap();

    let before = solar.all_events().into_iter().filter(|event| matches!(event.kind(), EventKind::Holiday)).count();
    assert_eq!(before, 0);

    let injected_data = format!("{original_data}209901020020990102");
    holiday_util::set_holiday_data(injected_data).unwrap();

    let holiday = solar
        .all_events()
        .into_iter()
        .find(|event| matches!(event.kind(), EventKind::Holiday))
        .expect("expected injected holiday event");
    assert_eq!(holiday.source(), &EventSource::HolidayData);
    assert_eq!(holiday.source_id(), Some("holiday:2099-01-02:元旦节"));
}
