# Task 67: NineDay / HideHeavenStemDay 文化日对象

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 在文化日对象中提供：

- `Nine`
- `NineDay`
- `HideHeavenStemType`
- `HideHeavenStem`
- `HideHeavenStemDay`

其中 `NineDay` 表示数九第几天，`HideHeavenStemDay` 表示人元司令分野中的藏干、类型和天数。

本仓库此前已有 `ShuJiu` 数九对象，但缺少与 `tyme4rs` 同名的 `Nine / NineDay` 循环对象；也缺少藏干日对象。

## 实现内容

- 新增 `Nine` 与 `NineDay`：
  - 支持 `from_index`
  - 支持 `from_name`
  - 支持 `CycleItem`
  - `NineDay` 支持 `CultureDay`
- 新增藏干日对象：
  - `HideHeavenStemType`
  - `HideHeavenStem`
  - `HideHeavenStemDay`
- 新增 `Solar` 入口：
  - `Solar::nine_day`
  - `Solar::hide_heaven_stem_day`
- `HideHeavenStemDay` 复用 `tyme4rs` 的节气分野表和天数分段算法。

## 验收标准

- `Solar::from_ymd(2020, 12, 21).unwrap().nine_day()` 返回 `一九第1天`；
- `Solar::from_ymd(2021, 1, 8).unwrap().nine_day()` 返回 `三九第1天`；
- `Solar::from_ymd(2021, 7, 5).unwrap().nine_day()` 返回 `None`；
- `Solar::from_ymd(2024, 12, 4).unwrap().hide_heaven_stem_day()` 返回 `本气 / 壬 / 壬水第16天`。

## 验证记录

- `cargo fmt`
- `cargo test --test phase2_typed`
