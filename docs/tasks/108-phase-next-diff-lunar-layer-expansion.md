# Task 108: 差分中的 LunarYear/LunarMonth 文化字段扩容

状态：已完成
最近更新：2026-07-07
上游基准：`6tail/tyme4rs` `ba6ab75`（2026-06-15，v1.5.0）

## 对标结论

`Task 107` 已把差分协议扩到 `v6`，补进了一批日级与时级稳定文化字段。继续收口时，最自然的一步就是把同样稳定、而且已经完成兼容层实现的 `LunarYear` / `LunarMonth` 字段纳入外部差分：

- `lunar_year_twenty`
- `lunar_year_nine_star`
- `lunar_month_minor_ren`
- `lunar_month_nine_star`

这几项都来自已经完成的 getter 兼容层，不需要额外引入新的对象建模。

## 子任务

1. 将差分协议扩到 `v7`。
2. 更新本地 `solar_snapshot`，输出 `LunarYear` / `LunarMonth` 稳定文化字段。
3. 更新 `tyme4rs` bridge 输出同名字段。
4. 更新 `tests/differential.rs` 与 `tests/differential_protocol.rs`。
5. 复用现有样例矩阵，优先让闰月与既有年份样例覆盖这些字段。

## 验证

- `cargo test --test differential_protocol`
- `bash scripts/run_tyme4rs_diff_check.sh`
- `cargo test`

## 后续

后续继续扩容时，建议优先顺序：

1. `LunarHour` 级别继续补 `twelve_star` / `nine_star`
2. `LunarMonth` / `LunarYear` 级别继续补 `jupiter_direction`
3. 继续把 `phase` / `phase_day` 这类已确认有定义分叉的字段留在本地协议，不做外部硬断言
