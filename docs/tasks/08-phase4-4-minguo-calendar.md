# Task 08：Phase 4.4 第五个历法对象 `Minguo` 接入

> 任务编号：`Task 08`
> 优先级：`P4.4`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 07` 已完成多历法公开 trait 之后，选择第五个新历法对象进行模板化落地，验证当前架构已经能够以较低成本继续扩展。

本轮选型：

- `Minguo`（民国历）

选择原因：

- 规则稳定清晰，适合作为第一个基于 `Solar` 的轻量新历法样板；
- 不依赖复杂天文或节日数据库；
- 可以直接复用 `CalendarDay / CalendarSpan` 与事件体系；
- 能验证“新增历法对象”是否已经从重工程变成标准化接入流程。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Minguo`
   - `MinguoMonth`
   - `MinguoYear`
2. 接入 `Solar` 入口：
   - `Solar::minguo()`
   - `Solar::minguo_month()`
   - `Solar::minguo_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - solar 边界 helper
4. 补回归测试与文档状态

### 2.2 不包含

- `Minguo` 专属节日系统
- 新的 `EventKind` / `CalendarKind`
- 更复杂的时代/纪年体系

---

## 3. 进度清单

- [x] 确认第五个历法对象选型
- [x] 落地 `Minguo` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 08` 完成：

1. `Minguo / MinguoMonth / MinguoYear` 全部落地；
2. `Solar` 已提供 `minguo()` / `minguo_month()` / `minguo_year()`；
3. `Minguo` 已接入 `CalendarDay`，`MinguoMonth / MinguoYear` 已接入 `CalendarSpan`；
4. 事件代理、区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test minguo --test multi_calendar_traits`
  - `cargo test --test multi_calendar_ranges --test hijri --test rab_byung --test foto --test tao`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第五个历法对象：`Minguo`
  - 新增 `MinguoMonth` / `MinguoYear`
  - 新增 `Solar::minguo()` / `minguo_month()` / `minguo_year()`
  - 新增民国历专测 `tests/minguo.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Minguo`
  - 顶层 crate 特性说明补入“民国历”
