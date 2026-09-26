# DateTimeRange 时间范围筛选

将开始和结束时间收纳在一个原生 Popover 中，适合数据列表的时间筛选。组件保留原生表单语义，点击“应用”会提交所在表单。

可选的 `language: UiLanguage` 参数控制组件内置文案；默认英文，传入 `UiLanguage::ChineseSimplified` 切换中文。

可通过 `from_attrs` 和 `to_attrs` 给两个原生输入框添加属性及事件处理器。快捷范围和清除操作修改值后会派发冒泡的 `input` 与 `change` 事件，便于同步宿主的 Topcoat signal。输入框的 `id`、`name`、`type` 和初始 `value` 仍由 `DateTimeRangeConfig` 控制。

```rust,ignore
date_time_range_filter(config: DateTimeRangeConfig::new(
    "run-time-range",
    "2026-09-20T10:00",
    "2026-09-21T10:00",
))
```

组件提供最近 24 小时、7 天和 30 天快捷范围。业务服务仍负责解析、校验查询参数以及确定时区语义。
