# Task 81: Zodiac 生肖循环对象补强

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 中 `Zodiac` 是固定 12 项生肖循环对象，支持按序号、名称和循环步进访问。本地 `lunar-rs` 已有 `Zodiac` typed 对象，并已接入 `NamedCulture` 与 `CycleItem`，但此前只暴露 `new` 与 `name`，缺少显式的 `from_index`、`from_name` 和 `index` API。

本任务补齐这些轻量构造与反查能力，继续复用现有 `lunar_util::tables::SHENG_XIAO` 表，不新增重复常量。

## 子任务拆分

1. 为 `Zodiac` 增加 `from_index`，按 12 生肖回绕。
2. 为 `Zodiac` 增加 `from_name`，支持名称反查。
3. 为 `Zodiac` 增加 `index`，返回 0 基循环序号。
4. 将 `CycleItem for Zodiac` 改为复用新增 inherent API。
5. 补充 typed API 测试，覆盖名称反查、索引和循环回绕。

## 实现范围

- `src/culture.rs`
  - 扩展 `Zodiac` API。
  - 收敛 `CycleItem for Zodiac` 的实现路径。
- `tests/phase2_typed.rs`
  - 覆盖 `Zodiac::from_name`、`Zodiac::from_index`、`Zodiac::index` 和 `CycleItem::next`。
- `docs/next-iteration-index.md`
  - 登记 `Task 81` 状态。

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

下一步可转向 `Taboo`、`God`、`Phase` 或 `KitchenGodSteed` 这类剩余对象；其中 `Phase` 与 `tyme4rs` 的定义存在语义差异，应优先单独建模或继续保留设计备注，避免把月相日名称直接等同为 8 相循环。
