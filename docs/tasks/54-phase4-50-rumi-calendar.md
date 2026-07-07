# Task 54：Phase 4.50 第二十八个历法对象 `Rumi` 接入

> 任务编号：`Task 54`
> 优先级：`P4.50`
> 当前状态：`已完成`
> 最近更新：`2026-07-07`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 53` 已完成 `Venetian` 的 i18n 与描述层收口之后，继续推进第二十八个历法对象：

- `Rumi`（鲁米历 / Ottoman civil Rumi calendar）

本轮重点：

- 新增 `RumiYear / RumiMonth / Rumi`；
- 将 `Rumi` 接入 `Solar` 入口与统一导出；
- 复用 `CalendarDay / CalendarSpan / EventQuery`；
- 验证它能承接“儒略制月份 + 1917 对齐 + 1918 起改为 1 月起年”的分阶段规则。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增 `src/rumi.rs`
2. 新增：
   - `RumiYear`
   - `RumiMonth`
   - `Rumi`
3. `Solar` 新增：
   - `rumi()`
   - `rumi_year()`
   - `rumi_month()`
4. 新增 `tests/rumi.rs`
5. 将 `Rumi` 接入 `tests/multi_calendar_traits.rs`

### 2.2 不包含

- `Rumi` 的 i18n 与描述层英文模板
- `Rumi` 专属节期数据库
- 1926 之后与现代土耳其公历显示层融合策略

---

## 3. 进度清单

- [x] 确认第二十八个历法对象选型
- [x] 完成 `Rumi` year/month/day 接入
- [x] 接入统一 trait 与 `Solar` 入口
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 54` 完成：

1. `Solar` 已能解析 `Rumi` 及其 companion；
2. `RumiYear / Month / Day` 已提供基础边界与推进能力；
3. `Rumi` 已复用统一事件与 trait 能力；
4. `tests/rumi.rs` 与相关 trait 测试通过。

---

## 5. 验证记录

### 2026-07-07

- 状态：`已完成`
- 已验证：
  - `cargo test --test rumi`
  - `cargo test --test multi_calendar_traits`
  - `cargo test`
- 当前已完成：
  - `src/rumi.rs`
  - `Solar::rumi() / rumi_year() / rumi_month()`
  - `tests/rumi.rs`
  - `tests/multi_calendar_traits.rs` 接入 `Rumi`
