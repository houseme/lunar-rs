# Task 75: Nayin 纳音循环对象补强

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 中 `Sound` 是固定 30 项纳音循环对象，支持按序号取值、按名称定位、前后循环移动。本地 `lunar-rs` 已有 `Nayin` typed wrapper，并且年、月、日、时纳音已经能返回 `Nayin`，但此前只保留名称和五行推导，缺少循环定位能力。

本任务将 `Nayin` 从“字符串 typed wrapper”补强为“30 纳音循环对象”，继续沿用本库已引入的 `NamedCulture` / `CycleItem` 模式。

## 子任务拆分

1. 增加固定 30 项纳音表，顺序与干支纳音映射保持一致。
2. 为 `Nayin` 增加 `from_index`、`from_name`、`index`。
3. 为 `Nayin` 实现 `CycleItem`，复用 `next`、`steps_to`、`steps_back_to`、`steps_close_to`。
4. 补充 typed API 测试，确认农历日纳音可反查、可按 index 重建、可循环步进。

## 实现范围

- `src/culture.rs`
  - 新增 `NAYIN_NAMES`。
  - 扩展 `Nayin` 构造和索引 API。
  - 实现 `CycleItem for Nayin`。
- `tests/phase2_typed.rs`
  - 覆盖 `Nayin::from_name`、`Nayin::from_index`、`Nayin::index`、`CycleItem::next`。
- `docs/next-iteration-index.md`
  - 登记 `Task 75` 状态。

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

下一步可继续梳理 `tyme4rs` 中剩余较小文化循环对象，例如 `Land`、`Ten`、`Sixty/Twenty` 等与本库现有 `Terrain`、`Xun`、`YuanCycle/YunCycle` 的命名和行为差异，优先选择能小步闭环且不破坏现有 API 的对象补强。
