# Task 52：Phase 4.48 第二十七个历法对象 `Venetian` 接入

> 任务编号：`Task 52`
> 优先级：`P4.48`
> 当前状态：`已完成`
> 最近更新：`2026-07-07`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 51` 已完成 `Rattanakosin` 的 i18n 与描述层收口之后，继续推进第二十七个历法对象：

- `Venetian`（威尼斯纪年 / More Veneto）

本轮重点：

- 新增 `VenetianYear / VenetianMonth / Venetian`；
- 将 `Venetian` 接入 `Solar` 入口与统一导出；
- 复用 `CalendarDay / CalendarSpan / EventQuery`；
- 用独立测试与统一 trait 测试验证“3 月 1 日换年”模板复用。

---

## 2. 本任务范围

### 2.1 需要完成

1. 新增 `src/venetian.rs`
2. 新增：
   - `VenetianYear`
   - `VenetianMonth`
   - `Venetian`
3. `Solar` 新增：
   - `venetian()`
   - `venetian_year()`
   - `venetian_month()`
4. 新增 `tests/venetian.rs`
5. 将 `Venetian` 接入 `tests/multi_calendar_traits.rs`

### 2.2 不包含

- `Venetian` 的 i18n 与描述层英文模板
- `Venetian` 专属事件或节期数据库
- 与历史儒略历切换细节绑定的地区化差异

---

## 3. 进度清单

- [x] 确认第二十七个历法对象选型
- [x] 完成 `Venetian` year/month/day 接入
- [x] 接入统一 trait 与 `Solar` 入口
- [x] 补测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 52` 完成：

1. `Solar` 已能解析 `Venetian` 及其 companion；
2. `VenetianYear / Month / Day` 已提供基础边界与推进能力；
3. `Venetian` 已复用统一事件与 trait 能力；
4. `tests/venetian.rs` 与相关 trait 测试通过。

---

## 5. 验证记录

### 2026-07-07

- 状态：`已完成`
- 已验证：
  - `cargo test --test venetian`
  - `cargo test --test multi_calendar_traits`
  - `cargo test`
- 当前已完成：
  - `src/venetian.rs`
  - `Solar::venetian() / venetian_year() / venetian_month()`
  - `tests/venetian.rs`
  - `tests/multi_calendar_traits.rs` 接入 `Venetian`
