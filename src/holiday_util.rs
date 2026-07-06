//! 法定节假日工具（自 2001-12-29 起）。对应 lunar-go `HolidayUtil/HolidayUtil.go`。

use std::sync::{LazyLock, RwLock};

use crate::LunarError;
use crate::holiday::Holiday;

include!("holiday_data.rs");

const SIZE: usize = 18;

// 运行期可覆盖（Fix）。
static NAMES_IN_USE: LazyLock<RwLock<Vec<String>>> =
    LazyLock::new(|| RwLock::new(NAMES.iter().map(|name| (*name).to_string()).collect()));
static DATA_IN_USE: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new(RAW_DATA.to_string()));

fn validate_holiday_data(data: &str, names_len: usize) -> Result<(), LunarError> {
    if names_len > 10 {
        return Err(LunarError::Parse("holiday names exceed one-digit encoding capacity".to_string()));
    }
    if data.len() % SIZE != 0 {
        return Err(LunarError::Parse(format!("holiday raw data length {} is not a multiple of {SIZE}", data.len())));
    }
    for (chunk_index, chunk) in data.as_bytes().chunks(SIZE).enumerate() {
        if !chunk[0..8].iter().all(u8::is_ascii_digit) || !chunk[10..SIZE].iter().all(u8::is_ascii_digit) {
            return Err(LunarError::Parse(format!(
                "holiday raw data chunk {chunk_index} contains non-digit date fields"
            )));
        }
        if !chunk[8].is_ascii_digit() {
            return Err(LunarError::Parse(format!("holiday raw data chunk {chunk_index} has invalid name index")));
        }
        let name_index = usize::from(chunk[8] - b'0');
        if name_index >= names_len {
            return Err(LunarError::Parse(format!(
                "holiday raw data chunk {chunk_index} references missing holiday name index {name_index}"
            )));
        }
        if !matches!(chunk[9], b'0' | b'1') {
            return Err(LunarError::Parse(format!("holiday raw data chunk {chunk_index} has invalid work flag")));
        }
    }
    Ok(())
}

/// 当前节假日名称表快照。
pub fn holiday_names() -> Vec<String> {
    NAMES_IN_USE.read().unwrap().clone()
}

/// 当前节假日原始数据快照。
pub fn holiday_data() -> String {
    DATA_IN_USE.read().unwrap().clone()
}

/// 重置为内置节假日数据。
pub fn reset_holidays() {
    *NAMES_IN_USE.write().unwrap() = NAMES.iter().map(|name| (*name).to_string()).collect();
    *DATA_IN_USE.write().unwrap() = RAW_DATA.to_string();
}

/// 替换完整节假日名称表与原始数据。
pub fn set_holidays(names: Vec<String>, raw_data: impl Into<String>) -> Result<(), LunarError> {
    let raw_data = raw_data.into();
    validate_holiday_data(&raw_data, names.len())?;
    *NAMES_IN_USE.write().unwrap() = names;
    *DATA_IN_USE.write().unwrap() = raw_data;
    Ok(())
}

/// 仅替换原始节假日数据，保留当前名称表。
pub fn set_holiday_data(raw_data: impl Into<String>) -> Result<(), LunarError> {
    let raw_data = raw_data.into();
    let names_len = NAMES_IN_USE.read().unwrap().len();
    validate_holiday_data(&raw_data, names_len)?;
    *DATA_IN_USE.write().unwrap() = raw_data;
    Ok(())
}

fn build_holiday_forward(s: &str) -> Holiday {
    let day = &s[0..8];
    let name = {
        let names = NAMES_IN_USE.read().unwrap();
        names[(s.as_bytes()[8] - b'0') as usize].clone()
    };
    let work = s.as_bytes()[9] == b'0';
    let target = &s[10..SIZE];
    Holiday::new(day, &name, work, target)
}

fn build_holiday_backward(s: &str) -> Holiday {
    let length = s.len();
    let day = &s[length - 18..length - 10];
    let name = {
        let names = NAMES_IN_USE.read().unwrap();
        names[(s.as_bytes()[length - 10] - b'0') as usize].clone()
    };
    let work = s.as_bytes()[length - 9] == b'0';
    let target = &s[length - 8..];
    Holiday::new(day, &name, work, target)
}

fn find_forward(key: &str, data: &str) -> Option<usize> {
    let start = data.find(key)?;
    let n = (data.len() - start) % SIZE;
    let mut pos = start + n;
    while pos + SIZE <= data.len() {
        if data[pos..].starts_with(key) {
            return Some(pos);
        }
        pos += SIZE;
    }
    None
}

fn find_backward(key: &str, data: &str) -> Option<usize> {
    let start = data.rfind(key)?;
    let mut end = start + key.len();
    let n = end % SIZE;
    end -= n;
    while end >= SIZE {
        if data[..end].ends_with(key) {
            return Some(end - SIZE);
        }
        end -= SIZE;
    }
    None
}

/// 按阳历日期（YYYY-MM-DD 或 YYYYMMDD）查找首个节假日。
pub fn get_holiday(ymd: &str) -> Option<Holiday> {
    let key = ymd.replace('-', "");
    let data = DATA_IN_USE.read().unwrap().clone();
    let pos = find_forward(&key, &data)?;
    Some(build_holiday_forward(&data[pos..pos + SIZE]))
}

/// 按年月日查找首个节假日。
pub fn get_holiday_by_ymd(year: i32, month: i32, day: i32) -> Option<Holiday> {
    get_holiday(&format!("{year:04}{month:02}{day:02}"))
}

/// 按年月查找全部节假日。
pub fn get_holidays_by_ym(year: i32, month: i32) -> Vec<Holiday> {
    collect_forward(&format!("{year:04}{month:02}"))
}

/// 按年查找全部节假日。
pub fn get_holidays_by_year(year: i32) -> Vec<Holiday> {
    collect_forward(&format!("{year:04}"))
}

/// 按日期查找全部节假日。
pub fn get_holidays(ymd: &str) -> Vec<Holiday> {
    collect_forward(&ymd.replace('-', ""))
}

fn collect_forward(key: &str) -> Vec<Holiday> {
    let data = DATA_IN_USE.read().unwrap().clone();
    let mut out = Vec::new();
    if let Some(mut pos) = find_forward(key, &data) {
        loop {
            if pos + SIZE > data.len() || !data[pos..pos + SIZE].starts_with(key) {
                break;
            }
            out.push(build_holiday_forward(&data[pos..pos + SIZE]));
            pos += SIZE;
        }
    }
    out
}

/// 按目标节日日期反查全部节假日（调休映射）。
pub fn get_holidays_by_target_ymd(year: i32, month: i32, day: i32) -> Vec<Holiday> {
    let key = format!("{year:04}{month:02}{day:02}");
    let data = DATA_IN_USE.read().unwrap().clone();
    let mut out = Vec::new();
    if let Some(mut pos) = find_backward(&key, &data) {
        loop {
            let seg = &data[pos..pos + SIZE];
            if !seg.ends_with(&key) {
                break;
            }
            out.push(build_holiday_backward(seg));
            if pos < SIZE {
                break;
            }
            pos -= SIZE;
        }
        out.reverse();
    }
    out
}
