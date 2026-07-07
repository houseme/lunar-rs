# Task 91: 核心日时与儒略日命名兼容

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs v1.5` 的 README 和核心 API 大量使用以下入口：

- `SolarDay`
- `SolarTime`
- `LunarDay`
- `LunarHour`
- `SolarTerm`
- `JulianDay`
- `LegalHoliday`

本地原有模型以 `Solar` / `Lunar` 为核心，单个类型同时覆盖日期和时间，因此不是功能缺失，而是迁移入口命名不一致。本任务补齐兼容层：

- `SolarDay = Solar`
- `SolarTime = Solar`
- `LunarDay = Lunar`
- `LunarHour<'a> = LunarTime<'a>`
- `SolarTerm = JieQi`
- `LegalHoliday = Holiday`
- 新增独立 `JulianDay`

## 子任务拆分

1. 新增 `JulianDay` 对象，支持 `from_julian_day`、`from_ymd_hms`、`day`、`week`、`solar_time`、`solar_day`、`next`。
2. 在 crate 根导出 `JulianDay`。
3. 在 crate 根新增核心兼容别名。
4. 补充 typed 测试覆盖 README 风格入口。

## 验证计划

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

本任务完成后，tyme4rs v1.5 的核心日期/时间命名入口已具备迁移层。后续剩余差异更偏严格方法名、builder API 和文档示例，而不是底层日历能力。
