# Task 87: 8 相天文月相对象设计与实现

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs v1.5` 的 `Phase` 表示 8 相天文月相：

- `新月`
- `蛾眉月`
- `上弦月`
- `盈凸月`
- `满月`
- `亏凸月`
- `下弦月`
- `残月`

本地已有 `Phase` / `PhaseDay`，但语义是农历日月相名称，例如 `朔`、`既朔`、`望`。如果直接改现有 `Phase`，会破坏既有 API 语义。因此本任务采用新增独立对象的方式对标：

- `MoonPhase`
- `MoonPhaseDay`
- `Lunar::moon_phase()`
- `Lunar::moon_phase_day()`

## 子任务拆分

1. 新增 8 相月相常量表。
2. 在寿星天文引擎中暴露月相儒略日计算函数。
3. 新增 `MoonPhase`，支持 `from_index`、`from_name`、`next`、`solar_time`、`solar_day`。
4. 新增 `MoonPhaseDay`，支持名称、相位对象与日序输出。
5. 接入 `NamedCulture` 与 `CultureDay`。
6. 在 crate 根导出新对象。
7. 在 `Lunar` 上新增日级查询入口。
8. 用 `tyme4rs` 样例锁定：`2023-09-17 -> 蛾眉月第2天`。

## 实现说明

月相定位按 `tyme4rs` 的策略处理：

- 先从当前农历月的下一个月定位新月；
- 如果该相位公历日落在目标日期之后，则逐个向前回退相位；
- 奇数相位按上游规则顺延一天；
- 比较月首时使用日粒度，避免“同一天但时刻更早”被误判为上一周期。

## 验证计划

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

完成后进入 `Task 88`：年月周单位对象 API 差异评估。
