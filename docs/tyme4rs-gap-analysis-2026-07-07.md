# lunar-rs 对标 tyme4rs 差距结论

> 日期：2026-07-07
> 本地范围：`src/`、`tests/`、`docs/roadmap.md`、`docs/next-iteration-index.md`
> 对标来源：[`6tail/tyme4rs`](https://github.com/6tail/tyme4rs)，本次读取 `master` 分支源码 tarball

## 1. 结论先行

当前 `lunar-rs` 已经不再是昨天 roadmap 中的 Phase 4 起步状态。按今天代码看，它已经完成大量多历法扩展、typed 文化对象、事件聚合、事件索引缓存、`i18n` 与 `serde` 基础能力。

与 `tyme4rs` 对比，剩余差距不主要在“历法数量”，而在以下四类：

1. **公共抽象层仍偏薄**
   - `tyme4rs` 有 `Culture`、`Tyme`、`LoopTyme`、`AbstractCultureDay` 这样的统一基础抽象。
   - `lunar-rs` 目前已有 `CalendarDay` / `CalendarSpan`，但文化循环对象仍是各自实现，缺少统一的 `next`、`steps_to`、`from_name`、`from_index` 语义层。

2. **细粒度文化日对象还未完全对象化**
   - `tyme4rs` 有 `SolarTermDay`、`PhenologyDay`、`PhaseDay`、`NineDay`、`HideHeavenStemDay`、`DogDay`、`PlumRainDay` 等“某文化概念第几天”的对象。
   - `lunar-rs` 已有 `DogDay`、`PlumRainDay`、`Fu`、`ShuJiu`，但节气日、物候日、月相日、九日、藏干日等还没有统一形态。

3. **命理/干支对象层仍有缺口**
   - `tyme4rs` 把 `SixtyCycleYear`、`SixtyCycleMonth`、`SixtyCycleDay`、`SixtyCycleHour`、`ThreePillars` 独立成对象。
   - `lunar-rs` 已有 `SixtyCycle` 与 `EightChar`，但年/月/日/时干支对象仍主要从 `Lunar` / `LunarTime` getter 中派生，没有独立可组合对象。

4. **规则化事件与 provider 化计算还未对齐**
   - `tyme4rs` 的 `Event` 支持压缩数据、按规则计算节日日期，并提供 `EventBuilder` / `EventManager`。
   - `tyme4rs` 的八字和童限计算有 provider trait，可切换算法口径。
   - `lunar-rs` 当前事件模型更适合直接聚合输出，已经有索引缓存和查询，但还不是规则 DSL；八字/运程口径也还不是 provider 化。

## 2. 已经达到或超过 tyme4rs 的部分

| 领域 | 当前判断 |
| --- | --- |
| 多历法数量 | `lunar-rs` 已接入 Hijri、RabByung、Minguo、ThaiSolar、Japanese、Juche、Dangi、Julian、Holocene、Byzantine、Coptic、Ethiopian、Armenian、AUC、Assyrian、HispanicEra、Saka、Bengali、Koki、ThaiBuddhist、Fasli、Nanakshahi、Seleucid、Rattanakosin、Venetian、Rumi、AnnoLucis 等，数量已经明显更广。 |
| 事件聚合 | `lunar-rs` 已有 `Event`、`EventKind`、`EventQuery`、日/周分组、范围扫描、索引缓存，比 `tyme4rs` 的通用事件规则不同，但已经可用于日历 UI/API 输出。 |
| 工程约束 | `lunar-rs` 使用 `forbid(unsafe_code)`、可选 `serde`、零默认依赖、`cargo test` 测试矩阵，工程约束更偏 Rust crate 发布维护。 |
| typed 文化对象 | 已有 `Direction`、`Element`、`Zodiac`、`Duty`、`Phase`、`Phenology`、`HeavenStem`、`EarthBranch`、`SixtyCycle`、`God`、`Taboo`、`PengZu`、`TianShen`、`Xiu`、`Lu`、`ChongSha`、`Xun`、`TaiSuiPosition`、`Nayin` 等第一批和第二批对象。 |

## 3. 仍缺的功能清单

### F1. 基础抽象：Culture / Tyme / LoopTyme 等价层

目标不是照搬 `tyme4rs` 的继承式结构，而是在 `lunar-rs` 中补一个轻量 trait 层：

- `NamedCulture`：统一 `name()`；
- `CycleItem`：统一 `index()`、`size()`、`next()`、`steps_to()`；
- `CultureDay`：统一 `name()` + `day_index()`。

优先级：P1。

### F2. 星座 Constellation typed API

`tyme4rs` 有 `Constellation` 对象；`lunar-rs` 当前只有 `Solar::xing_zuo() -> &str`。

本轮优先补：

- `culture::Constellation`；
- `Solar::constellation()`；
- 保留 `Solar::xing_zuo()` 兼容 API；
- 增加 typed 测试。

优先级：P1。适合作为第一步小任务。

### F3. 胎神 Fetus typed API

`tyme4rs` 有：

- `FetusDay`
- `FetusHeavenStem`
- `FetusEarthBranch`
- `FetusMonth`

`lunar-rs` 当前有胎神/胎元相关字符串能力，但没有按天干、地支、月、日拆成稳定对象。

优先级：P2。

### F4. 小六壬 MinorRen

`tyme4rs` 有 `MinorRen`；`lunar-rs` 当前有 `LiuYao`、`LiuYue`、`LiuNian` 等，但没有完整小六壬对象。

优先级：P2。

### F5. SixtyCycle 年/月/日/时对象

需要补：

- `SixtyCycleYear`
- `SixtyCycleMonth`
- `SixtyCycleDay`
- `SixtyCycleHour`
- 可选 `ThreePillars`

它们应包裹已有 `Lunar` / `LunarTime` 计算，不重写底层算法。

优先级：P2。

### F6. SolarTermDay / PhenologyDay / PhaseDay 等文化日对象

优先补统一形态：

- `SolarTermDay`
- `PhenologyDay`
- `PhaseDay`
- `NineDay`
- `HideHeavenStemDay`

这些对象应与现有 `DogDay`、`PlumRainDay`、`Fu`、`ShuJiu` 保持 API 形状一致。

优先级：P2。

### F7. 事件规则 DSL / EventManager

`lunar-rs` 当前事件系统偏“已知事件聚合 + 查询索引”；`tyme4rs` 的 `EventManager` 则偏“规则数据驱动”。

建议后置实现：

- 先定义规则模型；
- 支持公历固定日、农历固定日、公历第 N 个星期、节气偏移、节气后天干/地支；
- 再考虑压缩编码兼容层。

优先级：P3。

### F8. EightChar provider 化

补 `EightCharProvider` / `ChildLimitProvider` 等口径切换接口，避免晚子时、童限、流派差异继续塞进单一实现。

优先级：P3。

## 4. 本轮可执行子任务拆分

| 任务 | 名称 | 范围 | 验收 |
| --- | --- | --- | --- |
| Task 58 | typed `Constellation` API | 增加 `Constellation` 对象、`Solar::constellation()`、测试、导出 | `cargo test --test phase2_typed` 通过 |
| Task 59 | 轻量 Culture/Cycle trait 层 | 增加 `NamedCulture`、`CycleItem`、`CultureDay`，先给核心 typed 对象实现 | 现有 API 不破坏，新增 trait 测试通过 |
| Task 60 | Fetus typed API | 胎神天干/地支/月/日对象化 | 与现有字符串 getter 对齐 |
| Task 61 | MinorRen 小六壬 | 新增小六壬对象和 Lunar 入口 | 参考日期测试通过 |
| Task 62 | SixtyCycle 年/月/日/时对象 | 包裹现有干支计算，补可组合对象 | 与 `EightChar` / `LunarTime` 输出一致 |
| Task 63 | 文化日对象统一 | 补 SolarTermDay、PhenologyDay、PhaseDay 等 | 形状与 Fu/ShuJiu/DogDay 一致 |
| Task 64 | EventManager 规则模型 | 增加规则化节日事件计算 | 固定日/星期/节气偏移用例通过 |
| Task 65 | EightChar provider 化 | 抽象八字/童限算法 provider | 默认行为不变，新增 provider 测试 |

## 5. 执行建议

先做 Task 58，因为它：

- 改动小；
- 与 `tyme4rs` 的 `Constellation` 缺口一一对应；
- 不需要引入新算法；
- 能继续验证 typed API 扩展模式。

随后做 Task 59，把 `Constellation`、`Zodiac`、`Element`、`Duty`、`Phase` 等对象统一到轻量 trait 层，再继续推进胎神、小六壬和干支年/月/日/时对象。
