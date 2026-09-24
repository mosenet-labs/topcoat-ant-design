# DateTimeRange filter

Keep start and end date-time fields in a native popover for filtering data lists. The component retains native form semantics: selecting **Apply** submits the containing form.

The optional `language: UiLanguage` property controls the built-in labels. English is the default; use `UiLanguage::ChineseSimplified` for Chinese.

```rust,ignore
date_time_range_filter(config: DateTimeRangeConfig::new(
    "run-time-range",
    "2026-09-20T10:00",
    "2026-09-21T10:00",
))
```

Shortcuts cover the last 24 hours, 7 days, and 30 days. The application remains responsible for parsing and validating query parameters and defining the time zone semantics.
