# Task 65: EightChar provider 化

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 将八字生成策略抽象为 `EightCharProvider`，默认 provider 与不同流派 provider 可以替换。

本仓库此前 `Lunar::eight_char()` 固定返回 `sect = 2` 的 `EightChar` 视图；调用方可以再手动 `set_sect`，但没有 provider 扩展点。对标差距主要在“生成策略可替换”而不是四柱基础计算本身。

## 实现内容

- 新增 `EightCharProvider` trait：
  - `sect(&self, lunar: &Lunar) -> u8`
  - `eight_char(&self, lunar: &Lunar) -> EightChar`
- 新增 provider：
  - `DefaultEightCharProvider`
  - `LunarSect1EightCharProvider`
  - `LunarSect2EightCharProvider`
- 新增 `Lunar::eight_char_with_provider`。
- 保持 `Lunar::eight_char()` 默认行为不变，继续返回 `sect = 2`。
- 公开导出 provider trait 与 provider 类型。

## 验收标准

- 默认 provider 与现有 `Lunar::eight_char()` 输出一致；
- `LunarSect1EightCharProvider` 与手动 `set_sect(1)` 输出一致；
- `LunarSect2EightCharProvider` 可通过 trait object 调用；
- 不破坏既有 `EightChar` / `Yun` 相关测试。

## 验证记录

- `cargo fmt`
- `cargo test --test eight_char`
