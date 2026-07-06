//! Experimental internationalization helpers.
//!
//! The default API remains Chinese-first for compatibility and performance.
//! This module provides explicit language-aware helpers behind the `i18n`
//! feature without changing existing method signatures.

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
