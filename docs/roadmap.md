# lunar-rs 下一步功能 Roadmap

> 产出时间：2026-07-06  
> 核查范围：`docs/DESIGN.md`、`README.md`、`README_CN.md`、`CHANGELOG.md`、`src/`、`tests/`  
> 参考项目：[`6tail/tyme4rs`](https://github.com/6tail/tyme4rs)

---

## 1. 结论先行

### 1.1 执行状态（2026-07-06 最新）

| 阶段 | 状态 | 说明 |
| --- | --- | --- |
| `Phase 0` | `已完成` | 文档状态、验证口径、bench 命令入口已同步。 |
| `Phase 1` | `进行中` | `serde`、benchmark、holiday override、测试拆分、差分协议已落地；`i18n` 已启动基础骨架。 |
| `Phase 2` | `进行中` | 第一批 typed API 已启动，已新增干支/生肖/方位/五行/六十甲子，以及 Duty / Phase / Phenology 等基础文化对象。 |
| `Phase 3` | `进行中` | `Event` 已具备统一事件出口、跨历法聚合、排序/去重、过滤查询，以及 `priority` / `source_id` / `is_observed` 规则层。 |
| `Phase 4` | `未开始` | `Hijri` / `RabByung` 等多历法扩展尚未开始。 |
| `Phase 5` | `未开始` | 反推与高级命理工具尚未开始。 |

当前 `lunar-rs` 的状态，已经明显领先于 `docs/DESIGN.md` 与 README/CHANGELOG 中描述的“仍在对齐参考实现”的阶段：

- 核心历法能力已经具备可用形态：
  - `Solar ⇄ Lunar` 双向转换；
  - 节气、干支、生肖、纳音、宜忌、吉神凶煞、九星、数九、三伏、物候；
  - 八字、运程；
  - 佛历、道历；
  - 法定节假日；
  - `SolarWeek/Month/Year/HalfYear/Season` 辅助模型。
- 本地验证结果已经是全绿：
  - `cargo test` 通过；
  - 17 个集成测试 + 1 个 doctest 全部通过。

但从“设计承诺”和“下一阶段演进”来看，仓库仍然存在两类缺口：

1. **工程化缺口**
   - 文档状态漂移；
   - 测试矩阵不完整；
   - `bench` / 差分测试 / i18n / serde 等设计项尚未落地。

2. **架构与能力扩展缺口**
   - 当前 API 仍以 `Lunar`/`Solar` 大对象 + `String`/`&str` 返回为主；
   - 还没有演进到 `tyme4rs` 那种“通用历法域模型 + 多历法 + 事件抽象”的层级。

---

## 2. 本次核查确认：已经实现的功能

### 2.1 已落地的核心能力

结合 `src/` 模块与 `tests/lunar_core.rs`，当前仓库已经实现：

- 阳历基础：
  - 日期构造、格式化、儒略日、星期、星座、节日、工作日推进、薪资倍率。
- 农历核心：
  - 阳历转农历、农历转阳历；
  - 闰月处理；
  - 1582 历法改革缺失日期；
  - 节气精确时刻；
  - 干支、生肖、纳音、旬、旬空；
  - 冲煞、方位、彭祖、宜忌、吉神、凶煞、建除、宿、月相、数九、三伏、物候、九星。
- 八字与运程：
  - `EightChar`；
  - `Yun / DaYun / LiuNian / LiuYue / XiaoYun`。
- 周边历法：
  - 佛历 `Foto`；
  - 道历 `Tao`。
- 聚合时间模型：
  - `SolarWeek`、`SolarMonth`、`SolarYear`、`SolarSeason`、`SolarHalfYear`。
- 节假日：
  - `Holiday` 和内置假期查询。

### 2.2 当前验证结论

本地执行结果：

```bash
cargo test
```

结果：

- `tests/lunar_core.rs` 共 17 项测试全部通过；
- `src/lib.rs` 中 1 个 doctest 通过。

这说明“当前仓库还有一个黄金用例未对齐”的说法已经过时，不再符合实际状态。

---

## 3. 本次核查确认：仍未完成或未闭环的部分

以下内容分为“功能未实现”和“工程未闭环”两类。

### 3.1 功能/设计项未实现

#### F1. `i18n` 已从占位 feature 进入基础骨架阶段，但尚未完整落地

现状：

- 已新增 `i18n` 模块与 `Language` 枚举。
- 已提供星期、星座、节气、生肖、干支的显式语言辅助方法。
- 已扩展到 `Solar`、`Lunar`、`Foto`、`Tao`、`NineStar` 的显式语言完整输出链路。
- 仍未实现设计文档中规划的全表占位符重写、语言包切换、完整输出链路国际化。

影响：

- 当前库默认仍然是中文输出；
- `i18n` 已具备起步能力，但距离“完整国际化”还有明显差距。

#### F2. `serde` 可选序列化已完成

现状：

- 已新增 `serde` feature 与核心拥有所有权类型的可选派生实现。

影响：

- 该项主目标已达成，后续只需按新增类型继续补派生覆盖。

#### F3. 节假日运行时覆盖接口已完成

现状：

- 已提供 `holiday_names`、`holiday_data`、`set_holidays`、
  `set_holiday_data`、`reset_holidays` 等公开 API。

影响：

- 该项主目标已达成，后续可按需要继续增强数据来源接入方式。

#### F4. 设计文档中的 `bench` 能力已完成第一版

现状：

- 已新增 `benches/convert.rs`；
- 已可运行 `cargo bench --bench convert`。

影响：

- 已具备第一版性能观测能力；
- 后续可继续补更细粒度基准与阈值断言。

### 3.2 工程化未闭环

#### E1. 测试体系已完成第一轮拆分，但仍未达到全量移植强度

现状：

- 已拆分为 `solar`、`lunar`、`jieqi`、`eight_char` 等测试文件；
- 但距离参考实现的“全量测试移植”仍有差距。

影响：

- 现在更像“关键路径 smoke + 黄金样例”；
- 还不是“完整参考实现对齐”的测试强度。

#### E2. 差分测试已完成协议、自检脚本与样例矩阵，但当前仓库选择仅保留 Rust 自检链路

现状：

- 已有 `tests/differential.rs`、协议测试、sample driver、
  自检脚本、样例矩阵；
- 当前仓库已明确移除 Go 适配链路，仅保留 Rust 自检与协议验证。

影响：

- 差分链路在仓库内已闭环；
- 若未来需要真实跨语言对照，建议放到仓库外部工具链或 CI 环境处理。

#### E3. 文档状态已过期

现状：

- README / README_CN / CHANGELOG 仍写着“仍有一个 full-string 对齐问题待修正”；
- 实际 `cargo test` 已全部通过。

影响：

- 对外状态表达不准确；
- 容易误导后续开发优先级。

#### E4. 设计文档与实际仓库结构存在偏差

现状：

- 设计文档列出 `i18n.rs`、`packed.rs`、`benches/`、多份测试文件；
- 实际仓库并未完全对应。

影响：

- `docs/DESIGN.md` 更像“设计蓝图”，不是“当前状态说明”；
- 后续 roadmap 需要明确区分“已实现”和“待实现”。

---

## 4. `tyme4rs` 给出的重要启发

`tyme4rs` 不是简单的 `lunar-rs` 同类替代品，而是更高一层的“通用历法域模型”。

### 4.1 结构层面的启发

从其仓库结构可以看出，`tyme4rs` 重点不在于把更多逻辑塞进 `Lunar`，而在于把概念独立成类型：

- 核心抽象：
  - `Culture`
  - `Tyme`
  - `LoopTyme`
- 历法模块：
  - `solar`
  - `lunar`
  - `sixtycycle`
  - `eightchar`
  - `holiday`
  - `festival`
  - `event`
  - `hijri`
  - `rabbyung`
- 民俗文化模块：
  - `animal`
  - `direction`
  - `element`
  - `god`
  - `taboo`
  - `phase`
  - `phenology`
  - `peng_zu`
  - `dog`
  - `plumrain`
  - `star`

这意味着 `tyme4rs` 的设计重点是：

1. **强类型表达文化概念**，而不是统一返回字符串；
2. **多历法并行**，不只限于公历/农历；
3. **事件化抽象**，把节日、现代纪念日、传统节日统一建模；
4. **更适合组合与扩展**，便于继续加新历法、新事件、新文化实体。

### 4.2 能力层面的启发

从 `tyme4rs` README 与 CHANGELOG 可以提炼出几条对 `lunar-rs` 最有价值的方向：

- 已支持并持续强化：
  - 回历 `Hijri`
  - 藏历 `RabByung`
  - 事件 `Event`
  - 节日 `Festival`
  - 小六壬
  - 灶马头 `KitchenGodSteed`
  - 候精确到公历时刻
  - 月相对象化
  - 三柱 `ThreePillars`
  - 干支日/干支时等独立对象化

这些并不意味着 `lunar-rs` 要立刻全部照搬，而是说明：

- 如果 `lunar-rs` 继续只围绕 `Lunar`/`Solar` 堆 getter，会越来越难扩展；
- 下一阶段更合理的方向，是先把核心概念“对象化、类型化、事件化”。

---

## 5. 建议路线图

下面的路线按照“先稳当前库，再做可扩展演进”的顺序安排。

### Phase 0：状态校准与基线收口

目标：先让仓库的文档、验证状态、交付口径一致。

建议项：

1. 更新 `README.md` / `README_CN.md` / `CHANGELOG.md`
   - 去掉“仍有 1 个 full-string 用例失败”的旧描述；
   - 明确当前 `cargo test` 已全绿。
2. 补一份“当前实现矩阵”
   - 按模块列出已实现 / 未实现；
   - 让后续 issue 和 PR 都能直接对齐。
3. 固化当前验证命令
   - `cargo test`
   - `cargo clippy`
   - 后续补 `cargo bench`

验收标准：

- 项目对外状态与实际代码一致；
- 后续 roadmap 的起点清晰可追踪。

### Phase 1：补齐设计文档中的工程化缺口

目标：把 `docs/DESIGN.md` 里已经承诺、但仓库还没落地的基础能力补齐。

建议项：

1. 实现真正的 `i18n`
   - 至少支持 `zh-CN` / `en`；
   - 采用 feature gate；
   - 不影响默认零依赖与性能。
2. 增加 `serde` feature
   - 为 `Solar`、`Lunar`、`Holiday`、`JieQi`、`Foto`、`Tao` 等核心类型提供可选序列化支持。
3. 补 `benches/`
   - 至少覆盖：
     - `Solar::from_ymd(...).lunar()`
     - `Lunar::from_ymd(...).solar()`
     - `to_full_string()`
     - `jie_qi_table()` 构建。
4. 补差分测试
   - 与 `lunar-go` 或 `lunar-javascript` 做批量结果比对；
   - 优先覆盖 1900-2100、闰月年、1582、节气边界、晚子时。
5. 暴露节假日覆盖 API
   - 例如 `replace_holiday_data(...)`
   - 或 `load_holiday_data(...)`
   - 让内部 `RwLock` 真正对外可用。

验收标准：

- 设计文档中的 M5 基础工程项基本闭环；
- 当前库从“可用实现”升级为“可发布维护的库”。

### Phase 2：API 强类型化与去 God Object 化

目标：借鉴 `tyme4rs`，把当前以字符串为中心的接口逐步升级为领域对象。

建议项：

1. 独立基础文化类型
   - `HeavenStem`
   - `EarthBranch`
   - `SixtyCycle`
   - `Direction`
   - `Element`
   - `Zodiac`
   - `Duty`
   - `Phase`
2. 独立黄历/民俗对象
   - `God`
   - `Taboo`
   - `PengZu`
   - `Phenology`
   - `DogDay`
   - `PlumRainDay`
3. 让 `Lunar` 从“所有内容的出口”变成“聚合入口”
   - 保留现有兼容 getter；
   - 新增 typed API；
   - 逐步减少直接拼接字符串的逻辑。

验收标准：

- 不破坏现有 API；
- 新 API 更适合扩展、组合、序列化和跨历法复用。

### Phase 3：事件与节日模型统一

当前状态：

- 已完成最小 `Event` / `EventKind` 模型。
- 已完成 `Solar::events()`、`Lunar::events()`、`Foto::events()`、`Tao::events()`。
- 已完成 `all_events()` 聚合、`sort_events()`、`dedup_events()`。
- 已完成 `EventQuery` 与 `find_events(...)`。
- 已完成 `scan_events_in_range(...)`、`scan_events_in_range_filtered(...)`，
  以及 `Solar` / `Lunar` 的区间事件扫描便捷入口。
- 已把 `Holiday`、`JieQi`、`FotoFestival`、`TaoFestival` 的事件构造规则收拢到源对象自身。
- 已补 `priority`、`source_id`、`is_observed` 等事件规则字段，并统一在事件构造阶段落值。
- 尚未完成更丰富的统一事件类型层级、事件级时间范围建模，以及更强的筛选/索引能力。

目标：把当前零散的 `festivals()` / `other_festivals()` / `Holiday` 查询，统一为事件体系。

建议项：

1. 新增 `Event` 抽象
   - 名称
   - 类型
   - 来源（公历、农历、节气、法定假日、纪念日）
   - 日期/时刻
2. 统一节日模型
   - `SolarFestival`
   - `LunarFestival`
   - `HolidayEvent`
   - `JieQiEvent`
3. 增加日期到事件列表的统一入口
   - `Solar::events()`
   - `Lunar::events()`

下一步优先建议：

1. 继续增强 `Event` 的规则层
   - 继续扩展 `detail` 语义规范；
   - 视需要补 `range_kind` / `is_primary` / `tags` 等字段。
2. 视使用需求补更高阶查询
   - 多条件组合过滤；
   - 指定日期区间事件扫描；
   - 分类索引缓存。
3. 当事件模型足够稳定后，再考虑进入 `Phase 4`
   - 引入 `Hijri` 等多历法对象，并直接复用当前 `Event` 体系。

验收标准：

- 对“日历 UI / API 输出 / 前端集成”更友好；
- 后续接入更多节日与纪念日时无需继续堆字符串列表。

### Phase 4：扩展历法能力

目标：将 `lunar-rs` 从“农历算法库”升级为“中文语境下的多历法库”。

优先顺序建议：

1. **回历 Hijri**
   - 相对独立；
   - 用户认知明确；
   - 对现有 `Solar` 可直接建立映射。
2. **藏历 RabByung**
   - 价值高，但结构更复杂；
   - 更适合在 Phase 2/3 完成后引入。
3. **精确候时刻**
   - 当前仅有 `hou()/wu_hou()` 文字能力；
   - 下一步可升级为“候对象 + 对应公历时刻”。
4. **灶马头 / 小六壬 / 三柱**
   - 作为高级民俗/命理扩展模块；
   - 放在核心工程化完成之后引入。

验收标准：

- 新历法作为独立模块接入；
- 不把 `Solar` / `Lunar` 再次变成更大的巨型对象。

### Phase 5：逆向推算与高级命理工具

目标：补足 `tyme4rs` 中更高阶的“反推/求解”型能力。

建议项：

1. 八字反推公历时刻
2. 三柱反推公历日
3. 干支日 / 干支时独立对象与运算
4. 晚子时规则的系统化切换

说明：

- 这一阶段价值很高，但复杂度也最高；
- 必须建立在 Phase 1-4 完成之后，否则测试与模型都不够稳。

---

## 6. 推荐优先级

如果下一步只做一轮迭代，建议严格按这个顺序推进：

1. **P0**
   - 更新 README / README_CN / CHANGELOG 的过时状态描述。
2. **P1**
   - 补全测试矩阵；
   - 增加差分测试；
   - 增加 benchmark。
3. **P2**
   - 实现真正的 `i18n`；
   - 增加 `serde`；
   - 暴露节假日覆盖 API。
4. **P3**
   - 开始做 typed API 和 `Event` 抽象。
5. **P4**
   - 扩展 `Hijri`；
   - 再评估 `RabByung`、三柱、小六壬等高级能力。

---

## 7. 建议的下一批 issue 切分

为了便于落地，建议拆成以下 issue：

1. `docs: sync current project status after cargo test all green`
2. `test: split lunar_core into domain-specific integration suites`
3. `test: add differential validation against lunar-go/lunar-javascript`
4. `bench: add conversion and formatting benchmarks`
5. `feat: implement runtime i18n with zh/en language packs`
6. `feat: add optional serde support for core calendar types`
7. `feat: expose holiday data override API`
8. `refactor: introduce typed ganzhi/direction/element/event domain objects`
9. `feat: unify solar/lunar/holiday/jieqi output under event model`
10. `feat: add Hijri calendar support`

---

## 8. 最终判断

`lunar-rs` 当前最需要的，不是继续盲目堆更多“字符串 getter”，而是：

1. 先把当前实现的正确性证明做扎实；
2. 再把设计文档承诺但尚未落地的工程化能力补齐；
3. 最后再按 `tyme4rs` 的方向，演进到更强类型、更易扩展、多历法、事件化的架构。

一句话总结：

> 当前库的“农历核心功能”已经可用，下一阶段的主线应该从“补更多字段”切换为“补工程闭环 + 做架构升级”。
