# Task 83: KitchenGodSteed 灶马头 typed 聚合对象

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 已提供 `KitchenGodSteed`，用于从农历年的正月初一干支推导灶马头相关的十四项杂占：几鼠偷粮、草子几分、几牛耕田、花收几分、几龙治水、几马驮谷、几鸡抢米、几姑看蚕、几屠共猪、甲田几分、几人分饼、几日得金、几人几丙、几人几锄。

本地 `lunar-rs` 已在 `LunarYear` 上有这些分散字符串方法和 `YearFortune` wrapper，但缺少 `tyme4rs` 风格的聚合 typed 对象。本任务新增 `KitchenGodSteed`，并将 `ren_bing` / `ren_chu` 修正为与 `tyme4rs` 一致的“地支算人数、天干算丙/锄”。

## 子任务拆分

1. 新增 `KitchenGodSteed` typed 对象，保存农历年正月初一的天干、地支索引。
2. 实现 `name`、`Display` 和十四项杂占输出方法。
3. 在 `LunarYear` 上新增 `kitchen_god_steed()` 聚合入口。
4. 修正 `LunarYear::ren_bing` 与 `LunarYear::ren_chu` 的双占位计算。
5. 将 `KitchenGodSteed` 导出到 crate 根，并纳入 `NamedCulture`。
6. 补充测试，将新对象十四项输出逐一对齐既有 `LunarYear` 字符串方法。

## 实现范围

- `src/culture.rs`
  - 新增 `KitchenGodSteed` 类型。
  - 实现灶马头十四项输出。
- `src/lunar_year.rs`
  - 新增 `kitchen_god_steed()`。
  - 修正 `ren_bing()` 与 `ren_chu()`。
- `src/lib.rs`
  - 导出 `KitchenGodSteed`。
- `tests/phase2_typed.rs`
  - 覆盖聚合对象和既有方法一致性。
- `docs/next-iteration-index.md`
  - 登记 `Task 83` 状态。

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

剩余高价值对标项主要是 `God` / `Taboo` 大表循环反查，以及 `Phase` 的 8 相月相语义补齐。`Phase` 与本地现有月相日名称不同，建议单独设计新对象或明确兼容层后再实现。
