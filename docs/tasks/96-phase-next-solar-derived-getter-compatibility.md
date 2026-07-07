# Task 96 - Solar 派生 getter 与公历聚合对象兼容

状态：已完成
最近更新：2026-07-07
进度：5/5

## 对标结论

本轮继续以 `6tail/tyme4rs` `master` 为基准，已重新拉取确认基准仍为：

- commit：`ba6ab75`
- 时间：`2026-06-15 21:22:53 +0800`
- 标题：`v1.5.0 移除节日类型FestivalType；新增回历；优化代码和算法。`

继 Task 95 补齐 `SolarDay/SolarTime` 基础 getter 后，本地仍有一组常用迁移入口没有对齐：`Solar` 到月/周/星座/物候/回历/法定节假日/干支日/藏历日的 `get_*` 命名，以及 `SolarYear` / `SolarHalfYear` / `SolarSeason` / `SolarMonth` / `SolarWeek` 的聚合对象 getter。该缺口主要影响从 tyme4rs 迁移来的调用点，不改变现有主 API。

## 实施范围

- [x] 为 `Solar` 增加派生 getter：
  - `get_solar_month`
  - `get_solar_week`
  - `get_index_in_year`
  - `get_constellation`
  - `get_phenology`
  - `get_phenology_day`
  - `get_hijri_day`
  - `get_nine_day`
  - `get_hide_heaven_stem_day`
  - `get_dog_day`
  - `get_plum_rain_day`
  - `get_legal_holiday`
  - `get_sixty_cycle_day`
  - `get_rab_byung_day`
- [x] 为 `Holiday` / `LegalHoliday` 增加 `from_ymd`、`get_day`、`get_name`、`get_target`。
- [x] 为 `JulianDay` 增加 `get_week` 和 `subtract`。
- [x] 增加 `HijriDay = Hijri` 兼容别名。
- [x] 为 `SolarYear` / `SolarHalfYear` / `SolarSeason` / `SolarMonth` / `SolarWeek` 增加 `from_index/from_ym`、`get_year`、`get_month`、`get_day_count`、`is_leap`、`get_months`、`get_weeks`、`get_seasons`、`get_half_years`、`get_solar_year`、`get_solar_month` 等聚合 getter。

## 验证记录

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`

## 风险与边界

- 本轮保持“兼容命名入口”策略，不改变现有 `Solar`、`Lunar`、`Holiday` 的主语义。
- `Holiday::get_day()` / `get_target()` 复用现有节假日数据中的 `YYYYMMDD` / `YYYY-MM-DD` 字符串，解析失败时回落到 `0001-01-01`，避免兼容 getter 在数据异常时 panic。
- `SolarYear::get_day_count()` 保留 1582 年格里历缺口处理，对齐本地 `Solar::from_ymd` 的 Gregorian gap 语义。

## 后续子任务建议

1. 继续补 `Lunar` / `LunarTime` 更细的 `get_*` 迁移入口。
2. 梳理 `SolarFestival` / `LunarFestival` typed wrapper 与现有事件模型之间的兼容边界。
3. 建立一组 tyme4rs 示例驱动的差分测试，覆盖高频迁移 API。
