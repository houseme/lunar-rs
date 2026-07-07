# Task 48：Phase 4.44 第二十五个历法对象 `Seleucid` 接入

> 任务编号：`Task 48`
> 优先级：`P4.44`
> 当前状态：`已完成`
> 最近更新：`2026-07-07`
> 当前进度：`4/4`

---

## 1. 任务目标

在已经接入多种 offset / era / rule-variant 历法对象之后，继续接入第二十五个历法对象：

- `Seleucid`（塞琉古纪元 / Anno Graecorum）

本轮重点：

- 继续验证当前 `day/month/year` 模板对“秋季月首切年”的太阳历 era 是否稳定；
- 保持 `CalendarDay / CalendarSpan`、事件代理和 companion 形态一致；
- 以低风险方式继续扩大多历法覆盖面。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Seleucid`
   - `SeleucidMonth`
   - `SeleucidYear`
2. 接入 `Solar`：
   - `Solar::seleucid()`
   - `Solar::seleucid_month()`
   - `Solar::seleucid_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - `10 月 1 日` 年界校验
4. 补测试与文档状态

### 2.2 不包含

- `Seleucid` 的 i18n 输出
- `Seleucid` 专属节日数据库
- 更复杂的古代历法版本差异

---

## 3. 进度清单

- [x] 确认第二十五个历法对象选型
- [x] 落地 `Seleucid` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 48` 完成：

1. `Seleucid / SeleucidMonth / SeleucidYear` 全部落地；
2. `Solar` 已提供 `seleucid()` / `seleucid_month()` / `seleucid_year()`；
3. `Seleucid` 已接入 `CalendarDay`，`SeleucidMonth / SeleucidYear` 已接入 `CalendarSpan`；
4. `10 月 1 日` 年界切换、区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-07

- 状态：`已完成`
- 已验证：
  - `cargo test --test seleucid --test multi_calendar_traits`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第二十五个历法对象：`Seleucid`
  - 新增 `SeleucidMonth` / `SeleucidYear`
  - 新增 `Solar::seleucid()` / `seleucid_month()` / `seleucid_year()`
  - 新增塞琉古纪元专测 `tests/seleucid.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Seleucid`
  - 顶层 crate 与 README 特性说明补入“塞琉古纪元”
