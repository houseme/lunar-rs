# Task 10：Phase 4.6 第六个历法对象 `ThaiSolar` 接入

> 任务编号：`Task 10`
> 优先级：`P4.6`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 08` 已完成第五个历法对象 `Minguo`，`Task 09` 又补齐其语言层之后，继续真正推进第六个历法对象，验证当前多历法模板已经能持续复用。

本轮选型：

- `ThaiSolar`（泰阳历 / 泰佛历公历纪年）

选择原因：

- 是基于公历的稳定偏移纪年，规则清晰；
- 与 `Foto` 的佛历语义不同：`Foto` 是农历佛历，`ThaiSolar` 是太阳历佛历纪年；
- 可以直接验证 `Solar` 系 companion 与 `CalendarDay / CalendarSpan` 的继续复用；
- 能证明当前模板不只适用于 `Minguo` 一种线性 solar-offset 历法。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `ThaiSolar`
   - `ThaiSolarMonth`
   - `ThaiSolarYear`
2. 接入 `Solar`：
   - `Solar::thai_solar()`
   - `Solar::thai_solar_month()`
   - `Solar::thai_solar_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - 边界 helper
4. 补测试与文档状态

### 2.2 不包含

- `ThaiSolar` 专属节日系统
- `ThaiSolar` 的 i18n 输出
- 更复杂的泰国历法改革细节建模

---

## 3. 进度清单

- [x] 确认第六个历法对象选型
- [x] 落地 `ThaiSolar` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 10` 完成：

1. `ThaiSolar / ThaiSolarMonth / ThaiSolarYear` 全部落地；
2. `Solar` 已提供 `thai_solar()` / `thai_solar_month()` / `thai_solar_year()`；
3. `ThaiSolar` 已接入 `CalendarDay`，`ThaiSolarMonth / ThaiSolarYear` 已接入 `CalendarSpan`；
4. 事件代理、区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test thai_solar --test multi_calendar_traits`
  - `cargo test --test minguo --test multi_calendar_ranges --test hijri --test rab_byung --test foto --test tao`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第六个历法对象：`ThaiSolar`
  - 新增 `ThaiSolarMonth` / `ThaiSolarYear`
  - 新增 `Solar::thai_solar()` / `thai_solar_month()` / `thai_solar_year()`
  - 新增泰阳历专测 `tests/thai_solar.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `ThaiSolar`
  - 顶层 crate 特性说明补入“泰阳历”
