use lunar_rs::{CalendarDay, CalendarSpan, EventKind, EventQuery, Rumi, RumiMonth, RumiYear, Solar};

#[test]
fn rumi_round_trip_core_examples() {
    assert_eq!(Solar::from_ymd(1840, 3, 13).unwrap().rumi().unwrap().to_string(), "鲁米历1256年3月1日");
    assert_eq!(Solar::from_ymd(1917, 2, 28).unwrap().rumi().unwrap().to_string(), "鲁米历1332年2月15日");
    assert_eq!(Solar::from_ymd(1917, 3, 1).unwrap().rumi().unwrap().to_string(), "鲁米历1333年3月1日");
    assert_eq!(Solar::from_ymd(1918, 1, 1).unwrap().rumi().unwrap().to_string(), "鲁米历1334年1月1日");

    assert_eq!(Rumi::from_ymd(1256, 3, 1).unwrap().solar().to_ymd(), "1840-03-13");
    assert_eq!(Rumi::from_ymd(1332, 2, 15).unwrap().solar().to_ymd(), "1917-02-28");
    assert_eq!(Rumi::from_ymd(1333, 3, 1).unwrap().solar().to_ymd(), "1917-03-01");
}

#[test]
fn rumi_year_and_month_layers_capture_transition_ranges() {
    let year_1332 = RumiYear::from_year(1332).unwrap();
    assert_eq!(year_1332.first_day().solar().to_ymd(), "1916-03-14");
    assert_eq!(year_1332.last_day().solar().to_ymd(), "1917-02-28");
    assert_eq!(year_1332.months().len(), 12);

    let year_1333 = RumiYear::from_year(1333).unwrap();
    assert_eq!(year_1333.first_day().solar().to_ymd(), "1917-03-01");
    assert_eq!(year_1333.last_day().solar().to_ymd(), "1917-12-31");
    assert_eq!(year_1333.months().len(), 10);
    assert!(year_1333.contains_solar(Solar::from_ymd(1917, 8, 10).unwrap()));

    let year_1334 = RumiYear::from_year(1334).unwrap();
    assert_eq!(year_1334.first_day().solar().to_ymd(), "1918-01-01");
    assert_eq!(year_1334.last_day().solar().to_ymd(), "1918-12-31");
    assert_eq!(year_1334.months().len(), 12);

    let feb_1332 = RumiMonth::from_ym(1332, 2).unwrap();
    assert_eq!(feb_1332.day_count(), 15);
    assert_eq!(feb_1332.first_solar_day().to_ymd(), "1917-02-14");
    assert_eq!(feb_1332.last_solar_day().to_ymd(), "1917-02-28");

    let mar_1333 = RumiMonth::from_ym(1333, 3).unwrap();
    assert_eq!(mar_1333.day_count(), 31);
    assert_eq!(mar_1333.first_solar_day().to_ymd(), "1917-03-01");
    assert_eq!(mar_1333.last_solar_day().to_ymd(), "1917-03-31");
}

#[test]
fn rumi_next_and_subtract_follow_solar_days_and_reforms() {
    let last = Rumi::from_ymd(1332, 2, 15).unwrap();
    let next = last.next(1).unwrap();
    assert_eq!(next.to_string(), "鲁米历1333年3月1日");
    assert_eq!(next.subtract(last), 1);

    let feb = RumiMonth::from_ym(1332, 2).unwrap();
    assert_eq!(feb.next(1).unwrap().to_string(), "鲁米历1333年3月");

    let dec = RumiMonth::from_ym(1333, 12).unwrap();
    assert_eq!(dec.next(1).unwrap().to_string(), "鲁米历1334年1月");
}

#[test]
fn solar_can_resolve_rumi_companions() {
    let solar = Solar::from_ymd(1917, 3, 1).unwrap();
    assert_eq!(solar.rumi().unwrap().to_string(), "鲁米历1333年3月1日");
    assert_eq!(solar.rumi_year().unwrap().year(), 1333);
    assert_eq!(solar.rumi_month().unwrap().month(), 3);
}

#[test]
fn rumi_reuses_event_queries_and_public_traits() {
    let day = Solar::from_ymd(2024, 5, 1).unwrap().rumi().unwrap();
    let month = day.rumi_month();
    let year = day.rumi_year();

    let holidays = day.find_events(&EventQuery::new().with_kind(EventKind::Holiday));
    assert!(!holidays.is_empty());
    assert!(holidays.iter().all(|event| matches!(event.kind(), EventKind::Holiday)));

    assert_eq!(CalendarDay::solar(&day).to_ymd(), "2024-05-01");
    assert!(CalendarSpan::contains_solar(&month, Solar::from_ymd(2024, 5, 15).unwrap()));
    assert!(!year.find_events(&EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
}
