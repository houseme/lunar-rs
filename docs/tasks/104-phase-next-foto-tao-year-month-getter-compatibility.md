# Task 104: Foto/Tao Year/Month companion getter 兼容

状态：已完成
最近更新：2026-07-07
上游基准：`6tail/tyme4rs` `ba6ab75`（2026-06-15，v1.5.0）

## 对标结论

`Task 99` 与 `Task 100` 已把 `Foto` / `Tao` wrapper 自身的 owned 形态、常用 `get_*` 入口和布尔规则别名收口，但其 companion 年/月对象仍主要停留在 Rust 风格方法名：

- `FotoYear` / `TaoYear`
  - 仅有 `year`、`lunar_year`、`first_month`、`last_month`、`months`、`first_solar_day`、`last_solar_day`
- `FotoMonth` / `TaoMonth`
  - 仅有 `year`、`month`、`day_count`、`index`、`name`、`first_solar_day`、`last_solar_day`

对于已经按 tyme 风格迁移到 wrapper 层的调用方来说，这会形成一个小断层：wrapper 本身有 `get_*`，但 year/month companion 还要回退到短方法名。

本任务继续保持“只补 companion getter，不改原语义”的边界。

## 子任务

1. 为 `FotoYear` / `TaoYear` 增加常用 `get_*` companion getter。
2. 为 `FotoMonth` / `TaoMonth` 增加常用 `get_*` companion getter。
3. 在 `tests/foto.rs` / `tests/tao.rs` 中补对照断言，确认新旧入口等价。
4. 同步索引与总对标文档。

## 实施结果

- `FotoYear`
  - 新增：`get_year`、`get_lunar_year`、`get_first_month`、`get_last_month`、`get_months`、`get_first_solar_day`、`get_last_solar_day`
- `FotoMonth`
  - 新增：`get_year`、`get_month`、`get_day_count`、`get_index`、`get_name`、`get_first_solar_day`、`get_last_solar_day`
- `TaoYear`
  - 新增：`get_year`、`get_lunar_year`、`get_first_month`、`get_last_month`、`get_months`、`get_first_solar_day`、`get_last_solar_day`
- `TaoMonth`
  - 新增：`get_year`、`get_month`、`get_day_count`、`get_index`、`get_name`、`get_first_solar_day`、`get_last_solar_day`

## 验证

- `cargo fmt`
- `cargo test --test foto`
- `cargo test --test tao`
- `cargo test`

## 后续

`Foto/Tao` 这条链的 companion getter 基本已经收口。继续推进时，优先级更高的方向是：

- 外部 reference 同步与更严格差分矩阵；
- README/API 文档继续精炼，而不是继续机械式补别名；
- 只有当某个对象仍然明显阻碍迁移时，再增加额外兼容层。
