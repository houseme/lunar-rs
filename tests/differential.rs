//! Differential test skeleton for comparing lunar-rs against an external
//! reference implementation.
//!
//! The external reference command is configured via `LUNAR_RS_DIFF_REF_BIN`.
//! The executable is expected to accept:
//!
//! `solar <year> <month> <day> <hour> <minute> <second>`
//!
//! and print newline-delimited `key=value` pairs for:
//!
//! - `protocol_version`
//! - `calendar`
//! - `solar`
//! - `solar_full`
//! - `lunar`
//! - `lunar_full`
//! - `jieqi`
//! - `year_ganzhi`
//! - `month_ganzhi`
//! - `day_ganzhi`
//! - `time_ganzhi`

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use lunar_rs::Solar;
use lunar_rs::differential_support::{PROTOCOL_VERSION, SOLAR_SNAPSHOT_KEYS};

use crate::common::norm;

mod common;

const REF_BIN_ENV: &str = "LUNAR_RS_DIFF_REF_BIN";
const REF_CASES_ENV: &str = "LUNAR_RS_DIFF_CASES";
const DEFAULT_CASES_PATH: &str = "tests/fixtures/differential_cases.txt";

fn load_reference_bin() -> Option<String> {
    env::var(REF_BIN_ENV).ok().filter(|value| !value.trim().is_empty())
}

fn load_case_path() -> PathBuf {
    env::var(REF_CASES_ENV).map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(DEFAULT_CASES_PATH))
}

fn parse_case_line(line: &str) -> Option<(i32, i32, i32, i32, i32, i32)> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    assert_eq!(parts.len(), 6, "differential case line must contain 6 integer columns, got `{trimmed}`");

    Some((
        parts[0].parse().unwrap_or_else(|err| panic!("invalid year in `{trimmed}`: {err}")),
        parts[1].parse().unwrap_or_else(|err| panic!("invalid month in `{trimmed}`: {err}")),
        parts[2].parse().unwrap_or_else(|err| panic!("invalid day in `{trimmed}`: {err}")),
        parts[3].parse().unwrap_or_else(|err| panic!("invalid hour in `{trimmed}`: {err}")),
        parts[4].parse().unwrap_or_else(|err| panic!("invalid minute in `{trimmed}`: {err}")),
        parts[5].parse().unwrap_or_else(|err| panic!("invalid second in `{trimmed}`: {err}")),
    ))
}

fn load_cases(path: &Path) -> Vec<(i32, i32, i32, i32, i32, i32)> {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read differential case file `{}`: {err}", path.display()));
    let cases: Vec<_> = contents.lines().filter_map(parse_case_line).collect();
    assert!(!cases.is_empty(), "differential case file `{}` did not produce any runnable cases", path.display());
    cases
}

fn run_reference(
    reference_bin: &str,
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
) -> HashMap<String, String> {
    let output = Command::new(reference_bin)
        .args([
            "solar",
            &year.to_string(),
            &month.to_string(),
            &day.to_string(),
            &hour.to_string(),
            &minute.to_string(),
            &second.to_string(),
        ])
        .output()
        .unwrap_or_else(|err| panic!("failed to execute reference binary `{reference_bin}`: {err}"));

    assert!(
        output.status.success(),
        "reference binary `{reference_bin}` exited with status {} and stderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout)
        .expect("reference output must be utf-8")
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .collect()
}

fn assert_protocol_shape(reference: &HashMap<String, String>) {
    for key in SOLAR_SNAPSHOT_KEYS {
        assert!(reference.contains_key(*key), "reference output missing required key `{key}`");
    }
    assert_eq!(
        reference.get("protocol_version").map(String::as_str),
        Some(PROTOCOL_VERSION),
        "reference protocol version mismatch"
    );
    assert_eq!(reference.get("calendar").map(String::as_str), Some("solar"), "reference calendar kind mismatch");
}

#[test]
#[ignore = "requires an external reference binary configured via LUNAR_RS_DIFF_REF_BIN"]
fn diff_reference_sample_matrix() {
    let reference_bin = load_reference_bin().expect("set LUNAR_RS_DIFF_REF_BIN to run differential tests");
    let case_path = load_case_path();
    let cases = load_cases(&case_path);

    for (year, month, day, hour, minute, second) in cases {
        let solar = Solar::from_ymd_hms(year, month, day, hour, minute, second).unwrap();
        let lunar = solar.lunar();
        let reference = run_reference(&reference_bin, year, month, day, hour, minute, second);
        assert_protocol_shape(&reference);

        assert_eq!(
            reference.get("solar").map(String::as_str),
            Some(solar.to_ymd_hms().as_str()),
            "solar mismatch for {year}-{month}-{day}"
        );
        assert_eq!(
            reference.get("solar_full").map(|value| norm(value)),
            Some(norm(&solar.to_full_string())),
            "solar full string mismatch for {year}-{month}-{day}"
        );
        assert_eq!(
            reference.get("lunar").map(String::as_str),
            Some(lunar.to_string().as_str()),
            "lunar mismatch for {year}-{month}-{day}"
        );
        assert_eq!(
            reference.get("jieqi").map(String::as_str),
            Some(lunar.jie_qi()),
            "jieqi mismatch for {year}-{month}-{day}"
        );
        assert_eq!(
            reference.get("year_ganzhi").map(String::as_str),
            Some(lunar.year_in_gan_zhi().as_str()),
            "year ganzhi mismatch for {year}-{month}-{day}"
        );
        assert_eq!(
            reference.get("month_ganzhi").map(String::as_str),
            Some(lunar.month_in_gan_zhi().as_str()),
            "month ganzhi mismatch for {year}-{month}-{day}"
        );
        assert_eq!(
            reference.get("day_ganzhi").map(String::as_str),
            Some(lunar.day_in_gan_zhi().as_str()),
            "day ganzhi mismatch for {year}-{month}-{day}"
        );
        assert_eq!(
            reference.get("time_ganzhi").map(String::as_str),
            Some(lunar.time_in_gan_zhi().as_str()),
            "time ganzhi mismatch for {year}-{month}-{day}"
        );
        assert_eq!(
            reference.get("lunar_full").map(|value| norm(value)),
            Some(norm(&lunar.to_full_string())),
            "full string mismatch for {year}-{month}-{day}"
        );
    }
}

#[test]
fn parses_default_case_matrix() {
    let cases = load_cases(Path::new(DEFAULT_CASES_PATH));
    assert!(cases.len() >= 10);
    assert_eq!(cases[0], (2019, 5, 1, 0, 0, 0));
    assert!(cases.contains(&(1582, 10, 15, 0, 0, 0)));
    assert!(cases.contains(&(2024, 4, 22, 23, 30, 0)));
}
