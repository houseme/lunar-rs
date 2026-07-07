# Task 71: XiuAnimal / Beast 循环对象补强

## 对标结论

`tyme4rs` 在文化模块中提供独立的 `Animal`（二十八宿动物）和 `Beast`（四神兽）循环对象。`lunar-rs` 已有 `XiuAnimal` 和 `Shou`，但此前只作为字符串 wrapper，缺少索引、反查和循环能力。

本任务补齐：

- `XiuAnimal` 新增 `from_index`、`from_name`、`index`、`Display`。
- `Shou` 新增 `from_index`、`from_name`、`index`、`Display`。
- 新增 `Beast = Shou` 公共别名，用于对齐 `tyme4rs` 命名。
- `XiuAnimal` / `Shou` 实现 `CycleItem`。

## 验证

- `tests/phase2_typed.rs` 校验二十八宿动物与四神兽仍与既有 getter 一致。
- 校验 `next()` / `steps_back_to()` 循环语义可用。
