#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

tyme4rs_root="${TYME4RS_PATH:-/private/tmp/tyme4rs-latest}"

if [[ ! -f "$tyme4rs_root/Cargo.toml" ]]; then
  echo "[tyme4rs-diff] missing tyme4rs checkout at $tyme4rs_root" >&2
  echo "[tyme4rs-diff] set TYME4RS_PATH to a local tyme4rs repository" >&2
  exit 1
fi

bridge_dir="$(mktemp -d "${TMPDIR:-/tmp}/tyme4rs-ref-XXXXXX")"
trap 'rm -rf "$bridge_dir"' EXIT

cat >"$bridge_dir/Cargo.toml" <<EOF
[package]
name = "tyme4rs_ref_bridge"
version = "0.1.0"
edition = "2021"

[dependencies]
tyme4rs = { path = "$tyme4rs_root" }
EOF

mkdir -p "$bridge_dir/src"
cat >"$bridge_dir/src/main.rs" <<'EOF'
use std::env;
use std::process::ExitCode;

use tyme4rs::tyme::Culture;
use tyme4rs::tyme::festival::{LunarFestival, SolarFestival};
use tyme4rs::tyme::lunar::{LunarDay, LunarHour, LunarMonth, LunarYear};
use tyme4rs::tyme::solar::{SolarDay, SolarTime};

const PROTOCOL_VERSION: &str = "3";
const DIGITS: [&str; 10] = ["〇", "一", "二", "三", "四", "五", "六", "七", "八", "九"];

fn usage(program: &str) -> String {
    format!(
        "usage:\n  {program} solar <year> <month> <day> <hour> <minute> <second>\n\noutputs newline-delimited key=value pairs"
    )
}

fn parse_isize(name: &str, raw: &str) -> Result<isize, String> {
    raw.parse::<isize>()
        .map_err(|err| format!("failed to parse {name} `{raw}` as isize: {err}"))
}

fn parse_usize(name: &str, raw: &str) -> Result<usize, String> {
    raw.parse::<usize>()
        .map_err(|err| format!("failed to parse {name} `{raw}` as usize: {err}"))
}

fn chinese_year(year: isize) -> String {
    year.to_string()
        .chars()
        .map(|ch| {
            if ch == '-' {
                "负".to_string()
            } else {
                let digit = ch.to_digit(10).unwrap_or(0) as usize;
                DIGITS[digit].to_string()
            }
        })
        .collect()
}

fn solar_key(solar: SolarTime) -> String {
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        solar.get_year(),
        solar.get_month(),
        solar.get_day(),
        solar.get_hour(),
        solar.get_minute(),
        solar.get_second()
    )
}

fn lunar_key(lunar: LunarDay) -> String {
    format!(
        "{}年{}{}",
        chinese_year(lunar.get_year()),
        lunar.get_lunar_month().get_name(),
        lunar.get_name()
    )
}

fn current_term_name(solar_day: SolarDay) -> String {
    let term_day = solar_day.get_term_day();
    if term_day.get_day_index() == 0 {
        term_day.get_name()
    } else {
        String::new()
    }
}

fn festival_name(festival: Option<impl ToString>) -> String {
    festival.map_or_else(String::new, |festival| festival.to_string().split(' ').last().unwrap_or("").to_string())
}

fn solar_festival_index(festival: Option<SolarFestival>) -> String {
    festival.map_or_else(String::new, |festival| festival.get_index().to_string())
}

fn lunar_festival_index(festival: Option<LunarFestival>) -> String {
    festival.map_or_else(String::new, |festival| festival.get_index().to_string())
}

fn snapshot(solar_time: SolarTime) -> Vec<(&'static str, String)> {
    let solar_day = solar_time.get_solar_day();
    let lunar_day = solar_day.get_lunar_day();
    let lunar_hour = LunarHour::from_ymd_hms(
        lunar_day.get_year(),
        lunar_day.get_month(),
        lunar_day.get_day(),
        solar_time.get_hour(),
        solar_time.get_minute(),
        solar_time.get_second(),
    );
    let lunar_month: LunarMonth = lunar_day.get_lunar_month();
    let lunar_year: LunarYear = lunar_month.get_lunar_year();
    let solar_festival = solar_day.get_festival();
    let lunar_festival = lunar_day.get_festival();

    vec![
        ("protocol_version", PROTOCOL_VERSION.to_string()),
        ("calendar", "solar".to_string()),
        ("solar", solar_key(solar_time)),
        ("solar_full", String::new()),
        ("lunar", lunar_key(lunar_day.clone())),
        ("lunar_full", String::new()),
        ("solar_festival", festival_name(solar_festival.clone())),
        ("solar_festival_index", solar_festival_index(solar_festival)),
        ("lunar_festival", festival_name(lunar_festival.clone())),
        ("lunar_festival_index", lunar_festival_index(lunar_festival)),
        ("jieqi", current_term_name(solar_day)),
        ("week_name", solar_day.get_week().get_name()),
        ("week_index", solar_day.get_week().get_index().to_string()),
        ("constellation", solar_day.get_constellation().get_name()),
        (
            "legal_holiday",
            solar_day.get_legal_holiday().map_or_else(String::new, |holiday| holiday.get_name()),
        ),
        (
            "legal_holiday_work",
            solar_day.get_legal_holiday().map_or_else(String::new, |holiday| holiday.is_work().to_string()),
        ),
        ("year_ganzhi", lunar_day.get_year_sixty_cycle().get_name()),
        ("month_ganzhi", lunar_day.get_month_sixty_cycle().get_name()),
        ("day_ganzhi", lunar_day.get_sixty_cycle().get_name()),
        ("time_ganzhi", lunar_hour.get_sixty_cycle().get_name()),
        ("lunar_year_month_count", lunar_year.get_month_count().to_string()),
        ("lunar_year_leap_month", lunar_year.get_leap_month().to_string()),
        ("lunar_month", lunar_month.get_month().to_string()),
        (
            "lunar_month_with_leap",
            lunar_month.get_month_with_leap().to_string(),
        ),
        ("lunar_month_day_count", lunar_month.get_day_count().to_string()),
        (
            "lunar_month_index_in_year",
            lunar_month.get_index_in_year().to_string(),
        ),
    ]
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let program = args
        .first()
        .cloned()
        .unwrap_or_else(|| "tyme4rs_ref_bridge".to_string());

    if args.len() != 8 || args[1] != "solar" {
        return Err(usage(&program));
    }

    let year = parse_isize("year", &args[2])?;
    let month = parse_usize("month", &args[3])?;
    let day = parse_usize("day", &args[4])?;
    let hour = parse_usize("hour", &args[5])?;
    let minute = parse_usize("minute", &args[6])?;
    let second = parse_usize("second", &args[7])?;

    let solar = SolarTime::from_ymd_hms(year, month, day, hour, minute, second);

    for (key, value) in snapshot(solar) {
        println!("{key}={value}");
    }

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
EOF

echo "[tyme4rs-diff] building tyme4rs reference bridge"
cargo build --offline --manifest-path "$bridge_dir/Cargo.toml"

bridge_bin="$bridge_dir/target/debug/tyme4rs_ref_bridge"
if [[ ! -x "$bridge_bin" ]]; then
  echo "[tyme4rs-diff] missing bridge binary at $bridge_bin" >&2
  exit 1
fi

echo "[tyme4rs-diff] running ignored differential test against tyme4rs bridge"
LUNAR_RS_DIFF_REF_BIN="$bridge_bin" \
LUNAR_RS_DIFF_REF_FLAVOR="tyme4rs" \
LUNAR_RS_DIFF_CASES="$repo_root/tests/fixtures/differential_cases.txt" \
  cargo test diff_reference_sample_matrix -- --ignored
