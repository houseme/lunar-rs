# Task 23：Phase 4.19 `Byzantine` 的 i18n 与描述层收口

> 任务编号：`Task 23`
> 优先级：`P4.19`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 22` 已完成 `Byzantine` 对象接入之后，继续把它补到与 `Minguo`、`ThaiSolar`、`Japanese`、`Juche`、`Dangi`、`Julian`、`Holocene` 相同的“语言层可消费”状态，确保非 `1 月 1 日` 起年的对象也能完整进入显示层。

本轮重点：

- 为 `Byzantine` 增加显式语言输出；
- 为 `Byzantine` 增加完整描述字符串；
- 将 `Byzantine` 接入 `Locale` 模板层；
- 用 `i18n` 组合特性验证输出行为。

---

## 2. 本任务范围

### 2.1 需要完成

1. `Locale` 新增 `Byzantine` 相关模板入口
2. `Byzantine` 新增：
   - `to_string_cn()`
   - `to_full_string()`
   - `to_string_in_lang(...)`
   - `to_full_string_in_lang(...)`
3. `tests/i18n.rs` 补 `Byzantine` 用例
4. 完成 `--features i18n` 与 `--features "i18n serde"` 回归

### 2.2 不包含

- 第十三个全新历法对象接入
- `ByzantineYear / ByzantineMonth` 独立语言化 companion 族
- `Byzantine` 专属节期数据库

---

## 3. 进度清单

- [x] 确认本轮转向 `Byzantine` i18n/描述层
- [x] 接入 `Locale` 模板
- [x] 补 `Byzantine` 显式语言输出
- [x] 补 `i18n` 测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 23` 完成：

1. `Locale` 已具备 `Byzantine` 的模板入口；
2. `Byzantine` 已提供中英文显式输出与完整描述输出；
3. `tests/i18n.rs` 已覆盖 `Byzantine`；
4. `cargo test --features i18n` 与 `cargo test --features "i18n serde"` 全绿。

---

## 5. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --features i18n --test i18n`
  - `cargo test --features i18n`
  - `cargo test --features "i18n serde"`
  - `cargo test`
- 当前已完成：
  - `Locale::byzantine_prefix()`
  - `render_byzantine_string()` / `render_byzantine_full()`
  - `Byzantine::to_string_cn()`
  - `Byzantine::to_full_string()`
  - `Byzantine::to_string_in_lang()`
  - `Byzantine::to_full_string_in_lang()`
  - `tests/i18n.rs` 中新增 `Byzantine` 用例
