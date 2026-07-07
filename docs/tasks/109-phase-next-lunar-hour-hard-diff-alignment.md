# Task 109: LunarHour 稳定字段外部硬对齐

状态：已完成
最近更新：2026-07-07
上游基准：`6tail/tyme4rs` `ba6ab75`（2026-06-15，v1.5.0）

## 对标结论

`Task 107` 与 `Task 108` 已把 `LunarHour` 的一些稳定字段放进差分协议，但最初仍只在本地协议侧断言，因为当时还不确定这些字段和 `tyme4rs` 是否完全同义。

继续核对后发现：

- `lunar_hour_name`
- `lunar_hour_index_in_day`
- `lunar_hour_minor_ren`
- `lunar_hour_nine_star`

这几项已经能和 `tyme4rs` 直接硬对比；真正没对齐的是 `lunar_hour_twelve_star`，原因是本地仍在走 `tian_shen` 名称映射，而 tyme4rs 用的是基于时柱地支与日柱地支的公式。

## 子任务

1. 修正本地 `LunarTime::get_twelve_star()` 公式，对齐 tyme4rs。
2. 把 `LunarHour` 稳定字段从“本地协议 only”升级成外部 hard diff。
3. 继续复用既有 solar case matrix，不额外引入新的参考驱动格式。

## 实施结果

- [src/lunar_time.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/lunar_time.rs)
  - `get_twelve_star()` 改为公式推导，不再依赖 `tian_shen` 名称映射。
- [tests/differential.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/tests/differential.rs)
  - `lunar_hour_name`
  - `lunar_hour_index_in_day`
  - `lunar_hour_minor_ren`
  - `lunar_hour_twelve_star`
  - `lunar_hour_nine_star`
    已全部纳入外部 `tyme4rs` bridge 的硬断言。

## 验证

- `bash scripts/run_tyme4rs_diff_check.sh`
- `cargo test`

## 后续

后续更自然的下一步是：

1. 继续往 `LunarMonth` / `LunarYear` 的 `jupiter_direction` / `nine_star` 之外的稳定字段推进；
2. 只把已经确认同义的字段纳入外部硬对比；
3. 继续把 `phase` / `phase_day` 一类已确认分叉的字段留在本地协议侧。
