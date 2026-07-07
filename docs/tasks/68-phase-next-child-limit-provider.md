# Task 68: ChildLimit provider 与童限信息对象

## 对标结论

`tyme4rs` 在八字模块中把童限起运拆为 `ChildLimitProvider`、`ChildLimitInfo`、`ChildLimit`，并提供默认、China95、Lunar 流派 1、Lunar 流派 2 等不同口径。`lunar-rs` 原先只有 `EightChar::yun*` 与 `Yun` 起运结果，缺少可替换的童限 provider 与 typed 信息对象。

本任务补齐童限的第一层对象模型：

- 新增 `ChildLimitInfo`，显式保存出生时刻、起运时刻、年月日时分换算量。
- 新增 `ChildLimit`，保存性别、顺逆、童限信息，并作为八字入口的 typed 返回对象。
- 新增 `ChildLimitProvider` trait。
- 新增 `DefaultChildLimitProvider`、`China95ChildLimitProvider`、`LunarSect1ChildLimitProvider`、`LunarSect2ChildLimitProvider`。
- `EightChar` 新增 `child_limit()` 与 `child_limit_with_provider()`。
- `Yun` 新增 `child_limit_info()`，让既有起运算法可被 provider 包装复用。

## 实现边界

本任务先补 provider 与童限信息层，不在同一切片内引入 `DecadeFortune` / `Fortune` 的完整对象图。原因是后者会继续扩展十年大运、流年、流月等派生 API，适合作为后续独立任务验证。

## 验证

- `tests/eight_char.rs` 覆盖 LunarSect1 / LunarSect2 provider 与现有 `Yun` 起运结果一致。
- 覆盖默认 provider 与 China95 provider 可独立返回起运终点与换算量。
