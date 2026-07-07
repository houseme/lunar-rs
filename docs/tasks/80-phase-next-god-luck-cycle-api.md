# Task 80: GodLuck 吉凶循环对象补强

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 中 `Luck` 表示吉凶，固定 2 项：吉、凶，并支持按序号、名称和循环步进访问。本地 `lunar-rs` 已有 `GodLuck` enum，用于吉神凶煞、天神、小六壬等对象，但此前缺少 `from_index`、`from_name`、`index` 和 `CycleItem` 能力，也没有参与 `NamedCulture` 泛型接口。

本任务将 `GodLuck` 补强为轻量循环文化对象，保持 enum 表达不变，不引入额外 wrapper。

## 子任务拆分

1. 为 `GodLuck` 增加 `from_index`，按 `吉 -> 凶` 两项回绕。
2. 为 `GodLuck` 增加 `from_name`、`index`、`name`，其中 `name()` 复用既有 `label()`。
3. 将 `GodLuck` 纳入 `NamedCulture`。
4. 为 `GodLuck` 实现 `CycleItem`。
5. 补充 typed API 测试，覆盖名称反查、索引、回绕和小六壬返回值反查。

## 实现范围

- `src/culture.rs`
  - 扩展 `GodLuck` API。
  - 实现 `NamedCulture` 与 `CycleItem`。
- `tests/phase2_typed.rs`
  - 覆盖 `GodLuck::from_name`、`GodLuck::from_index`、`GodLuck::index`、`CycleItem::next`。
- `docs/next-iteration-index.md`
  - 登记 `Task 80` 状态。

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

下一步可继续分析 `Taboo` 与 `God` 这类大表对象是否适合补固定表反查；它们表量较大，建议单独切片处理。
