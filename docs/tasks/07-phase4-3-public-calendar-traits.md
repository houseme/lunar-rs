# Task 07：Phase 4.3 多历法公开 Trait 与统一访问层

> 任务编号：`Task 07`
> 优先级：`P4.3`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 06` 已完成内部共享抽象之后，把这套能力进一步提升为对外可用的公开 trait，让使用者可以通过统一泛型接口访问不同历法对象的核心边界与事件能力。

重点目标：

- 提供公开的 day-like / span-like trait；
- 先覆盖 `Solar / Lunar / Hijri / RabByung / Foto / Tao` 以及常用 `SolarYear / SolarMonth / LunarYear / LunarMonth`；
- 保持既有公开 API 不破坏，只新增统一访问入口；
- 用泛型测试验证 trait 层可直接复用。

---

## 2. 当前现状

当前已有：

- 内部模块 `src/multi_calendar.rs`
- 内部 day/span 共享逻辑
- `Hijri / RabByung / Foto / Tao` 已统一到共享实现路径

当前缺口：

1. 这些共享能力仍是 crate 内部实现细节；
2. 外部用户无法写出基于统一 trait 的多历法泛型逻辑；
3. `Solar / Lunar / SolarMonth / SolarYear / LunarMonth / LunarYear` 还未接入同一抽象层。

---

## 3. 本任务范围

### 3.1 需要完成

1. 设计并导出公开 trait：
   - `CalendarDay`
   - `CalendarSpan`
2. 为以下对象实现统一 trait：
   - `Solar`
   - `Lunar`
   - `Hijri`
   - `RabByungDay`
   - `SolarYear` / `SolarMonth`
   - `LunarYear` / `LunarMonth`
   - `HijriYear` / `HijriMonth`
   - `RabByungYear` / `RabByungMonth`
   - `FotoYear` / `FotoMonth`
   - `TaoYear` / `TaoMonth`
3. 补泛型测试，验证统一调用路径
4. 更新任务索引与验证记录

### 3.2 不包含

- 第五个历法对象新增
- 公共 trait 上的序列化协议设计
- 反推 / 求解类 Phase 5 能力

---

## 4. 进度清单

- [x] 确认 Task 07 推荐路线
- [x] 导出公开 trait
- [x] 接入多历法与基础历法对象
- [x] 补泛型测试与回归

---

## 5. 验收标准

达到以下条件可视为 `Task 07` 完成：

1. crate 对外导出统一的 `CalendarDay` / `CalendarSpan` trait；
2. `Solar / Lunar / Hijri / RabByungDay` 以及 year/month 对象能通过统一 trait 访问；
3. 泛型测试可以直接跨历法复用边界与事件查询；
4. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 6. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test multi_calendar_traits`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 对外导出 `CalendarDay` / `CalendarSpan`
  - `Solar / Lunar / Hijri / RabByungDay` 统一接入 day trait
  - `SolarYear / SolarMonth / LunarYear / LunarMonth / HijriYear / HijriMonth / RabByungYear / RabByungMonth / FotoYear / FotoMonth / TaoYear / TaoMonth` 统一接入 span trait
  - 新增跨历法泛型测试 `tests/multi_calendar_traits.rs`
  - `CalendarSpan::contains_solar()` 调整为按日粒度比较，兼容带时分秒的 solar 边界
