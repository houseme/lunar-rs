# Task 33：Phase 4.29 `Assyrian` 的 i18n 与描述层收口

> 任务编号：`Task 33`
> 优先级：`P4.29`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 32` 已完成 `Assyrian` 对象接入之后，继续把它补到与 `Minguo`、`ThaiSolar`、`Japanese`、`Juche`、`Dangi`、`Julian`、`Holocene`、`Byzantine`、`Coptic`、`Armenian`、`AUC`、`Ethiopian` 相同的“语言层可消费”状态，确保第十七个历法对象可以完整进入显示层。

本轮重点：

- 为 `Assyrian` 增加显式语言输出；
- 为 `Assyrian` 增加完整描述字符串；
- 将 `Assyrian` 接入 `Locale` 模板层；
- 用 `i18n` 组合特性验证输出行为。

---

## 2. 本任务范围

### 2.1 需要完成

1. `Locale` 新增 `Assyrian` 相关模板入口
2. `Assyrian` 新增：
   - `to_string_cn()`
   - `to_full_string()`
   - `to_string_in_lang(...)`
   - `to_full_string_in_lang(...)`
3. `tests/i18n.rs` 补 `Assyrian` 用例
4. 完成 `--features i18n` 与 `--features "i18n serde"` 回归

### 2.2 不包含

- 第十八个全新历法对象接入
- `AssyrianYear / AssyrianMonth` 独立语言化 companion 族
- `Assyrian` 专属节期数据库

---

## 3. 进度清单

- [x] 确认本轮转向 `Assyrian` i18n/描述层
- [x] 接入 `Locale` 模板
- [x] 补 `Assyrian` 显式语言输出
- [x] 补 `i18n` 测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 33` 完成：

1. `Locale` 已具备 `Assyrian` 的模板入口；
2. `Assyrian` 已提供中英文显式输出与完整描述输出；
3. `tests/i18n.rs` 已覆盖 `Assyrian`；
4. `cargo test --features i18n` 与 `cargo test --features "i18n serde"` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --features i18n --test i18n`
  - `cargo test --features "i18n serde"`
  - `cargo test`
- 当前已完成：
  - `Locale::assyrian_prefix()`
  - `render_assyrian_string()` / `render_assyrian_full()`
  - `Assyrian::to_string_cn()`
  - `Assyrian::to_full_string()`
  - `Assyrian::to_string_in_lang()`
  - `Assyrian::to_full_string_in_lang()`
  - `tests/i18n.rs` 中新增 `Assyrian` 用例
