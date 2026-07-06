# Changelog

All notable changes to this project will be documented in this file.

The format follows the spirit of [Keep a Changelog](https://keepachangelog.com/),
and this project uses semantic versioning once releases are published.

## [Unreleased]

### Added

- Added the initial Rust crate scaffold for `lunar-rs`.
- Added Solar/Gregorian date primitives with:
  - date and date-time constructors,
  - formatting helpers,
  - leap-year and month-day utilities,
  - Julian day conversion,
  - date comparison and traversal,
  - 1582 Gregorian reform gap handling,
  - constellation, holiday, workday, and salary-rate helpers.
- Added Solar aggregate models:
  - `SolarYear`,
  - `SolarMonth`,
  - `SolarWeek`,
  - `SolarSeason`,
  - `SolarHalfYear`.
- Added the ShouXing astronomical engine and extracted data constants for JieQi
  and lunar month calculations.
- Added Lunar core models and tables:
  - `Lunar`,
  - `LunarYear`,
  - `LunarMonth`,
  - `LunarTime`,
  - `JieQi`,
  - `lunar_util` tables and lookup helpers.
- Added Lunar features for:
  - Solar/Lunar conversion,
  - leap-month handling,
  - GanZhi,
  - ShengXiao,
  - NaYin,
  - Xun and XunKong,
  - JieQi lookup,
  - lunar festivals,
  - almanac positions,
  - PengZu,
  - Chong/Sha,
  - Yi/Ji,
  - JiShen and XiongSha,
  - lunar phase, Hou, WuHou, ShuJiu, SanFu, and Nine Star calculations.
- Added BaZi/EightChar and fortune-cycle primitives:
  - `EightChar`,
  - `Yun`,
  - `DaYun`,
  - `LiuNian`,
  - `LiuYue`,
  - `XiaoYun`.
- Added public holiday support with:
  - `Holiday`,
  - embedded raw holiday data,
  - holiday lookup helpers,
  - workday traversal integration.
- Added Buddhist calendar support groundwork:
  - `Foto`,
  - `FotoFestival`,
  - Buddhist festival maps,
  - 27-star lookup helpers.
- Added Taoist calendar support groundwork:
  - `Tao`,
  - `TaoFestival`,
  - Taoist festival maps,
  - BaHui, BaJie, SanHui, SanYuan, WuLa, and Wu-day helpers.
- Added `Lunar::foto()` and `Lunar::tao()` convenience wrappers for Buddhist
  and Taoist calendar access.
- Added `LunarError` as the unified error type for invalid Solar/Lunar inputs.
- Added golden integration tests in `tests/lunar_core.rs` for Solar, Lunar,
  JieQi, leap months, 1582 reform handling, festivals, traversal, and reference
  alignment.
- Added runtime holiday override APIs:
  - `holiday_util::holiday_names`,
  - `holiday_util::holiday_data`,
  - `holiday_util::set_holidays`,
  - `holiday_util::set_holiday_data`,
  - `holiday_util::reset_holidays`.
- Added a stable `cargo bench --bench convert` benchmark target for conversion,
  formatting, and JieQi lookup hot paths.
- Added domain-split integration suites for Solar, Lunar, JieQi, and
  EightChar golden cases.
- Added an ignored differential-test skeleton that can compare `lunar-rs`
  against an external reference binary.
- Added `lunar_ref_driver`, a sample differential-testing driver that emits the
  stable solar snapshot protocol used by the ignored diff suite.
- Added `scripts/run_differential_self_check.sh` to exercise the ignored diff
  suite against the in-repo sample driver.
- Added `tests/fixtures/differential_cases.txt` and
  `scripts/run_differential_matrix.sh` so differential validation can run
  against a maintained external case matrix.
- Added an initial `i18n` feature implementation with explicit language-aware
  helpers for weekday, constellation, JieQi, ShengXiao, GanZhi, NineStar, and
  full-string output on core calendar objects.
- Added the first Phase 2 typed domain API slice:
  - `HeavenStem`,
  - `EarthBranch`,
  - `SixtyCycle`,
  - `Zodiac`,
  - `Direction`,
  - `Element`,
  - `Duty`,
  - `Phase`,
  - `Phenology`,
  - plus typed getters on `Lunar` and `NineStar`.
- Added the first minimal Phase 3 event model:
  - `EventKind`,
  - `Event`,
  - `Solar::events()`,
  - `Lunar::events()`,
  - `Foto::events()`,
  - `Tao::events()`.
- Added second-layer event semantics:
  - `CalendarKind`,
  - `EventSource`,
  - source/calendar/detail label helpers on `Event`.
- Added event query utilities:
  - `EventQuery`,
  - `filter_events(...)`,
  - `find_events(...)` on `Solar`, `Lunar`, `Foto` and `Tao`.
- Added English and Chinese README documentation.

### Changed

- Updated crate metadata for Rust 2024 edition.
- Switched the toolchain file to `rust-toolchain.toml`.
- Updated CI workflow action references and crate package names.
- Exposed `solar_util` for integration tests and utility users.
- Added optional `serde` support for owned core calendar data types behind the
  `serde` feature flag.
- Replaced the starter example code with the calendar library API surface.
- Normalized ShouXing floating-point data literals and removed extracted inline
  comments from generated constant data.
- Documented Buddhist and Taoist calendar access in both English and Chinese
  READMEs.
- Scoped Clippy allow rules to ShouXing generated data and engine modules where
  precision-oriented extracted constants intentionally trigger numeric lints.
- Refactored time-branch index lookup to parse `HH:MM` with zero heap
  allocations instead of building comparison strings.
- Cleaned up compiler and Clippy warnings across calendar, holiday, lunar year,
  solar traversal, and fortune-cycle modules.

### Fixed

- Fixed several `i32`/`i64` arithmetic mismatches in Lunar and Yun calculations.
- Fixed Taoist HashMap lookups by using string-slice keys.
- Removed duplicate imports from generated Buddhist data includes.
- Refined Lunar WuHou indexing.
- Fixed `Lunar::eight_char()` to return an explicitly borrowed `EightChar<'_>`.
- Fixed two-digit upper bounds in `lunar_util::get_time_zhi_index`, keeping time
  branch comparisons lexicographically stable.

## [0.1.0] - Unreleased

### Added

- Initial development version.
