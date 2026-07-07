//! 阳历工具：纯函数 + 常量（闰年、月日数、儒略日、星期、节日、星座）。
//!
//! 对应 lunar-go `SolarUtil/SolarUtil.go`。

use crate::key_index::{month_day_key, month_weekday_key, parse_month_day_key, parse_month_weekday_key};

/// 星期：日、一、二、三、四、五、六。
pub const WEEK: [&str; 7] = ["日", "一", "二", "三", "四", "五", "六"];

const DAYS_OF_MONTH: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// 西方星座。
pub const XINGZUO: [&str; 12] =
    ["白羊", "金牛", "双子", "巨蟹", "狮子", "处女", "天秤", "天蝎", "射手", "摩羯", "水瓶", "双鱼"];

use std::collections::HashMap;
use std::sync::LazyLock;

/// 阳历节日（几月几日）。
pub static FESTIVAL: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("1-1", "元旦节");
    m.insert("2-14", "情人节");
    m.insert("3-8", "妇女节");
    m.insert("3-12", "植树节");
    m.insert("3-15", "消费者权益日");
    m.insert("4-1", "愚人节");
    m.insert("5-1", "劳动节");
    m.insert("5-4", "青年节");
    m.insert("6-1", "儿童节");
    m.insert("7-1", "建党节");
    m.insert("8-1", "建军节");
    m.insert("9-10", "教师节");
    m.insert("10-1", "国庆节");
    m.insert("10-31", "万圣节前夜");
    m.insert("11-1", "万圣节");
    m.insert("12-24", "平安夜");
    m.insert("12-25", "圣诞节");
    m
});

/// 几月第几个星期几对应的节日，key 形如 `"5-2-0"`（5 月第 2 个周日 = 母亲节）。
pub static WEEK_FESTIVAL: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("3-0-1", "全国中小学生安全教育日");
    m.insert("5-2-0", "母亲节");
    m.insert("5-3-0", "全国助残日");
    m.insert("6-3-0", "父亲节");
    m.insert("9-3-6", "全民国防教育日");
    m.insert("10-1-1", "世界住房日");
    m.insert("11-4-4", "感恩节");
    m
});

/// 其它阳历节日 / 纪念日。
pub static OTHER_FESTIVAL: LazyLock<HashMap<&'static str, Vec<&'static str>>> = LazyLock::new(|| {
    let mut m: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    let entries: &[(&str, &[&str])] = &[
        ("1-8", &["周恩来逝世纪念日"]),
        ("1-10", &["中国人民警察节"]),
        ("1-14", &["日记情人节"]),
        ("1-21", &["列宁逝世纪念日"]),
        ("1-26", &["国际海关日"]),
        ("1-27", &["国际大屠杀纪念日"]),
        ("2-2", &["世界湿地日"]),
        ("2-4", &["世界抗癌日"]),
        ("2-7", &["京汉铁路罢工纪念日"]),
        ("2-10", &["国际气象节"]),
        ("2-19", &["邓小平逝世纪念日"]),
        ("2-20", &["世界社会公正日"]),
        ("2-21", &["国际母语日"]),
        ("2-24", &["第三世界青年日"]),
        ("3-1", &["国际海豹日"]),
        ("3-3", &["世界野生动植物日", "全国爱耳日"]),
        ("3-5", &["周恩来诞辰纪念日", "中国青年志愿者服务日"]),
        ("3-6", &["世界青光眼日"]),
        ("3-7", &["女生节"]),
        ("3-12", &["孙中山逝世纪念日"]),
        ("3-14", &["马克思逝世纪念日", "白色情人节"]),
        ("3-17", &["国际航海日"]),
        ("3-18", &["全国科技人才活动日", "全国爱肝日"]),
        ("3-20", &["国际幸福日"]),
        ("3-21", &["世界森林日", "世界睡眠日", "国际消除种族歧视日"]),
        ("3-22", &["世界水日"]),
        ("3-23", &["世界气象日"]),
        ("3-24", &["世界防治结核病日"]),
        ("3-29", &["中国黄花岗七十二烈士殉难纪念日"]),
        ("4-2", &["国际儿童图书日", "世界自闭症日"]),
        ("4-4", &["国际地雷行动日"]),
        ("4-7", &["世界卫生日"]),
        ("4-8", &["国际珍稀动物保护日"]),
        ("4-12", &["世界航天日"]),
        ("4-14", &["黑色情人节"]),
        ("4-15", &["全民国家安全教育日"]),
        ("4-22", &["世界地球日", "列宁诞辰纪念日"]),
        ("4-23", &["世界读书日"]),
        ("4-24", &["中国航天日"]),
        ("4-25", &["儿童预防接种宣传日"]),
        ("4-26", &["世界知识产权日", "全国疟疾日"]),
        ("4-28", &["世界安全生产与健康日"]),
        ("4-30", &["全国交通安全反思日"]),
        ("5-2", &["世界金枪鱼日"]),
        ("5-3", &["世界新闻自由日"]),
        ("5-5", &["马克思诞辰纪念日"]),
        ("5-8", &["世界红十字日"]),
        ("5-11", &["世界肥胖日"]),
        ("5-12", &["全国防灾减灾日", "护士节"]),
        ("5-14", &["玫瑰情人节"]),
        ("5-15", &["国际家庭日"]),
        ("5-19", &["中国旅游日"]),
        ("5-20", &["网络情人节"]),
        ("5-22", &["国际生物多样性日"]),
        ("5-25", &["525 心理健康节"]),
        ("5-27", &["上海解放日"]),
        ("5-29", &["国际维和人员日"]),
        ("5-30", &["中国五卅运动纪念日"]),
        ("5-31", &["世界无烟日"]),
        ("6-3", &["世界自行车日"]),
        ("6-5", &["世界环境日"]),
        ("6-6", &["全国爱眼日"]),
        ("6-8", &["世界海洋日"]),
        ("6-11", &["中国人口日"]),
        ("6-14", &["世界献血日", "亲亲情人节"]),
        ("6-17", &["世界防治荒漠化与干旱日"]),
        ("6-20", &["世界难民日"]),
        ("6-21", &["国际瑜伽日"]),
        ("6-25", &["全国土地日"]),
        ("6-26", &["国际禁毒日", "联合国宪章日"]),
        ("7-1", &["香港回归纪念日"]),
        ("7-6", &["国际接吻日", "朱德逝世纪念日"]),
        ("7-7", &["七七事变纪念日"]),
        ("7-11", &["世界人口日", "中国航海日"]),
        ("7-14", &["银色情人节"]),
        ("7-18", &["曼德拉国际日"]),
        ("7-30", &["国际友谊日"]),
        ("8-3", &["男人节"]),
        ("8-5", &["恩格斯逝世纪念日"]),
        ("8-6", &["国际电影节"]),
        ("8-8", &["全民健身日"]),
        ("8-9", &["国际土著人日"]),
        ("8-12", &["国际青年节"]),
        ("8-14", &["绿色情人节"]),
        ("8-19", &["世界人道主义日", "中国医师节"]),
        ("8-22", &["邓小平诞辰纪念日"]),
        ("8-29", &["全国测绘法宣传日"]),
        ("9-3", &["中国抗日战争胜利纪念日"]),
        ("9-5", &["中华慈善日"]),
        ("9-8", &["世界扫盲日"]),
        ("9-9", &["毛泽东逝世纪念日", "全国拒绝酒驾日"]),
        ("9-14", &["世界清洁地球日", "相片情人节"]),
        ("9-15", &["国际民主日"]),
        ("9-16", &["国际臭氧层保护日"]),
        ("9-17", &["世界骑行日"]),
        ("9-18", &["九一八事变纪念日"]),
        ("9-20", &["全国爱牙日"]),
        ("9-21", &["国际和平日"]),
        ("9-27", &["世界旅游日"]),
        ("9-30", &["中国烈士纪念日"]),
        ("10-1", &["国际老年人日"]),
        ("10-2", &["国际非暴力日"]),
        ("10-4", &["世界动物日"]),
        ("10-11", &["国际女童日"]),
        ("10-10", &["辛亥革命纪念日"]),
        ("10-13", &["国际减轻自然灾害日", "中国少年先锋队诞辰日"]),
        ("10-14", &["葡萄酒情人节"]),
        ("10-16", &["世界粮食日"]),
        ("10-17", &["全国扶贫日"]),
        ("10-20", &["世界统计日"]),
        ("10-24", &["世界发展信息日", "程序员节"]),
        ("10-25", &["抗美援朝纪念日"]),
        ("11-5", &["世界海啸日"]),
        ("11-8", &["记者节"]),
        ("11-9", &["全国消防日"]),
        ("11-11", &["光棍节"]),
        ("11-12", &["孙中山诞辰纪念日"]),
        ("11-14", &["电影情人节"]),
        ("11-16", &["国际宽容日"]),
        ("11-17", &["国际大学生节"]),
        ("11-19", &["世界厕所日"]),
        ("11-28", &["恩格斯诞辰纪念日"]),
        ("11-29", &["国际声援巴勒斯坦人民日"]),
        ("12-1", &["世界艾滋病日"]),
        ("12-2", &["全国交通安全日"]),
        ("12-3", &["世界残疾人日"]),
        ("12-4", &["全国法制宣传日"]),
        ("12-5", &["世界弱能人士日", "国际志愿人员日"]),
        ("12-7", &["国际民航日"]),
        ("12-9", &["世界足球日", "国际反腐败日"]),
        ("12-10", &["世界人权日"]),
        ("12-11", &["国际山岳日"]),
        ("12-12", &["西安事变纪念日"]),
        ("12-13", &["国家公祭日"]),
        ("12-14", &["拥抱情人节"]),
        ("12-18", &["国际移徙者日"]),
        ("12-26", &["毛泽东诞辰纪念日"]),
    ];
    for (k, v) in entries {
        m.insert(*k, v.to_vec());
    }
    m
});

static FESTIVAL_INDEX: LazyLock<HashMap<i32, &'static str>> = LazyLock::new(|| {
    FESTIVAL.iter().filter_map(|(key, value)| parse_month_day_key(key).map(|parsed| (parsed, *value))).collect()
});

static WEEK_FESTIVAL_INDEX: LazyLock<HashMap<i32, &'static str>> = LazyLock::new(|| {
    WEEK_FESTIVAL
        .iter()
        .filter_map(|(key, value)| parse_month_weekday_key(key).map(|parsed| (parsed, *value)))
        .collect()
});

static OTHER_FESTIVAL_INDEX: LazyLock<HashMap<i32, Vec<&'static str>>> = LazyLock::new(|| {
    OTHER_FESTIVAL
        .iter()
        .filter_map(|(key, value)| parse_month_day_key(key).map(|parsed| (parsed, value.clone())))
        .collect()
});

pub fn festival(month: i32, day: i32) -> Option<&'static str> {
    FESTIVAL_INDEX.get(&month_day_key(month, day)).copied()
}

pub fn week_festival(month: i32, week_index: i32, week: i32) -> Option<&'static str> {
    WEEK_FESTIVAL_INDEX.get(&month_weekday_key(month, week_index, week)).copied()
}

pub fn other_festivals(month: i32, day: i32) -> &'static [&'static str] {
    OTHER_FESTIVAL_INDEX.get(&month_day_key(month, day)).map(Vec::as_slice).unwrap_or(&[])
}

/// 是否公历闰年（1600 年前用儒略历规则）。
#[inline]
pub const fn is_leap_year(year: i32) -> bool {
    if year < 1600 {
        return year % 4 == 0;
    }
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// 该年总天数（1582 年为 355）。
pub const fn days_of_year(year: i32) -> i32 {
    if year == 1582 {
        return 355;
    }
    if is_leap_year(year) { 366 } else { 365 }
}

/// 该月天数（1582-10 为 21）。
pub const fn days_of_month(year: i32, month: i32) -> i32 {
    if year == 1582 && month == 10 {
        return 21;
    }
    let d = DAYS_OF_MONTH[(month - 1) as usize];
    if month == 2 && is_leap_year(year) { d + 1 } else { d }
}

/// 该日在该年的第几天（自 1 起）。
pub fn days_in_year(year: i32, month: i32, day: i32) -> i32 {
    let mut days = 0;
    for i in 1..month {
        days += days_of_month(year, i);
    }
    let d = if year == 1582 && month == 10 { if day >= 15 { day - 10 } else { day } } else { day };
    days + d
}

/// 儒略日（含时分秒小数）。
pub fn julian_day(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> f64 {
    let mut normalized_year = year;
    let mut normalized_month = month;
    let day_fraction =
        f64::from(day) + (((f64::from(second) / 60.0 + f64::from(minute)) / 60.0 + f64::from(hour)) / 24.0);
    let gregorian = normalized_year * 372 + normalized_month * 31 + day_fraction as i32 >= 588_829;
    if normalized_month <= 2 {
        normalized_month += 12;
        normalized_year -= 1;
    }
    let correction = if gregorian {
        let nv = normalized_year / 100;
        2 - nv + nv / 4
    } else {
        0
    };
    ((365.25 * (f64::from(normalized_year) + 4716.0)) as i64) as f64
        + ((30.6001 * (f64::from(normalized_month) + 1.0)) as i64) as f64
        + day_fraction
        + f64::from(correction)
        - 1524.5
}

/// 星期几（0=周日）。
#[inline]
pub fn week(year: i32, month: i32, day: i32) -> i32 {
    let jd = julian_day(year, month, day, 0, 0, 0);
    (((jd + 0.5) as i64 + 7_000_001) % 7) as i32
}

/// 该月有几周（按指定起始星期）。
pub fn weeks_of_month(year: i32, month: i32, start: i32) -> i32 {
    let total = days_of_month(year, month) + week(year, month, 1) - start;
    (f64::from(total) / 7.0).ceil() as i32
}

/// a 是否早于 b（逐字段比较）。
#[allow(clippy::too_many_arguments)]
pub fn is_before(
    ay: i32,
    am: i32,
    ad: i32,
    ah: i32,
    ai: i32,
    as_: i32,
    by: i32,
    bm: i32,
    bd: i32,
    bh: i32,
    bi: i32,
    bs: i32,
) -> bool {
    (ay, am, ad, ah, ai, as_) < (by, bm, bd, bh, bi, bs)
}

/// 两个日期之间的天数差。返回值：若 a 早于 b 为正，a 晚于 b 为负。
pub fn days_between(ay: i32, am: i32, ad: i32, by: i32, bm: i32, bd: i32) -> i32 {
    match ay.cmp(&by) {
        std::cmp::Ordering::Equal => days_in_year(by, bm, bd) - days_in_year(ay, am, ad),
        std::cmp::Ordering::Greater => {
            let mut days = days_of_year(by) - days_in_year(by, bm, bd);
            for i in by + 1..ay {
                days += days_of_year(i);
            }
            days += days_in_year(ay, am, ad);
            -days
        }
        std::cmp::Ordering::Less => {
            let mut days = days_of_year(ay) - days_in_year(ay, am, ad);
            for i in ay + 1..by {
                days += days_of_year(i);
            }
            days += days_in_year(by, bm, bd);
            days
        }
    }
}
