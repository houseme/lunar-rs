# 多历法扩展模板

## 1. 最低对象层级

- `Day`
- `Month`
- `Year`

## 2. 最低 companion 能力

- `from_ymd` / `from_ym` / `from_year`
- `solar()`
- `next(...)`
- `subtract(...)`
- `contains_solar(...)`
- `first_day()` / `last_day()`
- `days()` / `months()`

## 3. 事件体系复用

- `events()`
- `all_events()`
- `find_events(query)`
- `events_until(end)`
- `find_events_until(end, query)`

## 4. 测试最小集合

- 核心往返样例
- 边界年份
- 闰年/闰月/特殊日
- `next` / `subtract`
- 事件体系代理

## 5. 文档同步

- 在 `docs/tasks/` 新建任务文档
- 在 `docs/next-iteration-index.md` 注册
- 完成后回写验证记录与收口结论
