# Task 95: SolarDay/SolarTime 核心 getter 兼容

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs v1.5` 区分 `SolarDay` 与 `SolarTime`，并广泛使用 `get_*` 风格方法：

- `get_year` / `get_month` / `get_day`
- `get_hour` / `get_minute` / `get_second`
- `get_week`
- `get_julian_day`
- `get_solar_day` / `get_solar_time`
- `get_lunar_day`
- `get_term`
- `get_term_day`
- `is_before` / `is_after`

本地 `SolarDay` 与 `SolarTime` 已作为 `Solar` 的兼容别名导出，底层能力已存在，但调用方从 `tyme4rs` 迁移时仍需要把 `get_*` 改成本地方法名。

本任务不新增平行 `SolarDay` / `SolarTime` 类型，而是在 `Solar` 上补齐核心 getter 兼容层。由于本地 `Solar` 同时携带日期和时间，`get_term()` 按具体时间点返回当前所属节气；这比纯日期对象更贴近本地模型，也能处理节气当天但尚未到节气时刻的边界。

## 子任务拆分

1. 为 `Solar` 增加 `get_year/get_month/get_day/get_hour/get_minute/get_second`。
2. 为 `Solar` 增加 `get_week`，返回 typed `Week`。
3. 为 `Solar` 增加 `get_julian_day`，返回 `JulianDay`。
4. 为 `Solar` 增加 `get_solar_day/get_solar_time`。
5. 为 `Solar` 增加 `get_lunar_day`。
6. 为 `Solar` 增加 `get_term` 与 `get_term_day`。
7. 为 `Solar` 增加 `is_before/is_after`。
8. 为 `JulianDay` 增加 `get_solar_day/get_solar_time`。
9. 补充测试覆盖 core getter、节气、儒略日、比较与农历转换入口。

## 验证计划

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`

结果：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

`SolarDay/SolarTime` 的核心迁移入口已覆盖。后续可继续补更细的 `get_*` 别名、节日对象形态、README/API 迁移示例和外部差分测试矩阵。
