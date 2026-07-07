use lunar_rs::{
    AnnoLucis, Armenian, Assyrian, Auc, Byzantine, CalendarDay, CalendarSpan, Coptic, Dangi, Ethiopian, EventKind,
    EventQuery, Fasli, FotoYear, Hijri, HijriMonth, HispanicEra, Holocene, Japanese, Juche, Julian, Koki, LunarMonth,
    LunarYear, Minguo, Nanakshahi, Rattanakosin, Rumi, Saka, Seleucid, Solar, SolarMonth, SolarYear, ThaiBuddhist,
    ThaiSolar, Venetian,
};

fn day_ymd<T: CalendarDay>(value: &T) -> String {
    value.solar().to_ymd()
}

fn span_bounds<T: CalendarSpan>(value: &T) -> (String, String) {
    (value.first_solar_day().to_ymd(), value.last_solar_day().to_ymd())
}

fn span_has_filtered_events<T: CalendarSpan>(value: &T, kind: EventKind) -> bool {
    !value.find_events(&EventQuery::new().with_kind(kind)).is_empty()
}

#[test]
fn calendar_day_trait_unifies_solar_lunar_and_multi_calendar_days() {
    let solar = Solar::from_ymd(2024, 1, 1).unwrap();
    let lunar = solar.lunar();
    let hijri = Hijri::from_ymd(1447, 11, 26).unwrap();
    let japanese = Japanese::from_solar(Solar::from_ymd(2024, 1, 1).unwrap()).unwrap();
    let juche = Juche::from_solar(Solar::from_ymd(2024, 1, 1).unwrap()).unwrap();
    let dangi = Dangi::from_solar(Solar::from_ymd(2024, 1, 1).unwrap()).unwrap();
    let julian = Julian::from_solar(Solar::from_ymd(2024, 1, 1).unwrap());
    let holocene = Holocene::from_solar(Solar::from_ymd(2024, 1, 1).unwrap()).unwrap();
    let byzantine = Byzantine::from_solar(Solar::from_ymd(2024, 1, 1).unwrap());
    let coptic = Coptic::from_solar(Solar::from_ymd(2024, 9, 11).unwrap());
    let armenian = Armenian::from_solar(Solar::from_ymd(2024, 7, 24).unwrap());
    let anno_lucis = AnnoLucis::from_solar(Solar::from_ymd(2024, 1, 1).unwrap()).unwrap();
    let auc = Auc::from_solar(Solar::from_ymd(2024, 1, 1).unwrap()).unwrap();
    let assyrian = Assyrian::from_solar(Solar::from_ymd(2024, 4, 1).unwrap()).unwrap();
    let hispanic = HispanicEra::from_solar(Solar::from_ymd(2024, 1, 1).unwrap()).unwrap();
    let saka = Saka::from_solar(Solar::from_ymd(2024, 3, 21).unwrap()).unwrap();
    let koki = Koki::from_solar(Solar::from_ymd(2026, 1, 1).unwrap()).unwrap();
    let thai_buddhist = ThaiBuddhist::from_solar(Solar::from_ymd(1912, 4, 1).unwrap()).unwrap();
    let fasli = Fasli::from_solar(Solar::from_ymd(2000, 3, 21).unwrap()).unwrap();
    let nanakshahi = Nanakshahi::from_solar(Solar::from_ymd(2024, 3, 14).unwrap()).unwrap();
    let rattanakosin = Rattanakosin::from_solar(Solar::from_ymd(2024, 4, 1).unwrap()).unwrap();
    let rumi = Rumi::from_solar(Solar::from_ymd(2024, 1, 1).unwrap()).unwrap();
    let seleucid = Seleucid::from_solar(Solar::from_ymd(2024, 10, 1).unwrap()).unwrap();
    let venetian = Venetian::from_solar(Solar::from_ymd(2024, 3, 1).unwrap()).unwrap();
    let ethiopian = Ethiopian::from_solar(Solar::from_ymd(2024, 9, 11).unwrap());
    let minguo = Minguo::from_ymd(113, 1, 1).unwrap();
    let thai = ThaiSolar::from_ymd(2567, 1, 1).unwrap();
    let rab_byung = Solar::from_ymd(2025, 4, 23).unwrap().rab_byung_day().unwrap();

    assert_eq!(day_ymd(&solar), "2024-01-01");
    assert_eq!(day_ymd(&lunar), "2024-01-01");
    assert_eq!(day_ymd(&hijri), "2026-05-13");
    assert_eq!(day_ymd(&japanese), "2024-01-01");
    assert_eq!(day_ymd(&juche), "2024-01-01");
    assert_eq!(day_ymd(&dangi), "2024-01-01");
    assert_eq!(day_ymd(&julian), "2024-01-01");
    assert_eq!(day_ymd(&holocene), "2024-01-01");
    assert_eq!(day_ymd(&byzantine), "2024-01-01");
    assert_eq!(day_ymd(&coptic), "2024-09-11");
    assert_eq!(day_ymd(&armenian), "2024-07-24");
    assert_eq!(day_ymd(&anno_lucis), "2024-01-01");
    assert_eq!(day_ymd(&auc), "2024-01-01");
    assert_eq!(day_ymd(&assyrian), "2024-04-01");
    assert_eq!(day_ymd(&hispanic), "2024-01-01");
    assert_eq!(day_ymd(&saka), "2024-03-21");
    assert_eq!(day_ymd(&koki), "2026-01-01");
    assert_eq!(day_ymd(&thai_buddhist), "1912-04-01");
    assert_eq!(day_ymd(&fasli), "2000-03-21");
    assert_eq!(day_ymd(&nanakshahi), "2024-03-14");
    assert_eq!(day_ymd(&rattanakosin), "2024-04-01");
    assert_eq!(day_ymd(&rumi), "2024-01-01");
    assert_eq!(day_ymd(&seleucid), "2024-10-01");
    assert_eq!(day_ymd(&venetian), "2024-03-01");
    assert_eq!(day_ymd(&ethiopian), "2024-09-11");
    assert_eq!(day_ymd(&minguo), "2024-01-01");
    assert_eq!(day_ymd(&thai), "2024-01-01");
    assert_eq!(day_ymd(&rab_byung), "2025-04-23");
}

#[test]
fn calendar_span_trait_unifies_core_month_and_year_types() {
    let solar_year = SolarYear::from_year(2024);
    let solar_month = SolarMonth::from_ym(2024, 2);
    let lunar_year = LunarYear::from_year(2024);
    let lunar_month = LunarMonth::from_ym(2024, 1).unwrap();
    let hijri_month = HijriMonth::from_ym(1445, 6).unwrap();
    let foto_year = FotoYear::from_year(2568);
    let tao_month = Solar::from_ymd(2021, 12, 21).unwrap().lunar().tao().tao_month();

    assert_eq!(span_bounds(&solar_year), ("2024-01-01".to_string(), "2024-12-31".to_string()));
    assert_eq!(span_bounds(&solar_month), ("2024-02-01".to_string(), "2024-02-29".to_string()));
    assert!(CalendarSpan::contains_solar(lunar_year.as_ref(), Solar::from_ymd(2024, 2, 10).unwrap()));
    assert!(lunar_month.contains_solar(lunar_month.first_solar_day()));
    assert!(hijri_month.contains_solar(hijri_month.first_solar_day()));
    assert!(foto_year.contains_solar(foto_year.first_solar_day()));
    assert!(tao_month.contains_solar(tao_month.last_solar_day()));
}

#[test]
fn calendar_traits_preserve_generic_event_queries() {
    let solar = Solar::from_ymd(2024, 1, 1).unwrap();
    let hijri = Hijri::from_ymd(1445, 6, 19).unwrap();
    let solar_month = SolarMonth::from_ym(2024, 1);
    let foto_year = FotoYear::from_year(2568);

    assert!(!CalendarDay::find_events(&solar, &EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
    assert!(!CalendarDay::find_events(&hijri, &EventQuery::new().with_kind(EventKind::Holiday)).is_empty());
    assert!(span_has_filtered_events(&solar_month, EventKind::Holiday));
    assert!(span_has_filtered_events(&foto_year, EventKind::FotoOtherFestival));
}
