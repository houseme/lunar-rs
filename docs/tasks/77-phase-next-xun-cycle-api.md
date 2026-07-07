# Task 77: Xun 旬循环对象补强

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 中 `Ten` 表示旬，固定 6 项：甲子、甲戌、甲申、甲午、甲辰、甲寅，并支持按序号、名称和循环步进访问。本地 `lunar-rs` 已有 `Xun` 与 `XunKong` typed wrapper，且年、月、日、时都能返回旬对象，但此前 `Xun` 还缺少 `from_index`、`from_name`、`index` 与通用 `CycleItem` 能力。

本任务将 `Xun` 从 getter 包装对象补强为 6 项循环对象，同时继续复用现有 `lunar_util::tables::XUN` / `XUN_KONG`，避免新增重复表。

## 子任务拆分

1. 为 `Xun` 增加 `from_index`，按旬表同步构造 `XunKong`。
2. 为 `Xun` 增加 `from_name` 与 `index`。
3. 为 `Xun` 实现 `CycleItem`，复用 `next`、`steps_to`、`steps_back_to`、`steps_close_to`。
4. 补充 typed API 测试，确认 legacy getter 结果可反查、可按 index 重建，并能在尾部回绕。

## 实现范围

- `src/culture.rs`
  - 扩展 `Xun` 构造和索引 API。
  - 实现 `CycleItem for Xun`。
- `tests/phase2_typed.rs`
  - 覆盖 `Xun::from_name`、`Xun::from_index`、`Xun::index`、`CycleItem::next`。
- `docs/next-iteration-index.md`
  - 登记 `Task 77` 状态。

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

下一步可继续对齐 `tyme4rs` 的 `Sixty` / `Twenty`，将本库当前字符串型 `YuanCycle` / `YunCycle` 补强为固定循环对象，并补 `YunCycle -> YuanCycle` 的关系。
