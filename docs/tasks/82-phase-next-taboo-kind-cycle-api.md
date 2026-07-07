# Task 82: TabooKind 宜忌类型循环对象补强

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 中 `Taboo` 是固定宜忌事项名称表上的循环对象。本地 `lunar-rs` 已把宜忌事项建模为 `Taboo { name, kind }`，其中 `TabooKind` 表示 `宜` / `忌` 两类语义。直接把本地 `Taboo` 改成无 kind 的大表循环对象会丢失现有 API 语义，因此本任务先补强 `TabooKind` 这个轻量类型层。

本任务保持 `Taboo` 结构和 `day_recommends`、`day_avoids`、`time_recommends`、`time_avoids` 行为不变，只补充 `TabooKind` 的名称、索引和循环能力，为后续大表 `Taboo` 反查单独切片预留边界。

## 子任务拆分

1. 为 `TabooKind` 增加 `from_index`，按 `宜 -> 忌` 两项回绕。
2. 为 `TabooKind` 增加 `from_name`、`index` 和 `name`。
3. 将 `TabooKind` 纳入 `NamedCulture`。
4. 为 `TabooKind` 实现 `CycleItem`。
5. 补充 typed API 测试，覆盖名称反查、索引、泛型名称接口和循环回绕。

## 实现范围

- `src/culture.rs`
  - 扩展 `TabooKind` API。
  - 实现 `NamedCulture` 与 `CycleItem`。
- `tests/phase2_typed.rs`
  - 覆盖 `TabooKind::from_name`、`TabooKind::from_index`、`TabooKind::index`、`CycleItem::next`。
- `docs/next-iteration-index.md`
  - 登记 `Task 82` 状态。

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

下一步可以把 `Taboo` 的事项名称大表抽成稳定常量并增加 `from_name/index`；该任务会比本切片更大，需要确认本地表项是否与 `tyme4rs::TABOO_NAMES` 完全一致。
