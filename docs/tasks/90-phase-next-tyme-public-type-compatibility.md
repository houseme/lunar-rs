# Task 90: tyme4rs 公开类型名兼容补强

状态：已完成
最近更新：2026-07-07

## 对标结论

重新拉取 `6tail/tyme4rs` 后，上游仍停留在 `ba6ab75`：

- 最近提交时间：`2026-06-15 21:22:53 +0800`
- 最近提交说明：`v1.5.0 移除节日类型FestivalType；新增回历；优化代码和算法。`

在 `Task 86` - `Task 89` 完成后，剩余差异主要不是算法功能缺失，而是公开类型名迁移体验：

- `Animal` 对应本地 `XiuAnimal`。
- `Luck` 对应本地 `GodLuck`。
- `Sixty` 对应本地 `YuanCycle`。
- `Sound` 对应本地 `Nayin`。
- `Ten` 对应本地 `Xun`。
- `Twenty` 对应本地 `YunCycle`。
- `Dipper` 需要新增独立循环对象，用于北斗九星名称。

## 子任务拆分

1. 新增 `Dipper` 循环对象。
2. 为 `Dipper` 接入 `NamedCulture` 与 `CycleItem`。
3. 为 `NineStar` 增加 `dipper()`。
4. 在 crate 根新增 tyme4rs 兼容 type alias：`Animal`、`Luck`、`Sixty`、`Sound`、`Ten`、`Twenty`。
5. 补充 typed 测试，验证兼容类型名可直接按 tyme4rs 名称使用。

## 验证计划

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

本任务完成后，tyme4rs v1.5 的主要公开文化对象、月相、单位对象、事件规则类型和轻量迁移别名均已补齐。后续如果继续推进，应转向差分测试矩阵、README/API 文档或严格方法名兼容层。
