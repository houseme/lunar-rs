# Task 25：Phase 4.21 第十四个历法对象 `Ethiopian` 接入

> 任务编号：`Task 25`
> 优先级：`P4.21`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Coptic` 已验证 `13` 月结构之后，继续接入第十四个历法对象：

- `Ethiopian`（埃塞俄比亚历）

本轮重点：

- 复用 `13` 月结构，但切换到不同的纪年锚点；
- 验证当前多历法模板已经可以在“同月制，不同纪元”的情况下稳定复用；
- 保持 `CalendarDay / CalendarSpan`、事件代理和 companion 形态一致。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Ethiopian`
   - `EthiopianMonth`
   - `EthiopianYear`
2. 接入 `Solar`：
   - `Solar::ethiopian()`
   - `Solar::ethiopian_month()`
   - `Solar::ethiopian_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - `13` 月边界校验
4. 补测试与文档状态

### 2.2 不包含

- `Ethiopian` 的 i18n 输出
- 宗教节期数据库
- 更复杂的地区差异与旧制映射

---

## 3. 进度清单

- [x] 确认第十四个历法对象选型
- [x] 落地 `Ethiopian` year/month/day
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 25` 完成：

1. `Ethiopian / EthiopianMonth / EthiopianYear` 全部落地；
2. `Solar` 已提供 `ethiopian()` / `ethiopian_month()` / `ethiopian_year()`；
3. `Ethiopian` 已接入 `CalendarDay`，`EthiopianMonth / EthiopianYear` 已接入 `CalendarSpan`；
4. `13` 月结构、闰尾月、区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test ethiopian --test coptic --test multi_calendar_traits`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第十四个历法对象：`Ethiopian`
  - 新增 `EthiopianMonth` / `EthiopianYear`
  - 新增 `Solar::ethiopian()` / `ethiopian_month()` / `ethiopian_year()`
  - 新增埃塞俄比亚历专测 `tests/ethiopian.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Ethiopian`
  - 顶层 crate 特性说明补入“埃塞俄比亚历”
