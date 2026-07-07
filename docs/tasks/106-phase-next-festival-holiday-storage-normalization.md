# Task 106: 节日与假日存储归一化

状态：已完成
最近更新：2026-07-07
上游基准：`6tail/tyme4rs` `ba6ab75`（2026-06-15，v1.5.0）

## 对标结论

在 `Task 103` 的日期 key 索引优化之后，`FotoFestival`、`TaoFestival`、`Holiday` 这些轻量对象内部仍保留了一些不必要的 `String` 持有：

- `FotoFestival` / `TaoFestival` 本质上来自静态表，没必要复制成 `String`
- `Holiday` 的 `day` / `target` 只是固定格式的日期文本，也不需要散装字符串拼接

这些地方不会直接造成错误，但会让事件细节对象和节日 wrapper 的内存形态不够稳定，也增加一些无意义分配。

## 子任务

1. 将 `FotoFestival` 的 `name/result/remark` 收敛为 `&'static str`。
2. 将 `TaoFestival` 的 `name/remark` 收敛为 `&'static str`。
3. 将 `Holiday.day/target` 规范成固定 `YYYY-MM-DD` 存储，并避免反复格式化。
4. 保持外部公开方法语义不变，仅做内部承载优化。

## 实施结果

- [src/foto.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/foto.rs)
  - `FotoFestival` 改为静态字符串承载
- [src/tao.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/tao.rs)
  - `TaoFestival` 改为静态字符串承载
- [src/holiday.rs](/Users/zhi/Documents/code/rust/houseme/lunar-rs/src/holiday.rs)
  - 新增 `FixedYmd`
  - `Holiday` 内部日期字段改为固定规范化表示

## 验证

- `cargo test --test foto`
- `cargo test --test tao`
- `cargo test --test phase3_events`
- `cargo test`

## 后续

如果继续沿这条线收口，可以再考虑把 `event.rs` 中当前 crate 内部 typed detail/source/tag 边界的可见性收得更紧，顺手消掉 `private_bounds` 一类编译警告。
