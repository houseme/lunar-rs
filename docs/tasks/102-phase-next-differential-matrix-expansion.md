# Task 102: 差分协议与样例矩阵扩容

状态：已完成
最近更新：2026-07-07
上游基准：`6tail/tyme4rs` `ba6ab75`（2026-06-15，v1.5.0）

## 对标结论

前面 `Task 97` - `Task 100` 已补完 `SolarFestival` / `LunarFestival`、`LunarYear` / `LunarMonth` 严格 getter，以及本地 `Foto` / `Tao` wrapper 兼容层，但外部差分校验一开始还只覆盖：

- 公历字符串
- 农历字符串
- 节气
- 年月日时干支

这意味着最近新增的兼容面即使回归，现有差分协议也感知不到。因此本任务先把差分协议升级到 `v2`，随后继续扩展到当前落地的 `v3`，并把新增协议字段落到 sample matrix 和本地稳定性测试里。

## 子任务

1. 升级 `src/differential_support.rs` 协议版本与键集合。
2. 为 `solar_snapshot` 增加以下稳定字段：
   - `solar_festival`
   - `solar_festival_index`
   - `lunar_festival`
   - `lunar_festival_index`
   - `lunar_year_month_count`
   - `lunar_year_leap_month`
   - `lunar_month`
   - `lunar_month_with_leap`
   - `lunar_month_day_count`
   - `lunar_month_index_in_year`
   - `week_name`
   - `week_index`
   - `constellation`
   - `legal_holiday`
   - `legal_holiday_work`
3. 扩大 `tests/fixtures/differential_cases.txt`，补节日、闰月、清明、除夕等样例。
4. 更新 `tests/differential.rs` 与 `tests/differential_protocol.rs`，让本地协议稳定测试能直接覆盖新增字段。

## 关键样例

- `2019-02-05 00:00:00`：春节
- `2020-01-24 00:00:00`：除夕
- `2020-05-23 00:00:00`：闰四月初一
- `1985-09-10 00:00:00`：教师节起始年样例
- `1949-10-01 00:00:00`：国庆节起始年样例
- `2024-04-04 12:00:00`：清明节 / 清明节气
- `2024-09-17 00:00:00`：中秋节
- `2024-10-01 00:00:00`：国庆节法定假日
- `2024-02-18 00:00:00`：春节调休工作日

## 验证

- `cargo test --test differential_protocol`
- `cargo test --test differential parses_default_case_matrix`
- `cargo test`

说明：`tests/differential.rs` 的外部 reference 主测试仍按设计保持 `ignored`，需要设置 `LUNAR_RS_DIFF_REF_BIN` 后才会执行。

当前状态补充：

- 协议现已从最初的 `v1` 迭代到当前代码中的 `v5`；
- `v3` 额外覆盖星期、星座和法定假日字段；
- `v4` / `v5` 继续加入九星、六曜、小六壬、十二神、二十八宿，以及文化日对象字符串；
- 本地稳定性测试与外部 `tyme4rs` bridge 都已跑通当前样例矩阵；
- 对已确认存在实现口径差异的文化字段，外部 `tyme4rs` flavor 仅保留本地协议覆盖，不做严格断言。

## 后续

后续如果继续收口，优先级建议是：

1. 给外部 reference driver 同步 v2 协议，真正跑通节日与闰月差分。
2. 再补 README/API 迁移文档，说明本地 `Option` 风格与 tyme4rs panic 构造的差异。
