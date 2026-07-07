# Task 44：Phase 4.40 第二十三个历法对象 `Fasli` 接入

> 任务编号：`Task 44`
> 优先级：`P4.40`
> 当前状态：`已完成`
> 最近更新：`2026-07-07`
> 当前进度：`4/4`

---

## 1. 任务目标

在已经接入多种 offset / era / rule-variant 历法对象之后，继续接入第二十三个历法对象：

- `Fasli`（法斯里历 / Zoroastrian Fasli calendar）

本轮重点：

- 验证当前 `day/month/year` 模板是否能承接“12 x 30 + 年末 5/6 补日”的太阳历对象；
- 保持 `CalendarDay / CalendarSpan`、事件代理和 companion 形态一致；
- 以低风险方式继续扩大多历法覆盖面。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Fasli`
   - `FasliMonth`
   - `FasliYear`
2. 接入 `Solar`：
   - `Solar::fasli()`
   - `Solar::fasli_month()`
   - `Solar::fasli_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - 年末补日结构校验
4. 补测试与文档状态

### 2.2 不包含

- `Fasli` 的 i18n 输出
- `Fasli` 专属节日数据库
- 更复杂的宗教历变体

---

## 3. 进度清单

- [x] 确认第二十三个历法对象选型
- [x] 落地 `Fasli` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 44` 完成：

1. `Fasli / FasliMonth / FasliYear` 全部落地；
2. `Solar` 已提供 `fasli()` / `fasli_month()` / `fasli_year()`；
3. `Fasli` 已接入 `CalendarDay`，`FasliMonth / FasliYear` 已接入 `CalendarSpan`；
4. 年末补日结构、区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-07

- 状态：`已完成`
- 已验证：
  - `cargo test --test fasli --test multi_calendar_traits`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第二十三个历法对象：`Fasli`
  - 新增 `FasliMonth` / `FasliYear`
  - 新增 `Solar::fasli()` / `fasli_month()` / `fasli_year()`
  - 新增法斯里历专测 `tests/fasli.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Fasli`
  - 顶层 crate 与 README 特性说明补入“法斯里历”
