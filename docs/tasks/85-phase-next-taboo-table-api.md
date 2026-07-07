# Task 85: Taboo 宜忌事项大表反查与步进 API

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 中 `Taboo` 是基于固定 `TABOO_NAMES` 的宜忌事项循环对象。本地 `lunar-rs` 已有 `Taboo { name, kind }`，并通过 `TabooKind` 保留 `宜` / `忌` 上下文，因此不能简单退化成只有名称的无类型循环对象。

本任务复用本地 `lunar_util::tables::YI_JI` 大表，为 `Taboo` 增加显式表驱动 API，同时保留 `TabooKind`：`from_index(index, kind)`、`from_name(name, kind)`、`index()`、`next(offset)` 和 `size()`。

## 子任务拆分

1. 为 `Taboo` 增加 `from_index`，按 `YI_JI` 表回绕并携带传入的 `TabooKind`。
2. 为 `Taboo` 增加 `from_name`，支持事项名称反查。
3. 为 `Taboo` 增加 `index`，对表内名称返回 `Some(index)`，对自定义名称返回 `None`。
4. 为 `Taboo` 增加 `next` 和 `size`，步进时保留当前 `TabooKind`。
5. 补充 typed API 测试，覆盖推荐事项、步进保留 kind、自定义名称边界。

## 实现范围

- `src/culture.rs`
  - 扩展 `Taboo` 表驱动 API。
- `tests/phase2_typed.rs`
  - 覆盖 `Taboo::from_name`、`Taboo::index`、`Taboo::next` 和自定义名称边界。
- `docs/next-iteration-index.md`
  - 登记 `Task 85` 状态。

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

剩余最敏感的对标项是 `Phase`。`tyme4rs::Phase` 是 8 相月相循环，而本地 `Phase` 表示农历日月相名称；建议单独设计兼容对象，避免破坏现有语义。
