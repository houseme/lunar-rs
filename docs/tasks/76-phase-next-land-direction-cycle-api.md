# Task 76: Land 九野与 Direction 九宫循环对象

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 中 `Land` 表示九野，固定 9 项，并能按相同索引映射到九宫 `Direction`。本地 `lunar-rs` 已有 `Direction` 名称包装，但此前没有九宫方位循环，也没有独立 `Land` typed object。

本任务补齐 `Land` 与九宫 `Direction` 的轻量循环能力，让它们复用现有 `NamedCulture` / `CycleItem` 模式，继续减少裸字符串文化概念。

## 子任务拆分

1. 增加九宫方位表：北、西南、东、东南、中、西北、西、东北、南。
2. 为 `Direction` 增加 `from_index`、`from_name`、`index`，保留既有 `new` 自由名称构造。
3. 新增 `Land` 九野对象：玄天、朱天、苍天、阳天、钧天、幽天、颢天、变天、炎天。
4. 为 `Direction` 与 `Land` 实现 `CycleItem`。
5. 为 `Land` 增加 `direction()`，按同索引返回九宫方位。
6. 补充 typed API 测试，覆盖九宫循环、九野回绕和九野到方位映射。

## 实现范围

- `src/culture.rs`
  - 新增 `DIRECTION_NAMES`、`LAND_NAMES`。
  - 扩展 `Direction` 索引/反查 API。
  - 新增 `Land` typed object。
  - 实现 `CycleItem for Direction` 与 `CycleItem for Land`。
- `src/lib.rs`
  - 导出 `Land`。
- `tests/phase2_typed.rs`
  - 覆盖 `Direction` / `Land` 循环与映射。
- `docs/next-iteration-index.md`
  - 登记 `Task 76` 状态。

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

下一步可继续推进 `Sixty` / `Twenty` 与本库 `YuanCycle` / `YunCycle` 的循环对象语义对齐，或检查 `Ten` 与现有 `Xun` 的命名兼容关系。
