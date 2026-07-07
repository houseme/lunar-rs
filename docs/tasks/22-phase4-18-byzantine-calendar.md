# Task 22：Phase 4.18 第十二个历法对象 `Byzantine` 接入

> 任务编号：`Task 22`
> 优先级：`P4.18`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在已经完成多种线性偏移纪年、带纪元起点限制的对象以及规则变体类对象之后，继续接入第十二个历法对象：

- `Byzantine`（拜占庭纪年）

本轮重点验证：

- 年界不再固定为 `1 月 1 日`，而是改为 `9 月 1 日`；
- `CalendarDay / CalendarSpan` 与事件体系是否能自然覆盖“跨年界但仍共享 solar 月日”的对象；
- 多历法模板是否已经足以承接“非自然年起点”的历法模型。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Byzantine`
   - `ByzantineMonth`
   - `ByzantineYear`
2. 接入 `Solar`：
   - `Solar::byzantine()`
   - `Solar::byzantine_month()`
   - `Solar::byzantine_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - `9 月 1 日` 年界校验
4. 补测试与文档状态

### 2.2 不包含

- `Byzantine` 的 i18n 输出
- 历史宗教节期数据库
- 更复杂的古代版本差异

---

## 3. 进度清单

- [x] 确认第十二个历法对象选型
- [x] 落地 `Byzantine` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 22` 完成：

1. `Byzantine / ByzantineMonth / ByzantineYear` 全部落地；
2. `Solar` 已提供 `byzantine()` / `byzantine_month()` / `byzantine_year()`；
3. `Byzantine` 已接入 `CalendarDay`，`ByzantineMonth / ByzantineYear` 已接入 `CalendarSpan`；
4. `9 月 1 日` 年界切换、区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test byzantine --test multi_calendar_traits`
  - `cargo test --test holocene --test julian --test dangi --test juche --test japanese --test thai_solar --test minguo --test multi_calendar_ranges --test hijri --test rab_byung --test foto --test tao`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第十二个历法对象：`Byzantine`
  - 新增 `ByzantineMonth` / `ByzantineYear`
  - 新增 `Solar::byzantine()` / `byzantine_month()` / `byzantine_year()`
  - 新增拜占庭纪年专测 `tests/byzantine.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Byzantine`
  - 顶层 crate 特性说明补入“拜占庭纪年”
