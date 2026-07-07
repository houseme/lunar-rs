# Task 43：Phase 4.39 `ThaiBuddhist` 的 i18n 与描述层收口

> 任务编号：`Task 43`
> 优先级：`P4.39`
> 当前状态：`已完成`
> 最近更新：`2026-07-07`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 42` 已完成 `ThaiBuddhist` 对象接入之后，继续把它补到与 `Minguo`、`ThaiSolar`、`Japanese`、`Juche`、`Dangi`、`Julian`、`Holocene`、`Byzantine`、`Coptic`、`Armenian`、`AUC`、`Assyrian`、`HispanicEra`、`Saka`、`Bengali`、`Koki`、`Ethiopian` 相同的“语言层可消费”状态，确保第二十二个历法对象可以完整进入显示层。

本轮重点：

- 为 `ThaiBuddhist` 增加显式语言输出；
- 为 `ThaiBuddhist` 增加完整描述字符串；
- 将 `ThaiBuddhist` 接入 `Locale` 模板层；
- 用 `i18n` 组合特性验证输出行为。

---

## 2. 本任务范围

### 2.1 需要完成

1. `Locale` 新增 `ThaiBuddhist` 相关模板入口
2. `ThaiBuddhist` 新增：
   - `to_string_cn()`
   - `to_full_string()`
   - `to_string_in_lang(...)`
   - `to_full_string_in_lang(...)`
3. `tests/i18n.rs` 补 `ThaiBuddhist` 用例
4. 完成 `--features i18n` 与 `--features "i18n serde"` 回归

### 2.2 不包含

- 第二十三个全新历法对象接入
- `ThaiBuddhistYear / ThaiBuddhistMonth` 独立语言化 companion 族
- `ThaiBuddhist` 专属节期数据库

---

## 3. 进度清单

- [x] 确认本轮转向 `ThaiBuddhist` i18n/描述层
- [x] 接入 `Locale` 模板
- [x] 补 `ThaiBuddhist` 显式语言输出
- [x] 补 `i18n` 测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 43` 完成：

1. `Locale` 已具备 `ThaiBuddhist` 的模板入口；
2. `ThaiBuddhist` 已提供中英文显式输出与完整描述输出；
3. `tests/i18n.rs` 已覆盖 `ThaiBuddhist`；
4. `cargo test --features i18n` 与 `cargo test --features "i18n serde"` 全绿。

---

## 5. 验证记录

### 2026-07-07

- 状态：`已完成`
- 已验证：
  - `cargo test --features i18n --test i18n`
  - `cargo test --features "i18n serde"`
  - `cargo test`
- 当前已完成：
  - `Locale::thai_buddhist_prefix()`
  - `render_thai_buddhist_string()` / `render_thai_buddhist_full()`
  - `ThaiBuddhist::to_string_cn()`
  - `ThaiBuddhist::to_full_string()`
  - `ThaiBuddhist::to_string_in_lang()`
  - `ThaiBuddhist::to_full_string_in_lang()`
  - `tests/i18n.rs` 中新增 `ThaiBuddhist` 用例
