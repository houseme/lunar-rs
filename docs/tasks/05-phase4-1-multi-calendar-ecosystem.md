# Task 05：Phase 4.1 多历法生态深化

> 任务编号：`Task 05`
> 优先级：`P4.1`
> 当前状态：`已完成`
> 最近更新：`2026-07-06`
> 当前进度：`5/5`

---

## 1. 任务目标

在 `Phase 4` 首轮已经落地 `Hijri` 与 `RabByung` 的基础上，继续把多历法能力从“可用对象”推进到“可组合、可复用、可继续扩展的多历法生态”，重点包括：

- 深化 `Hijri / RabByung` 的 year/month/day companion 与边界工具；
- 补齐它们和 `Event` 体系的更深复用；
- 明确多历法对象之间的统一接口形态；
- 为后续继续增加新历法对象建立稳定模式。

---

## 2. 当前现状

已完成：

- `Hijri`
  - `Hijri`
  - `HijriYear`
  - `HijriMonth`
  - `Solar::hijri()`
  - `events()/find_events()/events_until()` 代理接口
- `RabByung`
  - `RabByungElement`
  - `RabByungYear`
  - `RabByungMonth`
  - `RabByungDay`
  - `Solar::rab_byung_year()`
  - `Solar::rab_byung_day()`
  - `events()/find_events()/events_until()` 代理接口（`RabByungDay`）
- 两套历法的基础回归测试已通过。

当前收口结论：

- `Hijri / RabByung` 的 year/month/day companion 已形成稳定统一形态；
- `Hijri / RabByung / Foto / Tao` 已全部验证 year/month 层模板可复用；
- 多历法对象与 `Event` 体系的复用边界已经清晰；
- 模板文档已足够作为后续新增历法对象的标准参考。

---

## 3. 本任务建议范围

### 3.1 需要完成

1. 为 `Hijri / RabByung` 统一补齐 year/month/day 常用 companion：
   - 层级导航；
   - 范围包含；
   - 相减/推进；
   - 首项/枚举；
   - 与 `Solar` 的互转辅助。
2. 评估并补充更深的事件复用：
   - `HijriYear / HijriMonth`
   - `RabByungYear / RabByungMonth`
   - 是否需要自己的历法事件源或别名事件。
3. 总结并固化多历法扩展模板：
   - 一个历法对象至少要有哪些层级；
   - 哪些 typed helper 是必需的；
   - 哪些测试维度必须补。
4. 补文档与测试，把 `Phase 4` 从“首次落地”推进到“可继续复制的方法论”。

### 3.2 不包含

- 再新增第三个新历法；
- 复杂国际节日数据库；
- 前端 UI 或外部服务层输出改造；
- Phase 5 的反推/求解能力。

---

## 4. 候选实现方向

### 方向 A：继续深化 `Hijri`

- `HijriYear / HijriMonth` 增加更细的 companion；
- 看是否值得引入 `HijriDay` 别名层（如果未来要统一三层命名）。

适合：

- 想先把已经最稳定的第二历法对象打磨完整；
- 想尽快形成“一个历法对象的标准模板”。

### 方向 B：继续深化 `RabByung`

- `RabByungMonth / RabByungDay` 补更多辅助能力；
- 再决定是否需要 `RabByung` 专属事件或节名体系。

适合：

- 想尽快把第二个多历法对象从“可跑”提升到“可长期维护”。

### 方向 C：抽象多历法统一模式

- 提炼 `Solar/Lunar/Hijri/RabByung` 共同能力；
- 先形成文档/接口模板，再决定继续补哪个历法。

适合：

- 想减少后面扩更多历法时的重复工作。

当前推荐：

- `A + C` 组合

原因：

- `Hijri` 规则简单、验证稳定，最适合作为统一模板的第一个样板；
- 做完后再反向校正 `RabByung` 的接口形态，成本更低。

---

## 5. 进度清单

- [x] 审计 `Hijri / RabByung` 剩余 companion 缺口
- [x] 选定优先深化对象与范围
- [x] 补齐 `Hijri` year/month/day 统一 companion
- [x] 补齐 `RabByung` year/month/day 统一 companion
- [x] 补齐更深的事件复用或历法事件策略
- [x] 整理多历法扩展模板文档
- [x] 用第三个历法对象验证模板可复用性（`FotoYear` / `FotoMonth`）
- [x] 用第四个历法对象验证模板可复用性（`TaoYear` / `TaoMonth`）

---

## 6. 验收标准

达到以下条件可视为 `Task 05` 完成：

1. `Hijri / RabByung` 至少一套达到稳定统一的 year/month/day 生态形态；
2. 多历法对象与 `Event` 体系的复用边界清晰；
3. 新增一个历法对象时的实现模板可以从文档中直接复用；
4. `cargo test` 与必要的专测全绿；
5. 后续继续扩展多历法时，不需要再从零重新设计结构。

---

## 7. 建议验证命令

```bash
cargo test --test hijri
cargo test --test rab_byung
cargo test
cargo test --features serde
```

---

## 8. 验证记录

### 2026-07-06

- 状态：`已完成`
- 已验证：
  - `cargo test --test foto`
  - `cargo test --test hijri`
  - `cargo test --test rab_byung`
  - `cargo test --test tao`
  - `cargo test`
  - `cargo test --features serde`
- 当前已完成：
  - `Hijri` 的 year/month/day companion 继续补强
  - `RabByung` 的 year/month/day companion 继续补强
  - `Hijri / RabByungDay / HijriYear / HijriMonth / RabByungYear / RabByungMonth` 与 `Event` 体系的更深复用
  - `FotoYear` / `FotoMonth` 的模板化落地
  - `TaoYear` / `TaoMonth` 的模板化落地
  - 多历法扩展模板文档：[docs/multi-calendar-template.md](/Users/zhi/Documents/code/rust/houseme/lunar-rs/docs/multi-calendar-template.md)
- 结论：
  - `Task 05` 已证明模板在 `Hijri / RabByung / Foto / Tao` 上都可复用；
  - 后续如果继续推进，应直接开新任务做“第五个历法对象”或“多历法共享抽象”。

---

## 9. 变更记录

### 2026-07-06

- 创建任务文档。

### 2026-07-06

- 状态改为 `进行中`。
- 已沿 `Hijri + 模板化` 路线完成第一轮实现与回归。

### 2026-07-06

- 状态改为 `已完成`。
- 已完成第四个历法对象模板化验证（`TaoYear` / `TaoMonth`）。
