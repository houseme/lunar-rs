# Task 18：Phase 4.14 第十个历法对象 `Julian` 接入

> 任务编号：`Task 18`
> 优先级：`P4.14`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在已经完成多种 era/offset 型多历法对象之后，继续接入一个真正规则不同的太阳历对象：

- `Julian`（儒略历）

本轮重点验证：

- 不同于 Gregorian 的历法规则是否能复用当前 `day/month/year` 模板；
- `CalendarDay / CalendarSpan` 与事件体系是否能自然承接不同历法规则的太阳历对象；
- 多历法扩展是否已经从“纪年变体”推进到“规则变体”。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Julian`
   - `JulianMonth`
   - `JulianYear`
2. 接入 `Solar`：
   - `Solar::julian_calendar()`
   - `Solar::julian_calendar_month()`
   - `Solar::julian_calendar_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - 历法规则转换
4. 补测试与文档状态

### 2.2 不包含

- `Julian` 的 i18n 输出
- 历史地区本地采用时间差异
- 专属节日数据库

---

## 3. 进度清单

- [x] 确认第十个历法对象选型
- [x] 落地 `Julian` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 18` 完成：

1. `Julian / JulianMonth / JulianYear` 全部落地；
2. `Solar` 已提供 `julian_calendar()` / `julian_calendar_month()` / `julian_calendar_year()`；
3. `Julian` 已接入 `CalendarDay`，`JulianMonth / JulianYear` 已接入 `CalendarSpan`；
4. Gregorian/Julian 转换、区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test julian --test dangi --test multi_calendar_traits`
  - `cargo test`
  - `cargo test --features "i18n serde"`
- 当前已完成：
  - 新增第十个历法对象：`Julian`
  - 新增 `JulianMonth` / `JulianYear`
  - 新增 `Solar::julian_calendar()` / `julian_calendar_month()` / `julian_calendar_year()`
  - 新增儒略历专测 `tests/julian.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Julian`
  - 顶层 crate 特性说明补入“儒略历”
