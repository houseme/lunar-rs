# Task 97 - Lunar/LunarTime 严格方法名与节日 wrapper 兼容

状态：已完成
最近更新：2026-07-07
进度：5/5

## 对标结论

本轮继续对标 `6tail/tyme4rs` `master`：

- commit：`ba6ab75`
- 时间：`2026-06-15 21:22:53 +0800`
- 标题：`v1.5.0 移除节日类型FestivalType；新增回历；优化代码和算法。`

Task 96 已补齐 `Solar` 与公历聚合对象的常用 `get_*` 入口。继续细化后，剩余高频迁移缺口集中在两类：

1. `LunarDay` / `LunarHour` 的严格方法名：上游大量使用 `get_year_sixty_cycle()`、`get_phase_day()`、`get_recommends()`、`get_nine_star()` 等命名，本地已有同等语义对象但主要使用 Rust 风格短方法名。
2. 节日 wrapper：上游提供 `SolarFestival` / `LunarFestival` 对象入口；本地此前以字符串节日列表与统一 `Event` 模型为主，缺少同名 wrapper 对象。

## 实施范围

- [x] 新增 `SolarFestival` wrapper：
  - `from_ymd`
  - `from_index`
  - `get_index`
  - `get_day`
  - `get_name`
  - `get_start_year`
  - `next`
  - `to_event`
- [x] 新增 `LunarFestival` wrapper：
  - `from_ymd`
  - `from_index`
  - `get_index`
  - `get_day`
  - `get_name`
  - `get_solar_term`
  - `next`
  - `to_event`
- [x] 为 `Solar` / `Lunar` 增加 `get_festival()` 主节日入口。
- [x] 为 `Lunar` 增加严格方法名兼容入口，覆盖基础字段、农历月、星期、干支、胎神、天神、星宿、九星、六曜、月相、宜忌、神煞、小六壬、三柱、八字与 `get_hours()` 等高频迁移路径。
- [x] 为 `LunarTime` 增加严格方法名兼容入口，覆盖 owned 时辰构造、所属农历日、公历时间、干支年/月/日/时、九星、天神、宜忌、小六壬、八字与前后比较。

## 验证记录

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`

## 风险与边界

- 本轮不改变既有 `festivals()` / `other_festivals()` 字符串输出，也不替换现有统一 `Event` 模型。
- `SolarFestival` 对齐 tyme4rs 的 10 个现代公历节日 wrapper；周序节日继续通过 `Solar::festivals()` 和事件模型暴露。
- `LunarFestival` 覆盖本地已有主节日、除夕，以及清明/冬至节气节日入口；更完整的 tyme4rs Event 编码差分可作为后续任务继续扩大。
- `LunarTime` 现保留 lifetime 形状但内部持有 `Lunar` 快照，因此可支持 `Lunar::get_hours()` 返回 13 个时辰点。

## 后续子任务建议

1. 建立 `tyme4rs` 示例驱动的差分测试矩阵，校准节日 wrapper 的起始年份与编码规则。
2. 为 `LunarYear` / `LunarMonth` 继续补更细 `get_*` 入口和严格类型别名。
3. 补 README/API 文档，整理 tyme4rs 迁移示例。
