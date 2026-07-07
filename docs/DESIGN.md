# lunar-rs 设计方案

> 基于参考实现 [lunar-javascript](https://github.com/6tail/lunar-javascript) 与 [lunar-go](https://github.com/6tail/lunar-go)，使用 Rust 实现的高性能、高水平、高可用的中国农历 / 阳历 / 佛历 / 道历 / 八字 / 九星日历库。
>
> 本文档为“先剖析、再设计、后编码”工作流的设计产出，存放于 `docs/` 目录（已加入 `.gitignore`，禁止提交）。

---

## 0. 文档导航

| 章节 | 内容 |
| --- | --- |
| [1. 背景与目标](#1-背景与目标) | 项目目标、非目标、成功标准 |
| [2. 参考仓库剖析](#2-参考仓库剖析) | lunar-javascript / lunar-go 的架构、模块、数据流对比 |
| [3. 核心算法原理](#3-核心算法原理) | 寿星天文历、干支推导、闰月、节气、八字、九星 |
| [4. Rust 项目结构设计](#4-rust-项目结构设计) | crate 布局、模块划分、依赖 |
| [5. 模块级 API 设计](#5-模块级-api-设计) | 每个类型的字段、构造、关键方法、Rust 化映射 |
| [6. 性能策略](#6-性能策略) | `Arc` 缓存、`Cow`、常量表、零拷贝、数值算法 |
| [7. 错误处理策略](#7-错误处理策略) | `Result` vs `panic`、错误类型分层 |
| [8. 国际化 (i18n) 策略](#8-国际化-i18n-策略) | 占位符重写表的 Rust 实现 |
| [9. 测试策略](#9-测试策略) | 用例来源、等价对照、模糊测试、性能基准 |
| [10. 实施路线图](#10-实施路线图) | 分阶段交付计划与里程碑 |
| [11. 风险与决策记录](#11-风险与决策记录) | 关键取舍 |

---

## 1. 背景与目标

### 1.1 项目定位

`lunar-rs` 是 `lunar` 系列（作者 6tail）的 Rust 实现。`lunar` 是一款**无第三方依赖**的中国历法工具，覆盖：

- 公历（阳历）/ 农历（阴历）互转
- 二十四节气、节令（节气 / 中气）
- 干支（天干地支）、生肖、纳音、五行
- 八字（四柱）、十神、藏干、十二长生（地势）
- 大运 / 流年 / 流月 / 小运（运程）
- 九星（九宫飞星：玄空 / 北斗 / 奇门 / 太乙）
- 每日宜忌、吉神宜趋、凶煞宜忌、彭祖百忌
- 吉神方位（喜神 / 福神 / 财神 / 阳贵 / 阴贵 / 胎神 / 太岁）
- 冲煞、建除十二值星、二十八宿、月相、数九、三伏、物候（七十二候）
- 佛历、道历
- 法定节假日（含调休）与薪资倍率

### 1.2 设计目标

| 目标 | 含义 |
| --- | --- |
| **高性能 (Performance)** | 单次 `Solar → Lunar` 转换 + 完整 GanZhi 计算在微秒级；常量查表零分配；天文计算结果按年缓存复用。 |
| **高水平 (Quality)** | 与 Go/JS 参考实现**逐位对齐**的输出（以参考仓库测试用例为黄金标准）；`#![deny(unsafe_code)]`；`clippy::pedantic` 通过；100% safe Rust。 |
| **高可用 (Usability)** | 符合 Rust 习惯的 API（`from_ymd`/`next`/迭代器/`Display`）；`Result` 化的错误处理而非 `panic`；完善文档与示例；可选 serde 支持。 |

### 1.3 非目标

- **不**重写天文算法本身：寿星天文历的 VSOP87/ELP 级数与 `ΔT` 模型必须**原样移植**，以保证与参考实现数值一致。
- **不**支持任意历史年代的天文回推（参考实现的有效范围约公元 1–9999 年，受 `LEAP_11/LEAP_12` 表与 `ShouXing` 范围约束）。
- 第一版**不**内置图形界面或 HTTP 服务；定位为**纯算法库 (library crate)**。

### 1.4 成功标准

1. 移植 lunar-go 的全部 `_test.go` 为 Rust 集成测试，**全部通过**。
2. 对 1900–2100 年（含闰月、1582 历法改革、跨年边界）每日随机采样，与 Go 版输出字符串完全一致。
3. `cargo bench` 下，单次 `Solar::from_ymd(2020,5,1).lunar()` 全量字段计算 < 5µs（命中缓存时 < 500ns）。

---

## 2. 参考仓库剖析

### 2.1 lunar-javascript（权威实现）

- **形态**：单文件 `lunar.js`，8538 行，UMD/IIFE 工厂模式。每个“类”是匿名对象 + 私有构造 + 静态工厂方法（`Solar.fromYmdHms`、`Lunar.fromYmd` 等），状态封装在 `_p` 私有袋中。
- **特性最全**：是 Java/Python/Go/C#/PHP 等所有衍生版的源头，唯一包含 **国际化 (I18n)** 机制。
- **关键发现**：
  - `Yun / DaYun / LiuNian / LiuYue / XiaoYun` 在 JS 中**不是独立类**，而是 `EightChar.prototype.getYun(gender, sect)` 内联构建的匿名闭包对象图（`buildDaYun/buildLiuNian/buildLiuYue/buildXiaoYun`）。Python 版把它们拆成了真类。
  - `JieQi / Fu / ShuJiu` 同样**不是类**，而是 `Lunar` 上的方法 / `{name, index}` 形状对象。
  - 导出面共 21 个名字：`ShouXingUtil, SolarUtil, LunarUtil, FotoUtil, TaoUtil, NineStarUtil, Solar, Lunar, Foto, Tao, NineStar, EightChar, SolarWeek, SolarMonth, SolarSeason, SolarHalfYear, SolarYear, LunarMonth, LunarYear, LunarTime, HolidayUtil, I18n`。
- **i18n 机制**：查找表**不存中文常量**，而存占位符（如 `'{tg.jia}'`、`'{jq.dongZhi}'`）。`I18n` 维护各语言消息字典，`setLanguage` 时用正则把所有占位符就地替换为目标语言。这是 JS 版独有的精巧设计。

### 2.2 lunar-go（移植蓝本）

- **形态**：多包，Go module `github.com/6tail/lunar-go`，约 8245 行 Go 代码。
- **结构**（本 Rust 项目的**主蓝本**，因其模块拆分清晰）：

```
lunar-go/
├── ShouXingUtil/   # 寿星天文历：VSOP87/ELP 级数、ΔT、章动、节气/合朔求解器 (657 行)
├── SolarUtil/      # 阳历纯函数：闰年、月日数、儒略日、星期、节日 (294 行)
├── LunarUtil/      # 农历核心常量 + 查找表 + 纯函数 (1004 行)
├── FotoUtil/       # 佛历数据 (295 行)
├── TaoUtil/        # 道历数据 (140 行)
├── HolidayUtil/    # 法定节假日（可运行时覆盖） (198+75 行)
└── calendar/       # 领域对象：Solar/Lunar/.../Yun (577+1963+... 行)
```

- **关键发现**：
  - **没有静态农历月大月小表**（不同于老式库的 `lunarInfo[1900..2100]` 十六进制表）。月天数、节气时刻全部由 `ShouXingUtil` **运行时天文计算**得到。静态表只有 `LEAP_11 / LEAP_12 / YMC`（历史/未来闰月兜底 + 月序映射）。
  - Go 版**无 i18n**：所有名称硬编码简体中文。
  - 单条目 `CACHE_YEAR`（`*LunarYear` + `sync.Mutex`）缓存最近构建的 `LunarYear`，因为天文 pass 很贵。
  - `*list.List`（双向链表）泛滥用于有序集合，但仅用 `Front/Back/Next/PushBack`——可无损映射为 `Vec<T>`。

### 2.3 两版对比与选型

| 维度 | lunar-javascript | lunar-go | lunar-rs 取舍 |
| --- | --- | --- | --- |
| 模块组织 | 单文件 21 模块 | 多包清晰拆分 | **采用 Go 的模块拆分**作为 crate 内 mod 布局 |
| i18n | 有（占位符重写） | 无 | **移植 JS 的 i18n**，作为可选 feature（默认中文，零开销） |
| 运 (Yun) 类 | 内联在 EightChar | 真类 `Yun/DaYun/...` | **采用 Go 的真类**，API 更清晰、更易测试 |
| 节气/伏/九 | Lunar 方法 / 字面量对象 | Lunar 方法 + `JieQi/Fu/ShuJiu` 结构体 | **采用 Go 的结构体**，类型更安全 |
| 错误处理 | 抛异常 | `panic` | **`Result<T, LunarError>`**（Rust 习惯，不 panic） |
| 集合 | Array / 链表 | `container/list` | `Vec<T>` / 迭代器 |
| 缓存 | 单条 `CACHE_YEAR` | 单条 `CACHE_YEAR` + Mutex | `OnceLock` / `RwLock<HashMap<year, Arc<LunarYear>>>`（可配容量） |

### 2.4 数据流（Solar ⇄ Lunar 的核心闭环）

```
                    ┌─────────────────────────────────────────────────────┐
                    │  LunarYear::compute()  （每年仅算一次，结果缓存）       │
                    │                                                       │
   year ──────────▶ │  1. 估冬至儒略日 jd                                    │
                    │  2. CalcQi × 26 → 24 节气儒略日（每 15.2184 天一格）    │
                    │  3. CalcShuo × 16 → 合朔儒略日（每 29.5306 天）        │
                    │  4. hs[i+1]-hs[i] → 每月天数（29/30）                   │
                    │  5. 无中气置闰 / LEAP_11 / LEAP_12 → 定闰月             │
                    │  6. 产出 15 个 LunarMonth（跨年边界各延伸一月）          │
                    │  7. 产出 31 个节气儒略日（对齐 JIE_QI_IN_USE）          │
                    └─────────────────────────────────────────────────────┘
                                            │
        ┌───────────────────────────────────┼───────────────────────────────────┐
        ▼                                   ▼                                   ▼
  Solar → Lunar                        Lunar → Solar                      JieQi / GanZhi
  遍历 15 月，找到           取当月 firstJulianDay + (day-1)            compute():
  firstJulianDay ≤ JD <                  的正午 Solar，                      jieqi → year → month
  firstJulianDay + dayCount 的月           补回原始时分秒                       → day → time → week
```

---

## 3. 核心算法原理

> 本节是为编码实现提供“为什么这样算”的理论底座。所有公式与 Go 参考实现逐行对齐。

### 3.1 寿星天文历（ShouXing / VSOP87+ELP）

这是整个库的**精度根基**。它由许剑伟先生提出，是中文历算圈广泛使用的高精度天文算法。

**核心组件**：

1. **太阳黄经级数 `XL0`**（VSOP87 截断版）：形如 `A·cos(φ + ω·t)` 的数千项三角级数，按精度分 6 档（`eLon` 中 `n` 参数控制取项数）。
2. **月亮黄经级数 `XL1`**（ELP 类似）：4×2652 的二维数组，分 4 档。
3. **章动 `NUT_B`**：10 项 IAU 风格的黄经章动模型。
4. **ΔT 表 `DT_AT`**：从公元前 4000 到公元 2050 的分段三次多项式拟合（TT−UT），近期项用 skyfield 的 DE440s 预测数据拟合。
5. **根求解器**：牛顿法 + 二分回退。
   - `saLonT(w)` / `saLonT2(w)`：解“太阳视黄经 = w”。**节气**即太阳黄经走到 `k·15°`。
   - `msaLonT(w)` / `msaLonT2(w)`：解“日月黄经差 = w”。**合朔（初一）**即差为 0。
6. **历史长历查表 `QI_KB / SHUO_KB`**：经典长历的周期+余数表，让中段历史年份**无需展开级数**即可秒算。
7. **修正位串 `QB / SB`**：用 `decode()` 函数把紧缩字母串展开为 `0/1/2` 位串，对低精度区间的节气/合朔做 ±1 天修正。

**三段精度策略**（`CalcQi` / `CalcShuo`）：

```
if jd < f1 或 jd >= f3:   用 qiHigh（全级数，高精度）        ← 远古代 / 远未来
elif jd < f2:             用 QI_KB 长历查表（历史区间）       ← 公元 ~1600 之前
elif jd < f3:             用 qiLow（低精度级数）+ QB 修正     ← 近现代
```

**节气精化** `QiAccurate2(jd)`：以 `π/12`（15°）为步长，找到最近的节气锚点，再做一次 `SaLonT` 高精度求解。

> ⚠️ **移植要点**：`XL0/XL1/NUT_B/DT_AT/QI_KB/SHUO_KB/QB/SB` 全部数据必须**逐字符原样复制**；浮点运算顺序必须与 Go 一致（否则末位差异会导致节气分钟级偏移）。Rust 中用 `f64`，`%` 用 `rem_euclid` 对齐 Go 的负数取模行为需逐处核对。

### 3.2 干支（GanZhi）推导

六十甲子 = 10 天干 × 12 地支的最小公倍循环。索引体系（**注意 1-based vs 0-based 混用**）：

| 维度 | 天干索引 | 地支索引 | 锚点 / 公式 |
| --- | --- | --- | --- |
| **年柱** | `(year − 4) mod 10` | `(year − 4) mod 12` | 以公元 4 年 = 甲子为锚；按正月初一 / 立春日 / 立春时刻三种 `sect` 在立春前后 ±1 修正 |
| **月柱** | `((yearGanIdx mod 5 + 1)·2 + monthOffset) mod 10` | `(monthOffset + 2) mod 12` | `BASE_MONTH_ZHI_INDEX = 2`（寅为正月）；`monthOffset` 由沿 `JIE_QI_IN_USE` 的“节”序列定位（起始 `index = −3`） |
| **日柱** | `(⌊JD_noon⌋ − 11) mod 10` | `(⌊JD_noon⌋ − 11) mod 12` | 以正午为日界；“晚子时”（23:00–23:59）`Exact` 变体 +1 进入次日 |
| **时柱** | `(dayGanExact mod 5 · 2 + timeZhi) mod 10` | `GetTimeZhiIndex("HH:MM")` | “五鼠遁时”规则 |

> **三种年起算约定 `sect`**：`1` = 正月初一、`2` = 立春当日（默认）、`3` = 立春交接时刻。这是命理 / 民俗分歧的根源，库需全部支持并暴露切换。

### 3.3 闰月（Leap Month）

农历闰月由 **“无中气置闰”** 规则决定：一个朔望月（初一到下一个初一）内若**不含中气**（即第 `2i` 个节气），则为闰月。

```
leapIndex 默认 = 16（无闰）
if year ∈ LEAP_11:        leapIndex = 13      # 历史/未来兜底
elif year ∈ LEAP_12:      leapIndex = 14
elif hs[13] ≤ jq[24]:                          # 冬至前有 13 个朔 → 必须插闰
    找首个 i 使 hs[i+1] ≤ jq[2·i] → leapIndex = i
```

闰月用**负数月份**表示（`month < 0` 即 `IsLeap`），全代码库统一；查表时一律取 `|month|`。

### 3.4 节气表（JieQi）

`JIE_QI_IN_USE` 是长度 31 的查找表，**顺序是承载逻辑的**：

```
["DA_XUE","冬至","小寒",...,"大雪","DONG_ZHI","XIAO_HAN","DA_HAN","LI_CHUN","YU_SHUI","JING_ZHE"]
```

- 中间 24 个是当年标准节气（冬至起算）。
- 头部 `DA_XUE` 与尾部 6 个拼音 token 是**跨年边界**的相邻年节气副本，供年柱 / 月柱 / 九星算法取“前一年冬至 / 后一年立春”。
- `convertJieQi` 负责把拼音 token 映射回中文名。

### 3.5 八字（BaZi / EightChar）

四柱 = 年柱 + 月柱 + 日柱 + 时柱，每柱 = 天干 + 地支。`EightChar` 在 `Lunar` 之上提供：

- **藏干 `ZHI_HIDE_GAN`**：每个地支内藏 1–3 个天干（本气 / 中气 / 余气）。
- **十神 `SHI_SHEN`**：以日干为我，其余天干与之的五行生克关系（比肩 / 劫财 / 食神 / 伤官 / 偏财 / 正财 / 七杀 / 正官 / 偏印 / 正印），100 种干干组合查表。
- **十二长生（地势）`CHANG_SHENG`**：长生 / 沐浴 / 冠带 / 临官 / 帝旺 / 衰 / 病 / 死 / 墓 / 绝 / 胎 / 养，按日干阴阳决定顺逆，`CHANG_SHENG_OFFSET` 给偏移。
- **胎元 / 胎息 / 命宫 / 身宫**：命理辅助概念。
- **运程**：见 3.6。

### 3.6 运程（Yun / 大运 / 流年 / 流月 / 小运）

**大运顺逆**：`阳男 / 阴女 → 顺行；阴男 / 阳女 → 逆行`（`yang = yearGanIdxExact % 2 == 0`，`man = gender == 1`）。

**起运岁**（出生到最近一个“节”的时间距离，换算成岁）：

- `sect=2`（真太阳时）：`分钟差 / 4320` 得年（3 天 = 1 年，因 4320 分 = 3 天），余数依次除 360 / 12、×2 得月 / 日 / 时。
- `sect=1`（时辰法）：`1 天 = 4 个月，1 时辰 = 10 天`。

**大运干支**：从月柱的六十甲子索引，按顺 / 逆每 10 年进一格；`LiuNian`（流年）每年一格、`LiuYue`（流月）每月一格、`XiaoYun`（小运）从时柱起每年一格。

### 3.7 九星（NineStar / 九宫飞星）

- **年飞星**：`yuan = ⌊(year + offset + 2696)/60⌋ mod 3`（上 / 中 / 下元），`offset = (62 + yuan·3 − jiaZiIndex) mod 9`。
- **月飞星**：`n = 27 − (yearZhiIdx mod 3)·3`，冬至后 / 夏至后顺逆。
- **日飞星**：以冬至 / 夏至 / 前一年冬至的“甲子”锚日为基准，按“冬顺夏逆、冬至到夏至顺布白、夏至到冬至逆布紫”的飞星规则，用日差 `mod 9` 定位。
- **时飞星**：依日支组（子午卯酉 / 辰戌丑未 / 寅申巳亥）取起始星，按时辰顺 / 逆步进。

---

## 4. Rust 项目结构设计

### 4.1 Crate 形态

单 crate 库 `lunar-rs`（`[lib]`），edition 2024。未来若拆分（如 `lunar-rs-core` + `lunar-rs-chrono`）再演进。

### 4.2 目录布局

```
lunar-rs/
├── Cargo.toml
├── src/
│   ├── lib.rs                     # 顶层 mod 声明 + 重导出 + crate 文档
│   ├── error.rs                   # LunarError 错误类型
│   │
│   ├── shou_xing/                 # 寿星天文历（精度根基，无业务依赖）
│   │   ├── mod.rs
│   │   ├── data.rs                # XL0/XL1/NUT_B/DT_AT/QI_KB/SHUO_KB/QB/SB
│   │   └── engine.rs              # calc_qi / calc_shuo / qi_accurate2 / sa_lon_t ...
│   │
│   ├── solar_util.rs              # 阳历纯函数 + 常量
│   ├── lunar_util/                # 农历常量 + 查找表 + 纯函数
│   │   ├── mod.rs                 # 公开 API + 公开常量
│   │   ├── tables.rs              # GAN/ZHI/JIA_ZI/NAYIN/positions/pengzu/chong/...
│   │   └── packed.rs              # day_yi_ji / time_yi_ji / day_shen_sha 解码
│   ├── foto_util.rs               # 佛历数据
│   ├── tao_util.rs                # 道历数据
│   ├── nine_star_util.rs          # 九星查找表（i18n 占位符版）
│   ├── holiday_util.rs            # 法定节假日（可运行时覆盖，内部 RwLock）
│   │
│   ├── solar.rs                   # Solar 领域对象
│   ├── lunar.rs                   # Lunar 领域对象（核心 god-class）
│   ├── lunar_time.rs              # 时辰
│   ├── lunar_month.rs             # 农历月
│   ├── lunar_year.rs              # 农历年（天文计算 + 缓存）
│   ├── eight_char.rs              # 八字
│   ├── nine_star.rs               # 九星
│   ├── foto.rs                    # 佛历
│   ├── tao.rs                     # 道历
│   ├── jieqi.rs                   # 节气包装
│   ├── fu.rs                      # 三伏
│   ├── shu_jiu.rs                 # 数九
│   ├── solar_week.rs              # 阳历周
│   ├── solar_month.rs             # 阳历月
│   ├── solar_year.rs              # 阳历年
│   ├── solar_half_year.rs         # 阳历半年
│   ├── solar_season.rs            # 阳历季节
│   ├── holiday.rs                 # 节假日对象
│   │
│   ├── yun/                       # 运程
│   │   ├── mod.rs                 # Yun
│   │   ├── da_yun.rs              # DaYun
│   │   ├── liu_nian.rs            # LiuNian
│   │   ├── liu_yue.rs             # LiuYue
│   │   └── xiao_yun.rs            # XiaoYun
│   │
│   └── i18n.rs                    # 国际化（可选 feature）
│
├── tests/                         # 集成测试（移植自 lunar-go 的 *_test.go）
│   ├── solar.rs
│   ├── lunar.rs
│   ├── jieqi.rs
│   ├── eightchar.rs
│   └── ...
└── benches/                       # 性能基准
    └── convert.rs
```

### 4.3 依赖策略

```toml
[dependencies]
# 默认零依赖，保证库轻量、可嵌入/no_std 友好

[dev-dependencies]
# 仅测试期：可引入对照实现做差分测试

[features]
default = ["i18n-zh"]
i18n       = []        # 启用 i18n 框架
i18n-zh    = ["i18n"]  # 默认中文
i18n-en    = ["i18n"]  # 英文消息
serde      = ["dep:serde", ...]  # 可选序列化
```

> **决策**：移除现有 `Cargo.toml` 中的 `chrono`（仅用于 `from_date`，可改为接收 `i64` 时间戳或独立 feature）与无用的 `wifiscanner`。农历计算本身**不需要**任何时间库——`Solar` 用 6 个 `i32` 字段表示，儒略日自算。

### 4.4 编译期保证

- `#![forbid(unsafe_code)]`
- `#![warn(clippy::pedantic, clippy::nursery)]`
- 模块边界强制依赖方向：`shou_xing` ← `lunar_year` ← `lunar` ← `eight_char`（无环）。

---

## 5. 模块级 API 设计

> 命名约定：Go 的 `GetXxx()` → Rust 的 `xxx()`；`NewXxx()` → `Xxx::from_*()` 或关联函数；`IsLeap` → `is_leap`；`String()`/`ToFullString()` → 实现 `Display` + `to_full_string()`。

### 5.1 `Solar`

```rust
pub struct Solar { year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32 }

impl Solar {
    // 构造（返回 Result，非法日期不 panic）
    pub fn from_ymd_hms(y, mo, d, h, mi, s) -> Result<Solar, LunarError>;
    pub fn from_ymd(y, mo, d) -> Result<Solar, LunarError>;
    pub fn from_julian_day(jd: f64) -> Solar;
    pub fn from_unix_instant(...) -> Solar;     // 可选

    // 转换
    pub fn lunar(&self) -> Lunar;
    pub fn julian_day(&self) -> f64;

    // 推移
    pub fn next_year(&self, n: i32) -> Solar;
    pub fn next_month(&self, n: i32) -> Solar;
    pub fn next_day(&self, n: i32) -> Solar;
    pub fn next(&self, n: i32, only_workday: bool) -> Solar;
    pub fn next_hour(&self, n: i32) -> Solar;
    pub fn subtract(&self, other: &Solar) -> i32;       // 天差
    pub fn subtract_minute(&self, other: &Solar) -> i32;

    // 查询
    pub fn is_leap_year(&self) -> bool;
    pub fn week(&self) -> u32;       // 0=周日
    pub fn xingzuo(&self) -> &'static str;  // 星座
    pub fn festivals(&self) -> Vec<&'static str>;
    pub fn salary_rate(&self) -> u8;        // 1/2/3 倍薪资

    // 格式
    pub fn to_ymd(&self) -> String;         // "2020-05-01"
    pub fn to_ymd_hms(&self) -> String;
}
impl Display for Solar { ... }              // to_ymd
```

**字段类型决策**：用 `i32`（年）/ `u32`（月日时分秒）而非全 `i32`，在类型层挡住负月日，但保留 `i32` 年以表达公元前。月份 / 日有效性在构造期校验并返回 `Err`。

### 5.2 `Lunar`（核心）

```rust
pub struct Lunar {
    year: i32, month: i32 /* 负=闰 */, day: i32, hour, minute, second,
    // 预计算并缓存的干支索引（构造时一次算清）
    year_gan_index: i8, year_zhi_index: i8,
    year_gan_index_by_li_chun: i8, year_zhi_index_by_li_chun: i8,
    year_gan_index_exact: i8, year_zhi_index_exact: i8,
    month_gan_index: i8, month_zhi_index: i8,
    month_gan_index_exact: i8, month_zhi_index_exact: i8,
    day_gan_index: i8, day_zhi_index: i8,
    day_gan_index_exact: i8, day_zhi_index_exact: i8,
    day_gan_index_exact2: i8, day_zhi_index_exact2: i8,
    time_gan_index: i8, time_zhi_index: i8,
    week_index: i8,
    jie_qi: Box<[(JieQiKey, Solar); 31]>,   // 借自 LunarYear 的 Arc，或克隆
    solar: Solar,
}
```

**关键决策**：
- Go 版每个 GanZhi 索引单独 getter 触发计算；Rust 版**构造时一次性算清全部索引**（避免重复计算 + 字段紧凑用 `i8`），getter 退化为纯字段读取——**O(1) 零分配**。
- `jie_qi` 表来自 `Arc<LunarYear>`，`Lunar` 持有 `Arc` 引用而非克隆 31 个 `Solar`，省内存与分配。
- 方法按域分组，每组返回 `&'static str`（查常量表）或 `Vec<&'static str>`（节日 / 宜忌）。

方法域（对应 Go 的 ~200 方法，节选）：

| 域 | 代表方法 |
| --- | --- |
| 转换 | `lunar()`, `solar()`, `next(n)`, `time()`, `times()` |
| 干支 | `year_gan/zhi()`, `year_in_gan_zhi()`, `..._by_li_chun()`, `..._exact()`, `month_*`, `day_*(_exact/_exact2)`, `time_*`, 各 `*_index()` |
| 纳音 | `year_nayin()`, `month_nayin()`, `day_nayin()`, `time_nayin()` |
| 节气 | `jie()`, `qi()`, `jie_qi()`, `jie_qi_table()`, `next_jie()`, `prev_jie()`, `next_qi()`, `prev_qi()`, `next_jie_qi()`, `*by_whole_day()` |
| 节日 | `festivals()`, `other_festivals()` |
| 生肖 | `year_sheng_xiao()`, `..._by_li_chun()`, `..._exact()`, `month/day/time_sheng_xiao()` |
| 彭祖 | `peng_zu_gan()`, `peng_zu_zhi()` |
| 宜忌 | `day_yi/ji()`, `day_yi/ji_by_sect(sect)`, `day_ji_shen()`, `day_xiong_sha()`, `time_yi/ji()` |
| 方位 | `day_position_xi/yang_gui/yin_gui/fu/cai()(+_desc)`, `year/month/day_position_tai_sui()`, 胎神 `day/month_position_tai()` |
| 冲煞 | `day_chong()`, `day_chong_gan()`, `day_chong_gan_tie()`, `day_chong_sheng_xiao()`, `day_sha()`, `time_*` |
| 天神 | `day_tian_shen()`, `day_tian_shen_type()`, `day_tian_shen_luck()`, `time_*` |
| 杂 | `zhi_xing()`（建除）, `xiu/xiu_luck/xiu_song()`, `zheng/animal/gong/shou()`, `yue_xiang()`（月相）, `liu_yao()`（六曜）, `shu_jiu()`（数九）, `fu()`（三伏）, `hou()/wu_hou()`（物候）, `day_lu()`（日禄）, `*_nine_star()`（九星） |
| 旬空 | `*_xun()`, `*_xun_kong()` |
| 包装 | `eight_char()`, `foto()`, `tao()` |

### 5.3 `LunarTime`（时辰）

```rust
pub struct LunarTime { gan_index: i8, zhi_index: i8, lunar: Lunar }
// 方法镜像 Lunar 的 day/time 域：gan_zhi, nayin, tian_shen(_type/_luck),
// chong/sha, yi/ji, nine_star, xun/xun_kong, position_*, min_hm/max_hm
```

### 5.4 `LunarMonth` / `LunarYear`

```rust
pub struct LunarMonth {
    year: i32, month: i32, day_count: u8, index: i8,
    zhi_index: i8, first_julian_day: f64,
}
impl LunarMonth { /* gan_zhi, position_*, nine_star, next(n) */ }

pub struct LunarYear {
    year: i32, gan_index: i8, zhi_index: i8,
    months: Vec<LunarMonth>,           // 15 个（跨年边界）
    jie_qi_julian_days: [f64; 31],
}
impl LunarYear {
    pub fn from_year(year: i32) -> Arc<LunarYear>;   // 命中缓存
    fn compute(&mut self);                            // 天文 master 算法
    pub fn months_in_year(&self) -> impl Iterator + '_;
    pub fn month(&self, m: i32) -> Option<&LunarMonth>;
    pub fn leap_month(&self) -> i32;
    /* 元/运/九星/方位/杂占(get_tou_liang/get_cao_zi/...) */
}
```

### 5.5 `EightChar` + 运程

```rust
pub struct EightChar { lunar: Lunar, sect: u8 /* 1|2 */ }
impl EightChar {
    pub fn from_lunar(lunar: Lunar) -> EightChar;
    pub fn year/month/day/time() -> &'static str;        // gan_zhi
    pub fn *_hide_gan/wu_xing/nayin/shi_shen_gan/shi_shen_zhi/dishi/xun/xun_kong()
    pub fn tai_yuan/tai_xi/ming_gong/shen_gong()(+_nayin)
    pub fn yun(gender: u8, sect: u8) -> Yun;
}

pub struct Yun { gender: u8, start_year/month/day/hour: i32, forward: bool, lunar: Lunar }
impl Yun { pub fn da_yun(n) -> Vec<DaYun>; pub fn start_solar() -> Solar; }
pub struct DaYun { ... }   // gan_zhi/lis_nian(n)/xiao_yun(n)
pub struct LiuNian { ... } // gan_zhi/liu_yue()
pub struct LiuYue { ... }  // gan_zhi (月飞星式)
pub struct XiaoYun { ... } // gan_zhi
```

### 5.6 `NineStar` / `Foto` / `Tao` / 节气 / 节假日

- `NineStar { index: u8 }`：纯查表（北斗 / 玄空 / 奇门 / 太乙 4 套命名 + 吉凶 + 歌诀）。
- `Foto`：佛历，`DEAD_YEAR` 偏移；节日 / 斋期 / 27 宿。
- `Tao`：道历，`BIRTH_YEAR` 偏移；八节 / 三会 / 三元 / 五腊 / 暗戊。
- `JieQi { name, solar }` / `Fu { name, index }` / `ShuJiu { name, index }`：轻量结构体。
- `SolarWeek/Month/Year/HalfYear/Season`：阳历区间枚举，支持 `next(n)` 与迭代。
- `Holiday { day, name, work, target }` + `HolidayUtil`（`RwLock` 持有可覆盖数据）。

---

## 6. 性能策略

| 手段 | 落点 | 收益 |
| --- | --- | --- |
| **常量查表用 `&'static [&'static str]`** | 所有 `tables.rs` | 零分配，getter 返回 `&'static str` |
| **构造期预计算 + 紧凑字段 (`i8`)** | `Lunar` 23 个索引 | getter O(1)；`Lunar` 约 32 字节（不含 `Arc`） |
| **`Arc<LunarYear>` 年级缓存** | `LunarYear::from_year` + 全局 `OnceLock<RwLock<LruCache>>` | 同年重复构建的天文 pass 只算一次 |
| **`jie_qi` 表 `Arc` 共享** | `Lunar` 持 `Arc<LunarYear>` 而非克隆 | 31×`Solar` 不复制 |
| **浮点避免 `Math.random`/`Date.now`** | 天文引擎纯函数 | 可缓存、确定性、可并行 |
| **迭代器而非 `Vec` 收集** | `months_in_year`, `festivals` | 调用方按需消费，省分配 |
| **`#[inline]` 热路径** | `julian_day`, `next_day`, 索引 getter | 跨 crate 内联 |
| **`Cow<'static, str>`** | i18n 开启时部分名称需格式化 | 默认 `Borrowed` 零分配 |
| **LTO + codegen-units=1** | release profile | 全局优化 |

**缓存设计**：默认一个有界 LRU（容量 64 年）。`Solar→Lunar` 连续遍历同一年的日期时，天文 pass 仅触发一次。线程安全用 `RwLock`（读多写少）；无锁路径在 `try_write` 失败时回退为直接计算（不阻塞）。

> **正确性优先**：缓存仅是加速，**不可**缓存任何会因 i18n / 节假日覆盖而变的内容（那些在 `Lunar` 方法层动态查表）。

---

## 7. 错误处理策略

### 7.1 错误类型

```rust
#[non_exhaustive]
pub enum LunarError {
    InvalidSolar { year: i32, month: i32, day: i32, /* ... */ },
    InvalidSolarHour { /* ... */ },
    GregorianGap { y: i32, m: i32, d: i32 },   // 1582-10-05..14 不存在
    InvalidLunar { year: i32, month: i32, day: i32 },
    LunarDayOverflow { year: i32, month: i32, day: i32, max: i32 },
    LeapMonthAbsent { year: i32, month: i32 }, // 指定闰月但该年无此闰月
}
```

### 7.2 策略

- **构造期**（`Solar::from_ymd_hms`、`Lunar::from_ymd`）→ `Result<_, LunarError>`。非法输入返回 `Err`，**绝不 panic**（Go 版在此 `panic`，Rust 化改进）。
- **查表/查找**（`LunarYear::month`、`get_near_jie_qi`）→ `Option<T>`。
- **业务方法**（getter）→ 不会失败，返回 `&'static str` 或 `Vec`。
- 提供 `try_` 变体与“就地 panic”便捷构造（`from_ymd_unchecked`，标 `#[track_caller]`），供测试 / 内部确定合法的场景。

---

## 8. 国际化 (i18n) 策略

### 8.1 移植 JS 的占位符重写

JS 版的精妙处：查找表存占位符（`'{tg.jia}'`），`setLanguage` 时正则替换。Rust 移植：

```rust
// 默认：tables.rs 直接存中文 &'static str（零开销，90%+ 用户场景）
// feature = "i18n"：
//   - tables 存占位符模板
//   - I18n 持消息字典（LazyLock<RwLock<HashMap<Lang, HashMap<&str, &str>>>>）
//   - 取值走 message(key) → 启动时或 setLanguage 时把模板重写进运行期数组
```

### 8.2 取舍

- **默认关闭 i18n**：常量表直接是 `&'static str`，无 `RwLock`、无 `HashMap` 查找，最快。
- 启用 i18n 时接受一次 `O(总表项)` 的重写开销（启动期），之后仍为 `&'static str` 级访问（重写后的表同样常驻）。
- 内置 `chs`（简中）、`en`（英）；`set_messages(lang, dict)` 允许用户扩展。

---

## 9. 测试策略

### 9.1 黄金用例：移植 `_test.go`

将 lunar-go 的 `test/*_test.go`（Lunar/Solar/JieQi/EightChar/NineStar/Foto/Tao/Yun/XingZuo/WuHou/LunarMonth/ShuJiu/Fu/LiuYao/Holiday/GanZhi/Year/SolarUtil/SolarWeek 等）逐条翻译为 Rust 集成测试，**断言字符串完全相等**。这是**首要验收标准**。

例（来自 `Lunar_test.go::TestLunar1`）：

```rust
#[test]
fn lunar_full_string() {
    let lunar = Lunar::from_ymd(2019, 3, 27).unwrap();
    assert_eq!(lunar.to_string(), "二〇一九年三月廿七");
    assert_eq!(lunar.to_full_string(),
        "二〇一九年三月廿七 己亥(猪)年 戊辰(龙)月 戊戌(狗)日 子(鼠)时 \
         纳音[平地木 大林木 平地木 桑柘木] 星期三 西方白虎 \
         星宿[参水猿](吉) 彭祖百忌[戊不受田田主不祥 戌不吃犬作怪上床] \
         喜神方位[巽](东南) 阳贵神方位[艮](东北) 阴贵神方位[坤](西南) \
         福神方位[艮](东北) 财神方位[坎](正北) 冲[(壬辰)龙] 煞[北]");
}
```

### 9.2 差分测试 (Differential Testing)

写一个 `tests/diff.rs`（`#[ignore]` 默认不跑，CI 定期触发）：对 1900–2100 年每日 + 随机历史日期，调用 Go 参考实现（通过子进程 / FFI / 预生成 JSON 快照）比对全字段输出。

### 9.3 边界用例

- 1582 历法改革（10/4 → 10/15，跳 10 天）。
- 闰月年（如 2020 闰四月、2033 闰冬月、2025 闰六月、公元 37 闰腊月）。
- 跨年节气边界（正月初一 vs 立春）。
- 晚子时（23:00–23:59）日柱 Exact 变体。
- 公元前 / 远古代（受天文范围约束，断言不 panic 即可）。

### 9.4 性能基准 (`benches/`)

`criterion` 基准：`Solar::from_ymd().lunar()` 冷启动 vs 热缓存、`to_full_string()`、`JieQi` 表构建。

---

## 10. 实施路线图

分 5 个里程碑（M），每个 M 可独立编译、测试、交付。

| 里程碑 | 内容 | 验收 |
| --- | --- | --- |
| **M1 基础设施** | `error.rs`、`Cargo.toml`、mod 骨架、`solar_util.rs`、`Solar`（构造 / 推移 / 儒略日 / 星期 / 星座 / 节日） | `Solar_test.go` 全过 |
| **M2 天文引擎** | `shou_xing/`（data + engine，`calc_qi/calc_shuo/qi_accurate2`） | 单测：已知节气儒略日对照（如 2012 白露 = `2012-09-07 13:29:01`） |
| **M3 农历核心** | `lunar_util/`（常量 + 解码）、`LunarYear`/`LunarMonth`（master compute + 缓存）、`Lunar`（转换 + 干支 + 节气 + 节日 + 生肖 + 位置 + 冲煞 + 宜忌 + 天神 + 建除 + 宿 + 月相 + 数九 + 三伏 + 物候 + 九星）、`LunarTime` | `Lunar_test.go` / `JieQi_test.go` / `GanZhi` / `XingZuo` / `WuHou` / `ShuJiu` / `Fu` / `LiuYao` / `LunarMonth` / `Year` 全过 |
| **M4 八字 + 运程** | `EightChar`、`yun/*` | `EightChar_test.go` / `Yun_test.go` 全过 |
| **M5 周边 + i18n + 交付** | `NineStar`、`Foto`/`FotoUtil`、`Tao`/`TaoUtil`、`SolarWeek/Month/Year/HalfYear/Season`、`Holiday`/`HolidayUtil`、`i18n`、serde、文档、bench | 全部 `_test.go` 移植过；差分测试；README + doc-tests |

### 10.1 本轮编码交付范围

本轮（本次会话）将完成 **M1–M3 的核心**并打通 **M4 关键路径**，确保：

- `Solar ⇄ Lunar` 双向转换正确（含闰月、1582、跨年）。
- 干支（三种 `sect`）、节气表、节日、生肖、纳音、彭祖、方位、冲煞、宜忌、天神、建除、二十八宿、月相、数九、三伏、物候、九星等核心方法可用且过测。
- `EightChar` 四柱 + 十神 + 藏干 + 纳音 + 地势 + 胎元 / 命宫 + `Yun` 基础可用。
- 移植的 Go 测试用例作为集成测试通过。

M5 的佛历 / 道历 / 节假日 / Solar 区间类型 / i18n / serde 在结构就绪后按文档快速补齐（数据为主，逻辑简单）。

---

## 11. 风险与决策记录

| # | 议题 | 决策 | 理由 |
| --- | --- | --- | --- |
| R1 | 浮点末位差异致节气分钟偏移 | `f64` + 严格对齐 Go 运算顺序；用黄金用例回归 | 历算对精度极敏感，差 1 秒可能跨分钟 |
| R2 | Go 负数取模 vs Rust `%` | 逐处用 `rem_euclid` 或手动 `+n` 修正 | Rust `%` 对负数向零取整，与 Go 不同 |
| R3 | `XL0/XL1` 巨型数据搬移 | 用 `const` 数组 + `include!` 或独立 `data.rs` | 避免污染主逻辑，便于校对 |
| R4 | 紧缩串 `dayYiJi/timeYiJi/dayShenSha` | 原样复制 + 复刻 `hex`/解码逻辑 | 宜忌 / 神煞全靠它，不可简化 |
| R5 | i18n 是否默认开 | 默认关（中文直存） | 满足性能目标与 90% 用户；i18n 作 feature |
| R6 | 缓存线程安全粒度 | 全局 `LruCache<year, Arc<LunarYear>>` + `RwLock` | 读多写极少；`Arc` 共享避免深拷贝 |
| R7 | 错误用 `Result` 还是 `Option` | 构造 `Result`，查表 `Option`，getter 不失败 | 符合 Rust 习惯，调用方灵活 |
| R8 | 是否引入 `chrono` | 否（默认），`from_unix` 用 `i64` 秒 | 减依赖；`Solar` 自带儒略日计算 |
| R9 | JS 内联 Yun vs Go 真类 | 采 Go 真类 | API 清晰、可测、符合 Rust 习惯 |
| R10 | `docs/` 提交 | 已在 `.gitignore`，禁止提交 | 用户明确要求 |

---

## 附录 A：术语速查

- **干支 GanZhi**：天干（甲…癸，10）+ 地支（子…亥，12）→ 六十甲子（60）。
- **节气 JieQi**：24 个，含 12 节（节令）+ 12 中气；冬至为天文年起算。
- **合朔 Shuo**：日月黄经差为 0，即农历初一。
- **ΔT**：地球自转不均匀性修正，TT−UT。
- **纳音 NaYin**：六十甲子对应的五行音（海中金…），30 对。
- **十神 ShiShen**：以日干为我，其余天干的生克关系名。
- **九星 NineStar**：一白…九紫，九宫飞星，含玄空 / 北斗 / 奇门 / 太乙四派命名。
- **sect**：年起算约定（1 正月初一 / 2 立春日 / 3 立春时刻），命理流派分歧点。

## 附录 B：参考文件索引（移植源头）

| Rust 模块 | Go 源文件 | JS 源（行） |
| --- | --- | --- |
| `shou_xing/` | `ShouXingUtil/ShouXingUtil.go` | `lunar.js` ShouXingUtil (2970–3590) |
| `solar_util.rs` / `solar.rs` | `SolarUtil/` + `calendar/Solar.go` | Solar (13–629) |
| `lunar_util/` | `LunarUtil/LunarUtil.go` | LunarUtil (3883–5349) |
| `lunar_year.rs` / `lunar_month.rs` | `calendar/LunarYear.go` / `LunarMonth.go` | LunarYear (2459–2788) / LunarMonth (2789–2969) |
| `lunar.rs` / `lunar_time.rs` | `calendar/Lunar.go` / `LunarTime.go` | Lunar (630–1972) / LunarTime (6038–6131) |
| `eight_char.rs` / `yun/` | `calendar/EightChar.go` / `Yun.go` / `DaYun.go` / `LiuNian.go` / `LiuYue.go` / `XiaoYun.go` | EightChar (5636–6037) |
| `nine_star.rs` / `nine_star_util.rs` | `calendar/NineStar.go` | NineStar (5569–5635) / NineStarUtil (6741–6810) |
| `foto.rs` / `foto_util.rs` | `calendar/Foto.go` / `FotoUtil/FotoUtil.go` | Foto (6465–6590) / FotoUtil (6132–6464) |
| `tao.rs` / `tao_util.rs` | `calendar/Tao.go` / `TaoUtil/TaoUtil.go` | Tao (6811–6913) / TaoUtil (6614–6740) |
| `holiday.rs` / `holiday_util.rs` | `HolidayUtil/` | HolidayUtil (5350–5568) |
| `solar_week/month/year/...` | `calendar/SolarWeek.go` 等 | SolarWeek… (1973–2458) |
| `i18n.rs` | （Go 无） | I18n (6914–8513) |
