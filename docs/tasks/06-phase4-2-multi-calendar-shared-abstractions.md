# Task 06：Phase 4.2 多历法共享抽象与边界一致化

> 任务编号：`Task 06`
> 优先级：`P4.2`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 05` 已经证明 `Hijri / RabByung / Foto / Tao` 模板可复用之后，进一步把这些历法对象里已经反复出现的“同一层能力”抽成共享抽象，减少后续新增历法时的重复实现成本，并把 year/month/day companion 的边界能力再统一一层。

重点目标：

- 提炼多历法 `day` 与 `span(year/month)` 的共享事件代理模式；
- 统一 `first_solar_day / last_solar_day` 这类高频边界 helper 的形态；
- 尽量保持现有公开 API 不破坏，只做增量补强；
- 补回归测试，证明共享层没有改变既有行为。

---

## 2. 当前现状

已观察到的重复点：

1. `HijriYear / HijriMonth / RabByungYear / RabByungMonth / FotoYear / FotoMonth / TaoYear / TaoMonth`
   - 都在重复实现：
     - `events()`
     - `all_events()`
     - `find_events()`
     - `events_until()`（部分类型）
     - `find_events_until()`（部分类型）
2. `Hijri / RabByungDay`
   - 都在重复实现 day 级事件代理：
     - `events()`
     - `all_events()`
     - `find_events()`
     - `events_until()`
     - `find_events_until()`
3. 一部分历法对象已经有 `first_solar_day / last_solar_day`，另一部分仍通过 `first_day().solar()` / `last_day().solar()` 内联展开，不够统一。

---

## 3. 本任务范围

### 3.1 需要完成

1. 新增内部共享抽象层：
   - day-like solar anchor
   - span-like solar range
2. 把多历法 year/month/day 的重复事件代理切到共享层
3. 为 `HijriYear / HijriMonth / RabByungYear / RabByungMonth` 补齐：
   - `first_solar_day()`
   - `last_solar_day()`
4. 补跨历法边界与事件代理回归测试

### 3.2 不包含

- 第五个新历法对象
- 新的节日数据库
- 事件模型字段扩充
- `Phase 5` 的逆算与求解

---

## 4. 进度清单

- [x] 审计多历法重复实现热点
- [x] 落地共享抽象层
- [x] 收拢 year/month/day 重复事件代理
- [x] 补统一边界测试与全量回归

---

## 5. 验收标准

达到以下条件可视为 `Task 06` 完成：

1. `Hijri / RabByung / Foto / Tao` 的 year/month/day 重复事件代理明显减少；
2. `HijriYear / HijriMonth / RabByungYear / RabByungMonth` 拥有统一的 solar 边界 helper；
3. `cargo test` 与 `cargo test --features serde` 全绿；
4. 后续新增历法对象时，可直接复用共享抽象而不再手写整套代理。

---

## 6. 建议验证命令

```bash
cargo test --test hijri
cargo test --test rab_byung
cargo test --test foto
cargo test --test tao
cargo test
cargo test --features serde
```

---

## 7. 变更记录

### 2026-07-06

- 创建任务文档。
- 状态设为 `进行中`。
- 当前方向：优先收拢多历法共享抽象与边界一致化。

### 2026-07-06

- 状态改为 `已完成`。
- 已新增内部共享模块，统一多历法 day/span 事件代理。
- 已为 `HijriYear / HijriMonth / RabByungYear / RabByungMonth` 补齐 `first_solar_day / last_solar_day`。
- 已补跨历法边界测试，并完成全量回归。

---

## 8. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test multi_calendar_ranges`
  - `cargo test --test hijri --test rab_byung --test foto --test tao`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 内部共享抽象 `src/multi_calendar.rs`
  - `Hijri / RabByung / Foto / Tao` 的 range/day 事件代理收拢
  - 多历法 year/month solar 边界 helper 进一步一致化
