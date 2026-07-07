# Task 38：Phase 4.34 第二十个历法对象 `Bengali` 接入

> 任务编号：`Task 38`
> 优先级：`P4.34`
> 当前状态：`已完成`
> 最近更新：`2026-07-07`
> 当前进度：`4/4`

---

## 1. 任务目标

在已经接入多种 offset / era / rule-variant 历法对象之后，继续接入第二十个历法对象：

- `Bengali`（孟加拉历 / Bangladeshi revised Bengali calendar）

本轮重点：

- 验证当前 `day/month/year` 模板是否能承接“独立月制 + 非 1 月 1 日新年”的太阳历对象；
- 保持 `CalendarDay / CalendarSpan`、事件代理和 companion 形态一致；
- 以低风险方式继续扩大多历法覆盖面。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Bengali`
   - `BengaliMonth`
   - `BengaliYear`
2. 接入 `Solar`：
   - `Solar::bengali()`
   - `Solar::bengali_month()`
   - `Solar::bengali_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - `Pohela Boishakh` 年界与月长校验
4. 补测试与文档状态

### 2.2 不包含

- `Bengali` 的 i18n 输出
- `Bengali` 专属节日数据库
- 更复杂的地区性变体

---

## 3. 进度清单

- [x] 确认第二十个历法对象选型
- [x] 落地 `Bengali` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 38` 完成：

1. `Bengali / BengaliMonth / BengaliYear` 全部落地；
2. `Solar` 已提供 `bengali()` / `bengali_month()` / `bengali_year()`；
3. `Bengali` 已接入 `CalendarDay`，`BengaliMonth / BengaliYear` 已接入 `CalendarSpan`；
4. `Pohela Boishakh` 年界、月长结构、区间边界与往返换算均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-07

- 状态：`已完成`
- 已验证：
  - `cargo test --test bengali --test multi_calendar_traits`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第二十个历法对象：`Bengali`
  - 新增 `BengaliMonth` / `BengaliYear`
  - 新增 `Solar::bengali()` / `bengali_month()` / `bengali_year()`
  - 新增孟加拉历专测 `tests/bengali.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Bengali`
  - 顶层 crate 与 README 特性说明补入“孟加拉历”
