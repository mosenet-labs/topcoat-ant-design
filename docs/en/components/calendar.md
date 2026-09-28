Render date-scoped entries in day, week, or month form. `calendar` is presentational; the host owns the current date, navigation controls, data loading, and view switch using Topcoat signals, procedures, or links. The component contains no native JavaScript.

The week starts on Monday. Month view includes leading and trailing dates. Entries have semantic text as well as color, and the current date is marked with `aria-current="date"`.

| Parameter | Type | Description |
| --- | --- | --- |
| `date` | `chrono::NaiveDate` | Date to display. |
| `today` | `chrono::NaiveDate` | Current local date supplied by the host. |
| `events` | `&[CalendarEvent]` | Entries to render on visible dates. |
| `mode` | `CalendarView` | Optional `Day`, `Week`, or `Month` (default). |
| `language` | `UiLanguage` | Optional English (default) or Simplified Chinese. |
| `attrs` | `Attributes` | Optional attributes on the root section; classes are merged. |

`CalendarEvent` contains `date`, `title`, and `kind`. `CalendarEventKind` has `Holiday`, `Workday`, and `Default` values.

```rust,ignore
use chrono::NaiveDate;
use topcoat::view::view;
use topcoat_ant_design::{calendar, CalendarEvent, CalendarEventKind, CalendarView, UiLanguage};

let date = NaiveDate::from_ymd_opt(2026, 10, 19).unwrap();
let entries = vec![CalendarEvent {
    date,
    title: "Holiday example".into(),
    kind: CalendarEventKind::Holiday,
}];
view! {
    calendar(date: date, today: date, events: &entries,
        mode: CalendarView::Month, language: UiLanguage::English)
}
```

Use the same data for week and day views by changing `mode`:

```rust,ignore
calendar(date: date, today: date, events: &entries,
    mode: CalendarView::Week, language: UiLanguage::English)
```

```rust,ignore
calendar(date: date, today: date, events: &entries,
    mode: CalendarView::Day, language: UiLanguage::English)
```
