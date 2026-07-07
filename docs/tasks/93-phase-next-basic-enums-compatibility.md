# Task 93: tyme4rs 基础枚举兼容

状态：已完成
最近更新：2026-07-07

## 对标结论

`tyme4rs v1.5` 的 `tyme::enums` 除 `EventType` 外，还提供三个基础枚举：

- `Gender`：`WOMAN` / `MAN`，名称为 `女` / `男`。
- `Side`：`IN` / `OUT`，名称为 `内` / `外`。
- `YinYang`：`YIN` / `YANG`，名称为 `阴` / `阳`。

本地此前只有 `Gender = u8` 的轻量别名，且没有 `Side` / `YinYang`。这导致对标 `tyme4rs` 的公开类型迁移仍需要调用方自行维护 code/name 映射。

本任务将 `Gender` 升级为 typed enum，同时通过 `From<u8>` / `From<i32>` 和泛型入口保留旧的 `0/1` 调用习惯；新增 `Side` 与 `YinYang`，并把 `YinYang` 挂接到 `HeavenStem` / `EarthBranch`。

## 子任务拆分

1. 新增 typed `Gender`，支持 `WOMAN` / `MAN` 关联常量。
2. 新增 typed `Side`，支持 `IN` / `OUT` 关联常量。
3. 新增 typed `YinYang`，支持 `YIN` / `YANG` 关联常量。
4. 三个枚举均提供 `from_code` / `from_name` / `code` / `get_code` / `name` / `get_name` / `Display`。
5. `Gender` 支持旧整数输入：`0` 为女，非 `0` 为男。
6. `EightChar::yun`、`EightChar::yun_by_sect`、`EightChar::child_limit`、`EightChar::child_limit_with_provider` 接受 `impl Into<Gender>`，保留旧整数调用。
7. `HeavenStem::yin_yang` 与 `EarthBranch::yin_yang` 返回 typed `YinYang`。
8. crate 根导出 `Gender`、`Side`、`YinYang`。
9. 补充测试覆盖 code/name/display、旧整数兼容、天干地支阴阳挂接。

## 验证计划

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test --test eight_char`
- `cargo test`

## 验证记录

已执行：

- `cargo fmt`
- `cargo test --test phase2_typed`
- `cargo test --test eight_char`
- `cargo test`

结果：通过；`tests/differential.rs` 中 1 个需要外部 `LUNAR_RS_DIFF_REF_BIN` 的差分测试按设计 ignored。

## 后续衔接

`tyme::enums` 层的明确公开类型缺口已补齐到 `EventType`、`Gender`、`Side`、`YinYang`。后续继续对齐时，应优先处理严格方法名别名、README/API 示例和外部差分测试，而不是继续扩展基础枚举。
