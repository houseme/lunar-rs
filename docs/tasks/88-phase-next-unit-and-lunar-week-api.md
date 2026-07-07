# Task 88: 年月周单位对象 API 差异补齐

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs v1.5` 在年月周对象形态上主要有两类能力：

1. 公历聚合对象：`SolarYear`、`SolarHalfYear`、`SolarSeason`、`SolarMonth`、`SolarWeek`。
2. 通用单位对象与农历周：`YearUnit`、`MonthUnit`、`DayUnit`、`SecondUnit`、`WeekUnit`、`LunarWeek`。

本地已经具备第一类公历聚合对象，本任务补齐第二类缺口，并保持本地风格：

- 非法周序、周起始或时间字段返回 `None`；
- 闰月比较索引沿用 `tyme4rs` 的正月偶数、闰月奇数规则；
- `LunarWeek` 使用 0-based 周序并提供中文周名。

## 子任务拆分

1. 新增 `unit.rs`，实现 `YearUnit`、`MonthUnit`、`DayUnit`、`SecondUnit`、`WeekUnit`。
2. 新增 `lunar_week.rs`，实现 `LunarWeek`。
3. 为单位对象提供字段访问与 `compare_index` / `seconds_in_day`。
4. 为 `LunarWeek` 提供 `first_day`、`days`、`week_count`、`next`。
5. 导出所有新增对象到 crate 根。
6. 补充 typed 测试覆盖单位比较、非法时间、农历周首日与步进。

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

完成后进入 `Task 89`：事件/节日 v1.5 API 兼容层评估。
