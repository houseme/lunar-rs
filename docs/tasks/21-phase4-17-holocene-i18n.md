# Task 21：Phase 4.17 `Holocene` 的 i18n 与描述层收口

> 任务编号：`Task 21`
> 优先级：`P4.17`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`4/4`

---

## 1. 任务目标

在 `Task 20` 已完成 `Holocene` 对象接入之后，继续把它补到与 `Minguo`、`ThaiSolar`、`Japanese`、`Juche`、`Dangi`、`Julian` 相同的“语言层可消费”状态，保证第十一个历法对象也能完整进入显示层。

本轮重点：

- 为 `Holocene` 增加显式语言输出；
- 为 `Holocene` 增加完整描述字符串；
- 将 `Holocene` 接入 `Locale` 模板层；
- 用 `i18n` 组合特性验证输出行为。

---

## 2. 本任务范围

### 2.1 需要完成

1. `Locale` 新增 `Holocene` 相关模板入口
2. `Holocene` 新增：
   - `to_string_cn()`
   - `to_full_string()`
   - `to_string_in_lang(...)`
   - `to_full_string_in_lang(...)`
3. `tests/i18n.rs` 补 `Holocene` 用例
4. 完成 `--features i18n` 与 `--features "i18n serde"` 回归

### 2.2 不包含

- 第十二个全新历法对象接入
- `HoloceneYear / HoloceneMonth` 独立语言化 companion 族
- `Holocene` 专属节日数据库

---

## 3. 进度清单

- [x] 确认本轮转向 `Holocene` i18n/描述层
- [x] 接入 `Locale` 模板
- [x] 补 `Holocene` 显式语言输出
- [x] 补 `i18n` 测试并完成回归

---

## 4. 验收标准

达到以下条件可视为 `Task 21` 完成：

1. `Locale` 已具备 `Holocene` 的模板入口；
2. `Holocene` 已提供中英文显式输出与完整描述输出；
3. `tests/i18n.rs` 已覆盖 `Holocene`；
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
  - `Locale::holocene_prefix()`
  - `render_holocene_string()` / `render_holocene_full()`
  - `Holocene::to_string_cn()`
  - `Holocene::to_full_string()`
  - `Holocene::to_string_in_lang()`
  - `Holocene::to_full_string_in_lang()`
  - `tests/i18n.rs` 中新增 `Holocene` 用例
