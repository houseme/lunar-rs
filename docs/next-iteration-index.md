# lunar-rs 下一轮实现任务索引

> 创建时间：2026-07-06
> 适用范围：`Phase 1`、`Phase 2`、`Phase 3`、`Phase 4`、`Phase 4.1`、`Phase 4.2`、`Phase 4.3`、`Phase 4.4`、`Phase 4.5`、`Phase 4.6`、`Phase 4.7`、`Phase 4.8`、`Phase 4.9`、`Phase 4.10`、`Phase 4.11`、`Phase 4.12`、`Phase 4.13`、`Phase 4.14`、`Phase 4.15`、`Phase 4.16`、`Phase 4.17`、`Phase 4.18`、`Phase 4.19`、`Phase 4.20`、`Phase 4.21`、`Phase 4.22` 的增量实现
> 目标：把当前阶段任务拆成可单独推进、可单独验收、可单独更新状态的任务文档

---

## 1. 使用方式

本索引只负责：

- 列出下一轮的 3 个独立实现子任务；
- 标明优先级、依赖关系、当前状态；
- 给出每个子任务文档的入口；
- 作为后续核对、检查、更新状态标识的统一入口。

后续推进规则：

1. 进入某个任务实现前，先把对应文档的 `状态` 从 `未开始` 改为 `进行中`。
2. 完成代码与测试后，把 `状态` 改为 `已完成`，并补 `验证记录`。
3. 如果发现范围变化或新增 blocker，只改对应子任务文档，不直接改本索引的细节描述。
4. 本索引只维护摘要状态，不记录实现细节。

---

## 2. 总体状态看板

| 优先级 | 任务编号 | 任务名称 | 当前状态 | 依赖 | 文档 |
| --- | --- | --- | --- | --- | --- |
| `P1` | `Task 01` | Phase 1 `i18n` 完整闭环 | `已完成` | 无 | [docs/tasks/01-phase1-i18n-closure.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/01-phase1-i18n-closure.md) |
| `P2` | `Task 02` | Phase 2 typed API 第二阶段补完 | `已完成` | `Task 01` 可并行弱依赖 | [docs/tasks/02-phase2-typed-api-expansion.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/02-phase2-typed-api-expansion.md) |
| `P3` | `Task 03` | Phase 3 事件模型独立化与多日事件内建 | `已完成` | `Task 02` 部分依赖 | [docs/tasks/03-phase3-event-model-expansion.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/03-phase3-event-model-expansion.md) |
| `P4` | `Task 04` | Phase 4 Hijri 多历法扩展起步 | `已完成` | `Task 03` 已完成 | [docs/tasks/04-phase4-hijri-support.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/04-phase4-hijri-support.md) |
| `P4.1` | `Task 05` | Phase 4.1 多历法生态深化 | `已完成` | `Task 04` 已完成 | [docs/tasks/05-phase4-1-multi-calendar-ecosystem.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/05-phase4-1-multi-calendar-ecosystem.md) |
| `P4.2` | `Task 06` | Phase 4.2 多历法共享抽象与边界一致化 | `已完成` | `Task 05` 已完成 | [docs/tasks/06-phase4-2-multi-calendar-shared-abstractions.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/06-phase4-2-multi-calendar-shared-abstractions.md) |
| `P4.3` | `Task 07` | Phase 4.3 多历法公开 Trait 与统一访问层 | `已完成` | `Task 06` 已完成 | [docs/tasks/07-phase4-3-public-calendar-traits.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/07-phase4-3-public-calendar-traits.md) |
| `P4.4` | `Task 08` | Phase 4.4 第五个历法对象 `Minguo` 接入 | `已完成` | `Task 07` 已完成 | [docs/tasks/08-phase4-4-minguo-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/08-phase4-4-minguo-calendar.md) |
| `P4.5` | `Task 09` | Phase 4.5 `Minguo` 的 i18n 与描述层收口 | `已完成` | `Task 08` 已完成 | [docs/tasks/09-phase4-5-minguo-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/09-phase4-5-minguo-i18n.md) |
| `P4.6` | `Task 10` | Phase 4.6 第六个历法对象 `ThaiSolar` 接入 | `已完成` | `Task 09` 已完成 | [docs/tasks/10-phase4-6-thai-solar-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/10-phase4-6-thai-solar-calendar.md) |
| `P4.7` | `Task 11` | Phase 4.7 `ThaiSolar` 的 i18n 与描述层收口 | `已完成` | `Task 10` 已完成 | [docs/tasks/11-phase4-7-thai-solar-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/11-phase4-7-thai-solar-i18n.md) |
| `P4.8` | `Task 12` | Phase 4.8 第七个历法对象 `Japanese` 接入 | `已完成` | `Task 11` 已完成 | [docs/tasks/12-phase4-8-japanese-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/12-phase4-8-japanese-calendar.md) |
| `P4.9` | `Task 13` | Phase 4.9 `Japanese` 的 i18n 与描述层收口 | `已完成` | `Task 12` 已完成 | [docs/tasks/13-phase4-9-japanese-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/13-phase4-9-japanese-i18n.md) |
| `P4.10` | `Task 14` | Phase 4.10 第八个历法对象 `Juche` 接入 | `已完成` | `Task 13` 已完成 | [docs/tasks/14-phase4-10-juche-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/14-phase4-10-juche-calendar.md) |
| `P4.11` | `Task 15` | Phase 4.11 `Juche` 的 i18n 与描述层收口 | `已完成` | `Task 14` 已完成 | [docs/tasks/15-phase4-11-juche-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/15-phase4-11-juche-i18n.md) |
| `P4.12` | `Task 16` | Phase 4.12 第九个历法对象 `Dangi` 接入 | `已完成` | `Task 15` 已完成 | [docs/tasks/16-phase4-12-dangi-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/16-phase4-12-dangi-calendar.md) |
| `P4.13` | `Task 17` | Phase 4.13 `Dangi` 的 i18n 与描述层收口 | `已完成` | `Task 16` 已完成 | [docs/tasks/17-phase4-13-dangi-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/17-phase4-13-dangi-i18n.md) |
| `P4.14` | `Task 18` | Phase 4.14 第十个历法对象 `Julian` 接入 | `已完成` | `Task 17` 已完成 | [docs/tasks/18-phase4-14-julian-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/18-phase4-14-julian-calendar.md) |
| `P4.15` | `Task 19` | Phase 4.15 `Julian` 的 i18n 与描述层收口 | `已完成` | `Task 18` 已完成 | [docs/tasks/19-phase4-15-julian-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/19-phase4-15-julian-i18n.md) |
| `P4.16` | `Task 20` | Phase 4.16 第十一个历法对象 `Holocene` 接入 | `已完成` | `Task 19` 已完成 | [docs/tasks/20-phase4-16-holocene-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/20-phase4-16-holocene-calendar.md) |
| `P4.17` | `Task 21` | Phase 4.17 `Holocene` 的 i18n 与描述层收口 | `已完成` | `Task 20` 已完成 | [docs/tasks/21-phase4-17-holocene-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/21-phase4-17-holocene-i18n.md) |
| `P4.18` | `Task 22` | Phase 4.18 第十二个历法对象 `Byzantine` 接入 | `已完成` | `Task 21` 已完成 | [docs/tasks/22-phase4-18-byzantine-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/22-phase4-18-byzantine-calendar.md) |
| `P4.19` | `Task 23` | Phase 4.19 `Byzantine` 的 i18n 与描述层收口 | `已完成` | `Task 22` 已完成 | [docs/tasks/23-phase4-19-byzantine-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/23-phase4-19-byzantine-i18n.md) |
| `P4.20` | `Task 24` | Phase 4.20 第十三个历法对象 `Coptic` 接入 | `已完成` | `Task 23` 已完成 | [docs/tasks/24-phase4-20-coptic-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/24-phase4-20-coptic-calendar.md) |
| `P4.21` | `Task 25` | Phase 4.21 第十四个历法对象 `Ethiopian` 接入 | `已完成` | `Task 24` 已完成 | [docs/tasks/25-phase4-21-ethiopian-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/25-phase4-21-ethiopian-calendar.md) |
| `P4.22` | `Task 26` | Phase 4.22 `Ethiopian` 的 i18n 与描述层收口 | `已完成` | `Task 25` 已完成 | [docs/tasks/26-phase4-22-ethiopian-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/26-phase4-22-ethiopian-i18n.md) |
| `P4.23` | `Task 27` | Phase 4.23 `Coptic` 的 i18n 与描述层收口 | `已完成` | `Task 24` 已完成 | [docs/tasks/27-phase4-23-coptic-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/27-phase4-23-coptic-i18n.md) |
| `P4.24` | `Task 28` | Phase 4.24 第十五个历法对象 `Armenian` 接入 | `已完成` | `Task 27` 已完成 | [docs/tasks/28-phase4-24-armenian-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/28-phase4-24-armenian-calendar.md) |
| `P4.25` | `Task 29` | Phase 4.25 `Armenian` 的 i18n 与描述层收口 | `已完成` | `Task 28` 已完成 | [docs/tasks/29-phase4-25-armenian-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/29-phase4-25-armenian-i18n.md) |
| `P4.26` | `Task 30` | Phase 4.26 第十六个历法对象 `AUC` 接入 | `已完成` | `Task 29` 已完成 | [docs/tasks/30-phase4-26-auc-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/30-phase4-26-auc-calendar.md) |
| `P4.27` | `Task 31` | Phase 4.27 `AUC` 的 i18n 与描述层收口 | `已完成` | `Task 30` 已完成 | [docs/tasks/31-phase4-27-auc-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/31-phase4-27-auc-i18n.md) |
| `P4.28` | `Task 32` | Phase 4.28 第十七个历法对象 `Assyrian` 接入 | `已完成` | `Task 31` 已完成 | [docs/tasks/32-phase4-28-assyrian-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/32-phase4-28-assyrian-calendar.md) |
| `P4.29` | `Task 33` | Phase 4.29 `Assyrian` 的 i18n 与描述层收口 | `已完成` | `Task 32` 已完成 | [docs/tasks/33-phase4-29-assyrian-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/33-phase4-29-assyrian-i18n.md) |
| `P4.30` | `Task 34` | Phase 4.30 第十八个历法对象 `HispanicEra` 接入 | `已完成` | `Task 33` 已完成 | [docs/tasks/34-phase4-30-hispanic-era-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/34-phase4-30-hispanic-era-calendar.md) |
| `P4.31` | `Task 35` | Phase 4.31 `HispanicEra` 的 i18n 与描述层收口 | `已完成` | `Task 34` 已完成 | [docs/tasks/35-phase4-31-hispanic-era-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/35-phase4-31-hispanic-era-i18n.md) |
| `P4.32` | `Task 36` | Phase 4.32 第十九个历法对象 `Saka` 接入 | `已完成` | `Task 35` 已完成 | [docs/tasks/36-phase4-32-saka-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/36-phase4-32-saka-calendar.md) |
| `P4.33` | `Task 37` | Phase 4.33 `Saka` 的 i18n 与描述层收口 | `已完成` | `Task 36` 已完成 | [docs/tasks/37-phase4-33-saka-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/37-phase4-33-saka-i18n.md) |
| `P4.34` | `Task 38` | Phase 4.34 第二十个历法对象 `Bengali` 接入 | `已完成` | `Task 37` 已完成 | [docs/tasks/38-phase4-34-bengali-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/38-phase4-34-bengali-calendar.md) |
| `P4.35` | `Task 39` | Phase 4.35 `Bengali` 的 i18n 与描述层收口 | `已完成` | `Task 38` 已完成 | [docs/tasks/39-phase4-35-bengali-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/39-phase4-35-bengali-i18n.md) |
| `P4.36` | `Task 40` | Phase 4.36 第二十一个历法对象 `Koki` 接入 | `已完成` | `Task 39` 已完成 | [docs/tasks/40-phase4-36-koki-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/40-phase4-36-koki-calendar.md) |
| `P4.37` | `Task 41` | Phase 4.37 `Koki` 的 i18n 与描述层收口 | `已完成` | `Task 40` 已完成 | [docs/tasks/41-phase4-37-koki-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/41-phase4-37-koki-i18n.md) |
| `P4.38` | `Task 42` | Phase 4.38 第二十二个历法对象 `ThaiBuddhist` 接入 | `已完成` | `Task 41` 已完成 | [docs/tasks/42-phase4-38-thai-buddhist-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/42-phase4-38-thai-buddhist-calendar.md) |
| `P4.39` | `Task 43` | Phase 4.39 `ThaiBuddhist` 的 i18n 与描述层收口 | `已完成` | `Task 42` 已完成 | [docs/tasks/43-phase4-39-thai-buddhist-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/43-phase4-39-thai-buddhist-i18n.md) |
| `P4.40` | `Task 44` | Phase 4.40 第二十三个历法对象 `Fasli` 接入 | `已完成` | `Task 43` 已完成 | [docs/tasks/44-phase4-40-fasli-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/44-phase4-40-fasli-calendar.md) |
| `P4.41` | `Task 45` | Phase 4.41 `Fasli` 的 i18n 与描述层收口 | `已完成` | `Task 44` 已完成 | [docs/tasks/45-phase4-41-fasli-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/45-phase4-41-fasli-i18n.md) |
| `P4.42` | `Task 46` | Phase 4.42 第二十四个历法对象 `Nanakshahi` 接入 | `已完成` | `Task 45` 已完成 | [docs/tasks/46-phase4-42-nanakshahi-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/46-phase4-42-nanakshahi-calendar.md) |
| `P4.43` | `Task 47` | Phase 4.43 `Nanakshahi` 的 i18n 与描述层收口 | `已完成` | `Task 46` 已完成 | [docs/tasks/47-phase4-43-nanakshahi-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/47-phase4-43-nanakshahi-i18n.md) |
| `P4.44` | `Task 48` | Phase 4.44 第二十五个历法对象 `Seleucid` 接入 | `已完成` | `Task 47` 已完成 | [docs/tasks/48-phase4-44-seleucid-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/48-phase4-44-seleucid-calendar.md) |
| `P4.45` | `Task 49` | Phase 4.45 `Seleucid` 的 i18n 与描述层收口 | `已完成` | `Task 48` 已完成 | [docs/tasks/49-phase4-45-seleucid-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/49-phase4-45-seleucid-i18n.md) |
| `P4.46` | `Task 50` | Phase 4.46 第二十六个历法对象 `Rattanakosin` 接入 | `已完成` | `Task 49` 已完成 | [docs/tasks/50-phase4-46-rattanakosin-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/50-phase4-46-rattanakosin-calendar.md) |
| `P4.47` | `Task 51` | Phase 4.47 `Rattanakosin` 的 i18n 与描述层收口 | `已完成` | `Task 50` 已完成 | [docs/tasks/51-phase4-47-rattanakosin-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/51-phase4-47-rattanakosin-i18n.md) |
| `P4.48` | `Task 52` | Phase 4.48 第二十七个历法对象 `Venetian` 接入 | `已完成` | `Task 51` 已完成 | [docs/tasks/52-phase4-48-venetian-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/52-phase4-48-venetian-calendar.md) |
| `P4.49` | `Task 53` | Phase 4.49 `Venetian` 的 i18n 与描述层收口 | `已完成` | `Task 52` 已完成 | [docs/tasks/53-phase4-49-venetian-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/53-phase4-49-venetian-i18n.md) |
| `P4.50` | `Task 54` | Phase 4.50 第二十八个历法对象 `Rumi` 接入 | `已完成` | `Task 53` 已完成 | [docs/tasks/54-phase4-50-rumi-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/54-phase4-50-rumi-calendar.md) |
| `P4.51` | `Task 55` | Phase 4.51 `Rumi` 的 i18n 与描述层收口 | `已完成` | `Task 54` 已完成 | [docs/tasks/55-phase4-51-rumi-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/55-phase4-51-rumi-i18n.md) |
| `P4.52` | `Task 56` | Phase 4.52 第二十九个历法对象 `AnnoLucis` 接入 | `已完成` | `Task 55` 已完成 | [docs/tasks/56-phase4-52-anno-lucis-calendar.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/56-phase4-52-anno-lucis-calendar.md) |
| `P4.53` | `Task 57` | Phase 4.53 `AnnoLucis` 的 i18n 与描述层收口 | `已完成` | `Task 56` 已完成 | [docs/tasks/57-phase4-53-anno-lucis-i18n.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/57-phase4-53-anno-lucis-i18n.md) |

---

## 3. 优先级说明

### 3.1 `Task 01` 为什么优先

原因：

- 当前 `i18n` 仍是显式 helper 模式，不是设计文档里的完整语言层；
- 它会影响 typed API、事件输出、完整字符串渲染等多个下游接口；
- 如果不先收口语言模型，后续 typed/event 新增接口会继续复制中文直出逻辑。

### 3.2 `Task 02` 为什么第二

原因：

- 现在已经有第一批 typed primitive，但仍有大量文化概念停留在字符串层；
- 这会继续强化 `Lunar` 的 god object 倾向；
- 先补 typed domain，可为 Phase 3 事件包装和多历法扩展提供更稳的对象基础。

### 3.3 `Task 03` 为什么第三

原因：

- 当前事件模型已经可用，但仍偏“统一出口”而非“独立事件域模型”；
- 如果事件分类对象和多日事件规则先于 typed domain 大规模补齐，容易重复建模；
- 放在 `Task 02` 之后推进，复用 typed 文化对象更自然。

---

## 4. 当前边界结论

下一轮实现只覆盖以下三类工作：

1. `Phase 1`
   - 完整 `i18n` 闭环；
   - 统一语言包与占位符策略；
   - 收口当前“helper scattered”式语言输出。
2. `Phase 2`
   - 补全字符串型文化概念的 typed API；
   - 继续拆小 `Lunar` 的直接字符串输出职责；
   - 强化对象复用和序列化友好性。
3. `Phase 3`
   - 引入独立事件包装类型；
   - 补内建 multi-day 事件源；
   - 增强事件查询与规则语义。

明确不在本轮范围内：

- `Phase 4` 的 `Hijri` / `RabByung`；
- `Phase 5` 的反推、公历逆算、高级命理工具；
- 大规模文档重写或 README 全量重构；
- 跨仓库外部差分 CI 基建。

---

## 5. 推荐执行顺序

### 顺序 A：严格顺序

1. `Task 01`
2. `Task 02`
3. `Task 03`

适用于：

- 希望先把语言层收口；
- 让后续 typed/event API 不再重复补国际化接口；
- 追求接口一致性优先。

### 顺序 B：并行穿插

1. 先完成 `Task 01` 的语言基础设施；
2. 再先做 `Task 02` 中不依赖完整 i18n 的纯 typed 对象部分；
3. 最后集中完成 `Task 03`。

适用于：

- 想快速推进 typed domain；
- 但又不希望彻底跳过 i18n 基础层。

当前推荐：`顺序 A`

---

## 6. 状态标识约定

所有子任务文档统一使用以下状态值：

- `未开始`
- `进行中`
- `已完成`
- `已阻塞`
- `已取消`

建议更新格式：

```md
状态：进行中
最近更新：2026-07-06
进度：2/6
```

---

## 7. 后续维护入口

后续每次推进时：

- 先更新本索引中的摘要状态；
- 再更新对应子任务文档中的勾选项、验证记录、风险与备注；
- 如果实现范围扩大，再新建 `Task 04+` 文档，不要把原任务继续无限膨胀。

---

## 8. 2026-07-07 对标续作

今日重新读取 `6tail/tyme4rs` 当前 `master` 源码并核对本地代码后，结论已保存到：

- [docs/tyme4rs-gap-analysis-2026-07-07.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tyme4rs-gap-analysis-2026-07-07.md)

新增的下一批任务从 `Task 58` 开始：

| 优先级 | 任务编号 | 任务名称 | 当前状态 | 文档 |
| --- | --- | --- | --- | --- |
| `P1` | `Task 58` | typed `Constellation` API | `已完成` | [docs/tasks/58-phase-next-constellation-typed-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/58-phase-next-constellation-typed-api.md) |
| `P1` | `Task 59` | 轻量 Culture/Cycle trait 层 | `已完成` | [docs/tasks/59-phase-next-culture-cycle-traits.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/59-phase-next-culture-cycle-traits.md) |
| `P2` | `Task 60` | Fetus typed API | `已完成` | [docs/tasks/60-phase-next-fetus-typed-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/60-phase-next-fetus-typed-api.md) |
| `P2` | `Task 61` | 小六壬 typed API | `已完成` | [docs/tasks/61-phase-next-minor-ren-typed-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/61-phase-next-minor-ren-typed-api.md) |
| `P2` | `Task 62` | SixtyCycle 年/月/日/时对象 | `已完成` | [docs/tasks/62-phase-next-sixty-cycle-pillars.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/62-phase-next-sixty-cycle-pillars.md) |
| `P2` | `Task 63` | 文化日对象统一 | `已完成` | [docs/tasks/63-phase-next-culture-day-objects.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/63-phase-next-culture-day-objects.md) |
| `P3` | `Task 64` | EventManager 规则模型 | `已完成` | [docs/tasks/64-phase-next-event-manager-rule-model.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/64-phase-next-event-manager-rule-model.md) |
| `P3` | `Task 65` | EightChar provider 化 | `已完成` | [docs/tasks/65-phase-next-eight-char-provider.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/65-phase-next-eight-char-provider.md) |
| `P3` | `Task 66` | EventRule 节气后天干 / 地支规则 | `已完成` | [docs/tasks/66-phase-next-event-term-stem-branch-rules.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/66-phase-next-event-term-stem-branch-rules.md) |
| `P2` | `Task 67` | NineDay / HideHeavenStemDay 文化日对象 | `已完成` | [docs/tasks/67-phase-next-nine-hide-heaven-stem-days.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/67-phase-next-nine-hide-heaven-stem-days.md) |
| `P2` | `Task 68` | ChildLimit provider 与童限信息对象 | `已完成` | [docs/tasks/68-phase-next-child-limit-provider.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/68-phase-next-child-limit-provider.md) |
| `P2` | `Task 69` | DecadeFortune / Fortune 运势对象 | `已完成` | [docs/tasks/69-phase-next-decade-fortune-objects.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/69-phase-next-decade-fortune-objects.md) |
| `P2` | `Task 70` | PengZu 天干 / 地支 typed companion | `已完成` | [docs/tasks/70-phase-next-pengzu-typed-companions.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/70-phase-next-pengzu-typed-companions.md) |
| `P3` | `Task 71` | XiuAnimal / Beast 循环对象补强 | `已完成` | [docs/tasks/71-phase-next-animal-beast-cycle-objects.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/71-phase-next-animal-beast-cycle-objects.md) |
| `P2` | `Task 72` | Terrain 十二长生 typed API | `已完成` | [docs/tasks/72-phase-next-terrain-typed-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/72-phase-next-terrain-typed-api.md) |
| `P3` | `Task 73` | Week typed API | `已完成` | [docs/tasks/73-phase-next-week-typed-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/73-phase-next-week-typed-api.md) |
| `P3` | `Task 74` | Zone 四宫 typed API | `已完成` | [docs/tasks/74-phase-next-zone-typed-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/74-phase-next-zone-typed-api.md) |
| `P3` | `Task 75` | Nayin 纳音循环对象补强 | `已完成` | [docs/tasks/75-phase-next-nayin-cycle-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/75-phase-next-nayin-cycle-api.md) |
| `P3` | `Task 76` | Land 九野与 Direction 九宫循环对象 | `已完成` | [docs/tasks/76-phase-next-land-direction-cycle-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/76-phase-next-land-direction-cycle-api.md) |
| `P3` | `Task 77` | Xun 旬循环对象补强 | `已完成` | [docs/tasks/77-phase-next-xun-cycle-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/77-phase-next-xun-cycle-api.md) |
| `P3` | `Task 78` | YuanCycle / YunCycle 三元九运循环对象 | `已完成` | [docs/tasks/78-phase-next-yuan-yun-cycle-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/78-phase-next-yuan-yun-cycle-api.md) |
| `P3` | `Task 79` | Element / Duty 基础文化循环对象补强 | `已完成` | [docs/tasks/79-phase-next-element-duty-cycle-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/79-phase-next-element-duty-cycle-api.md) |
| `P3` | `Task 80` | GodLuck 吉凶循环对象补强 | `已完成` | [docs/tasks/80-phase-next-god-luck-cycle-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/80-phase-next-god-luck-cycle-api.md) |
| `P3` | `Task 81` | Zodiac 生肖循环对象补强 | `已完成` | [docs/tasks/81-phase-next-zodiac-cycle-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/81-phase-next-zodiac-cycle-api.md) |
| `P3` | `Task 82` | TabooKind 宜忌类型循环对象补强 | `已完成` | [docs/tasks/82-phase-next-taboo-kind-cycle-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/82-phase-next-taboo-kind-cycle-api.md) |
| `P2` | `Task 83` | KitchenGodSteed 灶马头 typed 聚合对象 | `已完成` | [docs/tasks/83-phase-next-kitchen-god-steed.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/83-phase-next-kitchen-god-steed.md) |
| `P3` | `Task 84` | God 神煞大表反查与步进 API | `已完成` | [docs/tasks/84-phase-next-god-table-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/84-phase-next-god-table-api.md) |
| `P3` | `Task 85` | Taboo 宜忌事项大表反查与步进 API | `已完成` | [docs/tasks/85-phase-next-taboo-table-api.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/85-phase-next-taboo-table-api.md) |
| `P2` | `Task 86` | 星曜循环对象一次性补齐 | `已完成` | [docs/tasks/86-phase-next-star-cycle-objects.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/tasks/86-phase-next-star-cycle-objects.md) |

后续建议继续按新对标文档和本索引中的 `Task 59` - `Task 86` 推进，不再把已经完成的 Phase 4 多历法工作重复列为待办。
