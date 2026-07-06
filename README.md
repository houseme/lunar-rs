# lunar-rs

A Rust calendar library for the Gregorian/Solar calendar, Chinese Lunar calendar,
JieQi, GanZhi, almanac data, Buddhist calendar, Taoist calendar, BaZi, and
fortune-cycle calculations.

The implementation is a Rust port inspired by
[lunar-go](https://github.com/6tail/lunar-go) and
[lunar-javascript](https://github.com/6tail/lunar-javascript), with ShouXing
astronomical algorithms and embedded data tables. The core crate is designed to
avoid third-party runtime dependencies.

> Status: active 0.1.0 port. `cargo test` and crate doctests currently pass
> against the local golden integration suite.

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
- Runtime holiday data override hooks via `holiday_util::set_holidays(...)`,
  `set_holiday_data(...)`, and `reset_holidays()`.
- Experimental explicit-language helpers behind the `i18n` feature for weekday,
  constellation, JieQi, ShengXiao, and GanZhi translation.
- Optional `serde` support for owned core calendar data types behind the
  `serde` feature.
- The default build keeps zero third-party runtime dependencies; optional
  features can opt into extra integration support.

## Installation

From crates.io after release:

```toml
[dependencies]
lunar-rs = "0.1"
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
LUNAR_RS_DIFF_REF_BIN=/path/to/reference-driver cargo test diff_reference_sample_matrix -- --ignored
sh scripts/run_differential_self_check.sh
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
- `src/bin/lunar_ref_driver.rs`: sample differential-testing driver that emits
  stable `key=value` snapshots.
- `scripts/run_differential_self_check.sh`: one-command local protocol smoke
  check using the sample driver itself.

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT License
