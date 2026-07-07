# Task 107: 差分文化字段继续扩容

状态：已完成
最近更新：2026-07-07
上游基准：`6tail/tyme4rs` `ba6ab75`（2026-06-15，v1.5.0）

## 对标结论

`Task 105` 已把 `tyme4rs` 外部 bridge 跑通，`Task 102` 已把协议扩到 `v3`。继续收口时，最值得补的不是展示字符串，而是两边语义已经明确一致的文化字段：

- `solar_nine_star`
- `lunar_six_star`
- `lunar_minor_ren`
- `lunar_twelve_star`
- `lunar_twenty_eight_star`
- `lunar_nine_star`

这些字段都来自已经完成的 typed 兼容层，而且能够明显提升差分矩阵的判别力。后续又继续补了一批 `LunarHour` 级别的稳定字段：

- `lunar_hour_name`
- `lunar_hour_index_in_day`
- `lunar_hour_minor_ren`

## 子任务

1. 把差分协议扩到 `v4`，随后继续扩到当前 `v6`。
2. 更新本地 `solar_snapshot` 输出上述文化字段。
3. 更新 `tyme4rs` bridge 输出同名字段。
4. 更新 `tests/differential.rs` 与 `tests/differential_protocol.rs`。
5. 保持外部桥接脚本仍可一键跑通 ignored differential test。

## 验证

- `cargo test --test differential_protocol`
- `bash scripts/run_tyme4rs_diff_check.sh`
- `cargo test`

当前状态补充：

- 差分协议已从本任务最初的 `v4` 继续演进到当前 `v6`；
- 外部 `tyme4rs` bridge 已能跑通当前 `v6` 样例矩阵；
- 对已确认存在实现口径分叉的文化字段，外部 `tyme4rs` flavor 仅保留本地协议覆盖，不做严格断言。

## 后续

后续继续扩容时，建议优先顺序是：

1. `LunarMonth` / `LunarYear` 级别的 `nine_star` / `minor_ren`
2. 如果需要，再为 `LunarHour` 级别补更多稳定字段
3. 继续避开仍有语义分叉的 `phase` / `phase_day`
