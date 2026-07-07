# Task 105: tyme4rs 外部 reference bridge 落地

状态：已完成
最近更新：2026-07-07
上游基准：`6tail/tyme4rs` `ba6ab75`（2026-06-15，v1.5.0）

## 对标结论

`Task 102` 已把差分协议升级到 v2，但仓库里仍只有本地 `lunar_ref_driver` 自检链路。也就是说：

- `LUNAR_RS_DIFF_REF_BIN` 这条路径已经存在；
- 但还没有一条仓库内可复用的方式，真正把 `6tail/tyme4rs` 编译成差分测试可消费的 bridge。

本任务把这一步补齐，让 ignored 的外部差分测试不再只是“理论上可以跑”。

## 子任务

1. 为 `tests/differential.rs` 增加 `LUNAR_RS_DIFF_REF_FLAVOR`，支持 `tyme4rs` 这种字段级 reference。
2. 新增脚本 `scripts/run_tyme4rs_diff_check.sh`：
   - 默认使用 `/private/tmp/tyme4rs-latest`
   - 支持 `TYME4RS_PATH` 覆盖
   - 在临时目录生成小型 cargo bridge
   - 构建后把 binary 注入 `LUNAR_RS_DIFF_REF_BIN`
3. bridge 输出差分协议 v2 所需全部键，并显式说明 `solar_full` / `lunar_full` 在 tyme4rs flavor 下不做严格比较。
4. 保持现有 `scripts/run_differential_self_check.sh` 不变，继续作为本地 driver 自检入口。

## 实施结果

- [tests/differential.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/tests/differential.rs)
  - 增加 `ReferenceFlavor`
  - 允许 `tyme4rs` flavor 跳过 `solar_full` / `lunar_full` 的严格断言
  - 为 `tyme4rs` flavor 增加农历月份中文同义归一化：`冬月 -> 十一月`、`腊月 -> 十二月`
- [scripts/run_tyme4rs_diff_check.sh](/Users/zhi/Documents/code/rust/houseme/lunar-rs/scripts/run_tyme4rs_diff_check.sh)
  - 自动生成并构建 `tyme4rs_ref_bridge`
  - 复用现有 ignored differential test
  - 增加 `use tyme4rs::tyme::Culture;`，修复 bridge 中 `get_name()` trait 未导入的编译问题
- [src/festival.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/festival.rs)
  - 修正 `LunarFestival::from_ymd(...)` 在节气节日上的年份基准，确保 `冬至节/清明节` 可被 `get_festival()` 正确命中

## 验证

- `bash scripts/run_tyme4rs_diff_check.sh`
- `cargo test`

结果：通过；`tyme4rs` 外部 bridge 已能够真正驱动 ignored 的 `diff_reference_sample_matrix` 跑通现有 v2 样例矩阵。

## 后续

后续如果继续收口，可以沿这条路继续做两件事：

1. 把 bridge 支持的键继续扩展到更多可稳定比较的字段；
2. 视需要新增单独的 `tyme4rs` case matrix，专门放上游已知边界例子。
