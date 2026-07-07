# Task 74: Zone 四宫 typed API

## 对标结论

`tyme4rs` 将四宫建模为 `Zone`，并能继续映射到 `Direction` 与 `Beast`。`lunar-rs` 此前已有二十八宿的 `gong()` 字符串、`Direction` 和 `Beast = Shou`，但缺少四宫 typed companion。

本任务补齐：

- 新增 `Zone`。
- 支持 `from_index`、`from_name`、`index`、`name`、`Display`。
- 实现 `NamedCulture` 与 `CycleItem`。
- `Zone` 支持 `direction()` 与 `beast()`。
- `Xiu` 新增 `zone()`。

## 验证

- `tests/phase2_typed.rs` 校验 `Xiu::zone()` 与既有 `gong()` / `shou()` 输出一致。
- 校验 `Zone` 的循环步进能力可用。
