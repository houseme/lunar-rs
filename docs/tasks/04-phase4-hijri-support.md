# Task 04：Phase 4 Hijri 多历法扩展起步

> 任务编号：`Task 04`
> 优先级：`P4`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`5/5`

---

## 1. 任务目标

以 `Hijri` 为第一批多历法对象，验证 `lunar-rs` 已具备从“农历主库”向“多历法库”演进的最小骨架。

---

## 2. 当前现状

已完成：

- `Phase 1`、`Phase 2`、`Phase 3` 已收口；
- 事件模型已可复用；
- typed domain 已具备基础复用能力。

本次已启动：

- 新增独立 `Hijri` 日期对象；
- 新增 `Solar::hijri()`；
- 新增 `Hijri::from_ymd()` / `Hijri::solar()` / `Hijri::next()`；
- 已补核心回归测试。
- 已继续补：
  - `HijriYear`
  - `HijriMonth`
  - `Solar::rab_byung_year()`
  - `RabByungElement`
  - `RabByungYear`
  - `Solar::rab_byung_day()`
  - `RabByungMonth`
  - `RabByungDay`
  - `Hijri::events()` / `find_events()`
  - `RabByungDay::events()` / `find_events()`
- 已继续补边界/层级工具：
  - `HijriYear::contains_solar()`
  - `HijriMonth::contains_solar()`
  - `Hijri::subtract()`
  - `RabByungMonth::alias()`
  - `RabByungMonth::contains_solar()`
  - `RabByungDay::subtract()`

当前收口结论：

- `Hijri` 已具备日/月/年层级、双向换算、边界工具与事件体系复用；
- `RabByung` 已具备年/月/日层级、闰月/闰日/缺日处理、双向换算与事件体系复用；
- Phase 4 首轮“先落两个可验证的多历法对象”目标已达成。

---

## 3. 实现范围

### 3.1 本任务需要完成

1. 落地 `Hijri` 独立对象；
2. 完成 `Solar <-> Hijri` 双向换算；
3. 补基础闰年/月天数/遍历能力；
4. 补测试；
5. 为后续 `HijriYear` / `HijriMonth` / 事件复用保留演进空间。

### 3.2 本任务不包含

- `RabByung` 具体实现；
- `Hijri` 节日系统；
- `Hijri` typed year/month 分层的完整铺开；
- 国际化文案扩展。

---

## 4. 进度清单

- [x] 设计 Phase 4 首个任务切入点
- [x] 落地 `Hijri` 最小对象与 `Solar` 互转
- [x] 补 `tests/hijri.rs`
- [x] 拆出 `HijriYear` / `HijriMonth`
- [x] 启动 `RabByung` year-level 对象
- [x] 补齐 `RabByungMonth` / `RabByungDay`
- [x] 把 `Hijri / RabByungDay` 接进现有事件体系
- [x] 补完首轮 year/month/day 边界工具与回归测试
- [x] 完成 Phase 4 首轮收口判断

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test hijri`
  - `cargo test --test rab_byung`
  - `cargo test`
  - `cargo test --features serde`
- 当前已落地：
  - `Hijri`
  - `HijriYear`
  - `HijriMonth`
  - `RabByungElement`
  - `RabByungYear`
  - `RabByungMonth`
  - `RabByungDay`
- 当前已接入事件体系：
  - `Hijri`
  - `RabByungDay`
- 当前已补首轮边界工具：
  - `contains_solar`
  - `subtract`
  - `alias`
- 结论：
  - Phase 4 首轮范围已达成；
  - 后续若继续深化 `Hijri / RabByung`，建议作为新任务继续推进，而不是继续挂在本任务下。
