# tyme4rs v1.5 对标结论（2026-07-07）

基准源码：

- 上游：`6tail/tyme4rs`
- 拉取位置：`/private/tmp/tyme4rs-latest`
- commit：`ba6ab75`
- 最近提交：`2026-06-15 21:22:53 +0800`，`v1.5.0 移除节日类型FestivalType；新增回历；优化代码和算法。`

## 总体结论

本地 `lunar-rs` 已完成大多数主干能力：公历/农历核心、节气、宜忌、吉神凶煞、三伏/数九/物候、八字 provider、童限/大运/流年、Hijri、RabByung、多历法生态、事件模型、i18n/serde 基础，以及 Task 58-85 的 typed 文化对象补强。

继续对标 `tyme4rs v1.5`，剩余缺口不再是大面积缺功能，而是“对象形态和少量语义精度”：

1. **星曜 typed 对象缺口**：`tyme4rs` 有 `SixStar`、`SevenStar`、`TwelveStar`、`Ecliptic`、`TenStar` 等纯循环对象；本地已有部分字符串能力或相邻对象，但缺少同等 typed wrapper。
2. **8 相天文月相缺口**：`tyme4rs::Phase` 是 `新月/蛾眉月/上弦月/盈凸月/满月/亏凸月/下弦月/残月` 8 项，并能计算开始的公历日/时；本地 `Phase` 是农历日月相名称（如 `朔`、`既朔`、`望`），语义不同，不能直接改名覆盖。
3. **年月周单位对象形态差异**：`tyme4rs` 有 `SolarYear`、`SolarHalfYear`、`SolarSeason`、`SolarMonth`、`SolarWeek`、`LunarWeek`、`YearUnit/MonthUnit/DayUnit/SecondUnit/WeekUnit` 等更细对象；本地有部分同名/近似对象，但 API 形态不同，优先级低于文化对象。
4. **事件和节日模型方向不同**：`tyme4rs v1.5` 移除了 `FestivalType` 并保留 `EventType`；本地事件模型已扩展为 `EventKind/EventSource/EventRule/EventQuery`，功能上更强，但不是同构 API。
5. **新增回历已覆盖**：上游 v1.5 的“新增回历”在本地已有 `HijriYear/HijriMonth/Hijri`，并已有范围、事件与测试覆盖。

## 子任务拆分

| 优先级 | 任务编号 | 任务名称 | 状态 | 说明 |
| --- | --- | --- | --- | --- |
| `P2` | `Task 86` | 星曜循环对象一次性补齐 | `已完成` | 新增 `SixStar`、`SevenStar`、`Ecliptic`、`TwelveStar`、`TenStar` typed API。 |
| `P2` | `Task 87` | 8 相天文月相对象设计与实现 | `已完成` | 新增独立 `MoonPhase` / `MoonPhaseDay`，避免破坏现有农历日月相 `Phase`。 |
| `P3` | `Task 88` | 年月周单位对象 API 差异补齐 | `已完成` | 新增 `YearUnit/MonthUnit/DayUnit/SecondUnit/WeekUnit` 与 `LunarWeek`。 |
| `P3` | `Task 89` | 事件/节日 v1.5 API 兼容层评估 | `未开始` | 明确 `EventType` 与本地 `EventKind` 的关系。 |

## 实施顺序

1. 先实现 `Task 86`：表驱动、低风险、能一次性闭环。
2. 再实现 `Task 87`：需要单独处理天文月相和现有 `Phase` 语义冲突。
3. 最后处理 `Task 88` / `Task 89`：更多是 API 形态和兼容层决策。

## 当前建议

`Task 86`、`Task 87` 与 `Task 88` 已完成。本地继续保留 `phase()` 的旧含义，并通过 `moon_phase()` / `moon_phase_day()` 暴露对标 `tyme4rs::Phase` 的 8 相天文月相；年月周差异已补 `Unit` 对象族与 `LunarWeek`。后续进入 `Task 89`，重点是事件/节日 v1.5 API 兼容评估。
