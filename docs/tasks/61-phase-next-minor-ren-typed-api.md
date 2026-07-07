# Task 61: 小六壬 typed API

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 提供 `MinorRen` 小六壬文化对象，并在农历月、农历日、农历时三个层级提供计算入口：

- 农历月：以农历月序从 `大安` 起六位循环；
- 农历日：在当月小六壬基础上推进 `day - 1`；
- 农历时：在当日小六壬基础上推进时辰地支序；
- 对象还暴露名称、吉凶与五行。

本仓库此前没有小六壬 typed API，对标缺口属于 Phase 2 文化对象补全范围。

## 实现内容

- 新增 `MinorRen` typed object：
  - `from_index`
  - `from_name`
  - `index`
  - `name`
  - `luck`
  - `element`
- 接入通用 trait：
  - `NamedCulture`
  - `CycleItem`
- 新增公开导出：
  - `lunar_rs::MinorRen`
- 新增农历访问入口：
  - `Lunar::month_minor_ren`
  - `Lunar::day_minor_ren`
  - `Lunar::time_minor_ren`
  - `Lunar::minor_ren`
  - `LunarMonth::minor_ren`
  - `LunarTime::minor_ren`

## 验收标准

- `MinorRen::from_name("空亡").unwrap().next(1).name()` 返回 `大安`；
- `Lunar::from_ymd(2024, 3, 5).unwrap().minor_ren().name()` 返回 `大安`；
- `Lunar::from_ymd_hms(2024, 9, 7, 10, 0, 0).unwrap().time_minor_ren().name()` 返回 `留连`；
- `LunarMonth::from_ym(1991, 3).unwrap().minor_ren().name()` 返回 `速喜`；
- 吉凶和五行与 `tyme4rs` 的 `Luck::from_index(index % 2)`、`[0,4,1,3,0,2]` 五行映射一致。

## 验证记录

- `cargo fmt`
- `cargo test --test phase2_typed`
