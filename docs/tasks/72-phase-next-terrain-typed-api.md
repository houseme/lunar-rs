# Task 72: Terrain 十二长生 typed API

## 对标结论

`tyme4rs` 将十二长生（地势）建模为 `Terrain` 循环对象。`lunar-rs` 此前在 `EightChar` 中已有 `year_di_shi()`、`month_di_shi()`、`day_di_shi()`、`time_di_shi()` 字符串 API，但缺少 typed companion。

本任务补齐：

- 新增 `Terrain`。
- 支持 `from_index`、`from_name`、`index`、`name`、`Display`。
- 实现 `NamedCulture` 与 `CycleItem`。
- `EightChar` 新增 `year_terrain()`、`month_terrain()`、`day_terrain()`、`time_terrain()`。

## 验证

- `tests/eight_char.rs` 校验四柱 `Terrain` 与既有 `*_di_shi()` 字符串输出一致。
- 校验 `Terrain` 可通过 `CycleItem` 执行循环步进。
