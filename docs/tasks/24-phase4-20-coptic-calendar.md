# Task 24：Phase 4.20 第十三个历法对象 `Coptic` 接入

> 任务编号：`Task 24`
> 优先级：`P4.20`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在已经完成多种 offset / era / rule-variant 历法对象之后，继续接入第十三个历法对象：

- `Coptic`（科普特历）

本轮重点：

- 引入与前面不同的 `13` 月结构；
- 验证 `CalendarDay / CalendarSpan` 与事件体系对不同月制模型的兼容性；
- 继续低风险扩展多历法覆盖面。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Coptic`
   - `CopticMonth`
   - `CopticYear`
2. 接入 `Solar`：
   - `Solar::coptic()`
   - `Solar::coptic_month()`
   - `Solar::coptic_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - `13` 月边界校验
4. 补测试与文档状态

### 2.2 不包含

- `Coptic` 的 i18n 输出
- 宗教节期数据库
- 更复杂的礼仪历细节

---

## 3. 进度清单

- [x] 确认第十三个历法对象选型
- [x] 落地 `Coptic` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 24` 完成：

1. `Coptic / CopticMonth / CopticYear` 全部落地；
2. `Solar` 已提供 `coptic()` / `coptic_month()` / `coptic_year()`；
3. `Coptic` 已接入 `CalendarDay`，`CopticMonth / CopticYear` 已接入 `CalendarSpan`；
4. `13` 月结构、闰尾月、区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test coptic --test multi_calendar_traits`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第十三个历法对象：`Coptic`
  - 新增 `CopticMonth` / `CopticYear`
  - 新增 `Solar::coptic()` / `coptic_month()` / `coptic_year()`
  - 新增科普特历专测 `tests/coptic.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Coptic`
  - 顶层 crate 特性说明补入“科普特历”
