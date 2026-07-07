# Task 73: Week typed API

## 对标结论

`tyme4rs` 将星期建模为 `Week` 循环对象。`lunar-rs` 此前已有 `Solar::week()` 与 `Solar::week_in_chinese()`，但缺少 typed companion。

本任务补齐：

- 新增 `Week`。
- 支持 `from_index`、`from_name`、`index`、`name`、`Display`。
- 实现 `NamedCulture` 与 `CycleItem`。
- `Solar` 新增 `week_info()`。

## 验证

- `tests/solar.rs` 校验 `Week` 与现有 `week()` / `week_in_chinese()` 输出一致。
- 校验 `Week` 的循环步进与名称反查可用。
