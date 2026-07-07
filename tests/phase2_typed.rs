use lunar_rs::{
    Animal, Constellation, CultureDay, CycleItem, DayUnit, Dipper, Direction, Duty, EarthBranch, Element, Gender, God,
    GodLuck, HeavenStem, HideHeavenStemType, HijriDay, Holiday, JulianDay, Land, LegalHoliday, Luck, Lunar, LunarDay,
    LunarFestival, LunarMonth, LunarWeek, LunarYear, MinorRen, MonthUnit, MoonPhase, NamedCulture, Nayin, NineStar,
    Phase, PlumRainKind, SecondUnit, SevenStar, Side, SixStar, Sixty, SixtyCycle, Solar, SolarDay, SolarFestival,
    SolarHalfYear, SolarMonth, SolarSeason, SolarTerm, SolarTime, SolarWeek, SolarYear, Sound, Taboo, TabooKind, Ten,
    TenStar, TwelveStar, Twenty, WeekUnit, Xun, YearUnit, YinYang, YuanCycle, YunCycle, Zodiac,
};

#[test]
fn typed_primitives_map_names_and_relationships() {
    let jia = HeavenStem::from_name("甲").unwrap();
    assert_eq!(jia.index(), 0);
    assert_eq!(jia.name(), "甲");
    assert_eq!(jia.element().name(), "木");
    assert_eq!(jia.element().direction().name(), "东");
    assert_eq!(jia.yin_yang(), YinYang::YANG);

    let chen = EarthBranch::from_name("辰").unwrap();
    assert_eq!(chen.index(), 4);
    assert_eq!(chen.name(), "辰");
    assert_eq!(chen.zodiac().name(), "龙");
    assert_eq!(chen.element().name(), "土");
    assert_eq!(chen.yin_yang(), YinYang::YANG);

    assert_eq!(Gender::from_code(1), Some(Gender::MAN));
    assert_eq!(Gender::from_name("女"), Some(Gender::WOMAN));
    assert_eq!(Gender::MAN.get_name(), "男");
    assert_eq!(Gender::from(1_u8), Gender::MAN);
    assert_eq!(Gender::MAN, 1);
    assert_eq!(Side::from_code(0), Some(Side::IN));
    assert_eq!(Side::from_name("外"), Some(Side::OUT));
    assert_eq!(Side::OUT.to_string(), "外");
    assert_eq!(YinYang::from_code(0), Some(YinYang::YIN));
    assert_eq!(YinYang::from_name("阳"), Some(YinYang::YANG));
    assert_eq!(YinYang::YIN.get_code(), 0);

    let dragon = Zodiac::from_name("龙").unwrap();
    assert_eq!(dragon.index(), 4);
    assert_eq!(Zodiac::from_index(11).next(1).name(), "鼠");

    let cycle = SixtyCycle::from_name("甲辰").unwrap();
    assert_eq!(cycle.index(), 40);
    assert_eq!(cycle.heaven_stem().name(), "甲");
    assert_eq!(cycle.earth_branch().name(), "辰");

    let direction = Direction::new("巽");
    assert_eq!(direction.name(), "巽");

    let element = Element::new("火");
    assert_eq!(element.direction().name(), "南");
    assert_eq!(Element::from_name("水").unwrap().index(), 4);
    assert_eq!(Element::from_index(4).next(1).name(), "木");
    assert_eq!(Element::from_name("土").unwrap().direction().name(), "中");

    let duty = Duty::new("建");
    assert_eq!(duty.name(), "建");
    assert_eq!(Duty::from_name("闭").unwrap().index(), 11);
    assert_eq!(Duty::from_name("闭").unwrap().next(1).name(), "建");

    let phase = Phase::new("望");
    assert_eq!(phase.name(), "望");

    let moon_phase = MoonPhase::from_name(2023, 8, "新月").unwrap();
    assert_eq!(moon_phase.index(), 0);
    assert_eq!(moon_phase.name(), "新月");
    assert_eq!(moon_phase.next(1).unwrap().name(), "蛾眉月");

    let luck = GodLuck::from_name("凶").unwrap();
    assert_eq!(luck.index(), 1);
    assert_eq!(luck.name(), "凶");
    assert_eq!(luck.next(1), GodLuck::Auspicious);

    let avoid = TabooKind::from_name("忌").unwrap();
    assert_eq!(avoid.index(), 1);
    assert_eq!(avoid.name(), "忌");
    assert_eq!(avoid.next(1), TabooKind::Recommend);

    let six_star = SixStar::from_name("佛灭").unwrap();
    assert_eq!(six_star.index(), 3);
    assert_eq!(six_star.next(1).name(), "大安");

    let seven_star = SevenStar::from_index(6);
    assert_eq!(seven_star.name(), "土");
    assert_eq!(seven_star.next(1).name(), "日");

    let twelve_star = TwelveStar::from_name("天刑").unwrap();
    assert_eq!(twelve_star.index(), 2);
    assert_eq!(twelve_star.ecliptic().name(), "黑道");
    assert_eq!(twelve_star.ecliptic().luck(), GodLuck::Inauspicious);

    let ten_star = TenStar::from_name("正印").unwrap();
    assert_eq!(ten_star.index(), 9);
    assert_eq!(ten_star.next(1).name(), "比肩");
}

#[test]
fn typed_constellation_wraps_solar_xing_zuo() {
    let aries = Constellation::from_name("白羊").unwrap();
    assert_eq!(aries.index(), 0);
    assert_eq!(aries.name(), "白羊");
    assert_eq!(aries.next(1).name(), "金牛");
    assert_eq!(aries.next(-1).name(), "双鱼");

    let solar = Solar::from_ymd(2019, 5, 1).unwrap();
    let constellation = solar.constellation();
    assert_eq!(constellation.name(), solar.xing_zuo());
    assert_eq!(constellation.name(), "金牛");
}

#[test]
fn common_culture_traits_unify_names_cycles_and_day_indices() {
    fn culture_name<T: NamedCulture>(value: &T) -> &str {
        value.name()
    }

    fn next_name<T: CycleItem>(value: T, offset: isize) -> String {
        value.next(offset).name().to_string()
    }

    assert_eq!(culture_name(&Element::new("火")), "火");
    assert_eq!(culture_name(&Duty::new("建")), "建");
    assert_eq!(culture_name(&GodLuck::Auspicious), "吉");
    assert_eq!(culture_name(&TabooKind::Recommend), "宜");
    assert_eq!(culture_name(&SixStar::from_name("先胜").unwrap()), "先胜");
    assert_eq!(culture_name(&TwelveStar::from_name("青龙").unwrap()), "青龙");
    assert_eq!(next_name(HeavenStem::from_name("癸").unwrap(), 1), "甲");
    assert_eq!(next_name(EarthBranch::from_name("亥").unwrap(), 1), "子");
    assert_eq!(next_name(SixtyCycle::from_name("癸亥").unwrap(), 1), "甲子");
    assert_eq!(next_name(Zodiac::from_name("猪").unwrap(), 1), "鼠");
    assert_eq!(next_name(Element::from_name("水").unwrap(), 1), "木");
    assert_eq!(next_name(Duty::from_name("闭").unwrap(), 1), "建");
    assert_eq!(next_name(GodLuck::Inauspicious, 1), "吉");
    assert_eq!(next_name(TabooKind::Avoid, 1), "宜");
    assert_eq!(next_name(SevenStar::from_name("土").unwrap(), 1), "日");
    assert_eq!(next_name(TenStar::from_name("正印").unwrap(), 1), "比肩");
    assert_eq!(next_name(Dipper::from_name("隐元").unwrap(), 1), "天枢");
    assert_eq!(next_name(MinorRen::from_name("空亡").unwrap(), 1), "大安");
    assert_eq!(Constellation::from_name("白羊").unwrap().steps_to(1), 1);
    assert_eq!(Constellation::from_name("白羊").unwrap().steps_back_to(11), -1);
    assert_eq!(Constellation::from_name("白羊").unwrap().steps_close_to(11), -1);

    let fu = Solar::from_ymd(2012, 7, 18).unwrap().lunar().fu_day().unwrap();
    assert_eq!(CultureDay::day_index(&fu), Some(1));

    let plum = Solar::from_ymd(2024, 7, 6).unwrap().lunar().plum_rain_day().unwrap();
    assert_eq!(CultureDay::day_index(&plum), None);
    assert!(plum.is_boundary());
}

#[test]
fn tyme_compatibility_type_names_map_to_existing_objects() {
    assert_eq!(Animal::from_name("蛟").unwrap().next(1).name(), "龙");
    assert_eq!(Luck::from_name("吉").unwrap().next(1).name(), "凶");
    assert_eq!(Sixty::from_name("上元").unwrap().next(1).name(), "中元");
    assert_eq!(Sound::from_name("海中金").unwrap().next(1).name(), "炉中火");
    assert_eq!(Ten::from_name("甲子").unwrap().next(1).name(), "甲戌");
    assert_eq!(Twenty::from_name("九运").unwrap().next(1).name(), "一运");

    let star = NineStar::from_index(8);
    assert_eq!(star.dipper().name(), "隐元");
    assert_eq!(star.dipper().next(1).name(), "天枢");
    assert_eq!(star.dipper().name(), star.name_in_bei_dou());
}

#[test]
fn tyme_core_day_time_and_term_names_are_available() {
    let solar_day: SolarDay = SolarDay::from_ymd(2024, 2, 10).unwrap();
    let solar_time: SolarTime = SolarTime::from_ymd_hms(2024, 2, 10, 8, 30, 0).unwrap();
    let lunar_day: LunarDay = LunarDay::from_ymd(2024, 1, 1).unwrap();
    fn accept_legal_holiday(_: Option<LegalHoliday>) {}
    accept_legal_holiday(None::<Holiday>);

    assert_eq!(solar_day.to_ymd(), "2024-02-10");
    assert_eq!(solar_time.to_ymd_hms(), "2024-02-10 08:30:00");
    assert_eq!(lunar_day.solar().to_ymd(), "2024-02-10");
    assert_eq!(solar_time.get_year(), 2024);
    assert_eq!(solar_time.get_month(), 2);
    assert_eq!(solar_time.get_day(), 10);
    assert_eq!(solar_time.get_hour(), 8);
    assert_eq!(solar_time.get_minute(), 30);
    assert_eq!(solar_time.get_second(), 0);
    assert_eq!(solar_time.get_solar_day().to_ymd_hms(), "2024-02-10 00:00:00");
    assert_eq!(solar_time.get_solar_time(), solar_time);
    assert_eq!(solar_time.get_lunar_day().solar().to_ymd(), "2024-02-10");
    let solar_lunar_hour = solar_time.get_lunar_hour();
    assert_eq!(solar_lunar_hour.get_lunar_day().solar().to_ymd(), "2024-02-10");
    assert_eq!(solar_lunar_hour.get_solar_time().to_ymd_hms(), "2024-02-10 08:30:00");
    assert_eq!(solar_lunar_hour.get_sixty_cycle_hour().name(), solar_time.lunar().sixty_cycle_hour().name());
    assert_eq!(solar_lunar_hour.get_minor_ren().name(), solar_time.lunar().time_minor_ren().name());
    assert_eq!(solar_time.get_week().name(), "六");
    assert!(solar_time.is_after(&SolarTime::from_ymd_hms(2024, 2, 10, 8, 29, 59).unwrap()));
    assert!(solar_time.is_before(&SolarTime::from_ymd_hms(2024, 2, 10, 8, 30, 1).unwrap()));
    assert_eq!(solar_day.get_solar_month().year(), 2024);
    assert_eq!(solar_day.get_solar_month().month(), 2);
    assert_eq!(solar_day.get_solar_week(0).first_day().to_ymd(), "2024-02-04");
    assert_eq!(solar_day.get_index_in_year(), 40);
    assert_eq!(solar_day.get_constellation(), Constellation::from_name("水瓶").unwrap());
    let hijri_day: HijriDay = solar_day.get_hijri_day();
    assert_eq!(hijri_day.solar().to_ymd(), "2024-02-10");
    assert_eq!(solar_day.get_sixty_cycle_day().name(), solar_day.lunar().sixty_cycle_day().name());
    assert_eq!(solar_time.get_sixty_cycle_hour().name(), solar_time.lunar().sixty_cycle_hour().name());
    assert_eq!(solar_day.get_phase().name(), solar_day.lunar().phase().name());
    assert_eq!(solar_day.get_phase_day().name(), solar_day.lunar().phase_day().name());
    assert_eq!(solar_day.get_nine_star().to_string(), solar_day.lunar().day_nine_star().to_string());
    assert_eq!(solar_day.get_rab_byung_day().unwrap().solar().to_ymd(), "2024-02-10");

    let jd = JulianDay::from_ymd_hms(2024, 2, 10, 0, 0, 0).unwrap();
    assert_eq!(jd.solar_day().to_ymd(), "2024-02-10");
    assert_eq!(jd.get_solar_day().to_ymd(), "2024-02-10");
    assert_eq!(jd.get_solar_time().to_ymd_hms(), "2024-02-10 00:00:00");
    assert_eq!(jd.get_week().name(), "六");
    assert_eq!(jd.next(1).subtract(jd), 1.0);
    assert_eq!(jd.next(1).solar_day().to_ymd(), "2024-02-11");

    let term: SolarTerm = Solar::from_ymd(2024, 2, 4).unwrap().lunar().current_jie_qi().unwrap();
    assert_eq!(term.name(), "立春");
    assert_eq!(term.index(), 3);
    assert!(term.is_jie());
    assert_eq!(term.next(1).name(), "雨水");

    let dong_zhi = SolarTerm::from_index(2023, 0);
    assert_eq!(dong_zhi.name(), "冬至");
    assert_eq!(dong_zhi.year(), 2023);
    assert_eq!(dong_zhi.get_index(), 0);
    assert!(dong_zhi.is_qi());
    assert_eq!(dong_zhi.get_solar_day().to_ymd(), "2022-12-22");
    assert_eq!(dong_zhi.get_julian_day().solar_day().to_ymd(), "2022-12-22");

    let da_xue = dong_zhi.next(23);
    assert_eq!(da_xue.name(), "大雪");
    assert_eq!(da_xue.year(), 2023);
    assert_eq!(da_xue.get_solar_day().to_ymd(), "2023-12-07");
    assert_eq!(da_xue.next(1).name(), "冬至");
    assert_eq!(da_xue.next(1).get_solar_day().to_ymd(), "2023-12-22");
    assert_eq!(SolarTerm::from_name(2023, "大雪").unwrap().get_solar_day().to_ymd(), "2023-12-07");
    assert_eq!(SolarTerm::new(2024, "未知"), None);

    let term_time = SolarTime::from_ymd_hms(2023, 12, 7, 18, 0, 0).unwrap();
    assert_eq!(term_time.get_term().name(), "大雪");
    assert_eq!(term_time.get_term_day().unwrap().name(), "大雪");

    let phenology_day = Solar::from_ymd(2021, 12, 21).unwrap();
    assert_eq!(phenology_day.get_phenology().wu_hou(), "蚯蚓结");
    assert_eq!(phenology_day.get_phenology_day().unwrap().name(), "蚯蚓结");

    let plum_day = Solar::from_ymd(2024, 6, 11).unwrap();
    assert_eq!(plum_day.get_plum_rain_day().unwrap().to_string(), "入梅第1天");

    let holiday = Solar::from_ymd(2024, 10, 1).unwrap().get_legal_holiday().unwrap();
    assert_eq!(holiday.get_name(), "国庆节");
    assert_eq!(holiday.get_day().to_ymd(), "2024-10-01");

    let festival = Solar::from_ymd(2024, 10, 1).unwrap().get_festival().unwrap();
    assert_eq!(festival.get_name(), "国庆节");
    assert_eq!(festival.get_day().to_ymd(), "2024-10-01");
    assert_eq!(festival.to_event().name(), "国庆节");

    assert_eq!(Solar::from_ymd(2022, 3, 28).unwrap().festivals().first().copied(), Some("全国中小学生安全教育日"));
    let indexed_festival = SolarFestival::from_index(2024, 3).unwrap();
    assert_eq!(indexed_festival.next(6).unwrap().get_name(), "国庆节");
    assert!(SolarFestival::from_ymd(1939, 5, 4).is_none());
    assert!(SolarFestival::from_ymd(1949, 10, 1).is_none());
    assert_eq!(SolarFestival::from_ymd(1950, 10, 1).unwrap().get_name(), "国庆节");

    let lunar_festival = Lunar::from_ymd(2024, 1, 1).unwrap().get_festival().unwrap();
    assert_eq!(lunar_festival.get_name(), "春节");
    assert_eq!(lunar_festival.get_day().solar().to_ymd(), "2024-02-10");
    assert_eq!(lunar_festival.to_event().name(), "春节");
    let winter_festival = Solar::from_ymd(2021, 12, 21).unwrap().lunar().get_festival().unwrap();
    assert_eq!(winter_festival.get_name(), "冬至节");
    assert_eq!(winter_festival.get_solar_term().unwrap().name(), "冬至");
    assert_eq!(LunarFestival::from_index(2021, 12).unwrap().get_name(), "除夕");
    assert!(LunarFestival::from_index(2024, 4).unwrap().get_solar_term().is_some());

    let lunar_day = solar_day.lunar();
    assert_eq!(lunar_day.get_year(), lunar_day.year());
    assert_eq!(lunar_day.get_month(), lunar_day.month());
    assert_eq!(lunar_day.get_day(), lunar_day.day());
    assert_eq!(lunar_day.get_solar_day().to_ymd(), "2024-02-10");
    assert_eq!(lunar_day.get_lunar_month().unwrap().month(), lunar_day.month());
    assert_eq!(lunar_day.get_week().name(), solar_day.get_week().name());
    assert_eq!(lunar_day.get_year_sixty_cycle().name(), lunar_day.year_sixty_cycle().name());
    assert_eq!(lunar_day.get_month_sixty_cycle().name(), lunar_day.month_sixty_cycle().name());
    assert_eq!(lunar_day.get_sixty_cycle().name(), lunar_day.day_sixty_cycle().name());
    assert_eq!(lunar_day.get_sixty_cycle_day().name(), lunar_day.sixty_cycle_day().name());
    assert_eq!(lunar_day.get_phase().name(), lunar_day.phase().name());
    assert_eq!(lunar_day.get_phase_day().name(), lunar_day.phase_day().name());
    assert_eq!(lunar_day.get_six_star().name(), lunar_day.liu_yao());
    assert_eq!(lunar_day.get_twelve_star().name(), lunar_day.day_tian_shen());
    assert_eq!(lunar_day.get_twenty_eight_star().name(), lunar_day.xiu());
    assert_eq!(lunar_day.get_fetus_day().to_string(), lunar_day.fetus_day().to_string());
    assert_eq!(lunar_day.get_nine_star().to_string(), lunar_day.day_nine_star().to_string());
    assert!(!lunar_day.get_gods().is_empty());
    assert!(!lunar_day.get_recommends().is_empty());
    assert!(!lunar_day.get_avoids().is_empty());
    assert_eq!(lunar_day.get_minor_ren().name(), lunar_day.minor_ren().name());
    assert_eq!(lunar_day.get_three_pillars().to_string(), lunar_day.eight_char().three_pillars().to_string());
    assert_eq!(lunar_day.get_hours().len(), 13);
    assert_eq!(lunar_day.get_hours().first().unwrap().get_solar_time().hour(), 0);
    assert_eq!(lunar_day.get_hours().last().unwrap().get_solar_time().hour(), 23);

    let lunar_hour = lunar_day.time();
    assert_eq!(lunar_hour.get_lunar_day().solar().to_ymd(), "2024-02-10");
    assert_eq!(lunar_hour.get_year(), lunar_day.year());
    assert_eq!(lunar_hour.get_month(), lunar_day.month());
    assert_eq!(lunar_hour.get_day(), lunar_day.day());
    assert_eq!(lunar_hour.get_hour(), lunar_day.hour());
    assert_eq!(lunar_hour.get_minute(), lunar_day.minute());
    assert_eq!(lunar_hour.get_second(), lunar_day.second());
    assert_eq!(lunar_hour.get_index_in_day(), ((lunar_hour.get_hour() + 1) / 2) as usize);
    assert_eq!(lunar_hour.get_year_sixty_cycle().name(), lunar_day.year_sixty_cycle().name());
    assert_eq!(lunar_hour.get_month_sixty_cycle().name(), lunar_day.month_sixty_cycle().name());
    assert_eq!(lunar_hour.get_day_sixty_cycle().name(), lunar_day.day_sixty_cycle().name());
    assert_eq!(lunar_hour.get_sixty_cycle().name(), lunar_hour.sixty_cycle().name());
    assert_eq!(lunar_hour.get_sixty_cycle_hour().name(), lunar_hour.sixty_cycle_hour().name());
    assert_eq!(lunar_hour.get_nine_star().to_string(), lunar_hour.nine_star().to_string());
    // The hour 黄黑道 twelve-star (`get_twelve_star`) is a distinct concept from the
    // 七曜 时天神 (`tian_shen`); the old `from_name(tian_shen())` impl wrongly
    // conflated them. `get_twelve_star` now uses the 建除 formula (matching the day
    // method and tyme4rs), and its values are pinned by the differential suite and
    // `tests/differential_protocol.rs` rather than equated to `tian_shen` here.
    assert!(!lunar_hour.get_twelve_star().name().is_empty());
    assert!(!lunar_hour.get_recommends().is_empty());
    assert!(!lunar_hour.get_avoids().is_empty());
    assert_eq!(lunar_hour.get_minor_ren().name(), lunar_hour.minor_ren().name());
    assert_eq!(lunar_hour.get_solar_time().to_ymd_hms(), solar_day.to_ymd_hms());
    assert_eq!(
        lunar_hour.get_eight_char().three_pillars().to_string(),
        lunar_day.eight_char().three_pillars().to_string()
    );

    let alias_holiday = LegalHoliday::from_ymd(2024, 10, 1).unwrap();
    assert_eq!(alias_holiday.get_target().to_ymd(), "2024-10-01");

    let solar_year = SolarYear::from_year(2024);
    assert!(solar_year.is_leap());
    assert_eq!(solar_year.get_year(), 2024);
    assert_eq!(solar_year.get_day_count(), 366);
    assert_eq!(solar_year.get_months().len(), 12);
    assert_eq!(solar_year.get_seasons().len(), 4);
    assert_eq!(solar_year.get_half_years()[1].get_index(), 1);
    assert_eq!(solar_year.get_rab_byung_year().unwrap().year(), 2024);

    let half_year = SolarHalfYear::from_index(2024, 1);
    assert_eq!(half_year.get_year(), 2024);
    assert_eq!(half_year.get_index(), 1);
    assert_eq!(half_year.get_solar_year().get_year(), 2024);
    assert_eq!(half_year.get_months()[0].month(), 7);
    assert_eq!(half_year.get_seasons()[0].get_index(), 2);

    let season = SolarSeason::from_index(2024, 2);
    assert_eq!(season.get_year(), 2024);
    assert_eq!(season.get_index(), 2);
    assert_eq!(season.get_solar_year().get_year(), 2024);
    assert_eq!(season.get_months().iter().map(|month| month.month()).collect::<Vec<_>>(), vec![7, 8, 9]);

    let month = SolarMonth::from_ym(2023, 1);
    assert_eq!(month.get_year(), 2023);
    assert_eq!(month.get_month(), 1);
    assert_eq!(month.get_solar_year().get_year(), 2023);
    assert_eq!(month.get_day_count(), 31);
    assert_eq!(month.get_index_in_year(), 0);
    assert_eq!(month.get_week_count(0), 5);
    assert_eq!(month.get_week_count(1), 6);
    assert_eq!(month.get_season().get_index(), 0);
    assert_eq!(month.get_first_day().to_ymd(), "2023-01-01");
    assert_eq!(month.get_days().len(), 31);
    assert_eq!(month.get_weeks(0).len(), 5);

    let week = SolarWeek::from_ym(2024, 2, 0, 0);
    assert_eq!(week.get_year(), 2024);
    assert_eq!(week.get_month(), 2);
    assert_eq!(week.get_start(), 0);
    assert_eq!(week.get_index(), 0);
    assert_eq!(week.get_solar_month().month(), 2);
    assert_eq!(week.get_first_day().to_ymd(), "2024-01-28");
    assert_eq!(week.get_days().len(), 7);
    assert_eq!(week.get_index_in_year(), 4);
}

#[test]
fn unit_objects_and_lunar_week_match_tyme_shapes() {
    assert_eq!(YearUnit::new(2024).compare_index(), 20_240_000);
    assert_eq!(MonthUnit::new(2024, -2).compare_index(), 20_240_500);
    assert_eq!(DayUnit::new(2024, -2, 3).compare_index(), 20_240_503);

    let second = SecondUnit::from_ymd_hms(2024, 2, 3, 4, 5, 6).unwrap();
    assert_eq!(second.seconds_in_day(), 14_706);
    assert!(SecondUnit::from_ymd_hms(2024, 2, 3, 24, 0, 0).is_none());

    let week_unit = WeekUnit::from_ym(2023, 8, 1, 0).unwrap();
    assert_eq!(week_unit.name(), "第二周");
    assert!(WeekUnit::from_ym(2023, 8, 6, 0).is_none());

    let week = LunarWeek::from_ym(2023, 8, 1, 0).unwrap();
    assert_eq!(week.name(), "第二周");
    assert_eq!(week.first_day().unwrap().solar().to_ymd(), "2023-09-17");
    assert_eq!(week.days().unwrap().len(), 7);
    assert_eq!(week.next(1).unwrap().name(), "第三周");
}

#[test]
fn typed_land_and_direction_follow_tyme_cycles() {
    let north = Direction::from_name("北").unwrap();
    assert_eq!(north.index(), 0);
    assert_eq!(north.next(1).name(), "西南");
    assert_eq!(Direction::from_index(8).next(1).name(), "北");

    let land = Land::from_name("炎天").unwrap();
    assert_eq!(land.index(), 8);
    assert_eq!(land.name(), "炎天");
    assert_eq!(land.direction().name(), "南");
    assert_eq!(land.next(1).name(), "玄天");
    assert_eq!(land.steps_close_to(0), 1);
    assert_eq!(Land::from_index(13).name(), "钧天");
}

#[test]
fn typed_minor_ren_matches_tyme_cycle_examples() {
    let lunar_day = Lunar::from_ymd(2024, 3, 5).unwrap();
    assert_eq!(lunar_day.minor_ren().name(), "大安");
    assert_eq!(lunar_day.day_minor_ren().luck(), GodLuck::Auspicious);
    assert_eq!(GodLuck::from_name(lunar_day.day_minor_ren().luck().name()).unwrap(), lunar_day.day_minor_ren().luck());
    assert_eq!(lunar_day.day_minor_ren().element().name(), "木");

    let lunar_hour = Lunar::from_ymd_hms(2024, 9, 7, 10, 0, 0).unwrap();
    assert_eq!(lunar_hour.time_minor_ren().name(), "留连");
    assert_eq!(lunar_hour.time().minor_ren(), lunar_hour.time_minor_ren());
    assert_eq!(lunar_hour.time_minor_ren().luck(), GodLuck::Inauspicious);
    assert_eq!(lunar_hour.time_minor_ren().element().name(), "水");
    let hour = lunar_rs::LunarTime::from_ymd_hms(2024, 9, 7, 10, 0, 0).unwrap();
    assert_eq!(hour.get_lunar_day().solar().to_ymd_hms(), lunar_hour.solar().to_ymd_hms());
    assert_eq!(hour.get_index_in_day(), 5);
    assert_eq!(hour.get_minor_ren().name(), "留连");
    assert_eq!(hour.get_nine_star().to_string(), lunar_hour.time_nine_star().to_string());

    let zi_hour = lunar_rs::LunarTime::from_ymd_hms(2023, 11, 14, 23, 0, 0).unwrap();
    assert_eq!(zi_hour.get_index_in_day(), 12);

    let lunar_month = LunarMonth::from_ym(1991, 3).unwrap();
    assert_eq!(lunar_month.minor_ren().name(), "速喜");
    assert_eq!(lunar_month.minor_ren().element().name(), "火");
}

#[test]
fn typed_nine_day_and_hide_heaven_stem_day_match_reference_examples() {
    let first = Solar::from_ymd(2020, 12, 21).unwrap().nine_day().unwrap();
    assert_eq!(first.name(), "一九");
    assert_eq!(first.nine().name(), "一九");
    assert_eq!(first.day_index(), Some(1));
    assert_eq!(first.to_string(), "一九第1天");

    let third = Solar::from_ymd(2021, 1, 8).unwrap().nine_day().unwrap();
    assert_eq!(third.name(), "三九");
    assert_eq!(third.day_index(), Some(1));
    assert!(Solar::from_ymd(2021, 7, 5).unwrap().nine_day().is_none());

    let hide = Solar::from_ymd(2024, 12, 4).unwrap().hide_heaven_stem_day().unwrap();
    assert_eq!(hide.hide_heaven_stem().kind(), HideHeavenStemType::Main);
    assert_eq!(hide.hide_heaven_stem().name(), "壬");
    assert_eq!(hide.name(), "壬水");
    assert_eq!(hide.day_index(), Some(16));
    assert_eq!(hide.to_string(), "壬水第16天");
}

#[test]
fn lunar_typed_api_exposes_cycle_zodiac_and_direction_objects() {
    let lunar = Solar::from_ymd(2024, 4, 22).unwrap().lunar();

    assert_eq!(lunar.year_heaven_stem().name(), "甲");
    assert_eq!(lunar.year_earth_branch().name(), "辰");
    assert_eq!(lunar.year_sixty_cycle().name(), "甲辰");
    assert_eq!(lunar.year_zodiac().name(), "龙");
    assert_eq!(lunar.zhi_xing_info(), Duty::from_name(lunar.zhi_xing()).unwrap());

    assert_eq!(lunar.month_heaven_stem().name(), "戊");
    assert_eq!(lunar.month_earth_branch().name(), "辰");
    assert_eq!(lunar.month_sixty_cycle().name(), "戊辰");
    assert_eq!(lunar.month_zodiac().name(), "龙");

    assert_eq!(lunar.day_heaven_stem().name(), "丙");
    assert_eq!(lunar.day_earth_branch().name(), "辰");
    assert_eq!(lunar.day_sixty_cycle().name(), "丙辰");
    assert_eq!(lunar.day_zodiac().name(), "龙");

    assert_eq!(lunar.time_heaven_stem().name(), "戊");
    assert_eq!(lunar.time_earth_branch().name(), "子");
    assert_eq!(lunar.time_sixty_cycle().name(), "戊子");
    assert_eq!(lunar.time_zodiac().name(), "鼠");

    assert_eq!(lunar.day_position_xi_direction().name(), "坤");
    assert_eq!(lunar.day_position_yang_gui_direction().name(), "兑");
    assert_eq!(lunar.day_position_yin_gui_direction().name(), "乾");
    assert_eq!(lunar.day_position_fu_direction().name(), "乾");
    assert_eq!(lunar.day_position_cai_direction().name(), "坤");

    assert_eq!(lunar.duty().name(), "建");
    assert_eq!(lunar.phase().name(), "小望");

    let phenology = Solar::from_ymd(2021, 12, 21).unwrap().lunar().phenology();
    assert_eq!(phenology.term(), "冬至");
    assert_eq!(phenology.three_hou(), "初候");
    assert_eq!(phenology.wu_hou(), "蚯蚓结");

    let lunar = Solar::from_ymd(2021, 12, 21).unwrap().lunar();
    let solar_term_day = lunar.solar_term_day().unwrap();
    assert_eq!(solar_term_day.name(), "冬至");
    assert_eq!(CultureDay::day_index(&solar_term_day), Some(1));

    let phenology_day = lunar.phenology_day().unwrap();
    assert_eq!(phenology_day.name(), "蚯蚓结");
    assert_eq!(CultureDay::day_index(&phenology_day), Some(1));

    let phase_day = lunar.phase_day();
    assert_eq!(phase_day.name(), lunar.phase().name());
    assert_eq!(CultureDay::day_index(&phase_day), Some(lunar.day()));

    let moon_phase_day = Solar::from_ymd(2023, 9, 17).unwrap().lunar().moon_phase_day().unwrap();
    assert_eq!(moon_phase_day.to_string(), "蛾眉月第2天");
    assert_eq!(moon_phase_day.phase(), MoonPhase::from_name(2023, 8, "蛾眉月").unwrap());
    assert_eq!(CultureDay::day_index(&moon_phase_day), Some(2));

    let peng_zu = lunar.peng_zu();
    assert_eq!(peng_zu.heaven_stem(), lunar.peng_zu_gan());
    assert_eq!(peng_zu.earth_branch(), lunar.peng_zu_zhi());
    assert_eq!(peng_zu.heaven_stem_item().name(), lunar.peng_zu_gan());
    assert_eq!(peng_zu.earth_branch_item().name(), lunar.peng_zu_zhi());
    assert_eq!(peng_zu.heaven_stem_item().steps_to(peng_zu.heaven_stem_item().next(1).index()), 1);
    assert_eq!(peng_zu.earth_branch_item().steps_to(peng_zu.earth_branch_item().next(1).index()), 1);
    assert_eq!(peng_zu.to_string(), format!("{} {}", lunar.peng_zu_gan(), lunar.peng_zu_zhi()));
}

#[test]
fn nine_star_typed_api_exposes_element_and_direction() {
    let star = NineStar::from_index(0);
    assert_eq!(star.element().name(), "水");
    assert_eq!(star.position_direction().name(), "坎");
}

#[test]
fn typed_god_and_taboo_api_wrap_existing_almanac_data() {
    let lunar = Solar::from_ymd(2004, 2, 16).unwrap().lunar();
    let gods = lunar.gods();

    let auspicious: Vec<_> =
        gods.iter().filter(|god| god.luck() == GodLuck::Auspicious).map(|god| god.name().to_string()).collect();
    let inauspicious: Vec<_> =
        gods.iter().filter(|god| god.luck() == GodLuck::Inauspicious).map(|god| god.name().to_string()).collect();

    let first_god = God::from_name("天恩").unwrap();
    assert_eq!(first_god.index(), Some(0));
    assert_eq!(first_god.luck(), GodLuck::Auspicious);
    assert_eq!(first_god.next(1).unwrap().name(), "鸣吠");
    assert_eq!(God::from_name("月煞").unwrap().luck(), GodLuck::Inauspicious);
    assert_eq!(God::new("自定义", GodLuck::Auspicious).index(), None);

    assert_eq!(auspicious, vec!["天恩", "续世", "明堂"]);
    assert_eq!(inauspicious, vec!["月煞", "月虚", "血支", "天贼", "五虚", "土符", "归忌", "血忌"]);

    let lunar = Solar::from_ymd(2024, 4, 22).unwrap().lunar();
    let recommends = lunar.time_recommends();
    let avoids = lunar.time_avoids();

    assert_eq!(
        recommends.iter().map(|taboo| taboo.name().to_string()).collect::<Vec<_>>(),
        vec!["订婚", "嫁娶", "求财", "开市", "交易", "安床", "祭祀"]
    );
    assert!(recommends.iter().all(|taboo| taboo.kind() == TabooKind::Recommend));

    let engagement = Taboo::from_name("订婚", TabooKind::Recommend).unwrap();
    assert_eq!(engagement.index(), Some(16));
    assert_eq!(engagement.kind(), TabooKind::Recommend);
    assert_eq!(engagement.next(1).unwrap().name(), "纳采");
    assert_eq!(engagement.next(1).unwrap().kind(), TabooKind::Recommend);

    assert_eq!(
        avoids.iter().map(|taboo| taboo.name().to_string()).collect::<Vec<_>>(),
        vec!["赴任", "修造", "移徙", "出行", "词讼", "祈福", "求嗣"]
    );
    assert!(avoids.iter().all(|taboo| taboo.kind() == TabooKind::Avoid));
    assert_eq!(Taboo::new("自定义", TabooKind::Avoid).index(), None);
}

#[test]
fn typed_dog_day_and_plum_rain_day_cover_reference_dates() {
    assert!(Solar::from_ymd(2011, 7, 13).unwrap().lunar().dog_day().is_none());
    assert!(Solar::from_ymd(2011, 8, 23).unwrap().lunar().dog_day().is_none());

    let first = Solar::from_ymd(2012, 7, 18).unwrap().lunar().dog_day().unwrap();
    assert_eq!(first.name(), "初伏");
    assert_eq!(first.day_index(), 1);
    assert_eq!(first.to_string(), "初伏第1天");

    let middle = Solar::from_ymd(2012, 8, 5).unwrap().lunar().dog_day().unwrap();
    assert_eq!(middle.name(), "中伏");
    assert_eq!(middle.day_index(), 9);
    assert_eq!(middle.to_string(), "中伏第9天");

    let last = Solar::from_ymd(2012, 8, 8).unwrap().lunar().dog_day().unwrap();
    assert_eq!(last.name(), "末伏");
    assert_eq!(last.day_index(), 2);
    assert_eq!(last.to_string(), "末伏第2天");

    assert!(Solar::from_ymd(2024, 6, 10).unwrap().lunar().plum_rain_day().is_none());

    let start = Solar::from_ymd(2024, 6, 11).unwrap().lunar().plum_rain_day().unwrap();
    assert_eq!(start.kind(), PlumRainKind::Entering);
    assert_eq!(start.day_index(), Some(1));
    assert_eq!(start.to_string(), "入梅第1天");

    let ongoing = Solar::from_ymd(2024, 7, 5).unwrap().lunar().plum_rain_day().unwrap();
    assert_eq!(ongoing.kind(), PlumRainKind::Entering);
    assert_eq!(ongoing.day_index(), Some(25));
    assert_eq!(ongoing.to_string(), "入梅第25天");

    let end = Solar::from_ymd(2024, 7, 6).unwrap().lunar().plum_rain_day().unwrap();
    assert_eq!(end.kind(), PlumRainKind::Leaving);
    assert_eq!(end.day_index(), None);
    assert_eq!(end.to_string(), "出梅");
}

#[test]
fn typed_tian_shen_xiu_lu_and_chong_sha_match_legacy_getters() {
    let lunar = Solar::from_ymd(2019, 5, 1).unwrap().lunar();

    let day_tian_shen = lunar.day_tian_shen_info();
    assert_eq!(day_tian_shen.name(), lunar.day_tian_shen());
    assert_eq!(day_tian_shen.kind().name(), lunar.day_tian_shen_type());
    assert_eq!(day_tian_shen.luck().label(), lunar.day_tian_shen_luck());

    let time_tian_shen = lunar.time_tian_shen_info();
    assert_eq!(time_tian_shen.name(), lunar.time_tian_shen());
    assert_eq!(time_tian_shen.kind().name(), lunar.time_tian_shen_type());
    assert_eq!(time_tian_shen.luck().label(), lunar.time_tian_shen_luck());

    let xiu = lunar.xiu_info();
    assert_eq!(xiu.name(), lunar.xiu());
    assert_eq!(xiu.luck().label(), lunar.xiu_luck());
    assert_eq!(xiu.zheng(), lunar.zheng());
    assert_eq!(xiu.animal().name(), lunar.animal());
    assert_eq!(xiu.animal().next(1).steps_back_to(xiu.animal().index()), -1);
    assert_eq!(xiu.gong().name(), lunar.gong());
    assert_eq!(xiu.zone().name(), lunar.gong());
    assert_eq!(xiu.zone().direction().name(), lunar.gong());
    assert_eq!(xiu.zone().beast().name(), lunar.shou());
    assert_eq!(xiu.zone().next(1).steps_back_to(xiu.zone().index()), -1);
    assert_eq!(xiu.shou().name(), lunar.shou());
    assert_eq!(xiu.shou().next(1).steps_back_to(xiu.shou().index()), -1);

    let lu = Solar::from_ymd(2017, 2, 15).unwrap().lunar().day_lu_info();
    assert_eq!(lu.mutual(), "子");
    assert_eq!(lu.advancing(), Some("辛"));
    assert_eq!(lu.to_string(), "子命互禄 辛命进禄");

    let lu = Solar::from_ymd(2017, 2, 16).unwrap().lunar().day_lu_info();
    assert_eq!(lu.mutual(), "寅");
    assert_eq!(lu.advancing(), None);
    assert_eq!(lu.to_string(), "寅命互禄");

    let day_chong_sha = lunar.day_chong_sha();
    assert_eq!(day_chong_sha.gan(), lunar.day_chong_gan());
    assert_eq!(day_chong_sha.branch(), lunar.day_chong());
    assert_eq!(day_chong_sha.zodiac().name(), lunar.day_chong_sheng_xiao());
    assert_eq!(day_chong_sha.sha().name(), lunar.day_sha());
    assert_eq!(day_chong_sha.to_string(), lunar.day_chong_desc());

    let time_chong_sha = lunar.time_chong_sha();
    assert_eq!(time_chong_sha.gan(), lunar.time_chong_gan());
    assert_eq!(time_chong_sha.branch(), lunar.time_chong());
    assert_eq!(time_chong_sha.zodiac().name(), lunar.time_chong_sheng_xiao());
    assert_eq!(time_chong_sha.sha().name(), lunar.time_sha());
    assert_eq!(time_chong_sha.to_string(), lunar.time_chong_desc());
}

#[test]
fn fu_and_shu_jiu_objects_share_day_level_shape() {
    let shu_jiu = Solar::from_ymd(2021, 12, 26).unwrap().lunar().shu_jiu_day().unwrap();
    assert_eq!(shu_jiu.name(), "一九");
    assert_eq!(shu_jiu.index(), 6);
    assert_eq!(shu_jiu.day_index(), 6);
    assert_eq!(shu_jiu.to_string(), "一九第6天");

    let fu = Solar::from_ymd(2012, 7, 18).unwrap().lunar().fu_day().unwrap();
    assert_eq!(fu.name(), "初伏");
    assert_eq!(fu.index(), 1);
    assert_eq!(fu.day_index(), 1);
    assert_eq!(fu.to_string(), "初伏第1天");
}

#[test]
fn typed_xun_and_tai_positions_match_legacy_getters() {
    let lunar = Solar::from_ymd(2019, 5, 1).unwrap().lunar();

    let year_xun = lunar.year_xun_info();
    assert_eq!(year_xun.name(), lunar.year_xun());
    assert_eq!(year_xun.kong().name(), lunar.year_xun_kong());

    let month_xun = lunar.month_xun_info();
    assert_eq!(month_xun.name(), lunar.month_xun());
    assert_eq!(month_xun.kong().name(), lunar.month_xun_kong());

    let day_xun = lunar.day_xun_info();
    assert_eq!(day_xun.name(), lunar.day_xun());
    assert_eq!(day_xun.kong().name(), lunar.day_xun_kong());
    assert_eq!(Xun::from_name(day_xun.name()).unwrap(), day_xun);
    assert_eq!(Xun::from_index(day_xun.index()).kong().name(), day_xun.kong().name());
    assert_eq!(Xun::from_name("甲寅").unwrap().next(1).name(), "甲子");
    let (first, second) = day_xun.kong().branches().unwrap();
    assert_eq!(format!("{}{}", first.name(), second.name()), lunar.day_xun_kong());

    let time_xun = lunar.time_xun_info();
    assert_eq!(time_xun.name(), lunar.time_xun());
    assert_eq!(time_xun.kong().name(), lunar.time_xun_kong());

    let year_tai_sui = lunar.year_tai_sui_position();
    assert_eq!(year_tai_sui.direction().name(), lunar.year_position_tai_sui());
    assert_eq!(year_tai_sui.description(), lunar.year_position_tai_sui_desc());
    assert_eq!(
        year_tai_sui.to_string(),
        format!("{}({})", lunar.year_position_tai_sui(), lunar.year_position_tai_sui_desc())
    );

    let month_tai_sui = lunar.month_tai_sui_position();
    assert_eq!(month_tai_sui.direction().name(), lunar.month_position_tai_sui());
    assert_eq!(month_tai_sui.description(), lunar.month_position_tai_sui_desc());

    let day_tai_sui = lunar.day_tai_sui_position();
    assert_eq!(day_tai_sui.direction().name(), lunar.day_position_tai_sui());
    assert_eq!(day_tai_sui.description(), lunar.day_position_tai_sui_desc());

    let day_tai = Solar::from_ymd(2021, 11, 13).unwrap().lunar().day_tai_position();
    assert_eq!(day_tai.name(), "碓磨厕 外东南");
    assert_eq!(day_tai.to_string(), "碓磨厕 外东南");

    let fetus_day = Solar::from_ymd(2021, 11, 13).unwrap().lunar().fetus_day();
    assert_eq!(fetus_day.name(), "碓磨厕 外东南");
    assert_eq!(fetus_day.heaven_stem().name(), "碓磨");
    assert_eq!(fetus_day.earth_branch().name(), "厕");
    assert_eq!(fetus_day.position().name(), day_tai.name());

    let month_tai = Solar::from_ymd(2021, 11, 13).unwrap().lunar().month_tai_position();
    assert_eq!(month_tai.name(), "占房床");

    let fetus_month = Solar::from_ymd(2021, 11, 13).unwrap().lunar().fetus_month().unwrap();
    assert_eq!(fetus_month.name(), month_tai.name());
    assert_eq!(fetus_month.month_index(), 9);
}

#[test]
fn typed_year_month_and_time_layers_reuse_domain_objects() {
    let year = LunarYear::from_year(2020);
    assert_eq!(year.heaven_stem().name(), year.gan());
    assert_eq!(year.earth_branch().name(), year.zhi());
    assert_eq!(year.sixty_cycle().name(), year.gan_zhi());
    assert_eq!(year.yuan_cycle().name(), year.yuan());
    assert_eq!(year.yun_cycle().name(), year.yun());
    assert_eq!(YuanCycle::from_name(year.yuan_cycle().name()).unwrap(), year.yuan_cycle());
    assert_eq!(YunCycle::from_name(year.yun_cycle().name()).unwrap(), year.yun_cycle());
    assert_eq!(year.yun_cycle().yuan_cycle(), year.yuan_cycle());
    assert_eq!(YuanCycle::from_name("下元").unwrap().next(1).name(), "上元");
    assert_eq!(YunCycle::from_name("九运").unwrap().next(1).name(), "一运");
    assert_eq!(year.nayin_info().name(), year.nayin());
    assert_eq!(year.nayin_info().element().name(), "土");
    assert_eq!(year.xun_info().name(), year.xun());
    assert_eq!(year.xun_info().kong().name(), year.xun_kong());
    assert_eq!(year.position_xi_direction().name(), year.position_xi());
    assert_eq!(year.position_yang_gui_direction().name(), year.position_yang_gui());
    assert_eq!(year.position_yin_gui_direction().name(), year.position_yin_gui());
    assert_eq!(year.position_fu_direction().name(), year.position_fu());
    assert_eq!(year.position_cai_direction().name(), year.position_cai());
    assert_eq!(year.tai_sui_position().direction().name(), year.position_tai_sui());
    assert_eq!(year.tai_sui_position().description(), year.position_tai_sui_desc());

    let month = LunarMonth::from_ym(2020, 4).unwrap();
    assert_eq!(month.heaven_stem().name(), month.gan());
    assert_eq!(month.earth_branch().name(), month.zhi());
    assert_eq!(month.sixty_cycle().name(), month.gan_zhi());
    assert_eq!(month.nayin_info().name(), month.nayin());
    assert_eq!(month.season_info().name(), month.season());
    assert_eq!(month.xun_info().name(), month.xun());
    assert_eq!(month.xun_info().kong().name(), month.xun_kong());
    assert_eq!(month.position_xi_direction().name(), month.position_xi());
    assert_eq!(month.position_yang_gui_direction().name(), month.position_yang_gui());
    assert_eq!(month.position_yin_gui_direction().name(), month.position_yin_gui());
    assert_eq!(month.position_fu_direction().name(), month.position_fu());
    assert_eq!(month.position_cai_direction().name(), month.position_cai());
    assert_eq!(month.tai_sui_position().direction().name(), month.position_tai_sui());
    assert_eq!(month.tai_sui_position().description(), month.position_tai_sui_desc());

    let lunar = Solar::from_ymd_hms(2019, 5, 1, 0, 30, 0).unwrap().lunar();
    let time = lunar.time();
    assert_eq!(time.heaven_stem().name(), time.gan());
    assert_eq!(time.earth_branch().name(), time.zhi());
    assert_eq!(time.sixty_cycle().name(), time.gan_zhi());
    assert_eq!(time.nayin_info().name(), time.nayin());
    assert_eq!(time.position_xi_direction().name(), time.position_xi());
    assert_eq!(time.position_yang_gui_direction().name(), time.position_yang_gui());
    assert_eq!(time.position_yin_gui_direction().name(), time.position_yin_gui());
    assert_eq!(time.position_fu_direction().name(), time.position_fu());
    assert_eq!(time.position_cai_direction().name(), time.position_cai());
    assert_eq!(time.tian_shen_info().name(), time.tian_shen());
    assert_eq!(time.tian_shen_info().kind().name(), time.tian_shen_type());
    assert_eq!(time.tian_shen_info().luck().label(), time.tian_shen_luck());
    assert_eq!(time.chong_sha().gan(), time.chong_gan());
    assert_eq!(time.chong_sha().branch(), time.chong());
    assert_eq!(time.chong_sha().zodiac().name(), time.chong_sheng_xiao());
    assert_eq!(time.chong_sha().sha().name(), time.sha());
    assert_eq!(time.chong_sha().to_string(), time.chong_desc());
    assert_eq!(time.xun_info().name(), time.xun());
    assert_eq!(time.xun_info().kong().name(), time.xun_kong());
}

#[test]
fn typed_year_fortunes_wrap_miscellaneous_predictions() {
    let year = LunarYear::from_year(2020);
    let fortunes = year.year_fortunes();
    let steed = year.kitchen_god_steed();

    assert_eq!(fortunes.len(), 14);
    assert_eq!(fortunes[0].kind().label(), "鼠偷粮");
    assert_eq!(fortunes[0].text(), year.tou_liang());
    assert_eq!(fortunes[2].kind().label(), "牛耕田");
    assert_eq!(fortunes[2].text(), year.geng_tian());
    assert_eq!(fortunes[4].kind().label(), "龙治水");
    assert_eq!(fortunes[4].text(), year.zhi_shui());
    assert_eq!(fortunes[13].kind().label(), "人几锄");
    assert_eq!(fortunes[13].text(), year.ren_chu());

    assert_eq!(steed.name(), "灶马头");
    assert_eq!(steed.mouse(), year.tou_liang());
    assert_eq!(steed.grass(), year.cao_zi());
    assert_eq!(steed.cattle(), year.geng_tian());
    assert_eq!(steed.flower(), year.hua_shou());
    assert_eq!(steed.dragon(), year.zhi_shui());
    assert_eq!(steed.horse(), year.tuo_gu());
    assert_eq!(steed.chicken(), year.qiang_mi());
    assert_eq!(steed.silkworm(), year.kan_can());
    assert_eq!(steed.pig(), year.gong_zhu());
    assert_eq!(steed.field(), year.jia_tian());
    assert_eq!(steed.cake(), year.fen_bing());
    assert_eq!(steed.gold(), year.de_jin());
    assert_eq!(steed.people_cakes(), year.ren_bing());
    assert_eq!(steed.people_hoes(), year.ren_chu());
    assert_eq!(steed.to_string(), "灶马头");
}

#[test]
fn lunar_year_and_month_strict_getters_match_tyme_names() {
    let year = LunarYear::from_year(2020);
    let months = year.get_months();

    assert_eq!(year.get_year(), 2020);
    assert_eq!(year.get_day_count(), year.day_count());
    assert_eq!(year.get_month_count(), 13);
    assert_eq!(months.len(), 13);
    assert_eq!(year.months().len(), 15);
    assert_eq!(year.get_leap_month(), 4);
    assert_eq!(year.get_sixty_cycle().name(), year.sixty_cycle().name());
    assert_eq!(year.get_twenty().name(), Twenty::from_index(7).name());
    assert_eq!(year.get_jupiter_direction().name(), Direction::from_index(0).name());
    assert_eq!(year.get_nine_star().to_string(), year.nine_star().to_string());
    assert_eq!(year.get_kitchen_god_steed().to_string(), year.kitchen_god_steed().to_string());

    let leap_month = LunarMonth::from_ym(2020, -4).unwrap();
    assert_eq!(leap_month.get_lunar_year().get_year(), 2020);
    assert_eq!(leap_month.get_year(), 2020);
    assert_eq!(leap_month.get_month(), 4);
    assert_eq!(leap_month.get_month_with_leap(), -4);
    assert_eq!(leap_month.get_day_count(), leap_month.day_count());
    assert_eq!(leap_month.get_index_in_year(), leap_month.index() as usize - 1);
    assert_eq!(leap_month.get_season().name(), leap_month.season());
    assert_eq!(leap_month.get_first_julian_day().day(), leap_month.first_julian_day());
    assert_eq!(leap_month.get_week_count(0), leap_month.get_weeks(0).len());
    assert_eq!(leap_month.get_days().len(), leap_month.day_count() as usize);
    assert_eq!(leap_month.get_first_day().unwrap().day(), 1);
    assert_eq!(leap_month.get_sixty_cycle().name(), leap_month.sixty_cycle().name());
    assert_eq!(leap_month.get_jupiter_direction().name(), Direction::from_index(3).name());
    assert_eq!(leap_month.get_nine_star().to_string(), leap_month.nine_star().to_string());
    assert!(leap_month.get_fetus().is_none());
    assert_eq!(leap_month.get_minor_ren().name(), leap_month.minor_ren().name());
    assert_eq!(Solar::from_ymd(2020, 5, 24).unwrap().lunar().get_six_star().name(), "先负");

    let normal_month = LunarMonth::from_ym(2020, 4).unwrap();
    assert_eq!(normal_month.get_fetus().unwrap().name(), normal_month.get_fetus().unwrap().position().name());
}

#[test]
fn typed_liu_yao_season_and_nayin_companions_reduce_string_parsing() {
    let lunar = Solar::from_ymd(2024, 4, 22).unwrap().lunar();
    assert_eq!(lunar.season_info().name(), lunar.season());
    assert_eq!(lunar.liu_yao_info().name(), lunar.liu_yao());
    assert_eq!(lunar.year_nayin_info().name(), lunar.year_nayin());
    assert_eq!(lunar.month_nayin_info().name(), lunar.month_nayin());
    assert_eq!(lunar.day_nayin_info().name(), lunar.day_nayin());
    assert_eq!(lunar.time_nayin_info().name(), lunar.time_nayin());
    assert_eq!(lunar.day_nayin_info().element().name(), "土");

    let day_nayin = lunar.day_nayin_info();
    assert_eq!(Nayin::from_name(day_nayin.name()).unwrap(), day_nayin);
    assert_eq!(Nayin::from_index(day_nayin.index()).name(), day_nayin.name());
    assert_eq!(day_nayin.next(1).steps_back_to(day_nayin.index()), -1);
}
