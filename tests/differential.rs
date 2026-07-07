//! Differential test skeleton for comparing lunar-rs against an external
//! reference implementation.
//!
//! The external reference command is configured via `LUNAR_RS_DIFF_REF_BIN`.
//! The executable is expected to accept:
//!
//! `solar <year> <month> <day> <hour> <minute> <second>`
//!
//! and print newline-delimited `key=value` pairs. The canonical key set and
//! expected values live in `lunar_rs::differential_support::FIELDS` (a single
//! source of truth); this file only drives the comparison and flavor handling.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use lunar_rs::differential_support::{solar_snapshot, solar_snapshot_keys, PROTOCOL_VERSION, Scope};
use lunar_rs::Solar;

use crate::common::norm;

mod common;

const REF_BIN_ENV: &str = "LUNAR_RS_DIFF_REF_BIN";
const REF_CASES_ENV: &str = "LUNAR_RS_DIFF_CASES";
const REF_FLAVOR_ENV: &str = "LUNAR_RS_DIFF_REF_FLAVOR";
const DEFAULT_CASES_PATH: &str = "tests/fixtures/differential_cases.txt";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReferenceFlavor {
    Local,
    Tyme4rs,
}

fn load_reference_bin() -> Option<String> {
    env::var(REF_BIN_ENV).ok().filter(|value| !value.trim().is_empty())
}

fn load_reference_flavor() -> ReferenceFlavor {
    match env::var(REF_FLAVOR_ENV).ok().as_deref() {
        Some("tyme4rs") => ReferenceFlavor::Tyme4rs,
        _ => ReferenceFlavor::Local,
    }
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
    for key in solar_snapshot_keys() {
        assert!(reference.contains_key(key), "reference output missing required key `{key}`");
    }
    assert_eq!(
        reference.get("protocol_version").map(String::as_str),
        Some(PROTOCOL_VERSION),
        "reference protocol version mismatch"
    );
    assert_eq!(
        reference.get("calendar").map(String::as_str),
        Some("solar"),
        "reference calendar kind mismatch"
    );
}

fn normalize_lunar_compat(value: &str) -> String {
    norm(value).replace("冬月", "十一月").replace("腊月", "十二月")
}

/// Apply the field/flavor-aware normalization used on both sides of the comparison.
fn normalize(key: &str, value: &str, flavor: ReferenceFlavor) -> String {
    match (key, flavor) {
        // Full strings carry locale-dependent spacing; compare whitespace-agnostic.
        ("solar_full", _) | ("lunar_full", _) => norm(value),
        // `tyme4rs` uses 冬月/腊月; lunar-rs uses 十一/十二月. Normalize for parity.
        ("lunar", ReferenceFlavor::Tyme4rs) => normalize_lunar_compat(value),
        _ => value.to_string(),
    }
}

#[test]
#[ignore = "requires an external reference binary configured via LUNAR_RS_DIFF_REF_BIN"]
fn diff_reference_sample_matrix() {
    let reference_bin = load_reference_bin().expect("set LUNAR_RS_DIFF_REF_BIN to run differential tests");
    let reference_flavor = load_reference_flavor();
    let case_path = load_case_path();
    let cases = load_cases(&case_path);

    for (year, month, day, hour, minute, second) in cases {
        let solar = Solar::from_ymd_hms(year, month, day, hour, minute, second).unwrap();
        let reference = run_reference(&reference_bin, year, month, day, hour, minute, second);
        assert_protocol_shape(&reference);

        // The local expected values come from the single snapshot definition; the
        // comparison rules (which fields are local-only, which need normalization)
        // are expressed once here instead of repeated per field.
        for entry in solar_snapshot(solar) {
            if entry.scope == Scope::LocalOnly && reference_flavor == ReferenceFlavor::Tyme4rs {
                continue;
            }
            // Known convention divergence at the late zǐ hour (23:00–23:59): the
            // 时十二神 depends on the hour's day-branch, which lunar-rs (following
            // lunar-javascript's default sect) keeps on the current day while
            // tyme4rs rolls it to the next day. The two libraries' formulas are
            // otherwise identical; this is a sect流派 difference, not a defect.
            if reference_flavor == ReferenceFlavor::Tyme4rs
                && hour == 23
                && entry.key == "lunar_hour_twelve_star"
            {
                continue;
            }
            let reference_value = reference.get(entry.key).map(String::as_str).unwrap_or("");
            assert_eq!(
                normalize(entry.key, reference_value, reference_flavor),
                normalize(entry.key, entry.value.as_str(), reference_flavor),
                "{} mismatch for {year}-{month}-{day}",
                entry.key
            );
        }
    }
}

#[test]
fn parses_default_case_matrix() {
    let cases = load_cases(Path::new(DEFAULT_CASES_PATH));
    assert!(cases.len() >= 24);
    assert_eq!(cases[0], (2019, 5, 1, 0, 0, 0));
    assert!(cases.contains(&(1582, 10, 15, 0, 0, 0)));
    assert!(cases.contains(&(2024, 4, 22, 23, 30, 0)));
    assert!(cases.contains(&(2019, 2, 5, 0, 0, 0)));
    assert!(cases.contains(&(2020, 1, 24, 0, 0, 0)));
    assert!(cases.contains(&(2020, 5, 23, 0, 0, 0)));
    assert!(cases.contains(&(2024, 4, 4, 12, 0, 0)));
    assert!(cases.contains(&(2024, 9, 17, 0, 0, 0)));
    assert!(cases.contains(&(2024, 10, 1, 0, 0, 0)));
    assert!(cases.contains(&(2024, 2, 18, 0, 0, 0)));
    assert!(cases.contains(&(2020, 12, 21, 0, 0, 0)));
    assert!(cases.contains(&(2012, 7, 18, 0, 0, 0)));
    assert!(cases.contains(&(2024, 6, 11, 0, 0, 0)));
    assert!(cases.contains(&(2024, 12, 4, 0, 0, 0)));
    assert!(cases.contains(&(2023, 11, 14, 23, 0, 0)));
}
