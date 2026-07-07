# Task 103: 日期键查找与时辰索引优化

状态：已完成
最近更新：2026-07-07
上游基准：`6tail/tyme4rs` `ba6ab75`（2026-06-15，v1.5.0）

## 对标结论

这不是 `tyme4rs v1.5` 的直接公开 API 缺口，而是本地在补齐兼容层后暴露出来的一组重复热路径：

- 节日 / 其它节日 / 周序节日查找大量依赖 `format!("{}-{}", ...)` 之类的字符串拼接。
- 佛历 / 道历 wrapper 的节日与规则判断也重复走字符串 key。
- 时辰地支索引计算在 `Lunar`、`LunarTime`、`Yun` 等处重复构造 `"HH:MM"` 字符串再解析。

这些逻辑虽然功能正确，但在持续扩展兼容层后变得更分散，也更难给差分测试和文档示例提供稳定支点。因此本任务把公共查找逻辑收成共享 helper。

## 子任务

1. 新增 `src/key_index.rs`，统一 `month_day_key` / `month_weekday_key` 及解析函数。
2. 为 `solar_util`、`lunar_util`、`foto_util`、`tao_util` 增加基于整数 key 的索引缓存。
3. 将 `Solar`、`Lunar`、`Foto`、`Tao` 的节日/规则查找切换到共享 helper。
4. 将时辰地支索引计算收为 `lunar_util::time_zhi_index_from_hour()`，消除重复 `"HH:MM"` 构造。
5. 确保 `Yun`、`LunarTime`、`Lunar` 的相关路径都复用这一 helper。

## 实施结果

- 新增 [src/key_index.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/key_index.rs)：
  - `month_day_key`
  - `month_weekday_key`
  - `parse_month_day_key`
  - `parse_month_weekday_key`
- 工具模块索引化：
  - [src/solar_util.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/solar_util.rs)
  - [src/lunar_util/mod.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/lunar_util/mod.rs)
  - [src/foto_util.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/foto_util.rs)
  - [src/tao_util.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/tao_util.rs)
- 调用点切换：
  - [src/solar.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/solar.rs)
  - [src/lunar.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/lunar.rs)
  - [src/foto.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/foto.rs)
  - [src/tao.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/tao.rs)
  - [src/lunar_time.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/lunar_time.rs)
  - [src/yun/mod.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/yun/mod.rs)

## 验证

- `cargo fmt`
- `cargo test`

## 后续

后续如果继续做性能与稳定性收口，可以考虑：

1. 把更多只读字符串表访问迁移到预索引缓存；
2. 在 differential snapshot 中继续补这类 helper 驱动的字段；
3. 仅在真实热点上再做更细粒度 benchmark，避免为了“看起来更快”而继续扩散改动面。
