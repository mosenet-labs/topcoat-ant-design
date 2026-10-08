use chrono::{Duration, NaiveDate};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};
use topcoat_ant_design::{CalendarEvent, CalendarEventKind, CalendarView, calendar};

use crate::{
    app::page_header,
    demo::component_example,
    locale::Locale,
    markdown::{markdown_document, rust_code_block},
};

const CALENDAR_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/calendar.md"
));
const CALENDAR_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/calendar.md"
));

#[page]
pub(in crate::app) async fn calendar_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(CALENDAR_DOC_EN, CALENDAR_DOC_ZH);
    let month_source = rust_code_block(document, 0);
    let week_source = rust_code_block(document, 1);
    let day_source = rust_code_block(document, 2);
    let date = NaiveDate::from_ymd_opt(2026, 10, 19).expect("valid demo date");
    let events = vec![
        CalendarEvent {
            date,
            title: locale.select("Holiday example", "假日示例").to_owned(),
            kind: CalendarEventKind::Holiday,
        },
        CalendarEvent {
            date: date + Duration::days(1),
            title: locale
                .select("Adjusted workday example", "调休上班示例")
                .to_owned(),
            kind: CalendarEventKind::Workday,
        },
        CalendarEvent {
            date: date + Duration::days(2),
            title: locale
                .select("Team event example", "团队日程示例")
                .to_owned(),
            kind: CalendarEventKind::Default,
        },
    ];

    Ok(view! {
        page_header(
            eyebrow: "DATA DISPLAY",
            title: locale.select("Calendar", "Calendar 日历"),
            description: locale.select(
                "Show the same date-scoped entries in month, week, and day views. The host owns navigation and data loading.",
                "同一组日期事件可展示为月、周、日视图；日期导航和数据加载由调用方管理。",
            ),
        )
        <div class="grid gap-6">
            component_example(
                id: "calendar-month-preview",
                title: locale.select("Month view", "月视图"),
                description: locale.select("A complete month with adjacent dates and event types.", "展示完整月份、相邻日期与不同类型的事件。"),
                source: month_source,
                <div class="p-5 max-[640px]:p-3">
                    calendar(date: date, today: date, events: &events, mode: CalendarView::Month, language: locale.ui())
                </div>
            )
            component_example(
                id: "calendar-week-preview",
                title: locale.select("Week view", "周视图"),
                description: locale.select("The same sample entries in a Monday-first week.", "同一组示例事件按周一开始的一周展示。"),
                source: week_source,
                <div class="p-5 max-[640px]:p-3">
                    calendar(date: date, today: date, events: &events, mode: CalendarView::Week, language: locale.ui())
                </div>
            )
            component_example(
                id: "calendar-day-preview",
                title: locale.select("Day view", "日视图"),
                description: locale.select("A focused day with its entries shown in full.", "集中展示指定日期及其事件。"),
                source: day_source,
                <div class="p-5 max-[640px]:p-3">
                    calendar(date: date, today: date, events: &events, mode: CalendarView::Day, language: locale.ui())
                </div>
            )
            markdown_document(source: document)
        </div>
    })
}
