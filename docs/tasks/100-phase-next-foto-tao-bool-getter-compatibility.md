# Task 100: Foto/Tao 布尔规则 get_* 兼容补强

状态：已完成
最近更新：2026-07-07
上游基准：`6tail/tyme4rs` `ba6ab75`（2026-06-15，v1.5.0）

## 对标结论

`Task 99` 已把本地扩展的 `Foto` / `Tao` wrapper 收口为 owned 快照，并补齐了高频对象 getter。但这两个 wrapper 仍保留一批纯布尔规则接口只暴露为 `is_*`，没有对应的 `get_*` 迁移别名：

- `Foto`
  - `is_month_zhai`
  - `is_day_yang_gong`
  - `is_day_zhai_shuo_wang`
  - `is_day_zhai_six`
  - `is_day_zhai_ten`
  - `is_day_zhai_guan_yin`
- `Tao`
  - `is_day_san_hui`
  - `is_day_san_yuan`
  - `is_day_wu_la`
  - `is_day_ba_jie`
  - `is_day_ba_hui`
  - `is_day_ming_wu`
  - `is_day_an_wu`
  - `is_day_wu`

本任务不改变任何旧命名，也不调整规则语义，只增加一层显式 `get_*` 兼容别名，方便从 tyme 风格迁移时代码风格保持一致。

## 子任务

1. 为 `Foto` 上所有剩余 `is_*` 规则增加 `get_*` 兼容别名。
2. 为 `Tao` 上所有剩余 `is_*` 规则增加 `get_*` 兼容别名。
3. 在 `tests/foto.rs` / `tests/tao.rs` 中补对照断言，确保 `get_* == is_*`。
4. 同步总对标文档与下一轮索引。

## 实施结果

- `Foto`
  - 新增：`get_is_month_zhai`
  - 新增：`get_is_day_yang_gong`
  - 新增：`get_is_day_zhai_shuo_wang`
  - 新增：`get_is_day_zhai_six`
  - 新增：`get_is_day_zhai_ten`
  - 新增：`get_is_day_zhai_guan_yin`
- `Tao`
  - 新增：`get_is_day_san_hui`
  - 新增：`get_is_day_san_yuan`
  - 新增：`get_is_day_wu_la`
  - 新增：`get_is_day_ba_jie`
  - 新增：`get_is_day_ba_hui`
  - 新增：`get_is_day_ming_wu`
  - 新增：`get_is_day_an_wu`
  - 新增：`get_is_day_wu`

## 验证

- `cargo fmt`
- `cargo test --test foto`
- `cargo test --test tao`
- `cargo test`

## 后续

继续推进时，优先级更高的方向已经不再是机械式 `get_*` 收口，而是：

- 建立更严格的 tyme4rs 差分样例矩阵；
- 同步 README / API 迁移文档；
- 只在仍存在明显迁移摩擦的对象上继续补兼容层。
