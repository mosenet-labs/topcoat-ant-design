# 日历

`calendar` 展示日、周、月三种视图。调用方用 Topcoat Signal、Procedure 或链接管理日期、切换视图与加载数据；组件只负责日期布局和内容展示，不包含原生 JavaScript。

一周从周一开始；月视图补齐首尾跨月日期。事件除了颜色，还始终展示名称；今日使用 `aria-current="date"` 标记。

参数与示例见 [English API](../en/components/calendar.md)。`CalendarEventKind` 可取 `Holiday`、`Workday`、`Default`。

```rust,ignore
use chrono::NaiveDate;
use topcoat::view::view;
use topcoat_ant_design::{calendar, CalendarEvent, CalendarEventKind, CalendarView, UiLanguage};

let date = NaiveDate::from_ymd_opt(2026, 10, 19).unwrap();
let entries = vec![CalendarEvent {
    date,
    title: "假日示例".into(),
    kind: CalendarEventKind::Holiday,
}];
view! {
    calendar(date: date, today: date, events: &entries,
        mode: CalendarView::Month, language: UiLanguage::ChineseSimplified)
}
```

同一组数据只需切换 `mode` 即可展示周视图或日视图：

```rust,ignore
calendar(date: date, today: date, events: &entries,
    mode: CalendarView::Week, language: UiLanguage::ChineseSimplified)
```

```rust,ignore
calendar(date: date, today: date, events: &entries,
    mode: CalendarView::Day, language: UiLanguage::ChineseSimplified)
```
