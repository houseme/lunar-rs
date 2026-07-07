# Task 60: Fetus typed API

状态：已完成
最近更新：2026-07-07

## 背景

`tyme4rs` 将胎神拆为 `FetusDay`、`FetusHeavenStem`、`FetusEarthBranch`、`FetusMonth`。`lunar-rs` 已有日胎神和月胎神文本，本任务在不改变原 getter 的前提下补 typed object。

## 范围

- 新增 `FetusHeavenStem`。
- 新增 `FetusEarthBranch`。
- 新增 `FetusDay`。
- 新增 `FetusMonth`。
- `FetusHeavenStem` / `FetusEarthBranch` 实现 `CycleItem`。
- 给 `Lunar` 新增 `fetus_day()` 与 `fetus_month()`。

## 非范围

- 不改变现有 `day_tai_position()` / `month_tai_position()`。
- 不调整胎神表数据。
- 暂不新增 i18n 翻译表。

## 验收标准

- `cargo fmt` 通过。
- `cargo test --test phase2_typed` 通过。

## 验证记录

- 2026-07-07：`cargo fmt` 通过。
- 2026-07-07：`cargo test --test phase2_typed` 通过，13 个测试全部通过。
