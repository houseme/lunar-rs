# Task 02：Phase 2 typed API 第二阶段补完

> 任务编号：`Task 02`
> 优先级：`P2`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`5/5`

---

## 1. 任务目标

继续把当前仍停留在字符串层的黄历/民俗/文化概念对象化，降低 `Lunar` 作为 god object 的直接字符串输出职责，让 API 更适合：

- 组合使用；
- 序列化；
- 事件复用；
- 多历法扩展。

---

## 2. 当前现状

已完成：

- 第一批 typed primitive 已存在：
  - `HeavenStem`
  - `EarthBranch`
  - `SixtyCycle`
  - `Direction`
  - `Element`
  - `Zodiac`
  - `Duty`
  - `Phase`
  - `Phenology`
- 本轮已补：
  - `God`
  - `Taboo`
  - `PengZu`
  - `DogDay`
  - `PlumRainDay`
- 本次增量已补：
  - `TianShen`
  - `TianShenType`
  - `Xiu`
  - `XiuAnimal`
  - `Shou`
  - `Lu`
  - `ChongSha`
  - `Fu` / `ShuJiu` 的 day-level 统一语义补强
- 本次继续补：
  - `Xun`
  - `XunKong`
  - `TaiSuiPosition`
  - `TaiPosition`
- 本次继续向下扩层：
  - `LunarYear` 的 `HeavenStem` / `EarthBranch` / `SixtyCycle` / `Xun` / `TaiSuiPosition`
  - `LunarMonth` 的 `HeavenStem` / `EarthBranch` / `SixtyCycle` / `Xun` / `TaiSuiPosition`
  - `LunarTime` 的 `HeavenStem` / `EarthBranch` / `SixtyCycle` / `TianShen` / `ChongSha` / `Xun`
- 本次继续补高频 companion：
  - `Nayin`
  - `Season`
  - `LiuYao`
- 本次继续补剩余高频字符串概念：
  - `YuanCycle`
  - `YunCycle`
  - `YearFortune`
  - `YearFortuneKind`

当前收口结论：

- `Lunar` 主路径上的高频文化概念已经基本完成 typed 化；
- `LunarYear` / `LunarMonth` / `LunarTime` 已补齐同类高价值 companion；
- 剩余字符串接口主要是：
  - 原始兼容 getter；
  - 展示/格式化输出；
  - 节名列表等集合型原始数据；
  - `Foto` / `Tao` 等边缘层的低收益包装；
- 继续为这些剩余项补第三批对象，边际收益已经明显下降。

---

## 3. 实现范围

### 3.1 本任务已完成内容

1. 盘点剩余纯字符串文化输出接口。
2. 为高价值概念继续补 typed domain：
   - `God` 相关域对象增强；
   - `Taboo` 相关域对象增强；
   - `PengZu` 结构化增强；
   - `Fu` / `ShuJiu` / `DogDay` / `PlumRainDay` 之间的职责统一；
   - 视需要继续补 `PhaseDay`、`PhenologyDay` 一类对象。
3. 给 `Lunar` 增加 typed getter，同时保留兼容字符串 getter。
4. 明确哪些对象应该：
   - `Copy`
   - `Clone`
   - `Serialize`
   - `Display`
5. 补 typed API 的测试矩阵。

### 3.2 本任务不包含

- 引入 `Hijri` / `RabByung`；
- 八字逆推；
- 彻底删除旧字符串 API；
- 大规模 breaking change。

---

## 4. 优先对象顺序

### 第一层：继续补文化对象

- [ ] `God` 扩展能力与分类语义
- [ ] `Taboo` 扩展能力与分类语义
- [ ] `PengZu` 进一步结构化
- [ ] `Fu` / `DogDay` 关系收口
- [ ] `ShuJiu` / `Phenology` / `Phase` 的 day-level 语义补完

### 第二层：收口 `Lunar` typed getter

- [ ] 统一命名风格
- [ ] 避免重复字符串转换
- [ ] 补缺少的 typed companion getter

### 第三层：序列化与文档

- [ ] 检查新增对象的 `serde` 派生覆盖
- [ ] 补 API 注释
- [ ] 补 README/CHANGELOG 中的 typed API 说明

---

## 5. 建议拆分步骤

### Step 1：剩余字符串接口盘点

- 逐项列出仍只返回字符串的高价值接口；
- 评估哪些应先对象化，哪些保留字符串即可。

### Step 2：先补核心 domain 类型

- 先做能被多个调用方复用的对象；
- 避免只为单个 getter 增加“形式上的 struct”。

### Step 3：补 typed getter

- 在 `Lunar` 上增加新 getter；
- 保持旧 getter 向后兼容；
- 优先减少业务侧再次手工解析字符串的需要。

### Step 4：补测试与序列化

- 新对象行为测试；
- `serde` feature 下编译验证；
- 与现有字符串结果的一致性验证。

---

## 6. 验收结果

已满足：

1. 新增 typed 对象已覆盖剩余高价值字符串文化概念；
2. `Lunar` 的 typed getter 已明显比初始阶段完整；
3. 旧字符串 API 仍可正常使用；
4. `cargo test` 与 `cargo test --features serde` 已通过；
5. 新对象命名、职责、序列化边界已保持一致。

---

## 7. 风险点

- 如果对象切得太碎，会让 API 变重而收益不大；
- 如果继续把所有逻辑都留在 `Lunar`，typed 化会沦为薄包装；
- 新增对象后若缺少统一命名规则，后续会越来越难维护。

---

## 8. 建议验证命令

```bash
cargo test
cargo test --features serde
```

必要时追加：

```bash
cargo test --test phase2_typed
```

---

## 9. 进度清单

- [x] 完成剩余字符串型高价值接口盘点
- [x] 确定第二批 typed domain 名单
- [x] 补齐第一批 `Lunar` typed getter
- [x] 校验新增对象的 `serde` 覆盖
- [x] 扩展 `tests/phase2_typed.rs`
- [x] 继续补第二批高价值对象与 companion getter
- [x] 把已落地对象扩到 `LunarYear` / `LunarMonth` / `LunarTime`
- [x] 完成低频字符串概念审计并确认无需继续补第三批对象

---

## 10. 验证记录

### 2026-07-06

- 状态：`已完成`
- 备注：已完成第二阶段 typed domain 主线构建，并完成低频字符串概念审计，确认无需继续补第三批对象。
- 已验证：
  - `cargo test --test phase2_typed`
  - `cargo test`
  - `cargo test --features serde`
- 当前已落地对象：
  - `TianShen` / `TianShenType`
  - `Xiu` / `XiuAnimal` / `Shou`
  - `Lu`
  - `ChongSha`
  - `Xun` / `XunKong`
  - `TaiSuiPosition`
  - `TaiPosition`
  - `Nayin`
  - `Season`
  - `LiuYao`
  - `YuanCycle`
  - `YunCycle`
  - `YearFortune`
  - `YearFortuneKind`
  - `Fu` / `ShuJiu` day-level 统一语义
- 当前已扩展层级：
  - `Lunar`
  - `LunarYear`
  - `LunarMonth`
  - `LunarTime`
- 审计结论：
  - 应保留为字符串/集合的接口：
    - 中文展示值：`year_in_chinese` / `month_in_chinese` / `day_in_chinese`
    - 节名列表：`festivals` / `other_festivals` / `jie_qi_list`
    - 原始兼容 getter：`gan` / `zhi` / `gan_zhi` / `*_desc`
  - 暂不继续 typed 化的低收益区域：
    - `Foto` / `Tao` 的边缘文化字段
    - 纯格式化输出与长文本说明
  - 因此 `Task 02` 可以关闭。

---

## 11. 变更记录

### 2026-07-06

- 创建任务文档。

### 2026-07-06

- 状态改为 `进行中`。
- 已完成 Task 02 的第一批 typed domain 与测试闭环。

### 2026-07-06

- 已完成 Task 02 的第二批 typed domain 与测试闭环。
- 已把 `Xun / XunKong / TaiSui / Tai` 周边对象接入 `Lunar`。

### 2026-07-06

- 已把核心 typed 对象继续扩到 `LunarYear` / `LunarMonth` / `LunarTime`。
- 已补对应的 typed getter 与 `phase2_typed` 验证用例。

### 2026-07-06

- 已补 `Nayin` / `Season` / `LiuYao` 的 typed companion。
- 已把 `Nayin` companion 扩到 `Lunar` / `LunarYear` / `LunarMonth` / `LunarTime`。

### 2026-07-06

- 已补 `Yuan / Yun / 杂占类` 的 typed wrapper 与测试。
- 已把 `LiuYao / Yuan / Yun / 杂占类` 这批剩余高频字符串概念压到 typed companion 层。

### 2026-07-06

- 已完成剩余低频字符串概念审计。
- 结论：不再继续补第三批对象，`Task 02` 标记为 `已完成`。
