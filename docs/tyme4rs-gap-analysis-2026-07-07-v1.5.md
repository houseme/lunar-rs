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
| `P3` | `Task 89` | 事件/节日 v1.5 API 兼容层评估 | `已完成` | 新增 `EventType` 并映射到本地 `EventRule`。 |
| `P3` | `Task 90` | tyme4rs 公开类型名兼容补强 | `已完成` | 新增 `Dipper`，并提供 `Animal/Luck/Sixty/Sound/Ten/Twenty` 兼容别名。 |
| `P2` | `Task 91` | 核心日时与儒略日命名兼容 | `已完成` | 新增 `JulianDay`，并提供 `SolarDay/SolarTime/LunarDay/LunarHour/SolarTerm/LegalHoliday` 兼容别名。 |
| `P2` | `Task 92` | EventBuilder 规则构造器兼容 | `已完成` | 新增 `EventBuilder` 与 `Event::builder()`，覆盖上游事件规则 builder 入口。 |
| `P2` | `Task 93` | tyme4rs 基础枚举兼容 | `已完成` | 新增 `Gender/Side/YinYang` typed API，并把 `YinYang` 挂接到天干地支。 |
| `P2` | `Task 94` | SolarTerm 对象入口兼容 | `已完成` | 为 `SolarTerm` 别名补齐 `from_index/from_name/next/is_jie/is_qi` 等对象 API。 |
| `P3` | `Task 95` | SolarDay/SolarTime 核心 getter 兼容 | `已完成` | 为 `Solar` 补齐 tyme4rs 风格 `get_*` 迁移入口。 |
| `P3` | `Task 96` | Solar 派生 getter 与公历聚合对象兼容 | `已完成` | 补齐 `Solar` 派生 getter、`LegalHoliday` getter、`HijriDay` 别名、`JulianDay` 差值，以及 `SolarYear/SolarHalfYear/SolarSeason/SolarMonth/SolarWeek` 聚合 getter。 |
| `P2` | `Task 97` | Lunar/LunarTime 严格方法名与节日 wrapper 兼容 | `已完成` | 新增 `SolarFestival/LunarFestival` wrapper，并补齐 `Lunar/LunarTime` 高频 `get_*` 迁移入口。 |
| `P2` | `Task 98` | LunarYear/LunarMonth 严格 getter 兼容 | `已完成` | 补齐 `LunarYear/LunarMonth` 高频 `get_*` 迁移入口，并保留本地 15 个月内部窗口语义。 |
| `P3` | `Task 99` | Foto/Tao wrapper 生命周期与 getter 兼容补强 | `已完成` | 本地扩展项；`tyme4rs v1.5` 无独立佛历/道历模块，本任务补齐本地 wrapper 的 owned 快照与 `get_*` 迁移入口。 |
| `P3` | `Task 100` | Foto/Tao 布尔规则 get_* 兼容补强 | `已完成` | 为 `Foto/Tao` 仍保留 `is_*` 的布尔规则补齐 `get_*` 兼容别名。 |
| `P3` | `Task 101` | README/API 迁移示例同步 | `已完成` | 在 `README.md` / `README_CN.md` 中补充 tyme4rs 风格迁移示例与语义差异说明。 |
| `P2` | `Task 102` | 差分协议与样例矩阵扩容 | `已完成` | 将差分协议升级到 v2，并把节日 wrapper、闰月与 `LunarYear/LunarMonth` 关键字段纳入 sample matrix。 |
| `P3` | `Task 103` | 日期键查找与时辰索引优化 | `已完成` | 收拢日期 key 查找与时辰索引热路径，降低字符串拼接分散度，作为兼容层落地后的本地优化。 |

## 实施顺序

1. 先实现 `Task 86`：表驱动、低风险、能一次性闭环。
2. 再实现 `Task 87`：需要单独处理天文月相和现有 `Phase` 语义冲突。
3. 最后处理 `Task 88` / `Task 89`：更多是 API 形态和兼容层决策。

## 当前建议

`Task 86`、`Task 87`、`Task 88`、`Task 89`、`Task 90`、`Task 91`、`Task 92`、`Task 93`、`Task 94`、`Task 95`、`Task 96`、`Task 97`、`Task 98`、`Task 99`、`Task 100`、`Task 101`、`Task 102` 与 `Task 103` 已完成。本地继续保留 `phase()` 的旧含义，并通过 `moon_phase()` / `moon_phase_day()` 暴露对标 `tyme4rs::Phase` 的 8 相天文月相；年月周差异已补 `Unit` 对象族与 `LunarWeek`；事件规则侧已补 `EventType` 与 `EventBuilder` 兼容层；公开类型名迁移侧已补 `Dipper` 和轻量 type alias；核心日时入口侧已补 `JulianDay` 与 `SolarDay/SolarTime/LunarDay/LunarHour/SolarTerm/LegalHoliday/HijriDay` 兼容别名，且 `SolarTerm` 已补对象构造与步进 API，`SolarDay/SolarTime/SolarYear/SolarHalfYear/SolarSeason/SolarMonth/SolarWeek/LunarDay/LunarHour/LunarYear/LunarMonth` 已补常用 `get_*` 迁移入口；节日对象侧已补 `SolarFestival/LunarFestival` wrapper；基础枚举侧已补 `Gender/Side/YinYang`，并保留旧整数性别入口；本地扩展的 `Foto/Tao` wrapper 已改为 owned 快照，并继续补齐 `get_*` 与布尔规则兼容别名，同时在 README 中补了可复制的迁移示例；差分协议现已升级到 v2，开始覆盖节日 wrapper 与闰月元数据，同时本地日期键查找与时辰索引热路径也已完成收拢。后续如继续推进，应进入外部 reference 同步、严格差分测试和 README/API 文档收尾。

## 2026-07-07 重新拉取复核

本轮重新执行 `git fetch --depth 1 origin master` 后，`FETCH_HEAD` 仍为 `ba6ab75`，上游 v1.5 基准未变化。因此本文件中的核心对标结论仍有效；本轮新增的补强点是严格公开类型名兼容，而非新上游功能。
