# Task 63: culture day object unification

状态：已完成
最近更新：2026-07-07

## 背景

`tyme4rs` 为节气日、候日、月相日等提供了独立对象。`lunar-rs` 已有三伏、数九、梅雨等 day-level 对象，本任务继续把节气、物候、月相接入同一形态。

## 范围

- 新增 `SolarTermDay`。
- 新增 `PhenologyDay`。
- 新增 `PhaseDay`。
- 三者均实现 `NamedCulture` 与 `CultureDay`。
- 给 `Lunar` 增加 `solar_term_day()`、`phenology_day()`、`phase_day()`。

## 非范围

- 暂不实现 `NineDay` / `HideHeavenStemDay`。
- 不改变现有 `hou()`、`wu_hou()`、`phase()` 文本。
- 不调整节气计算算法。

## 验收标准

- `cargo fmt` 通过。
- `cargo test --test phase2_typed` 通过。

## 验证记录

- 2026-07-07：`cargo fmt` 通过。
- 2026-07-07：`cargo test --test phase2_typed` 通过，13 个测试全部通过。
