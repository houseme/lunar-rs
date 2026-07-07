# lunar-rs

A Rust calendar library for the Gregorian/Solar calendar, Chinese Lunar calendar,
JieQi, GanZhi, almanac data, Buddhist calendar, Taoist calendar, BaZi, and
fortune-cycle calculations.

The implementation is a Rust port inspired by
[lunar-go](https://github.com/6tail/lunar-go) and
[lunar-javascript](https://github.com/6tail/lunar-javascript), with ShouXing
astronomical algorithms and embedded data tables. The core crate is designed to
avoid third-party runtime dependencies.

> Status: **1.0.0-rc1** release candidate. `cargo test` and crate doctests pass
> against the local golden integration suite. A verified feature-by-feature
> comparison against [`6tail/tyme4rs`](https://github.com/6tail/tyme4rs) v1.5
> confirms `lunar-rs` is now a functional **superset** (30 calendar systems vs
> 4, plus Buddhist/Taoist calendars, runtime holiday override, i18n, and serde),
> with a `tyme4rs` compatibility layer of type aliases and `get_*` entry points.

## Documentation

- [`docs/usage-guide.md`](docs/usage-guide.md) — end-to-end user guide (Chinese)
  covering conversion, GanZhi, JieQi, almanac fields, EightChar, multi-calendar,
  events, holidays, i18n, serde, and tyme4rs migration.
- [`docs/tyme4rs-comparison-verified-2026-07-07.md`](docs/tyme4rs-comparison-verified-2026-07-07.md)
  — verified feature-gap analysis vs `tyme4rs` v1.5, with `file:line` evidence.
- [`docs/DESIGN.md`](docs/DESIGN.md) — internal design notes.

## Features

- Solar/Gregorian date construction, formatting, traversal, Julian day, weekdays,
  constellations, holidays, workday traversal, and salary rate.
- Solar to Lunar and Lunar to Solar conversion, including leap months and the
  1582 Gregorian reform gap.
- Lunar year/month/day/hour fields, Chinese date strings, GanZhi, ShengXiao,
  NaYin, Xun, XunKong, Chong, Sha, TaiSui, positions, PengZu, Yi/Ji, JiShen,
  XiongSha, lunar festivals, and other traditional almanac fields.
- JieQi and Qi/Jie lookup with precise timestamps from the astronomical engine.
- Lunar year/month metadata and month generation.
- Nine Star, ShuJiu, SanFu, SolarYear, SolarMonth, SolarWeek, SolarSeason, and
  SolarHalfYear helper models.
- BaZi/EightChar, DaYun, LiuNian, LiuYue, and XiaoYun fortune-cycle primitives.
- Buddhist and Taoist calendar models available from `Lunar::foto()` and
  `Lunar::tao()`.
- Additional multi-calendar companions now include Hijri, RabByung, Minguo,
  ThaiSolar, Japanese, Juche, Dangi, Julian, Holocene, Byzantine, Coptic,
  Ethiopian, Armenian, AUC, Assyrian, Hispanic Era, Saka, Koki, ThaiBuddhist,
  Fasli, Nanakshahi, Seleucid, Rattanakosin, Venetian, Rumi, and AnnoLucis
  variants from `Solar`.
- Minimal unified event aggregation via `Solar::events()`, `Lunar::events()`,
  `Foto::events()` and `Tao::events()` for festivals, holidays and JieQi.
- Events now carry `EventKind`, `CalendarKind`, `EventSource`, solar anchor
  date and optional detail text for more stable downstream consumption.
- Event queries are available through `EventQuery` plus `find_events(...)` on
  `Solar`, `Lunar`, `Foto` and `Tao`.
- Event rule metadata now also includes `priority`, `source_id` and
  `is_observed`, with construction normalized by the source objects.
- Event range scans are available through `scan_events_in_range(...)`,
  `scan_events_in_range_filtered(...)`, and the `events_until(...)` /
  `find_events_until(...)` convenience methods on `Solar` and `Lunar`.
- Runtime holiday data override hooks via `holiday_util::set_holidays(...)`,
  `set_holiday_data(...)`, and `reset_holidays()`.
- Experimental explicit-language helpers behind the `i18n` feature for weekday,
  constellation, JieQi, ShengXiao, GanZhi, NineStar, and full-string rendering
  on core calendar objects.
- Optional `serde` support for owned core calendar data types behind the
  `serde` feature.
- The default build keeps zero third-party runtime dependencies; optional
  features can opt into extra integration support.

## Installation

From crates.io after release:

```toml
[dependencies]
lunar-rs = "1.0.0-rc1"
```

From Git while the crate is under active development:

```toml
[dependencies]
lunar-rs = { git = "https://github.com/houseme/lunar-rs" }
```

The library crate name is `lunar_rs`:

```rust
use lunar_rs::{Lunar, Solar};
```

## Quick Start

Solar to Lunar:

```rust
use lunar_rs::Solar;

let solar = Solar::from_ymd(2020, 5, 24).unwrap();
let lunar = solar.lunar();

assert_eq!(lunar.to_string(), "二〇二〇年闰四月初二");
assert_eq!(lunar.year_in_gan_zhi(), "庚子");
assert_eq!(lunar.year_sheng_xiao(), "鼠");
```

Lunar to Solar:

```rust
use lunar_rs::Lunar;

let lunar = Lunar::from_ymd(2019, 3, 27).unwrap();
let solar = lunar.solar();

assert_eq!(solar.to_ymd(), "2019-05-01");
```

JieQi lookup:

```rust
use lunar_rs::Lunar;

let lunar = Lunar::from_ymd(2012, 9, 1).unwrap();
let bai_lu = lunar.jie_qi_table().get("白露").unwrap();

assert_eq!(bai_lu.to_ymd_hms(), "2012-09-07 13:29:01");
```

Date traversal:

```rust
use lunar_rs::Solar;

assert_eq!(
    Solar::from_ymd(1582, 10, 4).unwrap().next_day(1).to_ymd(),
    "1582-10-15"
);

assert_eq!(
    Solar::from_ymd(2023, 8, 31).unwrap().next_month(6).to_ymd(),
    "2024-02-29"
);
```

BaZi and fortune cycles:

```rust
use lunar_rs::{Gender, Solar};

let lunar = Solar::from_ymd(2021, 12, 21).unwrap().lunar();
let eight_char = lunar.eight_char();
let yun = eight_char.yun(1 as Gender);

println!("BaZi: {} {} {} {}", eight_char.year(), eight_char.month(), eight_char.day(), eight_char.time());
println!("Yun starts at {} years {} months", yun.start_year(), yun.start_month());
```

Buddhist and Taoist calendars:

```rust
use lunar_rs::Solar;

let lunar = Solar::from_ymd(2024, 5, 15).unwrap().lunar();
let foto = lunar.foto();
let tao = lunar.tao();

println!("Buddhist calendar: {}", foto.to_string_cn());
println!("Taoist calendar: {}", tao.to_string_cn());
```

Multi-calendar conversion (30 systems reachable from `Solar`):

```rust
use lunar_rs::Solar;

let solar = Solar::from_ymd(2024, 5, 15).unwrap();
println!("Hijri:    {}", solar.hijri());
println!("Tibetan:  {}", solar.rab_byung_day().unwrap());
println!("Minguo:   {}", solar.minguo());
println!("Japanese: {}", solar.japanese().unwrap());
println!("Julian:   {}", solar.julian_calendar());
```

Event aggregation (festivals, JieQi, legal holidays in one call):

```rust
use lunar_rs::Solar;

let solar = Solar::from_ymd(2024, 10, 1).unwrap();
for event in solar.events() {
    println!("[{:?}] {}", event.kind(), event.name());
}

// Range scan over a period
let end = Solar::from_ymd(2024, 10, 7).unwrap();
let count = solar.events_until(end).len();
```

## tyme4rs Migration Notes

`lunar-rs` now exposes a compatibility layer for many common `tyme4rs`-style
entry points. A few practical patterns:

```rust
use lunar_rs::{LunarHour, SolarDay, SolarFestival, SolarTime, SolarTerm};

let solar_time: SolarTime = SolarTime::from_ymd_hms(2024, 2, 10, 8, 30, 0).unwrap();
let lunar_hour: LunarHour<'static> = solar_time.get_lunar_hour();

assert_eq!(lunar_hour.get_solar_time().to_ymd_hms(), "2024-02-10 08:30:00");
assert_eq!(lunar_hour.get_minor_ren().name(), solar_time.lunar().time_minor_ren().name());

let solar_day: SolarDay = SolarDay::from_ymd(2024, 10, 1).unwrap();
let festival: SolarFestival = solar_day.get_festival().unwrap();
assert_eq!(festival.get_name(), "国庆节");

let term: SolarTerm = SolarTerm::from_name(2023, "大雪").unwrap();
assert_eq!(term.get_solar_day().to_ymd(), "2023-12-07");
```

For migrated code, keep two semantic differences in mind:

- Some local constructors and companion getters return `Option`/`Result` where
  `tyme4rs` may choose panic-style constructors.
- `Foto` and `Tao` wrappers are local extensions rather than direct
  `tyme4rs v1.5` modules, but they now support owned wrapper usage and common
  `get_*` naming.

## Validation

```bash
cargo check
cargo test
cargo bench --bench convert
cargo run --bin lunar_ref_driver -- solar 2024 4 22 23 30 0
cargo test --features i18n
```

Current local validation:

- `cargo check` passes.
- `cargo test` passes the current golden integration suite.
- Doc tests pass.

Differential test workflow:

```bash
cargo run --bin lunar_ref_driver -- solar 2024 4 22 23 30 0
sh scripts/run_differential_self_check.sh
bash scripts/run_tyme4rs_diff_check.sh
```

## Project Layout

- `src/solar*.rs`: Solar date primitives and aggregate periods.
- `src/lunar*.rs`: Lunar date core, years, months, times, and utility tables.
- `src/shou_xing/`: ShouXing astronomical engine and extracted constants.
- `src/eight_char.rs`, `src/yun/`: BaZi and fortune-cycle APIs.
- `src/foto*.rs`, `src/tao*.rs`: Buddhist and Taoist calendar support.
- `src/holiday*.rs`: Chinese public holiday data and lookup helpers.
- `tests/solar.rs`, `tests/lunar.rs`, `tests/jieqi.rs`, `tests/eight_char.rs`:
  Golden cases split by domain.
- `tests/differential.rs`: ignored differential-test skeleton for external
  reference implementations.
- `tests/fixtures/differential_cases.txt`: maintained case matrix for
  differential validation.
- `src/bin/lunar_ref_driver.rs`: sample differential-testing driver that emits
  stable `key=value` snapshots.
- `scripts/run_differential_self_check.sh`: one-command local protocol smoke
  check using the sample driver itself.
- `scripts/run_tyme4rs_diff_check.sh`: builds a temporary tyme4rs v2 reference
  bridge and runs the ignored external differential test matrix against it.

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT License
