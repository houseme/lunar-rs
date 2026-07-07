//! Experimental internationalization helpers.
//!
//! The default API remains Chinese-first for compatibility and performance.
//! This module provides explicit language-aware helpers behind the `i18n`
//! feature without changing existing method signatures.

use std::borrow::Cow;

use crate::{
    anno_lucis::AnnoLucis, armenian::Armenian, assyrian::Assyrian, auc::Auc, bengali::Bengali, byzantine::Byzantine,
    coptic::Coptic, dangi::Dangi, ethiopian::Ethiopian, fasli::Fasli, foto::Foto, hispanic_era::HispanicEra,
    holocene::Holocene, japanese::Japanese, juche::Juche, julian::Julian, koki::Koki, lunar::Lunar, minguo::Minguo,
    nanakshahi::Nanakshahi, nine_star::NineStar, rattanakosin::Rattanakosin, rumi::Rumi, saka::Saka,
    seleucid::Seleucid, solar::Solar, tao::Tao, thai_buddhist::ThaiBuddhist, thai_solar::ThaiSolar, venetian::Venetian,
};

/// Supported output languages for explicit i18n helpers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    /// Simplified Chinese.
    ZhCn,
    /// English.
    En,
}

fn pick<'a>(language: Language, zh: &'a str, en: &'a str) -> &'a str {
    match language {
        Language::ZhCn => zh,
        Language::En => en,
    }
}

pub const fn locale(language: Language) -> Locale {
    Locale { language }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Locale {
    language: Language,
}

impl Locale {
    pub const fn language(&self) -> Language {
        self.language
    }

    pub const fn is_zh(&self) -> bool {
        matches!(self.language, Language::ZhCn)
    }

    pub fn year_label(&self) -> &'static str {
        pick(self.language, "年", "Year")
    }

    pub fn month_label(&self) -> &'static str {
        pick(self.language, "月", "Month")
    }

    pub fn day_label(&self) -> &'static str {
        pick(self.language, "日", "Day")
    }

    pub fn hour_label(&self) -> &'static str {
        pick(self.language, "时", "Hour")
    }

    pub fn leap_year_label(&self) -> &'static str {
        pick(self.language, "闰年", "Leap Year")
    }

    pub fn weekday_label(&self) -> &'static str {
        pick(self.language, "星期", "Weekday")
    }

    pub fn sign_label(&self) -> &'static str {
        pick(self.language, "座", "Sign")
    }

    pub fn na_yin_label(&self) -> &'static str {
        pick(self.language, "纳音", "NaYin")
    }

    pub fn direction_label(&self) -> &'static str {
        pick(self.language, "方", "Direction")
    }

    pub fn beast_label(&self) -> &'static str {
        pick(self.language, "", "Beast")
    }

    pub fn xiu_label(&self) -> &'static str {
        pick(self.language, "星宿", "Xiu")
    }

    pub fn peng_zu_label(&self) -> &'static str {
        pick(self.language, "彭祖百忌", "PengZu")
    }

    pub fn xi_position_label(&self) -> &'static str {
        pick(self.language, "喜神方位", "Xi Position")
    }

    pub fn yang_gui_position_label(&self) -> &'static str {
        pick(self.language, "阳贵神方位", "YangGui Position")
    }

    pub fn yin_gui_position_label(&self) -> &'static str {
        pick(self.language, "阴贵神方位", "YinGui Position")
    }

    pub fn fu_position_label(&self) -> &'static str {
        pick(self.language, "福神方位", "Fu Position")
    }

    pub fn cai_position_label(&self) -> &'static str {
        pick(self.language, "财神方位", "Cai Position")
    }

    pub fn chong_label(&self) -> &'static str {
        pick(self.language, "冲", "Chong")
    }

    pub fn sha_label(&self) -> &'static str {
        pick(self.language, "煞", "Sha")
    }

    pub fn buddhist_prefix(&self) -> &'static str {
        pick(self.language, "佛历", "Buddhist")
    }

    pub fn taoist_prefix(&self) -> &'static str {
        pick(self.language, "道历", "Taoist")
    }

    pub fn minguo_prefix(&self) -> &'static str {
        pick(self.language, "民国", "Minguo")
    }

    pub fn thai_solar_prefix(&self) -> &'static str {
        pick(self.language, "泰历", "ThaiSolar")
    }

    pub fn japanese_prefix(&self) -> &'static str {
        pick(self.language, "和历", "Japanese")
    }

    pub fn juche_prefix(&self) -> &'static str {
        pick(self.language, "主体", "Juche")
    }

    pub fn dangi_prefix(&self) -> &'static str {
        pick(self.language, "檀纪", "Dangi")
    }

    pub fn julian_prefix(&self) -> &'static str {
        pick(self.language, "儒略历", "Julian")
    }

    pub fn holocene_prefix(&self) -> &'static str {
        pick(self.language, "全新世纪", "Holocene")
    }

    pub fn byzantine_prefix(&self) -> &'static str {
        pick(self.language, "拜占庭纪年", "Byzantine")
    }

    pub fn coptic_prefix(&self) -> &'static str {
        pick(self.language, "科普特", "Coptic")
    }

    pub fn armenian_prefix(&self) -> &'static str {
        pick(self.language, "亚美尼亚", "Armenian")
    }

    pub fn anno_lucis_prefix(&self) -> &'static str {
        pick(self.language, "光明纪年", "AnnoLucis")
    }

    pub fn auc_prefix(&self) -> &'static str {
        pick(self.language, "建城纪年", "AUC")
    }

    pub fn assyrian_prefix(&self) -> &'static str {
        pick(self.language, "亚述", "Assyrian")
    }

    pub fn hispanic_era_prefix(&self) -> &'static str {
        pick(self.language, "西班牙纪元", "HispanicEra")
    }

    pub fn saka_prefix(&self) -> &'static str {
        pick(self.language, "萨卡", "Saka")
    }

    pub fn bengali_prefix(&self) -> &'static str {
        pick(self.language, "孟加拉", "Bengali")
    }

    pub fn koki_prefix(&self) -> &'static str {
        pick(self.language, "皇纪", "Koki")
    }

    pub fn thai_buddhist_prefix(&self) -> &'static str {
        pick(self.language, "泰佛历", "ThaiBuddhist")
    }

    pub fn fasli_prefix(&self) -> &'static str {
        pick(self.language, "法斯里", "Fasli")
    }

    pub fn nanakshahi_prefix(&self) -> &'static str {
        pick(self.language, "纳纳克沙希", "Nanakshahi")
    }

    pub fn seleucid_prefix(&self) -> &'static str {
        pick(self.language, "塞琉古", "Seleucid")
    }

    pub fn rattanakosin_prefix(&self) -> &'static str {
        pick(self.language, "拉达那哥欣", "Rattanakosin")
    }

    pub fn venetian_prefix(&self) -> &'static str {
        pick(self.language, "威尼斯", "Venetian")
    }

    pub fn rumi_prefix(&self) -> &'static str {
        pick(self.language, "鲁米历", "Rumi")
    }

    pub fn ethiopian_prefix(&self) -> &'static str {
        pick(self.language, "埃塞", "Ethiopian")
    }

    pub fn tian_yun_label(&self) -> &'static str {
        pick(self.language, "天运", "TianYun")
    }

    pub fn lunar_prefix(&self) -> &'static str {
        pick(self.language, "农历", "Lunar")
    }

    pub fn leap_label(&self) -> &'static str {
        pick(self.language, "闰", "Leap")
    }

    pub fn format_ganzhi_with_unit(&self, ganzhi: &str, zodiac_name: &str, unit_label: &str) -> String {
        match self.language {
            Language::ZhCn => format!("{ganzhi}({zodiac_name}) {unit_label}"),
            Language::En => format!("{ganzhi}({zodiac_name}) {unit_label}"),
        }
    }

    pub fn format_position(&self, title: &str, position: &str, desc: &str) -> String {
        format!("{title} [{position}]({desc})")
    }

    pub fn append_named_items<'a>(&self, out: &mut String, names: impl IntoIterator<Item = &'a str>) {
        for name in names {
            out.push_str(" (");
            out.push_str(self.named_item(name).as_ref());
            out.push(')');
        }
    }

    pub fn named_item<'a>(&self, name: &'a str) -> Cow<'a, str> {
        match self.language {
            Language::ZhCn => Cow::Borrowed(name),
            Language::En => translate_named_item_en(name),
        }
    }

    pub fn render_solar_string(&self, solar: &Solar) -> String {
        solar.to_ymd()
    }

    pub fn render_solar_full(&self, solar: &Solar) -> String {
        if self.is_zh() {
            return solar.to_full_string();
        }

        let mut s = solar.to_ymd_hms();
        if solar.is_leap_year() {
            s.push(' ');
            s.push_str(self.leap_year_label());
        }
        s.push(' ');
        s.push_str(self.weekday_label());
        s.push(' ');
        s.push_str(solar.week_in_lang(self.language));
        self.append_named_items(&mut s, solar.festivals());
        self.append_named_items(&mut s, solar.other_festivals());
        s.push(' ');
        s.push_str(solar.xing_zuo_in_lang(self.language));
        s.push(' ');
        s.push_str(self.sign_label());
        s
    }

    pub fn render_lunar_string(&self, lunar: &Lunar) -> String {
        match self.language {
            Language::ZhCn => lunar.to_string(),
            Language::En => {
                let month = if lunar.month() < 0 {
                    format!("{}{:02}", self.leap_label(), lunar.month().abs())
                } else {
                    format!("{:02}", lunar.month())
                };
                format!("{} {}-{}-{:02}", self.lunar_prefix(), lunar.year(), month, lunar.day())
            }
        }
    }

    pub fn render_lunar_full(&self, lunar: &Lunar) -> String {
        if self.is_zh() {
            return lunar.to_full_string();
        }

        let mut s = String::new();
        s.push_str(&self.render_lunar_string(lunar));
        s.push(' ');
        s.push_str(&self.format_ganzhi_with_unit(
            &lunar.year_in_gan_zhi_in_lang(self.language),
            lunar.year_sheng_xiao_in_lang(self.language),
            self.year_label(),
        ));
        s.push(' ');
        s.push_str(&self.format_ganzhi_with_unit(
            &lunar.month_in_gan_zhi_in_lang(self.language),
            lunar.month_sheng_xiao_in_lang(self.language),
            self.month_label(),
        ));
        s.push(' ');
        s.push_str(&self.format_ganzhi_with_unit(
            &lunar.day_in_gan_zhi_in_lang(self.language),
            lunar.day_sheng_xiao_in_lang(self.language),
            self.day_label(),
        ));
        s.push(' ');
        s.push_str(&self.format_ganzhi_with_unit(
            &lunar.time_in_gan_zhi_in_lang(self.language),
            lunar.time_sheng_xiao_in_lang(self.language),
            self.hour_label(),
        ));
        s.push(' ');
        s.push_str(self.na_yin_label());
        s.push_str(" [");
        s.push_str(lunar.year_nayin_in_lang(self.language));
        s.push(' ');
        s.push_str(lunar.month_nayin_in_lang(self.language));
        s.push(' ');
        s.push_str(lunar.day_nayin_in_lang(self.language));
        s.push(' ');
        s.push_str(lunar.time_nayin_in_lang(self.language));
        s.push(']');
        s.push(' ');
        s.push_str(self.weekday_label());
        s.push(' ');
        s.push_str(lunar.week_in_lang(self.language));
        self.append_named_items(&mut s, lunar.festivals());
        self.append_named_items(&mut s, lunar.other_festivals());
        let jq = lunar.jie_qi_in_lang(self.language);
        if !jq.is_empty() {
            s.push_str(" [");
            s.push_str(jq);
            s.push(']');
        }
        s.push(' ');
        s.push_str(self.direction_label());
        s.push(' ');
        s.push_str(lunar.gong_in_lang(self.language));
        s.push(' ');
        s.push_str(self.beast_label());
        s.push(' ');
        s.push_str(lunar.shou_in_lang(self.language));
        s.push(' ');
        s.push_str(self.xiu_label());
        s.push_str(" [");
        s.push_str(lunar.xiu_in_lang(self.language));
        s.push(' ');
        s.push_str(lunar.zheng_in_lang(self.language));
        s.push(' ');
        s.push_str(lunar.animal_in_lang(self.language));
        s.push_str("](");
        s.push_str(lunar.xiu_luck_in_lang(self.language));
        s.push(')');
        s.push(' ');
        s.push_str(self.peng_zu_label());
        s.push_str(" [");
        s.push_str(lunar.peng_zu_gan_in_lang(self.language));
        s.push(' ');
        s.push_str(lunar.peng_zu_zhi_in_lang(self.language));
        s.push(']');
        s.push(' ');
        s.push_str(&self.format_position(
            self.xi_position_label(),
            direction(lunar.day_position_xi(), self.language),
            lunar.day_position_xi_desc_in_lang(self.language),
        ));
        s.push(' ');
        s.push_str(&self.format_position(
            self.yang_gui_position_label(),
            direction(lunar.day_position_yang_gui(), self.language),
            lunar.day_position_yang_gui_desc_in_lang(self.language),
        ));
        s.push(' ');
        s.push_str(&self.format_position(
            self.yin_gui_position_label(),
            direction(lunar.day_position_yin_gui(), self.language),
            lunar.day_position_yin_gui_desc_in_lang(self.language),
        ));
        s.push(' ');
        s.push_str(&self.format_position(
            self.fu_position_label(),
            direction(lunar.day_position_fu(), self.language),
            lunar.day_position_fu_desc_in_lang(self.language),
        ));
        s.push(' ');
        s.push_str(&self.format_position(
            self.cai_position_label(),
            direction(lunar.day_position_cai(), self.language),
            lunar.day_position_cai_desc_in_lang(self.language),
        ));
        s.push(' ');
        s.push_str(self.chong_label());
        s.push_str(" [");
        s.push_str(&lunar.day_chong_desc_in_lang(self.language));
        s.push_str("] ");
        s.push_str(self.sha_label());
        s.push_str(" [");
        s.push_str(lunar.day_sha_in_lang(self.language));
        s.push(']');
        s
    }

    pub fn render_foto_string(&self, foto: &Foto<'_>) -> String {
        match self.language {
            Language::ZhCn => foto.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.buddhist_prefix(), foto.year(), foto.month().abs(), foto.day())
            }
        }
    }

    pub fn render_foto_full(&self, foto: &Foto<'_>) -> String {
        if self.is_zh() {
            return foto.to_full_string();
        }
        let mut s = self.render_foto_string(foto);
        self.append_named_items(&mut s, foto.festivals().iter().map(|festival| festival.name()));
        s
    }

    pub fn render_tao_string(&self, tao: &Tao<'_>) -> String {
        match self.language {
            Language::ZhCn => tao.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.taoist_prefix(), tao.year(), tao.month().abs(), tao.day())
            }
        }
    }

    pub fn render_tao_full(&self, tao: &Tao<'_>) -> String {
        if self.is_zh() {
            return tao.to_full_string();
        }
        let tian_yun_year = format!("{} {}", tao.lunar().year_in_gan_zhi_in_lang(self.language), self.year_label(),);
        format!(
            "{} {} {}, {} {}. {} {}, {} {}. {} {} {}, {} {}.",
            self.taoist_prefix(),
            tao.year(),
            self.year_label(),
            self.tian_yun_label(),
            tian_yun_year,
            tao.lunar().month_in_gan_zhi_in_lang(self.language),
            self.month_label(),
            tao.lunar().day_in_gan_zhi_in_lang(self.language),
            self.day_label(),
            tao.month(),
            tao.day(),
            self.day_label(),
            tao.lunar().time_in_gan_zhi_in_lang(self.language),
            self.hour_label(),
        )
    }

    pub fn render_minguo_string(&self, minguo: &Minguo) -> String {
        match self.language {
            Language::ZhCn => minguo.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.minguo_prefix(), minguo.year(), minguo.month(), minguo.day())
            }
        }
    }

    pub fn render_minguo_full(&self, minguo: &Minguo) -> String {
        if self.is_zh() {
            return minguo.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.minguo_prefix(),
            minguo.year(),
            self.year_label(),
            minguo.month(),
            self.month_label(),
            minguo.day(),
            minguo.solar().to_ymd(),
            self.weekday_label(),
            minguo.solar().week_in_lang(self.language),
        )
    }

    pub fn render_thai_solar_string(&self, thai: &ThaiSolar) -> String {
        match self.language {
            Language::ZhCn => thai.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.thai_solar_prefix(), thai.year(), thai.month(), thai.day())
            }
        }
    }

    pub fn render_thai_solar_full(&self, thai: &ThaiSolar) -> String {
        if self.is_zh() {
            return thai.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.thai_solar_prefix(),
            thai.year(),
            self.year_label(),
            thai.month(),
            self.month_label(),
            thai.day(),
            thai.solar().to_ymd(),
            self.weekday_label(),
            thai.solar().week_in_lang(self.language),
        )
    }

    pub fn render_japanese_string(&self, japanese: &Japanese) -> String {
        match self.language {
            Language::ZhCn => japanese.to_string_cn(),
            Language::En => format!(
                "{} {} {}-{:02}-{:02}",
                self.japanese_prefix(),
                japanese.era().name(),
                japanese.year(),
                japanese.month(),
                japanese.day()
            ),
        }
    }

    pub fn render_japanese_full(&self, japanese: &Japanese) -> String {
        if self.is_zh() {
            return japanese.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.japanese_prefix(),
            japanese.era().name(),
            japanese.year(),
            self.year_label(),
            japanese.month(),
            japanese.day(),
            japanese.solar().to_ymd(),
            self.weekday_label(),
            japanese.solar().week_in_lang(self.language),
        )
    }

    pub fn render_juche_string(&self, juche: &Juche) -> String {
        match self.language {
            Language::ZhCn => juche.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.juche_prefix(), juche.year(), juche.month(), juche.day())
            }
        }
    }

    pub fn render_juche_full(&self, juche: &Juche) -> String {
        if self.is_zh() {
            return juche.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.juche_prefix(),
            juche.year(),
            self.year_label(),
            juche.month(),
            self.month_label(),
            juche.day(),
            juche.solar().to_ymd(),
            self.weekday_label(),
            juche.solar().week_in_lang(self.language),
        )
    }

    pub fn render_dangi_string(&self, dangi: &Dangi) -> String {
        match self.language {
            Language::ZhCn => dangi.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.dangi_prefix(), dangi.year(), dangi.month(), dangi.day())
            }
        }
    }

    pub fn render_dangi_full(&self, dangi: &Dangi) -> String {
        if self.is_zh() {
            return dangi.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.dangi_prefix(),
            dangi.year(),
            self.year_label(),
            dangi.month(),
            self.month_label(),
            dangi.day(),
            dangi.solar().to_ymd(),
            self.weekday_label(),
            dangi.solar().week_in_lang(self.language),
        )
    }

    pub fn render_julian_string(&self, julian: &Julian) -> String {
        match self.language {
            Language::ZhCn => julian.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.julian_prefix(), julian.year(), julian.month(), julian.day())
            }
        }
    }

    pub fn render_julian_full(&self, julian: &Julian) -> String {
        if self.is_zh() {
            return julian.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.julian_prefix(),
            julian.year(),
            self.year_label(),
            julian.month(),
            self.month_label(),
            julian.day(),
            julian.solar().to_ymd(),
            self.weekday_label(),
            julian.solar().week_in_lang(self.language),
        )
    }

    pub fn render_holocene_string(&self, holocene: &Holocene) -> String {
        match self.language {
            Language::ZhCn => holocene.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.holocene_prefix(), holocene.year(), holocene.month(), holocene.day())
            }
        }
    }

    pub fn render_holocene_full(&self, holocene: &Holocene) -> String {
        if self.is_zh() {
            return holocene.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.holocene_prefix(),
            holocene.year(),
            self.year_label(),
            holocene.month(),
            self.month_label(),
            holocene.day(),
            holocene.solar().to_ymd(),
            self.weekday_label(),
            holocene.solar().week_in_lang(self.language),
        )
    }

    pub fn render_byzantine_string(&self, byzantine: &Byzantine) -> String {
        match self.language {
            Language::ZhCn => byzantine.to_string_cn(),
            Language::En => format!(
                "{} {}-{:02}-{:02}",
                self.byzantine_prefix(),
                byzantine.year(),
                byzantine.month(),
                byzantine.day()
            ),
        }
    }

    pub fn render_byzantine_full(&self, byzantine: &Byzantine) -> String {
        if self.is_zh() {
            return byzantine.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.byzantine_prefix(),
            byzantine.year(),
            self.year_label(),
            byzantine.month(),
            self.month_label(),
            byzantine.day(),
            byzantine.solar().to_ymd(),
            self.weekday_label(),
            byzantine.solar().week_in_lang(self.language),
        )
    }

    pub fn render_coptic_string(&self, coptic: &Coptic) -> String {
        match self.language {
            Language::ZhCn => coptic.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.coptic_prefix(), coptic.year(), coptic.month(), coptic.day())
            }
        }
    }

    pub fn render_coptic_full(&self, coptic: &Coptic) -> String {
        if self.is_zh() {
            return coptic.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.coptic_prefix(),
            coptic.year(),
            self.year_label(),
            coptic.month(),
            self.month_label(),
            coptic.day(),
            coptic.solar().to_ymd(),
            self.weekday_label(),
            coptic.solar().week_in_lang(self.language),
        )
    }

    pub fn render_armenian_string(&self, armenian: &Armenian) -> String {
        match self.language {
            Language::ZhCn => armenian.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.armenian_prefix(), armenian.year(), armenian.month(), armenian.day())
            }
        }
    }

    pub fn render_armenian_full(&self, armenian: &Armenian) -> String {
        if self.is_zh() {
            return armenian.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.armenian_prefix(),
            armenian.year(),
            self.year_label(),
            armenian.month(),
            self.month_label(),
            armenian.day(),
            armenian.solar().to_ymd(),
            self.weekday_label(),
            armenian.solar().week_in_lang(self.language),
        )
    }

    pub fn render_anno_lucis_string(&self, anno_lucis: &AnnoLucis) -> String {
        match self.language {
            Language::ZhCn => anno_lucis.to_string_cn(),
            Language::En => format!(
                "{} {}-{:02}-{:02}",
                self.anno_lucis_prefix(),
                anno_lucis.year(),
                anno_lucis.month(),
                anno_lucis.day()
            ),
        }
    }

    pub fn render_anno_lucis_full(&self, anno_lucis: &AnnoLucis) -> String {
        if self.is_zh() {
            return anno_lucis.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.anno_lucis_prefix(),
            anno_lucis.year(),
            self.year_label(),
            anno_lucis.month(),
            self.month_label(),
            anno_lucis.day(),
            anno_lucis.solar().to_ymd(),
            self.weekday_label(),
            anno_lucis.solar().week_in_lang(self.language),
        )
    }

    pub fn render_auc_string(&self, auc: &Auc) -> String {
        match self.language {
            Language::ZhCn => auc.to_string_cn(),
            Language::En => format!("{} {}-{:02}-{:02}", self.auc_prefix(), auc.year(), auc.month(), auc.day()),
        }
    }

    pub fn render_auc_full(&self, auc: &Auc) -> String {
        if self.is_zh() {
            return auc.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.auc_prefix(),
            auc.year(),
            self.year_label(),
            auc.month(),
            self.month_label(),
            auc.day(),
            auc.solar().to_ymd(),
            self.weekday_label(),
            auc.solar().week_in_lang(self.language),
        )
    }

    pub fn render_assyrian_string(&self, assyrian: &Assyrian) -> String {
        match self.language {
            Language::ZhCn => assyrian.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.assyrian_prefix(), assyrian.year(), assyrian.month(), assyrian.day())
            }
        }
    }

    pub fn render_assyrian_full(&self, assyrian: &Assyrian) -> String {
        if self.is_zh() {
            return assyrian.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.assyrian_prefix(),
            assyrian.year(),
            self.year_label(),
            assyrian.month(),
            self.month_label(),
            assyrian.day(),
            assyrian.solar().to_ymd(),
            self.weekday_label(),
            assyrian.solar().week_in_lang(self.language),
        )
    }

    pub fn render_hispanic_era_string(&self, era: &HispanicEra) -> String {
        match self.language {
            Language::ZhCn => era.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.hispanic_era_prefix(), era.year(), era.month(), era.day())
            }
        }
    }

    pub fn render_hispanic_era_full(&self, era: &HispanicEra) -> String {
        if self.is_zh() {
            return era.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.hispanic_era_prefix(),
            era.year(),
            self.year_label(),
            era.month(),
            self.month_label(),
            era.day(),
            era.solar().to_ymd(),
            self.weekday_label(),
            era.solar().week_in_lang(self.language),
        )
    }

    pub fn render_saka_string(&self, saka: &Saka) -> String {
        match self.language {
            Language::ZhCn => saka.to_string_cn(),
            Language::En => format!("{} {}-{:02}-{:02}", self.saka_prefix(), saka.year(), saka.month(), saka.day()),
        }
    }

    pub fn render_saka_full(&self, saka: &Saka) -> String {
        if self.is_zh() {
            return saka.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.saka_prefix(),
            saka.year(),
            self.year_label(),
            saka.month(),
            self.month_label(),
            saka.day(),
            saka.solar().to_ymd(),
            self.weekday_label(),
            saka.solar().week_in_lang(self.language),
        )
    }

    pub fn render_bengali_string(&self, bengali: &Bengali) -> String {
        match self.language {
            Language::ZhCn => bengali.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.bengali_prefix(), bengali.year(), bengali.month(), bengali.day())
            }
        }
    }

    pub fn render_bengali_full(&self, bengali: &Bengali) -> String {
        if self.is_zh() {
            return bengali.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.bengali_prefix(),
            bengali.year(),
            self.year_label(),
            bengali.month(),
            self.month_label(),
            bengali.day(),
            bengali.solar().to_ymd(),
            self.weekday_label(),
            bengali.solar().week_in_lang(self.language),
        )
    }

    pub fn render_koki_string(&self, koki: &Koki) -> String {
        match self.language {
            Language::ZhCn => koki.to_string_cn(),
            Language::En => format!("{} {}-{:02}-{:02}", self.koki_prefix(), koki.year(), koki.month(), koki.day()),
        }
    }

    pub fn render_koki_full(&self, koki: &Koki) -> String {
        if self.is_zh() {
            return koki.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.koki_prefix(),
            koki.year(),
            self.year_label(),
            koki.month(),
            self.month_label(),
            koki.day(),
            koki.solar().to_ymd(),
            self.weekday_label(),
            koki.solar().week_in_lang(self.language),
        )
    }

    pub fn render_thai_buddhist_string(&self, thai_buddhist: &ThaiBuddhist) -> String {
        match self.language {
            Language::ZhCn => thai_buddhist.to_string_cn(),
            Language::En => format!(
                "{} {}-{:02}-{:02}",
                self.thai_buddhist_prefix(),
                thai_buddhist.year(),
                thai_buddhist.month(),
                thai_buddhist.day()
            ),
        }
    }

    pub fn render_thai_buddhist_full(&self, thai_buddhist: &ThaiBuddhist) -> String {
        if self.is_zh() {
            return thai_buddhist.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.thai_buddhist_prefix(),
            thai_buddhist.year(),
            self.year_label(),
            thai_buddhist.month(),
            self.month_label(),
            thai_buddhist.day(),
            thai_buddhist.solar().to_ymd(),
            self.weekday_label(),
            thai_buddhist.solar().week_in_lang(self.language),
        )
    }

    pub fn render_fasli_string(&self, fasli: &Fasli) -> String {
        match self.language {
            Language::ZhCn => fasli.to_string_cn(),
            Language::En => format!("{} {}-{:02}-{:02}", self.fasli_prefix(), fasli.year(), fasli.month(), fasli.day()),
        }
    }

    pub fn render_fasli_full(&self, fasli: &Fasli) -> String {
        if self.is_zh() {
            return fasli.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.fasli_prefix(),
            fasli.year(),
            self.year_label(),
            fasli.month(),
            self.month_label(),
            fasli.day(),
            fasli.solar().to_ymd(),
            self.weekday_label(),
            fasli.solar().week_in_lang(self.language),
        )
    }

    pub fn render_nanakshahi_string(&self, nanakshahi: &Nanakshahi) -> String {
        match self.language {
            Language::ZhCn => nanakshahi.to_string_cn(),
            Language::En => format!(
                "{} {}-{:02}-{:02}",
                self.nanakshahi_prefix(),
                nanakshahi.year(),
                nanakshahi.month(),
                nanakshahi.day()
            ),
        }
    }

    pub fn render_nanakshahi_full(&self, nanakshahi: &Nanakshahi) -> String {
        if self.is_zh() {
            return nanakshahi.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.nanakshahi_prefix(),
            nanakshahi.year(),
            self.year_label(),
            nanakshahi.month(),
            self.month_label(),
            nanakshahi.day(),
            nanakshahi.solar().to_ymd(),
            self.weekday_label(),
            nanakshahi.solar().week_in_lang(self.language),
        )
    }

    pub fn render_seleucid_string(&self, seleucid: &Seleucid) -> String {
        match self.language {
            Language::ZhCn => seleucid.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.seleucid_prefix(), seleucid.year(), seleucid.month(), seleucid.day())
            }
        }
    }

    pub fn render_seleucid_full(&self, seleucid: &Seleucid) -> String {
        if self.is_zh() {
            return seleucid.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.seleucid_prefix(),
            seleucid.year(),
            self.year_label(),
            seleucid.month(),
            self.month_label(),
            seleucid.day(),
            seleucid.solar().to_ymd(),
            self.weekday_label(),
            seleucid.solar().week_in_lang(self.language),
        )
    }

    pub fn render_rattanakosin_string(&self, rattanakosin: &Rattanakosin) -> String {
        match self.language {
            Language::ZhCn => rattanakosin.to_string_cn(),
            Language::En => format!(
                "{} {}-{:02}-{:02}",
                self.rattanakosin_prefix(),
                rattanakosin.year(),
                rattanakosin.month(),
                rattanakosin.day()
            ),
        }
    }

    pub fn render_rattanakosin_full(&self, rattanakosin: &Rattanakosin) -> String {
        if self.is_zh() {
            return rattanakosin.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.rattanakosin_prefix(),
            rattanakosin.year(),
            self.year_label(),
            rattanakosin.month(),
            self.month_label(),
            rattanakosin.day(),
            rattanakosin.solar().to_ymd(),
            self.weekday_label(),
            rattanakosin.solar().week_in_lang(self.language),
        )
    }

    pub fn render_venetian_string(&self, venetian: &Venetian) -> String {
        match self.language {
            Language::ZhCn => venetian.to_string_cn(),
            Language::En => {
                format!("{} {}-{:02}-{:02}", self.venetian_prefix(), venetian.year(), venetian.month(), venetian.day())
            }
        }
    }

    pub fn render_venetian_full(&self, venetian: &Venetian) -> String {
        if self.is_zh() {
            return venetian.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.venetian_prefix(),
            venetian.year(),
            self.year_label(),
            venetian.month(),
            self.month_label(),
            venetian.day(),
            venetian.solar().to_ymd(),
            self.weekday_label(),
            venetian.solar().week_in_lang(self.language),
        )
    }

    pub fn render_rumi_string(&self, rumi: &Rumi) -> String {
        match self.language {
            Language::ZhCn => rumi.to_string_cn(),
            Language::En => format!("{} {}-{:02}-{:02}", self.rumi_prefix(), rumi.year(), rumi.month(), rumi.day()),
        }
    }

    pub fn render_rumi_full(&self, rumi: &Rumi) -> String {
        if self.is_zh() {
            return rumi.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.rumi_prefix(),
            rumi.year(),
            self.year_label(),
            rumi.month(),
            self.month_label(),
            rumi.day(),
            rumi.solar().to_ymd(),
            self.weekday_label(),
            rumi.solar().week_in_lang(self.language),
        )
    }

    pub fn render_ethiopian_string(&self, ethiopian: &Ethiopian) -> String {
        match self.language {
            Language::ZhCn => ethiopian.to_string_cn(),
            Language::En => format!(
                "{} {}-{:02}-{:02}",
                self.ethiopian_prefix(),
                ethiopian.year(),
                ethiopian.month(),
                ethiopian.day()
            ),
        }
    }

    pub fn render_ethiopian_full(&self, ethiopian: &Ethiopian) -> String {
        if self.is_zh() {
            return ethiopian.to_full_string();
        }

        format!(
            "{} {} {} {} {} {}, Solar {}, {} {}",
            self.ethiopian_prefix(),
            ethiopian.year(),
            self.year_label(),
            ethiopian.month(),
            self.month_label(),
            ethiopian.day(),
            ethiopian.solar().to_ymd(),
            self.weekday_label(),
            ethiopian.solar().week_in_lang(self.language),
        )
    }

    pub fn render_nine_star(&self, star: &NineStar) -> String {
        match self.language {
            Language::ZhCn => star.to_string(),
            Language::En => format!(
                "{} {} {} {}",
                star.number(),
                star.color_in_lang(self.language),
                star.wu_xing_in_lang(self.language),
                star.name_in_bei_dou_in_lang(self.language)
            ),
        }
    }
}

fn translate_named_item_en(name: &str) -> Cow<'_, str> {
    let translated = match name {
        "元旦节" => "New Year's Day",
        "情人节" => "Valentine's Day",
        "妇女节" => "Women's Day",
        "植树节" => "Arbor Day",
        "消费者权益日" => "Consumer Rights Day",
        "愚人节" => "April Fools' Day",
        "劳动节" => "Labour Day",
        "青年节" => "Youth Day",
        "儿童节" => "Children's Day",
        "建党节" => "CPC Founding Day",
        "建军节" => "Army Day",
        "教师节" => "Teachers' Day",
        "国庆节" => "National Day",
        "万圣节前夜" => "Halloween Eve",
        "万圣节" => "Halloween",
        "平安夜" => "Christmas Eve",
        "圣诞节" => "Christmas",
        "全国中小学生安全教育日" => "National School Safety Education Day",
        "母亲节" => "Mother's Day",
        "全国助残日" => "National Day for Helping the Disabled",
        "父亲节" => "Father's Day",
        "全民国防教育日" => "National Defense Education Day",
        "世界住房日" => "World Habitat Day",
        "感恩节" => "Thanksgiving",
        "春节" => "Spring Festival",
        "元宵节" => "Lantern Festival",
        "龙头节" => "Dragon Head-Raising Day",
        "端午节" => "Dragon Boat Festival",
        "七夕节" => "Qixi Festival",
        "中秋节" => "Mid-Autumn Festival",
        "重阳节" => "Double Ninth Festival",
        "腊八节" => "Laba Festival",
        "除夕" => "Chinese New Year's Eve",
        "寒食节" => "Cold Food Festival",
        "春社" => "Spring She Festival",
        "秋社" => "Autumn She Festival",
        "接神日" => "Welcoming the Gods Day",
        "隔开日" => "Separation Day",
        "人日" => "Human Day",
        "谷日" => "Grain Day",
        "顺星节" => "Following the Stars Festival",
        "天日" => "Heaven Day",
        "地日" => "Earth Day",
        "天穿节" => "Heaven-Piercing Festival",
        "填仓节" => "Granary Filling Festival",
        "正月晦" => "Last Day of the First Lunar Month",
        "中和节" => "Zhonghe Festival",
        "社日节" => "She Day Festival",
        "上巳节" => "Shangsi Festival",
        "分龙节" => "Dragon Division Festival",
        "会龙节" => "Dragon Gathering Festival",
        "天贶节" => "Tiankuang Festival",
        "观莲节" => "Lotus Viewing Festival",
        "五谷母节" => "Mother of Grains Festival",
        "中元节" => "Ghost Festival",
        "财神节" => "God of Wealth Festival",
        "地藏节" => "Ksitigarbha Festival",
        "天灸日" => "Heaven Moxibustion Day",
        "寒衣节" => "Cold Clothes Festival",
        "十成节" => "Ten Completeness Festival",
        "下元节" => "Xiayuan Festival",
        "驱傩日" => "Nuo Expulsion Day",
        "尾牙" => "Year-End Banquet",
        "祭灶日" => "Kitchen God Worship Day",
        "佛吉祥日" => "Buddha Auspicious Day",
        "佛欢喜日" => "Buddha Joyful Day",
        "月光菩萨圣诞" => "Moonlight Bodhisattva Birthday",
        "玉皇上帝诞" | "玉皇上帝圣诞" => "Jade Emperor Birthday",
        "上元天官圣诞" => "Heavenly Officer of Shangyuan Birthday",
        "太清道德天尊 (太上老君) 圣诞" => "Taishang Laojun Birthday",
        "慈航真人圣诞" => "Cihang Zhenren Birthday",
        "玄天上帝圣诞" => "Xuantian Shangdi Birthday",
        "天后妈祖圣诞" => "Mazu Birthday",
        "东岳大帝圣诞" => "Great Emperor of the Eastern Peak Birthday",
        "南极长生大帝圣诞" => "Longevity Emperor Birthday",
        "中元地官大帝圣诞" => "Earth Officer of Zhongyuan Birthday",
        "王母娘娘圣诞" => "Queen Mother of the West Birthday",
        "斗姥元君圣诞" => "Doumu Yuanjun Birthday",
        "下元水官大帝圣诞" => "Water Officer of Xiayuan Birthday",
        "福德正神诞" => "Fude Zhengshen Birthday",
        _ => return Cow::Borrowed(name),
    };
    Cow::Borrowed(translated)
}

pub fn week(name: &str, language: Language) -> &'static str {
    match name {
        "日" => pick(language, "日", "Sun"),
        "一" => pick(language, "一", "Mon"),
        "二" => pick(language, "二", "Tue"),
        "三" => pick(language, "三", "Wed"),
        "四" => pick(language, "四", "Thu"),
        "五" => pick(language, "五", "Fri"),
        "六" => pick(language, "六", "Sat"),
        _ => "",
    }
}

pub fn constellation(name: &str, language: Language) -> &'static str {
    match name {
        "白羊" => pick(language, "白羊", "Aries"),
        "金牛" => pick(language, "金牛", "Taurus"),
        "双子" => pick(language, "双子", "Gemini"),
        "巨蟹" => pick(language, "巨蟹", "Cancer"),
        "狮子" => pick(language, "狮子", "Leo"),
        "处女" => pick(language, "处女", "Virgo"),
        "天秤" => pick(language, "天秤", "Libra"),
        "天蝎" => pick(language, "天蝎", "Scorpio"),
        "射手" => pick(language, "射手", "Sagittarius"),
        "摩羯" => pick(language, "摩羯", "Capricorn"),
        "水瓶" => pick(language, "水瓶", "Aquarius"),
        "双鱼" => pick(language, "双鱼", "Pisces"),
        _ => "",
    }
}

pub fn sheng_xiao(name: &str, language: Language) -> &'static str {
    match name {
        "鼠" => pick(language, "鼠", "Rat"),
        "牛" => pick(language, "牛", "Ox"),
        "虎" => pick(language, "虎", "Tiger"),
        "兔" => pick(language, "兔", "Rabbit"),
        "龙" => pick(language, "龙", "Dragon"),
        "蛇" => pick(language, "蛇", "Snake"),
        "马" => pick(language, "马", "Horse"),
        "羊" => pick(language, "羊", "Goat"),
        "猴" => pick(language, "猴", "Monkey"),
        "鸡" => pick(language, "鸡", "Rooster"),
        "狗" => pick(language, "狗", "Dog"),
        "猪" => pick(language, "猪", "Pig"),
        _ => "",
    }
}

pub fn gan(name: &str, language: Language) -> &'static str {
    match name {
        "甲" => pick(language, "甲", "Jia"),
        "乙" => pick(language, "乙", "Yi"),
        "丙" => pick(language, "丙", "Bing"),
        "丁" => pick(language, "丁", "Ding"),
        "戊" => pick(language, "戊", "Wu"),
        "己" => pick(language, "己", "Ji"),
        "庚" => pick(language, "庚", "Geng"),
        "辛" => pick(language, "辛", "Xin"),
        "壬" => pick(language, "壬", "Ren"),
        "癸" => pick(language, "癸", "Gui"),
        _ => "",
    }
}

pub fn zhi(name: &str, language: Language) -> &'static str {
    match name {
        "子" => pick(language, "子", "Zi"),
        "丑" => pick(language, "丑", "Chou"),
        "寅" => pick(language, "寅", "Yin"),
        "卯" => pick(language, "卯", "Mao"),
        "辰" => pick(language, "辰", "Chen"),
        "巳" => pick(language, "巳", "Si"),
        "午" => pick(language, "午", "Wu"),
        "未" => pick(language, "未", "Wei"),
        "申" => pick(language, "申", "Shen"),
        "酉" => pick(language, "酉", "You"),
        "戌" => pick(language, "戌", "Xu"),
        "亥" => pick(language, "亥", "Hai"),
        _ => "",
    }
}

pub fn ganzhi(gan_name: &str, zhi_name: &str, language: Language) -> String {
    match language {
        Language::ZhCn => format!("{gan_name}{zhi_name}"),
        Language::En => format!("{} {}", gan(gan_name, language), zhi(zhi_name, language)),
    }
}

pub fn jieqi(name: &str, language: Language) -> &'static str {
    match name {
        "" => "",
        "冬至" => pick(language, "冬至", "Winter Solstice"),
        "小寒" => pick(language, "小寒", "Minor Cold"),
        "大寒" => pick(language, "大寒", "Major Cold"),
        "立春" => pick(language, "立春", "Start of Spring"),
        "雨水" => pick(language, "雨水", "Rain Water"),
        "惊蛰" => pick(language, "惊蛰", "Awakening of Insects"),
        "春分" => pick(language, "春分", "Spring Equinox"),
        "清明" => pick(language, "清明", "Pure Brightness"),
        "谷雨" => pick(language, "谷雨", "Grain Rain"),
        "立夏" => pick(language, "立夏", "Start of Summer"),
        "小满" => pick(language, "小满", "Grain Buds"),
        "芒种" => pick(language, "芒种", "Grain in Ear"),
        "夏至" => pick(language, "夏至", "Summer Solstice"),
        "小暑" => pick(language, "小暑", "Minor Heat"),
        "大暑" => pick(language, "大暑", "Major Heat"),
        "立秋" => pick(language, "立秋", "Start of Autumn"),
        "处暑" => pick(language, "处暑", "Limit of Heat"),
        "白露" => pick(language, "白露", "White Dew"),
        "秋分" => pick(language, "秋分", "Autumn Equinox"),
        "寒露" => pick(language, "寒露", "Cold Dew"),
        "霜降" => pick(language, "霜降", "Frost Descent"),
        "立冬" => pick(language, "立冬", "Start of Winter"),
        "小雪" => pick(language, "小雪", "Minor Snow"),
        "大雪" => pick(language, "大雪", "Major Snow"),
        _ => "",
    }
}

pub fn color(name: &str, language: Language) -> &'static str {
    match name {
        "白" => pick(language, "白", "White"),
        "黑" => pick(language, "黑", "Black"),
        "碧" => pick(language, "碧", "Jade"),
        "绿" => pick(language, "绿", "Green"),
        "黄" => pick(language, "黄", "Yellow"),
        "赤" => pick(language, "赤", "Red"),
        "紫" => pick(language, "紫", "Purple"),
        _ => "",
    }
}

pub fn direction(name: &str, language: Language) -> &'static str {
    match name {
        "东" => pick(language, "东", "East"),
        "南" => pick(language, "南", "South"),
        "西" => pick(language, "西", "West"),
        "北" => pick(language, "北", "North"),
        "中" => pick(language, "中", "Center"),
        "乾" => pick(language, "乾", "Qian"),
        "兑" => pick(language, "兑", "Dui"),
        "离" => pick(language, "离", "Li"),
        "震" => pick(language, "震", "Zhen"),
        "巽" => pick(language, "巽", "Xun"),
        "坎" => pick(language, "坎", "Kan"),
        "艮" => pick(language, "艮", "Gen"),
        "坤" => pick(language, "坤", "Kun"),
        _ => "",
    }
}

pub fn wu_xing(name: &str, language: Language) -> &'static str {
    match name {
        "木" => pick(language, "木", "Wood"),
        "火" => pick(language, "火", "Fire"),
        "土" => pick(language, "土", "Earth"),
        "金" => pick(language, "金", "Metal"),
        "水" => pick(language, "水", "Water"),
        _ => "",
    }
}

pub fn duty(name: &str, language: Language) -> &'static str {
    match name {
        "建" => pick(language, "建", "Establish"),
        "除" => pick(language, "除", "Remove"),
        "满" => pick(language, "满", "Full"),
        "平" => pick(language, "平", "Level"),
        "定" => pick(language, "定", "Stable"),
        "执" => pick(language, "执", "Hold"),
        "破" => pick(language, "破", "Break"),
        "危" => pick(language, "危", "Danger"),
        "成" => pick(language, "成", "Success"),
        "收" => pick(language, "收", "Receive"),
        "开" => pick(language, "开", "Open"),
        "闭" => pick(language, "闭", "Close"),
        _ => "",
    }
}

pub fn phase(name: &str, language: Language) -> &'static str {
    match name {
        "朔" => pick(language, "朔", "New Moon"),
        "既朔" => pick(language, "既朔", "Waxing Crescent"),
        "蛾眉新" => pick(language, "蛾眉新", "Young Crescent"),
        "蛾眉" => pick(language, "蛾眉", "Crescent Moon"),
        "夕" => pick(language, "夕", "Evening Moon"),
        "上弦" => pick(language, "上弦", "First Quarter"),
        "九夜" => pick(language, "九夜", "Ninth-Night Moon"),
        "宵" => pick(language, "宵", "Late-Evening Moon"),
        "渐盈凸" => pick(language, "渐盈凸", "Waxing Gibbous"),
        "小望" => pick(language, "小望", "Near Full Moon"),
        "望" => pick(language, "望", "Full Moon"),
        "既望" => pick(language, "既望", "Waning Gibbous"),
        "立待" => pick(language, "立待", "Standing-Wait Moon"),
        "居待" => pick(language, "居待", "Sitting-Wait Moon"),
        "寝待" => pick(language, "寝待", "Lying-Wait Moon"),
        "更待" => pick(language, "更待", "Late-Wait Moon"),
        "渐亏凸" => pick(language, "渐亏凸", "Waning Gibbous"),
        "下弦" => pick(language, "下弦", "Last Quarter"),
        "有明" => pick(language, "有明", "Dawn Moon"),
        "蛾眉残" => pick(language, "蛾眉残", "Waning Crescent"),
        "残" => pick(language, "残", "Fading Moon"),
        "晓" => pick(language, "晓", "Morning Moon"),
        "晦" => pick(language, "晦", "Dark Moon"),
        _ => "",
    }
}

pub fn bei_dou(name: &str, language: Language) -> &'static str {
    match name {
        "天枢" => pick(language, "天枢", "Tian Shu"),
        "天璇" => pick(language, "天璇", "Tian Xuan"),
        "天玑" => pick(language, "天玑", "Tian Ji"),
        "天权" => pick(language, "天权", "Tian Quan"),
        "玉衡" => pick(language, "玉衡", "Yu Heng"),
        "开阳" => pick(language, "开阳", "Kai Yang"),
        "摇光" => pick(language, "摇光", "Yao Guang"),
        "洞明" => pick(language, "洞明", "Dong Ming"),
        "隐元" => pick(language, "隐元", "Yin Yuan"),
        _ => "",
    }
}

pub fn nayin(name: &str, language: Language) -> &'static str {
    match name {
        "海中金" => pick(language, "海中金", "Gold in the Sea"),
        "沙中金" => pick(language, "沙中金", "Gold in Sand"),
        "炉中火" => pick(language, "炉中火", "Fire in the Furnace"),
        "山下火" => pick(language, "山下火", "Fire at the Foot of the Mountain"),
        "大林木" => pick(language, "大林木", "Wood of the Great Forest"),
        "平地木" => pick(language, "平地木", "Wood on the Plain"),
        "路旁土" => pick(language, "路旁土", "Earth by the Roadside"),
        "壁上土" => pick(language, "壁上土", "Earth on the Wall"),
        "剑锋金" => pick(language, "剑锋金", "Sword-Edge Metal"),
        "金箔金" => pick(language, "金箔金", "Gold Foil Metal"),
        "山头火" => pick(language, "山头火", "Fire on the Mountain Top"),
        "覆灯火" => pick(language, "覆灯火", "Covered Lamp Fire"),
        "涧下水" => pick(language, "涧下水", "Stream Water"),
        "天河水" => pick(language, "天河水", "Heavenly River Water"),
        "城头土" => pick(language, "城头土", "City Wall Earth"),
        "大驿土" => pick(language, "大驿土", "Great Post Earth"),
        "白蜡金" => pick(language, "白蜡金", "White Wax Metal"),
        "钗钏金" => pick(language, "钗钏金", "Hairpin and Bracelet Metal"),
        "杨柳木" => pick(language, "杨柳木", "Willow Wood"),
        "桑柘木" => pick(language, "桑柘木", "Mulberry Wood"),
        "泉中水" => pick(language, "泉中水", "Spring Water"),
        "大溪水" => pick(language, "大溪水", "Great Creek Water"),
        "屋上土" => pick(language, "屋上土", "Roof-Top Earth"),
        "沙中土" => pick(language, "沙中土", "Earth in Sand"),
        "霹雳火" => pick(language, "霹雳火", "Thunderbolt Fire"),
        "天上火" => pick(language, "天上火", "Fire in the Sky"),
        "松柏木" => pick(language, "松柏木", "Pine and Cypress Wood"),
        "石榴木" => pick(language, "石榴木", "Pomegranate Wood"),
        "长流水" => pick(language, "长流水", "Long Flowing Water"),
        "大海水" => pick(language, "大海水", "Great Sea Water"),
        _ => "",
    }
}

pub fn position_desc(name: &str, language: Language) -> &'static str {
    match name {
        "正北" => pick(language, "正北", "Due North"),
        "东北" => pick(language, "东北", "Northeast"),
        "正东" => pick(language, "正东", "Due East"),
        "东南" => pick(language, "东南", "Southeast"),
        "正南" => pick(language, "正南", "Due South"),
        "西南" => pick(language, "西南", "Southwest"),
        "正西" => pick(language, "正西", "Due West"),
        "西北" => pick(language, "西北", "Northwest"),
        "中宫" => pick(language, "中宫", "Central Palace"),
        _ => "",
    }
}

pub fn xiu(name: &str, language: Language) -> &'static str {
    match name {
        "角" => pick(language, "角", "Jiao"),
        "亢" => pick(language, "亢", "Kang"),
        "氐" => pick(language, "氐", "Di"),
        "房" => pick(language, "房", "Fang"),
        "心" => pick(language, "心", "Xin"),
        "尾" => pick(language, "尾", "Wei"),
        "箕" => pick(language, "箕", "Ji"),
        "斗" => pick(language, "斗", "Dou"),
        "牛" => pick(language, "牛", "Niu"),
        "女" => pick(language, "女", "Nu"),
        "虚" => pick(language, "虚", "Xu"),
        "危" => pick(language, "危", "Wei"),
        "室" => pick(language, "室", "Shi"),
        "壁" => pick(language, "壁", "Bi"),
        "奎" => pick(language, "奎", "Kui"),
        "娄" => pick(language, "娄", "Lou"),
        "胃" => pick(language, "胃", "Wei"),
        "昴" => pick(language, "昴", "Mao"),
        "毕" => pick(language, "毕", "Bi"),
        "觜" => pick(language, "觜", "Zui"),
        "参" => pick(language, "参", "Shen"),
        "井" => pick(language, "井", "Jing"),
        "鬼" => pick(language, "鬼", "Gui"),
        "柳" => pick(language, "柳", "Liu"),
        "星" => pick(language, "星", "Xing"),
        "张" => pick(language, "张", "Zhang"),
        "翼" => pick(language, "翼", "Yi"),
        "轸" => pick(language, "轸", "Zhen"),
        _ => "",
    }
}

pub fn xiu_luck(name: &str, language: Language) -> &'static str {
    match name {
        "吉" => pick(language, "吉", "Auspicious"),
        "凶" => pick(language, "凶", "Inauspicious"),
        _ => "",
    }
}

pub fn zheng(name: &str, language: Language) -> &'static str {
    match name {
        "木" | "火" | "土" | "金" | "水" => wu_xing(name, language),
        "日" => pick(language, "日", "Sun"),
        "月" => pick(language, "月", "Moon"),
        _ => "",
    }
}

pub fn xiu_animal(name: &str, language: Language) -> &'static str {
    match name {
        "蛟" => pick(language, "蛟", "Flood Dragon"),
        "獬" => pick(language, "獬", "Xiezhi"),
        "狼" => pick(language, "狼", "Wolf"),
        "犴" => pick(language, "犴", "Wild Dog"),
        "龙" => pick(language, "龙", "Dragon"),
        "牛" => pick(language, "牛", "Ox"),
        "狗" => pick(language, "狗", "Dog"),
        "羊" => pick(language, "羊", "Goat"),
        "蝠" => pick(language, "蝠", "Bat"),
        "貉" => pick(language, "貉", "Raccoon Dog"),
        "彘" => pick(language, "彘", "Boar"),
        "獐" => pick(language, "獐", "Roe Deer"),
        "兔" => pick(language, "兔", "Rabbit"),
        "鼠" => pick(language, "鼠", "Rat"),
        "鸡" => pick(language, "鸡", "Rooster"),
        "马" => pick(language, "马", "Horse"),
        "狐" => pick(language, "狐", "Fox"),
        "燕" => pick(language, "燕", "Swallow"),
        "乌" => pick(language, "乌", "Crow"),
        "鹿" => pick(language, "鹿", "Deer"),
        "虎" => pick(language, "虎", "Tiger"),
        "猪" => pick(language, "猪", "Pig"),
        "猴" => pick(language, "猴", "Monkey"),
        "蛇" => pick(language, "蛇", "Snake"),
        "豹" => pick(language, "豹", "Leopard"),
        "獝" => pick(language, "獝", "Xuyu"),
        "猿" => pick(language, "猿", "Ape"),
        "蚓" => pick(language, "蚓", "Worm"),
        _ => "",
    }
}

pub fn shou(name: &str, language: Language) -> &'static str {
    match name {
        "青龙" => pick(language, "青龙", "Azure Dragon"),
        "朱雀" => pick(language, "朱雀", "Vermilion Bird"),
        "白虎" => pick(language, "白虎", "White Tiger"),
        "玄武" => pick(language, "玄武", "Black Tortoise"),
        _ => "",
    }
}

pub fn pengzu_gan(name: &str, language: Language) -> &'static str {
    match name {
        "甲不开仓财物耗散" => {
            pick(language, "甲不开仓财物耗散", "Jia: avoid opening granaries; wealth may be lost.")
        }
        "乙不栽植千株不长" => {
            pick(language, "乙不栽植千株不长", "Yi: avoid planting; crops may fail to thrive.")
        }
        "丙不修灶必见灾殃" => {
            pick(language, "丙不修灶必见灾殃", "Bing: avoid repairing stoves; mishaps may follow.")
        }
        "丁不剃头头必生疮" => {
            pick(language, "丁不剃头头必生疮", "Ding: avoid haircuts; scalp troubles may arise.")
        }
        "戊不受田田主不祥" => {
            pick(language, "戊不受田田主不祥", "Wu: avoid taking land; the owner may face misfortune.")
        }
        "己不破券二比并亡" => {
            pick(language, "己不破券二比并亡", "Ji: avoid contract settlements; both sides may lose.")
        }
        "庚不经络织机虚张" => {
            pick(language, "庚不经络织机虚张", "Geng: avoid warping looms; the frame may sit idle.")
        }
        "辛不合酱主人不尝" => {
            pick(language, "辛不合酱主人不尝", "Xin: avoid making sauces; the host may not enjoy them.")
        }
        "壬不泱水更难提防" => {
            pick(language, "壬不泱水更难提防", "Ren: avoid drawing water; extra caution is needed.")
        }
        "癸不词讼理弱敌强" => {
            pick(language, "癸不词讼理弱敌强", "Gui: avoid lawsuits; your case may be weaker than the opponent's.")
        }
        _ => "",
    }
}

pub fn pengzu_zhi(name: &str, language: Language) -> &'static str {
    match name {
        "子不问卜自惹祸殃" => {
            pick(language, "子不问卜自惹祸殃", "Zi: avoid divination; it may invite trouble.")
        }
        "丑不冠带主不还乡" => {
            pick(language, "丑不冠带主不还乡", "Chou: avoid formal dressing; one may fail to return home.")
        }
        "寅不祭祀神鬼不尝" => {
            pick(language, "寅不祭祀神鬼不尝", "Yin: avoid sacrifices; spirits may not accept them.")
        }
        "卯不穿井水泉不香" => {
            pick(language, "卯不穿井水泉不香", "Mao: avoid digging wells; the water may turn poor.")
        }
        "辰不哭泣必主重丧" => {
            pick(language, "辰不哭泣必主重丧", "Chen: avoid loud mourning; it may signal repeated bereavement.")
        }
        "巳不远行财物伏藏" => {
            pick(language, "巳不远行财物伏藏", "Si: avoid long journeys; wealth may become hidden or lost.")
        }
        "午不苫盖屋主更张" => {
            pick(language, "午不苫盖屋主更张", "Wu: avoid roofing work; the household may be disrupted.")
        }
        "未不服药毒气入肠" => {
            pick(language, "未不服药毒气入肠", "Wei: avoid taking medicine; toxins may trouble the body.")
        }
        "申不安床鬼祟入房" => {
            pick(language, "申不安床鬼祟入房", "Shen: avoid moving beds; unsettling forces may enter the room.")
        }
        "酉不会客醉坐颠狂" => {
            pick(language, "酉不会客醉坐颠狂", "You: avoid receiving guests; drunken disorder may follow.")
        }
        "戌不吃犬作怪上床" => {
            pick(language, "戌不吃犬作怪上床", "Xu: avoid eating dog meat; strange disturbances may arise.")
        }
        "亥不嫁娶不利新郎" => {
            pick(language, "亥不嫁娶不利新郎", "Hai: avoid marriage rites; it may be unfavorable for the groom.")
        }
        _ => "",
    }
}

pub fn chong_desc(gan_name: &str, zhi_name: &str, zodiac_name: &str, language: Language) -> String {
    match language {
        Language::ZhCn => format!("({gan_name}{zhi_name}){zodiac_name}"),
        Language::En => {
            format!("({} {}){}", gan(gan_name, language), zhi(zhi_name, language), sheng_xiao(zodiac_name, language))
        }
    }
}
