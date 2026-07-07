# Task 28：Phase 4.24 第十五个历法对象 `Armenian` 接入

> 任务编号：`Task 28`
> 优先级：`P4.24`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在已经完成 `Coptic / Ethiopian` 这类固定月长与尾月补日模型之后，继续接入第十五个历法对象：

- `Armenian`（亚美尼亚历）

本轮重点：

- 继续验证“`12 x 30 + 5` 尾月补日”模板能否稳定复用；
- 接入 `CalendarDay / CalendarSpan` 与事件体系；
- 延续多历法对象扩展的低风险节奏。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Armenian`
   - `ArmenianMonth`
   - `ArmenianYear`
2. 接入 `Solar`：
   - `Solar::armenian()`
   - `Solar::armenian_month()`
   - `Solar::armenian_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - 固定尾月边界校验
4. 补测试与文档状态

### 2.2 不包含

- `Armenian` 的 i18n 输出
- 亚美尼亚专属节期数据库
- 更复杂的历法改革分支

---

## 3. 进度清单

- [x] 确认第十五个历法对象选型
- [x] 落地 `Armenian` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 28` 完成：

1. `Armenian / ArmenianMonth / ArmenianYear` 全部落地；
2. `Solar` 已提供 `armenian()` / `armenian_month()` / `armenian_year()`；
3. `Armenian` 已接入 `CalendarDay`，`ArmenianMonth / ArmenianYear` 已接入 `CalendarSpan`；
4. 固定尾月、区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test armenian --test multi_calendar_traits`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第十五个历法对象：`Armenian`
  - 新增 `ArmenianMonth` / `ArmenianYear`
  - 新增 `Solar::armenian()` / `armenian_month()` / `armenian_year()`
  - 新增亚美尼亚历专测 `tests/armenian.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Armenian`
  - 顶层 crate 与 README 特性说明补入“亚美尼亚历”
