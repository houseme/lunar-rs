# Task 64: EventManager 规则模型

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 的事件能力不只做“已有事件聚合”，还提供 `EventBuilder` / `EventManager`：

- 可按规则定义事件；
- 可按年份计算该事件落在的阳历日期；
- 可注册、更新、删除事件规则；
- 规则可覆盖固定公历日、固定农历日、月内星期、节气偏移等场景。

本仓库此前已有统一事件模型、查询索引和范围扫描，但缺少规则注册与规则求值层，因此无法用数据/规则扩展新的节日事件。

## 实现内容

- 新增 `EventRule` typed enum：
  - `solar_day`
  - `lunar_day`
  - `solar_week`
  - `solar_term_offset`
  - `with_offset_days`
  - `with_start_year`
  - `resolve_solar`
  - `to_event`
- 新增 `EventManager`：
  - `update`
  - `remove`
  - `clear`
  - `rules`
  - `event_for_year`
  - `events_for_day`
- 将 `EventManager::events_for_day` 并入现有 `all_events_for_day` 聚合链路，使现有：
  - `Solar::all_events`
  - `Solar::find_events`
  - `scan_events_in_range`
  - `EventQuery`
  能自动消费规则事件。
- `EventManager` 更新规则后会清理事件索引缓存，避免查询结果停留在旧规则状态。

## 验收标准

- 固定公历日规则可解析为目标阳历日期；
- 固定农历日规则可解析到对应阳历日期；
- 月内第 N 个星期规则可解析到对应阳历日期；
- 节气偏移规则可基于现有节气表解析；
- 注册后的规则事件可通过 `Solar::find_events(&EventQuery::new().with_tag("event_manager"))` 查询。

## 验证记录

- `cargo fmt`
- `cargo test --test phase3_events`
