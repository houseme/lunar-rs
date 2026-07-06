use lunar_rs::{Direction, Duty, EarthBranch, Element, HeavenStem, NineStar, Phase, SixtyCycle, Solar};

#[test]
fn typed_primitives_map_names_and_relationships() {
    let jia = HeavenStem::from_name("甲").unwrap();
    assert_eq!(jia.index(), 0);
    assert_eq!(jia.name(), "甲");
    assert_eq!(jia.element().name(), "木");
    assert_eq!(jia.element().direction().name(), "东");

    let chen = EarthBranch::from_name("辰").unwrap();
    assert_eq!(chen.index(), 4);
    assert_eq!(chen.name(), "辰");
    assert_eq!(chen.zodiac().name(), "龙");
    assert_eq!(chen.element().name(), "土");

    let cycle = SixtyCycle::from_name("甲辰").unwrap();
    assert_eq!(cycle.index(), 40);
    assert_eq!(cycle.heaven_stem().name(), "甲");
    assert_eq!(cycle.earth_branch().name(), "辰");

    let direction = Direction::new("巽");
    assert_eq!(direction.name(), "巽");

    let element = Element::new("火");
    assert_eq!(element.direction().name(), "南");

    let duty = Duty::new("建");
    assert_eq!(duty.name(), "建");

    let phase = Phase::new("望");
    assert_eq!(phase.name(), "望");
}

#[test]
fn lunar_typed_api_exposes_cycle_zodiac_and_direction_objects() {
    let lunar = Solar::from_ymd(2024, 4, 22).unwrap().lunar();

    assert_eq!(lunar.year_heaven_stem().name(), "甲");
    assert_eq!(lunar.year_earth_branch().name(), "辰");
    assert_eq!(lunar.year_sixty_cycle().name(), "甲辰");
    assert_eq!(lunar.year_zodiac().name(), "龙");

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
}

#[test]
fn nine_star_typed_api_exposes_element_and_direction() {
    let star = NineStar::from_index(0);
    assert_eq!(star.element().name(), "水");
    assert_eq!(star.position_direction().name(), "坎");
}
