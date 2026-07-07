# Task 94: SolarTerm 对象入口兼容

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs v1.5` 的 `SolarTerm` 不是简单类型名，而是独立节气对象，支持：

- `SolarTerm::from_index(year, index)`
- `SolarTerm::from_name(year, name)`
- `next`
- `is_jie`
- `is_qi`
- `get_year`
- `get_index`
- `get_julian_day`
- `get_solar_day`
- `get_cursory_julian_day`

本地此前将 `SolarTerm` 作为 `JieQi` 的兼容别名导出，已有“名称 + 精确阳历时刻”能力，但缺少按年和序号构造、节/气判断、步进等对象入口。

本任务保留 `SolarTerm = JieQi` 的兼容别名，不新增平行类型；通过增强 `JieQi` 让 `SolarTerm::from_index(...)` 等调用自然可用，避免破坏既有 `current_jie_qi()` / `next_jie_qi()` 返回值。

## 子任务拆分

1. 将内部 `JieQi` 私有构造器改名为 `from_solar`，避免和公开 `new(year, name)` 冲突。
2. 新增 `JieQi::new(year, name)` 与 `JieQi::from_name(year, name)`。
3. 新增 `JieQi::from_index(year, index)`，按 tyme4rs 的冬至起算 24 节气序列构造。
4. 新增 `year/get_year`、`index/get_index`、`is_jie`、`is_qi`。
5. 新增 `next(offset)`，支持跨年节气步进。
6. 新增 `julian_day/get_julian_day`、`solar_day/get_solar_day`、`solar_time`。
7. 新增 `cursory_julian_day/get_cursory_julian_day`，复用现有寿星天文历节气近似计算。
8. 补充 `Display` / `PartialEq` / `Eq`，与 tyme4rs 的按名称展示和比较习惯对齐。
9. 补充测试覆盖 2023 冬至、大雪、2024 立春节气和非法名称。

## 验证计划

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test --test jieqi`
- `cargo test`

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test --test jieqi`
- `cargo test`

结果：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

`SolarTerm` 已从“仅类型名兼容”提升为“对象入口兼容”。后续继续对齐时，应优先处理严格 `get_*` 方法别名、README/API 迁移示例和外部差分测试矩阵。
