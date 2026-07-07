# Task 66: EventRule 节气后天干 / 地支规则

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs` 的 `EventBuilder` 支持：

- `term_heaven_stem(term_index, heaven_stem_index, delay_days)`
- `term_earth_branch(term_index, earth_branch_index, delay_days)`

这类规则用于“从某个节气偏移若干天后，继续寻找第一个指定天干 / 地支日”的事件，例如入伏、出梅等。

本仓库 `Task 64` 已有固定公历日、固定农历日、星期规则、节气偏移规则，但还没有节气后天干 / 地支搜索规则。

## 实现内容

- 扩展 `EventRule`：
  - `SolarTermHeavenStem`
  - `SolarTermEarthBranch`
- 新增构造器：
  - `EventRule::solar_term_heaven_stem`
  - `EventRule::solar_term_earth_branch`
- 规则语义：
  - 先取指定节气日期；
  - 再应用 `search_start_offset_days` 得到搜索起点；
  - 再向后找到第一个目标天干 / 地支日；
  - 最后仍可通过 `with_offset_days` 做最终日期偏移。

## 验收标准

- `EventRule::solar_term_heaven_stem("夏至", 庚, 20)` 在 2024 年解析到 `2024-07-15`；
- `EventRule::solar_term_earth_branch("小暑", 未, 0)` 在 2024 年解析到 `2024-07-06`；
- 解析结果对应农历日的天干 / 地支与目标一致；
- 现有 EventManager 聚合和查询行为不回退。

## 验证记录

- `cargo fmt`
- `cargo test --test phase3_events`
