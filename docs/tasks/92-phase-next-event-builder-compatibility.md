# Task 92: EventBuilder 规则构造器兼容

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs v1.5` 的 `EventBuilder` 是事件规则压缩数据的链式构造入口，支持：

- `Event::builder()`
- `name`
- `solar_day`
- `lunar_day`
- `solar_week`
- `term_day`
- `term_heaven_stem`
- `term_earth_branch`
- `start_year`
- `offset`
- `build`

本地事件模型不使用上游的 9 字符压缩串，而是通过 `EventRule` 和 `EventManager` 保存强类型规则。因此本任务不复制压缩编码，改为提供语义等价的 builder：

- `EventBuilder::new`
- `Event::builder`
- 上游同名规则入口
- `term_*` 的节气序号入口
- `term_*_name` 的本地节气名入口
- `to_event(year)` / `build(year)`
- `update_manager`

## 子任务拆分

1. 新增 `EventBuilder` 结构体，内部持有事件名与可选 `EventRule`。
2. 增加上游同名的公历、农历、星期、节气规则 builder 方法。
3. 支持 `start_year` 与 `offset` 在规则前后任意顺序调用。
4. 提供 `rule` / `into_rule`，便于外部取出底层强类型规则。
5. 提供 `to_event(year)` / `build(year)`，按年份解析成具体 `Event`。
6. 提供 `update_manager`，把 builder 构造出的规则注册到 `EventManager`。
7. 通过 crate 根导出 `EventBuilder`。
8. 补充测试覆盖 builder 入口和 manager 注册链路。

## 验证计划

- `cargo fmt`
- `cargo test --test phase3_events`
- `cargo test`

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase3_events`
- `cargo test`

结果：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

事件规则侧已具备 `EventType`、`EventRule`、`EventBuilder`、`EventManager` 四层入口。后续若继续对齐，应优先处理严格方法名兼容、README/API 文档示例，以及外部差分测试矩阵，而不是继续扩展事件主模型。
