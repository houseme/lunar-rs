# Task 84: God 神煞大表反查与步进 API

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 中 `God` 是基于 `GOD_NAMES` 的固定神煞循环对象，并按序号 `< 60` 判定吉神，其余为凶煞。本地 `lunar-rs` 已有 `God { name, luck }`，用于 `Lunar::gods()` 返回每日吉神凶煞，但此前只能通过 `new` 构造，缺少从固定表名称反查、索引和步进的能力。

由于本地 `God` 当前持有 `String`，而 `CycleItem` 要求 `Copy`，本任务不强行改结构或实现 `CycleItem`，而是新增显式的表驱动 API：`from_index`、`from_name`、`index`、`next` 和 `size`。

## 子任务拆分

1. 为 `God` 增加 `from_index`，按 `SHEN_SHA` 表回绕并自动判定吉凶。
2. 为 `God` 增加 `from_name`，支持从神煞名称反查。
3. 为 `God` 增加 `index`，对表内名称返回 `Some(index)`，对自定义名称返回 `None`。
4. 为 `God` 增加 `next` 和 `size`，支持表内对象显式步进。
5. 补充 typed API 测试，覆盖吉神、凶煞、自定义名称和步进。

## 实现范围

- `src/culture.rs`
  - 扩展 `God` 表驱动 API。
- `tests/phase2_typed.rs`
  - 覆盖 `God::from_name`、`God::index`、`God::next` 和自定义名称边界。
- `docs/next-iteration-index.md`
  - 登记 `Task 84` 状态。

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

下一步可处理 `Taboo` 事项大表反查；它与 `God` 类似，但本地还携带 `TabooKind`，需要决定 `from_name` 是否要求传入 kind，或提供默认/显式两套构造。
