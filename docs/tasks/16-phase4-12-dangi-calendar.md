# Task 16：Phase 4.12 第九个历法对象 `Dangi` 接入

> 任务编号：`Task 16`
> 优先级：`P4.12`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在已经完成多个 solar-based 历法、带年号边界的 `Japanese`，以及带纪元起点限制的 `Juche` 之后，继续推进第九个历法对象：

- `Dangi`（檀纪 / Korean era calendar）

本轮重点验证：

- 继续复用当前 `day/month/year` 模板扩展新的文化纪年；
- 保持 `CalendarDay / CalendarSpan`、事件代理、solar 边界 helper 的一致性；
- 证明多历法扩展已经进入稳定复用阶段。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Dangi`
   - `DangiMonth`
   - `DangiYear`
2. 接入 `Solar`：
   - `Solar::dangi()`
   - `Solar::dangi_month()`
   - `Solar::dangi_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - 边界 helper
4. 补测试与文档状态

### 2.2 不包含

- `Dangi` 的 i18n 输出
- `Dangi` 专属节日数据库
- 更复杂的历史纪元制度

---

## 3. 进度清单

- [x] 确认第九个历法对象选型
- [x] 落地 `Dangi` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 16` 完成：

1. `Dangi / DangiMonth / DangiYear` 全部落地；
2. `Solar` 已提供 `dangi()` / `dangi_month()` / `dangi_year()`；
3. `Dangi` 已接入 `CalendarDay`，`DangiMonth / DangiYear` 已接入 `CalendarSpan`；
4. 区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test dangi --test multi_calendar_traits`
  - `cargo test --test juche --test japanese --test thai_solar --test minguo --test multi_calendar_ranges --test hijri --test rab_byung --test foto --test tao`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第九个历法对象：`Dangi`
  - 新增 `DangiMonth` / `DangiYear`
  - 新增 `Solar::dangi()` / `dangi_month()` / `dangi_year()`
  - 新增檀纪专测 `tests/dangi.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Dangi`
  - 顶层 crate 特性说明补入“檀纪”
