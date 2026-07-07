//! EightChar / Yun focused tests migrated from the reference implementations.

use lunar_rs::{
    DefaultEightCharProvider, EightCharProvider, Lunar, LunarSect1EightCharProvider, LunarSect2EightCharProvider,
    NamedCulture, SixtyCycleDay, SixtyCycleHour, SixtyCycleMonth, SixtyCycleYear,
};

#[test]
fn eight_char_bazi() {
    let lunar = Lunar::from_ymd_hms(2019, 12, 12, 11, 22, 0).unwrap();
    let ec = lunar.eight_char();
    assert_eq!(ec.year(), "己亥");
    assert_eq!(ec.month(), "丁丑");
    assert_eq!(ec.day(), "戊申");
    assert_eq!(ec.time(), "戊午");
}

#[test]
fn eight_char_exposes_typed_sixty_cycle_pillars() {
    fn pillar_name<T: NamedCulture>(pillar: &T) -> &str {
        pillar.name()
    }

    let lunar = Lunar::from_ymd_hms(2019, 12, 12, 11, 22, 0).unwrap();
    let ec = lunar.eight_char();

    let year: SixtyCycleYear = ec.year_pillar();
    let month: SixtyCycleMonth = ec.month_pillar();
    let day: SixtyCycleDay = ec.day_pillar();
    let hour: SixtyCycleHour = ec.time_pillar();

    assert_eq!(pillar_name(&year), ec.year());
    assert_eq!(month.heaven_stem().name(), ec.month_gan());
    assert_eq!(day.earth_branch().name(), ec.day_zhi());
    assert_eq!(hour.nayin().name(), ec.time_na_yin());

    let three = ec.three_pillars();
    assert_eq!(three.to_string(), format!("{} {} {}", ec.year(), ec.month(), ec.day()));
    assert_eq!(three.year().name(), year.name());
    assert_eq!(three.month().name(), month.name());
    assert_eq!(three.day().name(), day.name());
}

#[test]
fn eight_char_provider_api_preserves_default_and_allows_sect_selection() {
    let lunar = Lunar::from_ymd_hms(2019, 12, 12, 23, 30, 0).unwrap();

    let default_provider = DefaultEightCharProvider::new();
    let default_ec = lunar.eight_char_with_provider(&default_provider);
    assert_eq!(default_ec.sect(), 2);
    assert_eq!(default_ec.day(), lunar.eight_char().day());

    let sect1_provider = LunarSect1EightCharProvider::new();
    let sect1 = lunar.eight_char_with_provider(&sect1_provider);
    let mut manual = lunar.eight_char();
    manual.set_sect(1);
    assert_eq!(sect1.sect(), 1);
    assert_eq!(sect1.day(), manual.day());

    let sect2_provider = LunarSect2EightCharProvider::new();
    let provider: &dyn EightCharProvider = &sect2_provider;
    let sect2 = lunar.eight_char_with_provider(provider);
    assert_eq!(sect2.sect(), 2);
    assert_eq!(sect2.time(), lunar.eight_char().time());
}

#[test]
fn lunar_and_lunar_time_expose_typed_sixty_cycle_layers() {
    let lunar = Lunar::from_ymd_hms(2024, 3, 14, 0, 30, 0).unwrap();

    assert_eq!(lunar.sixty_cycle_year().name(), lunar.year_sixty_cycle().name());
    assert_eq!(lunar.sixty_cycle_month().name(), lunar.month_sixty_cycle().name());
    assert_eq!(lunar.sixty_cycle_day().name(), lunar.day_sixty_cycle().name());
    assert_eq!(lunar.sixty_cycle_hour().name(), lunar.time_sixty_cycle().name());
    assert_eq!(lunar.time().sixty_cycle_hour().name(), lunar.time_sixty_cycle().name());
}
