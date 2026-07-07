# Task 58: typed Constellation API

状态：已完成
最近更新：2026-07-07

## 背景

`tyme4rs` 在 `culture` 模块中有 `Constellation` 对象。当前 `lunar-rs` 的星座能力主要通过 `Solar::xing_zuo() -> &'static str` 暴露，缺少 typed API。

## 范围

- 新增 `culture::Constellation`。
- 从 `lib.rs` 导出 `Constellation`。
- 新增 `Solar::constellation() -> Constellation`。
- 保留 `Solar::xing_zuo()` 兼容行为。
- 增加 typed 测试，确保新对象与旧 getter 一致。

## 非范围

- 不重写星座边界算法。
- 不改变 `to_full_string()` 输出。
- 不调整 i18n 表结构，只复用现有 `i18n::constellation(...)`。

## 验收标准

- `cargo fmt` 通过。
- `cargo test --test phase2_typed` 通过。
- 如时间允许，运行完整 `cargo test`。

## 验证记录

- 2026-07-07：`cargo fmt` 通过。
- 2026-07-07：`cargo test --test phase2_typed` 通过，12 个测试全部通过。
- 2026-07-07：`cargo test` 通过，包含完整集成测试与 1 个 doctest。
