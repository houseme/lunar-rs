# Task 70: PengZu 天干 / 地支 typed companion

## 对标结论

`tyme4rs` 将彭祖百忌拆为 `PengZu`、`PengZuHeavenStem`、`PengZuEarthBranch`。`lunar-rs` 此前已经有 `PengZu` typed wrapper，但内部仍只保存天干、地支两段字符串，缺少可循环、可索引、可组合的 companion 对象。

本任务补齐：

- 新增 `PengZuHeavenStem`。
- 新增 `PengZuEarthBranch`。
- 二者支持 `from_index`、`from_name`、`index`、`name`、`Display`。
- 二者实现 `NamedCulture` 与 `CycleItem`。
- `PengZu` 保留原有字符串 getter，并新增 `heaven_stem_item()` / `earth_branch_item()`。

## 验证

- `tests/phase2_typed.rs` 校验 `PengZu` typed companion 与现有 `peng_zu_gan()` / `peng_zu_zhi()` 输出一致。
- 校验 companion 的循环能力可通过 `CycleItem::next()` / `steps_to()` 使用。
