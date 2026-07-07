# Task 89: 事件/节日 v1.5 API 兼容层评估

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs v1.5` 移除了 `FestivalType`，保留 `EventType`。该 `EventType` 不是事件输出分类，而是事件规则编码类型：

- `SolarDay`：公历日期。
- `SolarWeek`：几月第几个星期几。
- `LunarDay`：农历日期。
- `TermDay`：节气日期。
- `TermHs`：节气后第几个天干日。
- `TermEb`：节气后第几个地支日。

本地已有 `EventKind` / `EventSource` / `EventSourceFamily` 用于输出事件分类，也已有 `EventRule` 用于规则解析。因此本任务不改事件主模型，只新增轻量兼容层：

- `EventType`
- `EventType::from_code`
- `EventType::from_name`
- `EventType::code`
- `EventType::name`
- `EventRule::event_type`

## 子任务拆分

1. 新增 `EventType` 枚举并对齐 tyme4rs code：`0..=5`。
2. 支持中文名称反查。
3. 支持 `Display` 输出中文名称。
4. 为 `EventRule` 增加 `event_type()`。
5. 导出 `EventType` 到 crate 根。
6. 补充事件测试，覆盖 code/name 与规则映射。

## 验证计划

- `cargo fmt`
- `cargo test --test phase3_events`
- `cargo test`

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase3_events`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

至此本轮基于 `tyme4rs v1.5` 的剩余明确缺口已收口到文档和 API 层面。后续如果继续推进，应进入更细粒度的差分测试矩阵或完整命名兼容策略，而不是继续追加大范围功能。
