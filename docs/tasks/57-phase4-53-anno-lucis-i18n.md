# Task 57：Phase 4.53 `AnnoLucis` 的 i18n 与描述层收口

> 任务编号：`Task 57`
> 优先级：`P4.53`
> 当前状态：`已完成`
> 最近更新：`2026-07-07`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 56` 已完成 `AnnoLucis` 对象接入之后，继续把它补到与 `Holocene`、`Rumi`、`Venetian`、`Rattanakosin` 相同的“语言层可消费”状态，确保第二十九个历法对象可以完整进入显示层。

本轮重点：

- 为 `AnnoLucis` 增加显式语言输出；
- 为 `AnnoLucis` 增加完整描述字符串；
- 将 `AnnoLucis` 接入 `Locale` 模板层；
- 用 `i18n` 组合特性验证输出行为。

---

## 2. 本任务范围

### 2.1 需要完成

1. `Locale` 新增 `AnnoLucis` 相关模板入口
2. `AnnoLucis` 新增：
   - `to_string_cn()`
   - `to_full_string()`
   - `to_string_in_lang(...)`
   - `to_full_string_in_lang(...)`
3. `tests/i18n.rs` 补 `AnnoLucis` 用例
4. 完成 `--features i18n` 与 `--features "i18n serde"` 回归

### 2.2 不包含

- 第三十个全新历法对象接入
- `AnnoLucisYear / AnnoLucisMonth` 独立语言化 companion 族
- `AnnoLucis` 专属节期数据库

---

## 3. 进度清单

- [x] 确认本轮转向 `AnnoLucis` i18n/描述层
- [x] 接入 `Locale` 模板
- [x] 补 `AnnoLucis` 显式语言输出
- [x] 补 `i18n` 测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 57` 完成：

1. `Locale` 已具备 `AnnoLucis` 的模板入口；
2. `AnnoLucis` 已提供中英文显式输出与完整描述输出；
3. `tests/i18n.rs` 已覆盖 `AnnoLucis`；
4. `cargo test --features i18n` 与 `cargo test --features "i18n serde"` 全绿。

---

## 5. 验证记录

### 2026-07-07

- 状态：`已完成`
- 已验证：
  - `cargo test --features i18n --test i18n`
  - `cargo test --features "i18n serde"`
- 当前已完成：
  - `Locale::anno_lucis_prefix()`
  - `render_anno_lucis_string()` / `render_anno_lucis_full()`
  - `AnnoLucis::to_string_cn()`
  - `AnnoLucis::to_full_string()`
  - `AnnoLucis::to_string_in_lang()`
  - `AnnoLucis::to_full_string_in_lang()`
  - `tests/i18n.rs` 中新增 `AnnoLucis` 用例
