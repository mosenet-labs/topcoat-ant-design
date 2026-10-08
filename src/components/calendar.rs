use chrono::{Datelike, Duration, NaiveDate};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, View, attributes, class, component, view},
};

use crate::UiLanguage;

/// The amount of time shown by [`calendar`]. Navigation state belongs to the caller.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CalendarView {
    Day,
    Week,
    #[default]
    Month,
}

/// A calendar event's visual meaning. Its title remains visible as text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalendarEventKind {
    Holiday,
    Workday,
    Default,
}

/// A date-scoped entry. Multiple entries can be displayed on one date.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarEvent {
    pub date: NaiveDate,
    pub title: String,
    pub kind: CalendarEventKind,
}

struct CalendarCell {
    date: NaiveDate,
    outside_month: bool,
    today: bool,
    events: Vec<CalendarEvent>,
}

/// Presentational day, week, and month calendar. Use Topcoat signals, links, or procedures in
/// the host for navigation, view switching, and loading entries; no browser script is needed.
#[component]
pub async fn calendar(
    cx: &Cx,
    date: NaiveDate,
    today: NaiveDate,
    events: &[CalendarEvent],
    #[default] mode: CalendarView,
    #[default] language: UiLanguage,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let first = match mode {
        CalendarView::Day => date,
        CalendarView::Week => date - Duration::days(date.weekday().num_days_from_monday().into()),
        CalendarView::Month => {
            let month = date.with_day(1).expect("valid month start");
            month - Duration::days(month.weekday().num_days_from_monday().into())
        }
    };
    let title = match (language, mode) {
        (UiLanguage::English, CalendarView::Month) => date.format("%B %Y").to_string(),
        (UiLanguage::English, CalendarView::Week) => format!(
            "{} – {}",
            first.format("%b %-d, %Y"),
            (first + Duration::days(6)).format("%b %-d, %Y")
        ),
        (UiLanguage::English, CalendarView::Day) => date.format("%B %-d, %Y").to_string(),
        (UiLanguage::ChineseSimplified, CalendarView::Month) => {
            format!("{} 年 {} 月", date.year(), date.month())
        }
        (UiLanguage::ChineseSimplified, CalendarView::Week) => {
            let end = first + Duration::days(6);
            format!(
                "{} 年 {} 月 {} 日 – {} 年 {} 月 {} 日",
                first.year(),
                first.month(),
                first.day(),
                end.year(),
                end.month(),
                end.day()
            )
        }
        (UiLanguage::ChineseSimplified, CalendarView::Day) => {
            format!("{} 年 {} 月 {} 日", date.year(), date.month(), date.day())
        }
    };
    let count = match mode {
        CalendarView::Day => 1,
        CalendarView::Week => 7,
        CalendarView::Month => {
            let next_month = if date.month() == 12 {
                NaiveDate::from_ymd_opt(date.year() + 1, 1, 1)
            } else {
                NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
            }
            .expect("valid next month");
            ((next_month - first).num_days() as usize).div_ceil(7) * 7
        }
    };
    let cells: Vec<CalendarCell> = (0..count)
        .map(|offset| {
            let day = first + Duration::days(offset as i64);
            CalendarCell {
                date: day,
                outside_month: mode == CalendarView::Month && day.month() != date.month(),
                today: day == today,
                events: events
                    .iter()
                    .filter(|event| event.date == day)
                    .cloned()
                    .collect(),
            }
        })
        .collect();
    let weekdays = match language {
        UiLanguage::English => ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
        UiLanguage::ChineseSimplified => ["周一", "周二", "周三", "周四", "周五", "周六", "周日"],
    };
    let caller_class = attrs.remove("class");
    attrs.extend(attributes! { cx =>
        class=(class!("gr-calendar", caller_class))
        aria-label=(title.as_str())
    });

    Ok(view! {
        <section (attrs)>
            <h2 class="gr-calendar-title">(title)</h2>
            if mode == CalendarView::Day {
                for cell in &cells {
                    <div class="gr-calendar-day-panel" data-date=(cell.date.to_string())>
                        <div class="gr-calendar-day-heading"><span>(weekdays[cell.date.weekday().num_days_from_monday() as usize])</span><strong aria-current=(if cell.today { Some("date") } else { None })>(cell.date.day())</strong></div>
                        if cell.events.is_empty() { <p class="gr-calendar-empty">(language.select("No entries", "当天没有日程"))</p> }
                        for event in &cell.events {
                            <div class=(event_class(event.kind))><span class="gr-calendar-event-dot" aria-hidden="true"></span><span>(event.title.as_str())</span></div>
                        }
                    </div>
                }
            } else {
                <div class="gr-calendar-scroll">
                    <table class=(if mode == CalendarView::Week { "gr-calendar-grid gr-calendar-week" } else { "gr-calendar-grid gr-calendar-month" })>
                        <thead><tr>for weekday in weekdays { <th scope="col">(weekday)</th> }</tr></thead>
                        <tbody>
                            for week in cells.chunks(7) {
                                <tr>
                                    for cell in week {
                                        <td data-date=(cell.date.to_string()) class=(class!("gr-calendar-outside" if cell.outside_month, "gr-calendar-today" if cell.today))>
                                            <span class="gr-calendar-number" aria-label=(cell.date.format("%Y-%m-%d").to_string()) aria-current=(if cell.today { Some("date") } else { None })>(cell.date.day())</span>
                                            for event in &cell.events {
                                                <span class=(event_class(event.kind)) title=(event.title.as_str())><span class="gr-calendar-event-dot" aria-hidden="true"></span><span>(event.title.as_str())</span></span>
                                            }
                                        </td>
                                    }
                                </tr>
                            }
                        </tbody>
                    </table>
                </div>
            }
        </section>
    })
}

fn event_class(kind: CalendarEventKind) -> &'static str {
    match kind {
        CalendarEventKind::Holiday => "gr-calendar-event gr-calendar-event-holiday",
        CalendarEventKind::Workday => "gr-calendar-event gr-calendar-event-workday",
        CalendarEventKind::Default => "gr-calendar-event gr-calendar-event-default",
    }
}
