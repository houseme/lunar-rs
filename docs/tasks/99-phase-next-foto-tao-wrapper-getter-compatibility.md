# Task 99: Foto/Tao wrapper 生命周期与 getter 兼容补强

状态：已完成
最近更新：2026-07-07
上游基准：`6tail/tyme4rs` `ba6ab75`（2026-06-15，v1.5.0）

## 对标结论

`tyme4rs v1.5` 当前没有独立佛历 / 道历模块；本任务不是上游必需缺口，而是本地已有 `Foto` / `Tao` wrapper 在继续补齐严格方法名时暴露出的兼容加固点。

本地旧形态中，`Foto<'a>` / `Tao<'a>` 借用底层 `Lunar`，当用户写出链式调用时容易受临时值生命周期限制：

```rust
let foto = Solar::from_ymd(2024, 5, 15).unwrap().lunar().foto();
```

为了和已完成的 owned `LunarTime` 方向保持一致，本任务把 `Foto` / `Tao` 改为持有 `Lunar` 快照，同时保留 lifetime 参数作为兼容形状。

## 子任务

1. `Foto` / `Tao` 改为内部持有 `Lunar` clone，并通过 `PhantomData` 保留原 lifetime 形状。
2. `Lunar::foto()` / `Lunar::tao()` 返回可独立存活的 wrapper。
3. 为 wrapper 补齐常用 `get_*` 迁移入口：
   - `Foto`：`get_lunar`、`get_year`、`get_month`、`get_day`、`get_foto_year`、`get_foto_month`、`get_year_in_chinese`、`get_month_in_chinese`、`get_day_in_chinese`、`get_festivals`、`get_other_festivals`、`get_xiu`、`get_xiu_luck`、`get_xiu_song`、`get_zheng`、`get_animal`、`get_gong`、`get_shou`。
   - `Tao`：`get_lunar`、`get_year`、`get_month`、`get_day`、`get_tao_year`、`get_tao_month`、`get_year_in_chinese`、`get_month_in_chinese`、`get_day_in_chinese`、`get_festivals`。
4. 增加链式调用生命周期测试，防止 wrapper 再次退化为借用临时值。

## 验证

- `cargo test --test phase2_typed`
- `cargo test`

## 后续

后续若继续推进，可以把 `FotoYear/FotoMonth/TaoYear/TaoMonth` 也按同样策略补齐更细 `get_*` 方法，并将 `Foto/Tao` 上剩余 `is_*` 规则补成显式 `get_*` 别名；这部分已在 `Task 100` 中单独收口。整体仍应继续标注为本地扩展兼容，而非 tyme4rs v1.5 直接缺口。
