# Task 40：Phase 4.36 第二十一个历法对象 `Koki` 接入

> 任务编号：`Task 40`
> 优先级：`P4.36`
> 当前状态：`已完成`
> 最近更新：`2026-07-07`
> 当前进度：`4/4`

---

## 1. 任务目标

在已经接入多种 offset / era / rule-variant 历法对象之后，继续接入第二十一个历法对象：

- `Koki`（皇纪 / Japanese imperial year）

本轮重点：

- 继续验证当前 `day/month/year` 模板对线性 era 纪年的稳定复用；
- 保持 `CalendarDay / CalendarSpan`、事件代理和 companion 形态一致；
- 以低风险方式继续扩大多历法覆盖面。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Koki`
   - `KokiMonth`
   - `KokiYear`
2. 接入 `Solar`：
   - `Solar::koki()`
   - `Solar::koki_month()`
   - `Solar::koki_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - 边界 helper
4. 补测试与文档状态

### 2.2 不包含

- `Koki` 的 i18n 输出
- `Koki` 专属节日数据库
- 更复杂的历史变体

---

## 3. 进度清单

- [x] 确认第二十一个历法对象选型
- [x] 落地 `Koki` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 40` 完成：

1. `Koki / KokiMonth / KokiYear` 全部落地；
2. `Solar` 已提供 `koki()` / `koki_month()` / `koki_year()`；
3. `Koki` 已接入 `CalendarDay`，`KokiMonth / KokiYear` 已接入 `CalendarSpan`；
4. 区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-07

- 状态：`已完成`
- 已验证：
  - `cargo test --test koki --test multi_calendar_traits`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第二十一个历法对象：`Koki`
  - 新增 `KokiMonth` / `KokiYear`
  - 新增 `Solar::koki()` / `koki_month()` / `koki_year()`
  - 新增皇纪专测 `tests/koki.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Koki`
  - 顶层 crate 与 README 特性说明补入“皇纪”
