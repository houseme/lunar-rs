# Task 03：Phase 3 事件模型独立化与多日事件内建

> 任务编号：`Task 03`
> 优先级：`P3`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`5/5`

---

## 1. 任务目标

把当前“统一事件出口”继续推进到更完整的事件域模型，包括：

- 独立事件包装类型；
- 事件范围语义完善；
- 内建 multi-day 事件源；
- 更稳定的查询、分组和前端集成能力。

---

## 2. 当前现状

已完成：

- `Event` / `EventKind` / `EventSource` / `CalendarKind` 已存在；
- `Solar::events()`、`Lunar::events()`、`Foto::events()`、`Tao::events()` 已存在；
- 排序、去重、过滤、按日/周聚合、区间扫描已存在；
- 本轮已补：
  - `EventRangeKind`
  - `end_solar`
  - `with_range(...)`
  - `covers_solar(...)`
  - `spans_multiple_days()`
  - `EventQuery::with_range_kind(...)`
- 本次增量已补：
  - `SolarFestivalEvent`
  - `LunarFestivalEvent`
  - `HolidayEvent`
  - `HolidayPeriodEvent`
  - `JieQiEvent`
  - `FotoFestivalEvent`
  - `TaoFestivalEvent`
- 本次已落地首个真实 multi-day 内建事件源：
  - 基于法定节假日连续放假区间的 `HolidayPeriodEvent`

当前闭环结论：

- 第一批独立事件包装类型已落地；
- 首个真实 multi-day 内建事件源已落地；
- `EventQuery` 已支持 `range_kind`、`is_primary`、`is_observed`、`source_family`、`tag` 等稳定维度；
- 排序、分组、缓存失效、区间扫描已完成回归验证。

---

## 3. 实现范围

### 3.1 本任务需要完成

1. 明确事件层级：
   - 通用 `Event`
   - 独立源事件包装类型
2. 引入或补齐以下 typed wrapper：
   - `SolarFestivalEvent`
   - `LunarFestivalEvent`
   - `HolidayEvent`
   - `JieQiEvent`
   - 视需要再补 `FotoFestivalEvent` / `TaoFestivalEvent`
3. 增加真实 multi-day 内建事件源：
   - 优先考虑法定假期连续区间；
   - 或基于现有节假日数据构建连续观察事件。
4. 增强查询能力：
   - `range_kind`
   - 主事件/次事件
   - tags
   - 可能的 source family
5. 补测试，证明：
   - 单日与多日事件都可稳定查询；
   - 分组与排序规则不回退；
   - 节假日覆盖更新能正确影响事件缓存。

### 3.2 本任务不包含

- 新增多历法对象本身；
- 历法扩展到 `Hijri`；
- 复杂外部事件存储或数据库索引；
- Web/前端层实际 UI 开发。

---

## 4. 建议拆分步骤

### Step 1：事件类型层级设计

- 明确哪些源对象需要独立 wrapper；
- 决定 wrapper 与通用 `Event` 的关系：
  - `into_event`
  - `as_event`
  - 或 builder 模式。

### Step 2：多日事件数据源落地

- 从现有 `Holiday` 数据切入最稳；
- 优先实现“同一法定节日连续天”的聚合事件；
- 避免一开始就引入过于复杂的事件规则引擎。

### Step 3：查询与分组增强

- 让 `EventQuery` 支持更多稳定维度；
- 校验日/周分组对多日事件的展示口径。

### Step 4：测试与缓存回归

- 覆盖 multi-day 构造；
- 覆盖过滤、排序、去重、范围扫描；
- 覆盖节假日覆盖更新后的缓存失效逻辑。

---

## 5. 验收结果

已满足：

1. 至少一类真实内建 multi-day 事件源已落地；
2. 关键源对象已具备独立事件包装类型；
3. `EventQuery` 已能稳定过滤 `range_kind` 与主要事件语义；
4. `tests/phase3_events.rs` 已完成覆盖增强；
5. 默认测试、`serde` 特性测试均已通过。

---

## 6. 风险点

- 如果直接把所有事件源独立成类型，容易一次性扩张过快；
- 多日事件进入分组后，排序/去重/缓存可能出现边界问题；
- 事件包装类型若与 typed domain 不统一，会重复表达同一概念。

---

## 7. 建议验证命令

```bash
cargo test
cargo test --features serde
```

必要时追加：

```bash
cargo test --test phase3_events
```

---

## 8. 进度清单

- [x] 明确事件 wrapper 分层设计
- [x] 落地第一批独立事件包装类型
- [x] 接入真实 multi-day 内建事件源
- [x] 扩展 `EventQuery` 的高阶维度
- [x] 扩展 `tests/phase3_events.rs`

---

## 9. 验证记录

### 2026-07-06

- 状态：`已完成`
- 备注：已完成第一批事件 wrapper、首个真实 multi-day 内建事件源，以及 `EventQuery` 的高阶维度增强。
- 已验证：
  - `cargo test --test phase3_events`
  - `cargo test`
  - `cargo test --features serde`
- 当前已落地 wrapper：
  - `SolarFestivalEvent`
  - `LunarFestivalEvent`
  - `HolidayEvent`
  - `HolidayPeriodEvent`
  - `JieQiEvent`
  - `FotoFestivalEvent`
  - `TaoFestivalEvent`
- 当前已落地查询维度：
  - `range_kind`
  - `is_primary`
  - `is_observed`
  - `source_family`
  - `tag`
- 结论：
  - `Task 03` 已满足当前 roadmap 范围内的阶段验收；
  - 若后续继续扩展更多 multi-day 规则，应作为下一轮增量优化。

---

## 10. 变更记录

### 2026-07-06

- 创建任务文档。

### 2026-07-06

- 状态改为 `进行中`。
- 已完成第一批事件 wrapper 与首个真实 multi-day 内建事件源。

### 2026-07-06

- 已补 `EventQuery` 高阶维度：`source_family`、`is_observed`。
- 状态改为 `已完成`。
