# Task 12：Phase 4.8 第七个历法对象 `Japanese` 接入

> 任务编号：`Task 12`
> 优先级：`P4.8`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在前两轮已经验证线性 solar-offset 历法对象（`Minguo`、`ThaiSolar`）可以稳定接入之后，本轮引入一个更能体现“历法边界语义”的对象：

- `Japanese`（现代日本年号历）

重点验证：

- 非线性纪年是否也能复用当前 year/month/day 模板；
- era 边界切换是否能与 `CalendarDay / CalendarSpan` 和事件体系兼容；
- 多历法扩展是否已经从“偏移纪年”推进到“带制度边界的纪年模型”。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增：
   - `Japanese`
   - `JapaneseMonth`
   - `JapaneseYear`
   - `JapaneseEra`
2. 接入 `Solar`：
   - `Solar::japanese()`
   - `Solar::japanese_month()`
   - `Solar::japanese_year()`
3. 接入统一能力：
   - `CalendarDay`
   - `CalendarSpan`
   - 事件代理
   - era 边界校验
4. 补测试与文档状态

### 2.2 不包含

- `Japanese` 的 i18n 输出
- 明治以前或历史阴阳历映射
- 日本专属节日数据库

---

## 3. 进度清单

- [x] 确认第七个历法对象选型
- [x] 落地 `Japanese` year/month/day 与 era
- [x] 接入 `Solar` 与统一 trait
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 12` 完成：

1. `Japanese / JapaneseMonth / JapaneseYear / JapaneseEra` 全部落地；
2. `Solar` 已提供 `japanese()` / `japanese_month()` / `japanese_year()`；
3. `Japanese` 已接入 `CalendarDay`，`JapaneseMonth / JapaneseYear` 已接入 `CalendarSpan`；
4. 年号切换边界、区间边界、往返换算与推进能力均有测试覆盖；
5. `cargo test` 与 `cargo test --features serde` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test japanese --test multi_calendar_traits`
  - `cargo test --test thai_solar --test minguo --test multi_calendar_ranges --test hijri --test rab_byung --test foto --test tao`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - 新增第七个历法对象：`Japanese`
  - 新增 `JapaneseMonth` / `JapaneseYear` / `JapaneseEra`
  - 新增 `Solar::japanese()` / `japanese_month()` / `japanese_year()`
  - 新增现代日本年号历专测 `tests/japanese.rs`
  - 扩展跨历法泛型 trait 测试以覆盖 `Japanese`
  - 顶层 crate 特性说明补入“现代日本年号历”
