# DateTimeRange 时间范围筛选

将开始和结束时间收纳在一个原生 Popover 中，适合数据列表的时间筛选。组件保留原生表单语义，点击“应用”会提交所在表单。

```rust,ignore
date_time_range_filter(config: DateTimeRangeConfig::new(
    "run-time-range",
    "2026-09-20T10:00",
    "2026-09-21T10:00",
))
```

组件提供最近 24 小时、7 天和 30 天快捷范围。业务服务仍负责解析、校验查询参数以及确定时区语义。
