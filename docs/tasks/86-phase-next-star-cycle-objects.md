# Task 86: 星曜循环对象一次性补齐

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs v1.5` 中 `culture/star` 下有多个纯循环星曜对象：

- `SixStar`：六曜，`先胜/友引/先负/佛灭/大安/赤口`。
- `SevenStar`：七曜，`日/月/火/水/木/金/土`。
- `Ecliptic`：黄道黑道，`黄道/黑道`，并可映射吉凶。
- `TwelveStar`：黄道黑道十二神，且可取 `Ecliptic`。
- `TenStar`：十神，`比肩/劫财/.../正印`。

本地已有 `LiuYao`、`TianShen`、`GodLuck` 等相邻对象，但缺少这些同名 typed wrapper。本任务一次性补齐这些表驱动对象，复用 `NamedCulture` / `CycleItem` 形态。

## 子任务拆分

1. 新增常量表：六曜、七曜、黄黑道、十二神、十神。
2. 新增 typed 对象：`SixStar`、`SevenStar`、`Ecliptic`、`TwelveStar`、`TenStar`。
3. 实现 `from_index`、`from_name`、`index`、`name`、`Display`。
4. 接入 `NamedCulture` 与 `CycleItem`。
5. 为 `Ecliptic` 增加 `luck()`，为 `TwelveStar` 增加 `ecliptic()`。
6. 导出到 crate 根并补充测试。

## 验证计划

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test`：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

完成后进入 `Task 87`：新增独立 8 相天文月相对象。
