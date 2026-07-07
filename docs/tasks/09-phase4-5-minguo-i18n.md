# Task 09：Phase 4.5 `Minguo` 的 i18n 与描述层收口

> 任务编号：`Task 09`
> 优先级：`P4.5`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 08` 已完成 `Minguo` 对象接入之后，继续把它补到与 `Foto` / `Tao` 相近的“语言层可消费”状态，避免新增历法对象只停留在算法与 companion 层。

本轮重点：

- 为 `Minguo` 增加显式语言输出；
- 为 `Minguo` 增加完整描述字符串；
- 将 `Minguo` 接入 `Locale` 模板层；
- 用 `i18n` feature 测试验证输出行为。

---

## 2. 本任务范围

### 2.1 需要完成

1. `Locale` 新增 `Minguo` 相关模板入口
2. `Minguo` 新增：
   - `to_string_cn()`
   - `to_full_string()`
   - `to_string_in_lang(...)`
   - `to_full_string_in_lang(...)`
3. `tests/i18n.rs` 补 `Minguo` 用例
4. 完成 `--features i18n` 回归

### 2.2 不包含

- 第六个全新历法对象接入
- `MinguoYear / MinguoMonth` 独立语言化 companion 族
- `Minguo` 专属节日数据库

---

## 3. 进度清单

- [x] 确认本轮转向 `Minguo` i18n/描述层
- [x] 接入 `Locale` 模板
- [x] 补 `Minguo` 显式语言输出
- [x] 补 `i18n` 测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 09` 完成：

1. `Locale` 已具备 `Minguo` 的模板入口；
2. `Minguo` 已提供中英文显式输出与完整描述输出；
3. `tests/i18n.rs` 已覆盖 `Minguo`；
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
  - `Locale::minguo_prefix()`
  - `render_minguo_string()` / `render_minguo_full()`
  - `Minguo::to_string_cn()`
  - `Minguo::to_full_string()`
  - `Minguo::to_string_in_lang()`
  - `Minguo::to_full_string_in_lang()`
  - `tests/i18n.rs` 中新增 `Minguo` 用例
