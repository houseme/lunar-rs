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
- Added `LunarError` as the unified error type for invalid Solar/Lunar inputs.
- Added golden integration tests in `tests/lunar_core.rs` for Solar, Lunar,
  JieQi, leap months, 1582 reform handling, festivals, traversal, and reference
  alignment.
- Added English and Chinese README documentation.

### Changed

- Updated crate metadata for Rust 2024 edition.
- Switched the toolchain file to `rust-toolchain.toml`.
- Updated CI workflow action references and crate package names.
- Exposed `solar_util` for integration tests and utility users.
- Replaced the starter example code with the calendar library API surface.
- Normalized ShouXing floating-point data literals and removed extracted inline
  comments from generated constant data.

### Fixed

- Fixed several `i32`/`i64` arithmetic mismatches in Lunar and Yun calculations.
- Fixed Taoist HashMap lookups by using string-slice keys.
- Removed duplicate imports from generated Buddhist data includes.
- Refined Lunar WuHou indexing.

### Known Issues

- `cargo check` passes, but `cargo test` still has one golden full-string
  alignment case pending. The remaining difference is in detailed `Lunar`
  full-string output for one reference case.

## [0.1.0] - Unreleased

### Added

- Initial development version.
