# Task 62: typed SixtyCycle year/month/day/hour objects

状态：已完成
最近更新：2026-07-07

## 背景

`tyme4rs` 把年、月、日、时的干支层拆成 `SixtyCycleYear`、`SixtyCycleMonth`、`SixtyCycleDay`、`SixtyCycleHour` 等独立对象。`lunar-rs` 已有 `SixtyCycle` 和大量字符串 getter，本任务在不重写算法的前提下补可组合对象。

## 范围

- 新增 `SixtyCycleYear`、`SixtyCycleMonth`、`SixtyCycleDay`、`SixtyCycleHour`。
- 新增 `ThreePillars`。
- 给 `Lunar`、`LunarTime`、`EightChar` 增加 typed pillar 出口。
- 增加测试，确保 typed object 与原 getter 输出一致。

## 非范围

- 不改变八字 sect 默认口径。
- 不新增反推或 provider 逻辑。
- 不重算任何干支算法。

## 验收标准

- `cargo fmt` 通过。
- `cargo test --test eight_char` 通过。
- `cargo test --test phase2_typed` 通过。

## 验证记录

- 2026-07-07：`cargo fmt` 通过。
- 2026-07-07：`cargo test --test eight_char` 通过，3 个测试全部通过。
- 2026-07-07：`cargo test --test phase2_typed` 通过，13 个测试全部通过。
