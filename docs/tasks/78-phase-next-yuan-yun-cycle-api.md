# Task 78: YuanCycle / YunCycle 三元九运循环对象

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 中 `Sixty` 表示三元，固定 3 项：上元、中元、下元；`Twenty` 表示九运，固定 9 项：一运到九运，并提供 `Twenty -> Sixty` 的归属关系。本地 `lunar-rs` 已有 `YuanCycle` / `YunCycle` typed wrapper，但此前只是字符串包装，缺少固定循环、名称反查和九运到三元的关系。

本任务将 `YuanCycle` / `YunCycle` 补强为固定索引对象，继续复用 `NamedCulture` / `CycleItem` 模式，并保留既有 `new(...)` 构造入口。

## 子任务拆分

1. 增加固定三元表：上元、中元、下元。
2. 增加固定九运表：一运到九运。
3. 将 `YuanCycle` / `YunCycle` 改为索引型 typed object，并提供 `new`、`from_index`、`from_name`、`index`、`name`。
4. 为 `YuanCycle` / `YunCycle` 实现 `CycleItem`。
5. 为 `YunCycle` 增加 `yuan_cycle()`，按 `index / 3` 返回归属三元。
6. 补充 typed API 测试，确认 `LunarYear` legacy 输出、名称反查、循环回绕和九运归属三元一致。

## 实现范围

- `src/culture.rs`
  - 新增 `YUAN_CYCLE_NAMES`、`YUN_CYCLE_NAMES`。
  - 补强 `YuanCycle` / `YunCycle` 固定循环语义。
  - 实现 `CycleItem for YuanCycle` 与 `CycleItem for YunCycle`。
- `tests/phase2_typed.rs`
  - 覆盖三元九运反查、回绕和关系映射。
- `docs/next-iteration-index.md`
  - 登记 `Task 78` 状态。

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

下一步可继续梳理 `tyme4rs` 中命理对象的组合能力，优先选择不会改变现有计算结果、只增强 typed relation 的切片。
