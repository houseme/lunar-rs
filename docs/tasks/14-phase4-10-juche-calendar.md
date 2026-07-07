# Task 14：Phase 4.10 第八个历法对象 `Juche` 接入

> 任务编号：`Task 14`
> 优先级：`P4.10`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在已经完成多个线性 solar-offset 历法与一个带 era 边界的 `Japanese` 之后，继续接入第八个历法对象：

- `Juche`（主体纪年）

本轮重点验证：

- 带明确纪元起点、且不接受起点前日期的 solar-based 历法，是否也能复用当前 `day/month/year` 模板；
- `CalendarDay / CalendarSpan` 与事件体系是否能自然覆盖这种“有限有效区间”的对象；
- 多历法扩展是否已经足够稳定，可以继续低成本新增对象。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Juche`
   - `JucheMonth`
   - `JucheYear`
2. 接入 `Solar`：
   - `Solar::juche()`
   - `Solar::juche_month()`
   - `Solar::juche_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - 起点边界校验
4. 补测试与文档状态

### 2.2 不包含

- `Juche` 的 i18n 输出
- `Juche` 专属节日数据库
- 更复杂的历史纪元制度

---

## 3. 进度清单

- [x] 确认第八个历法对象选型
- [x] 落地 `Juche` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 14` 完成：

1. `Juche / JucheMonth / JucheYear` 全部落地；
2. `Solar` 已提供 `juche()` / `juche_month()` / `juche_year()`；
3. `Juche` 已接入 `CalendarDay`，`JucheMonth / JucheYear` 已接入 `CalendarSpan`；
4. 纪元起点边界、区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test juche --test multi_calendar_traits`
  - `cargo test --test japanese --test thai_solar --test minguo --test multi_calendar_ranges --test hijri --test rab_byung --test foto --test tao`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第八个历法对象：`Juche`
  - 新增 `JucheMonth` / `JucheYear`
  - 新增 `Solar::juche()` / `juche_month()` / `juche_year()`
  - 新增主体纪年专测 `tests/juche.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Juche`
  - 顶层 crate 特性说明补入“主体纪年”
