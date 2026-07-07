use lunar_rs::{
    Constellation, CultureDay, CycleItem, Direction, Duty, EarthBranch, Element, GodLuck, HeavenStem,
    HideHeavenStemType, Land, Lunar, LunarMonth, LunarYear, MinorRen, NamedCulture, Nayin, NineStar, Phase,
    PlumRainKind, SixtyCycle, Solar, TabooKind, Xun, YuanCycle, YunCycle, Zodiac,
};

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
    assert_eq!(next_name(HeavenStem::from_name("癸").unwrap(), 1), "甲");
    assert_eq!(next_name(EarthBranch::from_name("亥").unwrap(), 1), "子");
    assert_eq!(next_name(SixtyCycle::from_name("癸亥").unwrap(), 1), "甲子");
    assert_eq!(next_name(Zodiac::new("猪"), 1), "鼠");
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
    assert_eq!(lunar_day.day_minor_ren().element().name(), "木");

    let lunar_hour = Lunar::from_ymd_hms(2024, 9, 7, 10, 0, 0).unwrap();
    assert_eq!(lunar_hour.time_minor_ren().name(), "留连");
    assert_eq!(lunar_hour.time().minor_ren(), lunar_hour.time_minor_ren());
    assert_eq!(lunar_hour.time_minor_ren().luck(), GodLuck::Inauspicious);
    assert_eq!(lunar_hour.time_minor_ren().element().name(), "水");

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

    assert_eq!(
        avoids.iter().map(|taboo| taboo.name().to_string()).collect::<Vec<_>>(),
        vec!["赴任", "修造", "移徙", "出行", "词讼", "祈福", "求嗣"]
    );
    assert!(avoids.iter().all(|taboo| taboo.kind() == TabooKind::Avoid));
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

    assert_eq!(fortunes.len(), 14);
    assert_eq!(fortunes[0].kind().label(), "鼠偷粮");
    assert_eq!(fortunes[0].text(), year.tou_liang());
    assert_eq!(fortunes[2].kind().label(), "牛耕田");
    assert_eq!(fortunes[2].text(), year.geng_tian());
    assert_eq!(fortunes[4].kind().label(), "龙治水");
    assert_eq!(fortunes[4].text(), year.zhi_shui());
    assert_eq!(fortunes[13].kind().label(), "人几锄");
    assert_eq!(fortunes[13].text(), year.ren_chu());
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
