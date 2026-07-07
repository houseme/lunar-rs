# Task 59: light Culture and cycle trait layer

状态：已完成
最近更新：2026-07-07

## 背景

`tyme4rs` 通过 `Culture`、`Tyme`、`LoopTyme` 和 `AbstractCultureDay` 让大量文化对象具备统一访问形状。`lunar-rs` 已有很多 typed 对象，但它们主要依靠各自的 inherent methods。

## 范围

- 新增 `NamedCulture`，统一 `name()`。
- 新增 `CycleItem`，统一 `index()`、`size()`、`next()`、`steps_to()`、`steps_back_to()`、`steps_close_to()`。
- 新增 `CultureDay`，统一“某文化概念第几天”的 `day_index()`。
- 给现有 typed 对象、三伏、数九、梅雨日、伏日接入 trait。

## 非范围

- 不引入 `tyme4rs` 的继承式 `Abstract*` 类型。
- 不改变任何旧 getter 或显示文本。
- 不强制所有历史类型一次性实现 trait。

## 验收标准

- `cargo fmt` 通过。
- `cargo test --test phase2_typed` 通过。

## 验证记录

- 2026-07-07：`cargo fmt` 通过。
- 2026-07-07：`cargo test --test phase2_typed` 通过，13 个测试全部通过。
