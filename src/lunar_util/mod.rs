//! 农历核心常量、查找表与纯函数。
//!
//! 对应 lunar-go `LunarUtil/LunarUtil.go`。切片表（`tables`）与 map（`maps`）由脚本自动抽取。

pub mod maps;
pub mod tables;

pub use maps::*;
pub use tables::*;

/// 月地支基准索引（寅 = 2，正月建寅）。
pub const BASE_MONTH_ZHI_INDEX: i64 = 2;

/// 各地支对应的天神偏移（按地支索引 0..11：子丑寅卯辰巳午未申酉戌亥）。
pub const ZHI_TIAN_SHEN_OFFSET: [i64; 12] = [4, 2, 0, 10, 8, 6, 4, 2, 0, 10, 8, 6];

/// `"HH:MM"` → 时辰索引（0..11，子=0）。
pub fn get_time_zhi_index(hm: &str) -> i64 {
    if hm.is_empty() {
        return 0;
    }
    let hm = if hm.len() > 5 { &hm[..5] } else { hm };
    let mut x = 1_i64;
    let mut i = 1;
    while i < 22 {
        let lo = format!("{i:02}:00");
        let hi = format!("{}:59", i + 1);
        if hm >= lo.as_str() && hm <= hi.as_str() {
            return x;
        }
        x += 1;
        i += 2;
    }
    0
}

/// `"HH:MM"` → 地支名。
pub fn convert_time(hm: &str) -> &'static str {
    tables::ZHI[get_time_zhi_index(hm) as usize + 1]
}

/// 六十甲子序号（0..59），未找到返回 -1。
pub fn get_jia_zi_index(gan_zhi: &str) -> i64 {
    for (i, v) in tables::JIA_ZI.iter().enumerate() {
        if *v == gan_zhi {
            return i as i64;
        }
    }
    -1
}

/// 把整数格式化为 2 位大写十六进制（与 Go 的 `hex` 一致）。
fn hex_str(n: i64) -> String {
    let h = if n < 0 {
        format!("{:x}", n)
    } else {
        format!("{:x}", n as u64)
    };
    let h = if h.len() < 2 { format!("0{h}") } else { h };
    h.to_uppercase()
}

/// 在 `names` 中线性查找 `name`，命中返回 `i + offset`，否则 -1。
pub fn find(name: &str, names: &[&str], offset: i64) -> i64 {
    for (i, v) in names.iter().enumerate() {
        if *v == name {
            return i as i64 + offset;
        }
    }
    -1
}

/// 旬索引（0..5）。
pub fn get_xun_index(gan_zhi: &str) -> i64 {
    let chars: Vec<char> = gan_zhi.chars().collect();
    let gan: String = chars[..1].iter().collect();
    let zhi: String = chars[1..].iter().collect();
    let mut gan_index = 0_i64;
    for (i, v) in tables::GAN.iter().enumerate() {
        if *v == gan {
            gan_index = i as i64;
            break;
        }
    }
    let mut zhi_index = 0_i64;
    for (i, v) in tables::ZHI.iter().enumerate() {
        if *v == zhi {
            zhi_index = i as i64;
            break;
        }
    }
    let mut diff = gan_index - zhi_index;
    if diff < 0 {
        diff += 12;
    }
    diff / 2
}

/// 旬名。
pub fn get_xun(gan_zhi: &str) -> &'static str {
    tables::XUN[get_xun_index(gan_zhi) as usize]
}

/// 旬空（空亡）。
pub fn get_xun_kong(gan_zhi: &str) -> &'static str {
    tables::XUN_KONG[get_xun_index(gan_zhi) as usize]
}

/// 每日宜。
pub fn get_day_yi(month_gan_zhi: &str, day_gan_zhi: &str) -> Vec<&'static str> {
    get_day_yi_ji(month_gan_zhi, day_gan_zhi, true)
}

/// 每日忌。
pub fn get_day_ji(month_gan_zhi: &str, day_gan_zhi: &str) -> Vec<&'static str> {
    get_day_yi_ji(month_gan_zhi, day_gan_zhi, false)
}

fn get_day_yi_ji(month_gan_zhi: &str, day_gan_zhi: &str, yi: bool) -> Vec<&'static str> {
    let day = hex_str(get_jia_zi_index(day_gan_zhi));
    let month = hex_str(get_jia_zi_index(month_gan_zhi));
    let needle = format!("{day}=");
    let mut right = tables::DAY_YI_JI;
    let mut result = Vec::new();
    loop {
        let Some(idx) = right.find(&needle) else {
            break;
        };
        right = &right[idx + 3..];
        let mut left = right;
        if let Some(eq) = left.find('=') {
            if eq >= 2 {
                left = &left[..eq - 2];
            }
        }
        let Some(colon) = left.find(':') else {
            break;
        };
        let months = &left[..colon];
        let mut matched = false;
        let mut i = 0;
        while i + 2 <= months.len() {
            if &months[i..i + 2] == month.as_str() {
                matched = true;
                break;
            }
            i += 2;
        }
        if matched {
            let after = &left[colon + 1..];
            let end = after.find(',').unwrap_or(after.len());
            let seg = if yi { &after[..end] } else { &after[end + 1..] };
            let mut j = 0;
            while j + 2 <= seg.len() {
                if let Ok(num) = u8::from_str_radix(&seg[j..j + 2], 16) {
                    let idx = num as usize;
                    if idx < tables::YI_JI.len() {
                        result.push(tables::YI_JI[idx]);
                    }
                }
                j += 2;
            }
            break;
        }
    }
    if result.is_empty() {
        result.push("无");
    }
    result
}

/// 每个时辰宜。
pub fn get_time_yi(day_gan_zhi: &str, time_gan_zhi: &str) -> Vec<&'static str> {
    get_time_yi_ji(day_gan_zhi, time_gan_zhi, true)
}

/// 每个时辰忌。
pub fn get_time_ji(day_gan_zhi: &str, time_gan_zhi: &str) -> Vec<&'static str> {
    get_time_yi_ji(day_gan_zhi, time_gan_zhi, false)
}

fn get_time_yi_ji(day_gan_zhi: &str, time_gan_zhi: &str, yi: bool) -> Vec<&'static str> {
    let day = hex_str(get_jia_zi_index(day_gan_zhi));
    let time = hex_str(get_jia_zi_index(time_gan_zhi));
    let needle = format!("{day}{time}=");
    let mut result = Vec::new();
    if let Some(idx) = tables::TIME_YI_JI.find(&needle) {
        let mut left = &tables::TIME_YI_JI[idx + 5..];
        if let Some(eq) = left.find('=') {
            if eq >= 4 {
                left = &left[..eq - 4];
            }
        }
        let end = left.find(',').unwrap_or(left.len());
        let seg = if yi { &left[..end] } else { &left[end + 1..] };
        let mut j = 0;
        while j + 2 <= seg.len() {
            if let Ok(num) = u8::from_str_radix(&seg[j..j + 2], 16) {
                let idx2 = num as usize;
                if idx2 < tables::YI_JI.len() {
                    result.push(tables::YI_JI[idx2]);
                }
            }
            j += 2;
        }
    }
    if result.is_empty() {
        result.push("无");
    }
    result
}

/// 解析当日的神煞序号列表（`dayShenSha` 按月分段，`;XXdata` 格式）。
fn day_shen_sha_indices(month_zhi_index: i64, day_gan_zhi: &str) -> Vec<usize> {
    let m = ((month_zhi_index - 2 + 12) % 12) as usize;
    let idx = get_jia_zi_index(day_gan_zhi);
    if idx < 0 {
        return Vec::new();
    }
    let day_hex = format!("{idx:02X}");
    let s = tables::DAY_SHEN_SHA[m];
    let needle = format!(";{day_hex}");
    let mut out = Vec::new();
    if let Some(pos) = s.find(&needle) {
        let start = pos + needle.len();
        let rest = &s[start..];
        let data_end = rest.find(';').unwrap_or(rest.len());
        let data = &rest[..data_end];
        let mut i = 0;
        while i + 2 <= data.len() {
            if let Ok(num) = u8::from_str_radix(&data[i..i + 2], 16) {
                out.push(num as usize);
            }
            i += 2;
        }
    }
    out
}

/// 当日吉神（神煞序号 < 60）。
pub fn get_day_ji_shen(month_zhi_index: i64, day_gan_zhi: &str) -> Vec<&'static str> {
    let mut result = Vec::new();
    for idx in day_shen_sha_indices(month_zhi_index, day_gan_zhi) {
        if idx < 60 {
            result.push(tables::SHEN_SHA[idx]);
        }
    }
    if result.is_empty() {
        result.push("无");
    }
    result
}

/// 当日凶煞（神煞序号 >= 60）。
pub fn get_day_xiong_sha(month_zhi_index: i64, day_gan_zhi: &str) -> Vec<&'static str> {
    let mut result = Vec::new();
    for idx in day_shen_sha_indices(month_zhi_index, day_gan_zhi) {
        if idx >= 60 {
            result.push(tables::SHEN_SHA[idx]);
        }
    }
    if result.is_empty() {
        result.push("无");
    }
    result
}

// ---- 便捷 map 查询封装（按 key 取值，缺省返回 ""）。 ----

/// 纳音。
pub fn nayin(gan_zhi: &str) -> &'static str {
    maps::NAYIN.get(gan_zhi).copied().unwrap_or("")
}
/// 天干五行。
pub fn wu_xing_gan(gan: &str) -> &'static str {
    maps::WU_XING_GAN.get(gan).copied().unwrap_or("")
}
/// 地支五行。
pub fn wu_xing_zhi(zhi: &str) -> &'static str {
    maps::WU_XING_ZHI.get(zhi).copied().unwrap_or("")
}
/// 十神（key 为日干 + 时干两字）。
pub fn shi_shen(two_gans: &str) -> &'static str {
    maps::SHI_SHEN.get(two_gans).copied().unwrap_or("")
}
/// 藏干。
pub fn zhi_hide_gan(zhi: &str) -> &'static [&'static str] {
    maps::ZHI_HIDE_GAN.get(zhi).map(|v| v.as_slice()).unwrap_or(&[])
}
/// 煞方。
pub fn sha(zhi: &str) -> &'static str {
    maps::SHA.get(zhi).copied().unwrap_or("")
}
/// 八卦方位描述。
pub fn position_desc(bagua: &str) -> &'static str {
    maps::POSITION_DESC.get(bagua).copied().unwrap_or("")
}
/// 二十八宿吉凶。
pub fn xiu_luck(xiu: &str) -> &'static str {
    maps::XIU_LUCK.get(xiu).copied().unwrap_or("")
}
/// 二十八宿歌诀。
pub fn xiu_song(xiu: &str) -> &'static str {
    maps::XIU_SONG.get(xiu).copied().unwrap_or("")
}
pub fn zheng(xiu: &str) -> &'static str {
    maps::ZHENG.get(xiu).copied().unwrap_or("")
}
pub fn animal(xiu: &str) -> &'static str {
    maps::ANIMAL.get(xiu).copied().unwrap_or("")
}
pub fn gong(xiu: &str) -> &'static str {
    maps::GONG.get(xiu).copied().unwrap_or("")
}
pub fn shou(bagua: &str) -> &'static str {
    maps::SHOU.get(bagua).copied().unwrap_or("")
}
pub fn xiu(key: &str) -> &'static str {
    maps::XIU.get(key).copied().unwrap_or("")
}
pub fn lu(key: &str) -> &'static str {
    maps::LU.get(key).copied().unwrap_or("")
}
pub fn tian_shen_type(name: &str) -> &'static str {
    maps::TIAN_SHEN_TYPE.get(name).copied().unwrap_or("")
}
pub fn tian_shen_type_luck(tt: &str) -> &'static str {
    maps::TIAN_SHEN_TYPE_LUCK.get(tt).copied().unwrap_or("")
}
