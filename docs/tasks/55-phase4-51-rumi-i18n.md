# Task 55：Phase 4.51 `Rumi` 的 i18n 与描述层收口

> 任务编号：`Task 55`
> 优先级：`P4.51`
> 当前状态：`已完成`
> 最近更新：`2026-07-07`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 54` 已完成 `Rumi` 对象接入之后，继续把它补到与 `Venetian`、`Rattanakosin`、`Seleucid`、`Nanakshahi` 相同的“语言层可消费”状态，确保第二十八个历法对象可以完整进入显示层。

本轮重点：

- 为 `Rumi` 增加显式语言输出；
- 为 `Rumi` 增加完整描述字符串；
- 将 `Rumi` 接入 `Locale` 模板层；
- 用 `i18n` 组合特性验证输出行为。

---

## 2. 本任务范围

### 2.1 需要完成

1. `Locale` 新增 `Rumi` 相关模板入口
2. `Rumi` 新增：
   - `to_string_cn()`
   - `to_full_string()`
   - `to_string_in_lang(...)`
   - `to_full_string_in_lang(...)`
3. `tests/i18n.rs` 补 `Rumi` 用例
4. 完成 `--features i18n` 与 `--features "i18n serde"` 回归

### 2.2 不包含

- 第二十九个全新历法对象接入
- `RumiYear / RumiMonth` 独立语言化 companion 族
- `Rumi` 专属节期数据库

---

## 3. 进度清单

- [x] 确认本轮转向 `Rumi` i18n/描述层
- [x] 接入 `Locale` 模板
- [x] 补 `Rumi` 显式语言输出
- [x] 补 `i18n` 测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 55` 完成：

1. `Locale` 已具备 `Rumi` 的模板入口；
2. `Rumi` 已提供中英文显式输出与完整描述输出；
3. `tests/i18n.rs` 已覆盖 `Rumi`；
4. `cargo test --features i18n` 与 `cargo test --features "i18n serde"` 全绿。

---

## 5. 验证记录

### 2026-07-07

- 状态：`已完成`
- 已验证：
  - `cargo test --features i18n --test i18n`
  - `cargo test --features "i18n serde"`
- 当前已完成：
  - `Locale::rumi_prefix()`
  - `render_rumi_string()` / `render_rumi_full()`
  - `Rumi::to_string_cn()`
  - `Rumi::to_full_string()`
  - `Rumi::to_string_in_lang()`
  - `Rumi::to_full_string_in_lang()`
  - `tests/i18n.rs` 中新增 `Rumi` 用例
