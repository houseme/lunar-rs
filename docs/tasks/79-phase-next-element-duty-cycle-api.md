# Task 79: Element / Duty 基础文化循环对象补强

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 中 `Element` 是固定 5 项五行循环对象，`Duty` 是固定 12 项建除十二值神循环对象，二者都支持按序号、名称和循环步进访问。本地 `lunar-rs` 已有 `Element` / `Duty` typed wrapper，但此前主要是名称包装和关系查询，尚未补齐循环访问能力。

本任务只对齐语义完全一致的 `Element` 与 `Duty`。`tyme4rs::Phase` 是 8 项月形循环，而本地 `Phase` 表示传统月相日名，二者不是同一张表，暂不在本任务中混合处理。

## 子任务拆分

1. 为 `Element` 增加固定五行表：木、火、土、金、水。
2. 为 `Element` 增加 `from_index`、`from_name`、`index`。
3. 为 `Duty` 增加 `from_index`、`from_name`、`index`，并复用现有 `lunar_util::tables::ZHI_XING`。
4. 为 `Element` / `Duty` 实现 `CycleItem`。
5. 补充 typed API 测试，覆盖五行方向关系、五行回绕、建除回绕和 `Lunar::zhi_xing_info()` 反查。

## 实现范围

- `src/culture.rs`
  - 新增 `ELEMENT_NAMES`。
  - 扩展 `Element` / `Duty` 构造和索引 API。
  - 实现 `CycleItem for Element` 与 `CycleItem for Duty`。
- `src/lunar.rs`
  - 新增 `zhi_xing_info()`，并让既有 `duty()` 复用 typed companion。
- `tests/phase2_typed.rs`
  - 覆盖 `Element::from_name`、`Element::from_index`、`Duty::from_name`、`CycleItem::next`。
- `docs/next-iteration-index.md`
  - 登记 `Task 79` 状态。

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

下一步可单独分析本地 `Phase` 与 `tyme4rs::Phase` 的语义差异，再决定是否新增独立月形对象，避免把传统月相日名误当作 8 项月形循环。
