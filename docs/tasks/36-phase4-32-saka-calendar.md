# Task 36：Phase 4.32 第十九个历法对象 `Saka` 接入

> 任务编号：`Task 36`
> 优先级：`P4.32`
> 当前状态：`已完成`
> 最近更新：`2026-07-07`
> 当前进度：`4/4`

---

## 1. 任务目标

在已经接入多种 offset / era / rule-variant 历法对象之后，继续接入第十九个历法对象：

- `Saka`（印度国历 / Indian national calendar）

本轮重点：

- 验证当前 `day/month/year` 模板是否能承接“独立月制 + 非 1 月 1 日新年”的太阳历对象；
- 保持 `CalendarDay / CalendarSpan`、事件代理和 companion 形态一致；
- 以低风险方式继续扩大多历法覆盖面。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Saka`
   - `SakaMonth`
   - `SakaYear`
2. 接入 `Solar`：
   - `Solar::saka()`
   - `Solar::saka_month()`
   - `Solar::saka_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - `Chaitra 1` 年界与月长校验
4. 补测试与文档状态

### 2.2 不包含

- `Saka` 的 i18n 输出
- `Saka` 专属节日数据库
- 更复杂的区域性变体

---

## 3. 进度清单

- [x] 确认第十九个历法对象选型
- [x] 落地 `Saka` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 36` 完成：

1. `Saka / SakaMonth / SakaYear` 全部落地；
2. `Solar` 已提供 `saka()` / `saka_month()` / `saka_year()`；
3. `Saka` 已接入 `CalendarDay`，`SakaMonth / SakaYear` 已接入 `CalendarSpan`；
4. `Chaitra 1` 年界、月长结构、区间边界与往返换算均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-07

- 状态：`已完成`
- 已验证：
  - `cargo test --test saka --test multi_calendar_traits`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第十九个历法对象：`Saka`
  - 新增 `SakaMonth` / `SakaYear`
  - 新增 `Solar::saka()` / `saka_month()` / `saka_year()`
  - 新增萨卡历专测 `tests/saka.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Saka`
  - 顶层 crate 与 README 特性说明补入“印度国历”
