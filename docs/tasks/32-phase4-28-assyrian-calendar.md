# Task 32：Phase 4.28 第十七个历法对象 `Assyrian` 接入

> 任务编号：`Task 32`
> 优先级：`P4.28`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在已经接入多种 offset / era / rule-variant 历法对象之后，继续接入第十七个历法对象：

- `Assyrian`（亚述纪年）

本轮重点：

- 继续验证当前 `day/month/year` 模板对“年界不在 1 月 1 日、但仍位于月首”的太阳历纪年是否稳定；
- 保持 `CalendarDay / CalendarSpan`、事件代理和 companion 形态一致；
- 以低风险方式继续扩大多历法覆盖面。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Assyrian`
   - `AssyrianMonth`
   - `AssyrianYear`
2. 接入 `Solar`：
   - `Solar::assyrian()`
   - `Solar::assyrian_month()`
   - `Solar::assyrian_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - `4 月 1 日` 年界校验
4. 补测试与文档状态

### 2.2 不包含

- `Assyrian` 的 i18n 输出
- `Assyrian` 专属节日数据库
- 更复杂的历史地区变体

---

## 3. 进度清单

- [x] 确认第十七个历法对象选型
- [x] 落地 `Assyrian` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 32` 完成：

1. `Assyrian / AssyrianMonth / AssyrianYear` 全部落地；
2. `Solar` 已提供 `assyrian()` / `assyrian_month()` / `assyrian_year()`；
3. `Assyrian` 已接入 `CalendarDay`，`AssyrianMonth / AssyrianYear` 已接入 `CalendarSpan`；
4. `4 月 1 日` 年界切换、区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test assyrian --test multi_calendar_traits`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第十七个历法对象：`Assyrian`
  - 新增 `AssyrianMonth` / `AssyrianYear`
  - 新增 `Solar::assyrian()` / `assyrian_month()` / `assyrian_year()`
  - 新增亚述纪年专测 `tests/assyrian.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Assyrian`
  - 顶层 crate 与 README 特性说明补入“亚述纪年”
