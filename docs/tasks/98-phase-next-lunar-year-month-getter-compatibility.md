# Task 98: LunarYear/LunarMonth 严格 getter 兼容

状态：已完成
最近更新：2026-07-07
上游基准：`6tail/tyme4rs` `ba6ab75`（2026-06-15，v1.5.0）

## 对标结论

`Task 97` 已补齐 `Lunar` / `LunarTime` 的高频严格方法名，但 `tyme4rs` 的 `LunarYear` / `LunarMonth` 仍有一组 `get_*` 入口缺失。差异主要是公开 API 形态，不是底层历法算法：

- `LunarYear`：上游提供 `get_day_count`、`get_month_count`、`get_months`、`get_leap_month`、`get_sixty_cycle`、`get_twenty`、`get_jupiter_direction`、`get_nine_star`、`get_kitchen_god_steed`。
- `LunarMonth`：上游提供 `get_lunar_year`、`get_month`、`get_month_with_leap`、`get_day_count`、`get_index_in_year`、`get_season`、`get_first_julian_day`、`get_week_count`、`get_days`、`get_first_day`、`get_weeks`、`get_sixty_cycle`、`get_jupiter_direction`、`get_nine_star`、`get_fetus`、`get_minor_ren`。
- 本地 `LunarYear::months()` 是内部天文计算窗口，包含跨年边界 15 个月；上游 `get_months()` 是对外当年月份集合，通常 12 个月，闰年 13 个月。因此本任务只新增 `get_months()` 对外兼容语义，不改变原有 `months()`。
- 本地 `LunarMonth::month()` 保留闰月符号；本任务新增 `get_month()` 返回绝对月序，`get_month_with_leap()` 返回带符号月序。

## 子任务

1. 为 `LunarYear` 增加严格 `get_*` 入口，并复用现有 typed 对象与计算结果。
2. 为 `LunarMonth` 增加严格 `get_*` 入口，覆盖月份、周、儒略日、六十甲子、岁星方位、九星、胎神和小六壬。
3. 对 2020 闰四月增加集成测试，覆盖 13 个月、闰月符号、周列表、胎神和方向类 getter。
4. 同步总对标文档与任务索引。

## 实施结果

- `LunarYear`
  - 新增：`get_year`、`get_day_count`、`get_month_count`、`get_months`、`get_leap_month`、`get_sixty_cycle`、`get_twenty`、`get_jupiter_direction`、`get_nine_star`、`get_kitchen_god_steed`。
  - `get_months()` 返回 `months_in_year()` 收集结果，避免泄漏内部 15 个月窗口。
- `LunarMonth`
  - 新增：`get_lunar_year`、`get_year`、`get_month`、`get_month_with_leap`、`get_day_count`、`get_index_in_year`、`get_season`、`get_first_julian_day`、`get_week_count`、`get_days`、`get_first_day`、`get_weeks`、`get_sixty_cycle`、`get_jupiter_direction`、`get_nine_star`、`get_fetus`、`get_minor_ren`。
  - 对本地可失败的构造入口保留 `Option` 风格，例如 `get_first_day()` 返回 `Option<Lunar>`，以延续本地 API 的非 panic 习惯。

## 验证

- `cargo fmt`
- `cargo test --test phase2_typed lunar_year_and_month_strict_getters_match_tyme_names -- --nocapture`

## 后续

剩余工作继续收敛为更细的兼容面：

- 建立 tyme4rs 差分样例矩阵，优先覆盖 `LunarYear` / `LunarMonth` / `SolarFestival` / `LunarFestival`。
- 继续核对更边缘对象的严格 `get_name/get_index/next` 形态。
- 补 README/API 迁移示例，说明本地 `Option` 返回与 tyme4rs panic 构造的差异。
