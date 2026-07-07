# Task 69: DecadeFortune / Fortune 运势对象

## 对标结论

`tyme4rs` 的童限对象可以继续派生 `DecadeFortune`（大运）和 `Fortune`（小运 / 年运）。`lunar-rs` 已有 `Yun`、`DaYun`、`XiaoYun` 的底层算法，但缺少从 `ChildLimit` 出发的对象层 API。

本任务补齐轻量对象层：

- 新增 `DecadeFortune`，提供 `index`、起止年龄、起止年份、干支、名称和 `start_fortune()`。
- 新增 `Fortune`，提供 `index`、年龄、年份、干支年、干支、名称和 `next()`。
- `ChildLimit` 新增 `start_decade_fortune()`、`decade_fortune()`、`start_fortune()`、起止年龄和起止干支年。
- `SixtyCycle` 新增 `next()`，支持运势对象按干支序列前后移动。

## 验证

- `tests/eight_char.rs` 校验 `DecadeFortune` 与既有 `DaYun` 的干支一致。
- 校验 `Fortune` 与既有 `XiaoYun` 的起始干支一致。
