# lunar-rs

`lunar-rs` 是一个 Rust 日历库，覆盖公历/阳历、中国农历、二十四节气、干支、
黄历择日、佛历、道历、八字与运程等能力。

本项目参考 [lunar-go](https://github.com/6tail/lunar-go) 和
[lunar-javascript](https://github.com/6tail/lunar-javascript)，并移植寿星天文历
相关天文算法与数据表。核心 crate 设计目标是零第三方运行时依赖。

> 当前状态：0.1.0 移植阶段。当前本地黄金测试与 doctest 已全部通过。

## 功能特性

- 公历/阳历日期创建、格式化、日期推进、儒略日、星期、星座、节假日、工作日推进、
  薪资倍率。
- 阳历转农历、农历转阳历，支持闰月、1582 年历法改革缺失日期、跨年边界。
- 农历年/月/日/时字段、中文日期、干支、生肖、纳音、旬、旬空、冲、煞、太岁、
  方位、彭祖百忌、宜忌、吉神、凶煞、农历节日与传统黄历字段。
- 基于天文算法的二十四节气、节/气查询与精确时刻。
- 农历年、农历月元数据与月份生成。
- 九星、数九、三伏、阳历年/月/周/季度/半年等辅助模型。
- 八字四柱、大运、流年、流月、小运等运程基础能力。
- 可通过 `Lunar::foto()` 与 `Lunar::tao()` 获取佛历、道历模型。
- 提供 `holiday_util::set_holidays(...)`、`set_holiday_data(...)`、
  `reset_holidays()` 等法定节假日运行时覆盖接口。
- 提供可选 `serde` feature，为核心拥有所有权的数据类型提供序列化支持。
- 默认构建下 `[dependencies]` 仍保持零第三方运行时依赖；可选 feature
  可按需开启额外集成能力。

## 安装

发布到 crates.io 后可使用：

```toml
[dependencies]
lunar-rs = "0.1"
```

开发阶段可直接引用 Git 仓库：

```toml
[dependencies]
lunar-rs = { git = "https://github.com/houseme/lunar-rs" }
```

Rust 代码中的 crate 名称为 `lunar_rs`：

```rust
use lunar_rs::{Lunar, Solar};
```

## 快速开始

阳历转农历：

```rust
use lunar_rs::Solar;

let solar = Solar::from_ymd(2020, 5, 24).unwrap();
let lunar = solar.lunar();

assert_eq!(lunar.to_string(), "二〇二〇年闰四月初二");
assert_eq!(lunar.year_in_gan_zhi(), "庚子");
assert_eq!(lunar.year_sheng_xiao(), "鼠");
```

农历转阳历：

```rust
use lunar_rs::Lunar;

let lunar = Lunar::from_ymd(2019, 3, 27).unwrap();
let solar = lunar.solar();

assert_eq!(solar.to_ymd(), "2019-05-01");
```

节气查询：

```rust
use lunar_rs::Lunar;

let lunar = Lunar::from_ymd(2012, 9, 1).unwrap();
let bai_lu = lunar.jie_qi_table().get("白露").unwrap();

assert_eq!(bai_lu.to_ymd_hms(), "2012-09-07 13:29:01");
```

日期推进：

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

八字与运程：

```rust
use lunar_rs::{Gender, Solar};

let lunar = Solar::from_ymd(2021, 12, 21).unwrap().lunar();
let eight_char = lunar.eight_char();
let yun = eight_char.yun(1 as Gender);

println!("八字：{} {} {} {}", eight_char.year(), eight_char.month(), eight_char.day(), eight_char.time());
println!("起运：{} 年 {} 个月", yun.start_year(), yun.start_month());
```

佛历与道历：

```rust
use lunar_rs::Solar;

let lunar = Solar::from_ymd(2024, 5, 15).unwrap().lunar();
let foto = lunar.foto();
let tao = lunar.tao();

println!("佛历：{}", foto.to_string_cn());
println!("道历：{}", tao.to_string_cn());
```

## 验证

```bash
cargo check
cargo test
cargo bench --bench convert
```

当前本地验证结果：

- `cargo check` 通过。
- `cargo test` 当前黄金用例全部通过。
- doctest 通过。

## 目录结构

- `src/solar*.rs`：阳历日期与年/月/周/季度/半年模型。
- `src/lunar*.rs`：农历核心、农历年/月/时与基础表。
- `src/shou_xing/`：寿星天文历算法和抽取常量。
- `src/eight_char.rs`、`src/yun/`：八字与运程。
- `src/foto*.rs`、`src/tao*.rs`：佛历与道历。
- `src/holiday*.rs`：中国法定节假日数据与查询。
- `tests/lunar_core.rs`：从参考实现迁移的黄金用例。

## 许可证

本项目采用双许可证：

- Apache License, Version 2.0
- MIT License
