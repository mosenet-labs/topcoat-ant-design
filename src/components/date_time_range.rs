use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    runtime::Event,
    view::{Attributes, View, attributes, class, component, view},
};

use crate::UiLanguage;
use crate::icons::CALENDAR_OUTLINED;

#[derive(Clone, Copy, Debug)]
pub struct DateTimeRangeConfig<'a> {
    pub id: &'a str,
    pub from: &'a str,
    pub to: &'a str,
    pub from_name: &'a str,
    pub to_name: &'a str,
}

impl<'a> DateTimeRangeConfig<'a> {
    pub const fn new(id: &'a str, from: &'a str, to: &'a str) -> Self {
        Self {
            id,
            from,
            to,
            from_name: "from",
            to_name: "to",
        }
    }

    pub const fn with_names(mut self, from_name: &'a str, to_name: &'a str) -> Self {
        self.from_name = from_name;
        self.to_name = to_name;
        self
    }
}

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/date-time-range.md"
))]
#[component]
pub async fn date_time_range_filter(
    cx: &Cx,
    config: DateTimeRangeConfig<'_>,
    #[default] language: UiLanguage,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let DateTimeRangeConfig {
        id,
        from,
        to,
        from_name,
        to_name,
    } = config;
    let from_id = format!("{id}-from");
    let to_id = format!("{id}-to");
    let anchor = format!("anchor-name: --gr-{id}");
    let panel_anchor = format!("position-anchor: --gr-{id}");
    let caller_class = attrs.remove("class");
    let root_class = class!("gr-date-range", caller_class);
    attrs.extend(attributes! { cx => class=(root_class) });
    let label = range_label(from, to, language);
    let clear_from = from_id.clone();
    let clear_to = to_id.clone();
    let quick_day_from = from_id.clone();
    let quick_day_to = to_id.clone();
    let quick_week_from = from_id.clone();
    let quick_week_to = to_id.clone();
    let quick_month_from = from_id.clone();
    let quick_month_to = to_id.clone();
    let quick_day = recent_range_attributes(cx, quick_day_from, quick_day_to, 1.0);
    let quick_week = recent_range_attributes(cx, quick_week_from, quick_week_to, 7.0);
    let quick_month = recent_range_attributes(cx, quick_month_from, quick_month_to, 30.0);

    Ok(view! {
        <span (attrs)>
            <button class="gr-date-range-trigger" type="button" popovertarget=(id) popovertargetaction="toggle" aria-haspopup="dialog" aria-controls=(id) style=(anchor)>
                <span class="gr-date-range-calendar" aria-hidden="true">icon(data: CALENDAR_OUTLINED, size: 16)</span><span>(label)</span>
            </button>
            <aside id=(id) class="gr-date-range-panel" popover="auto" role="dialog" aria-label=(language.select("Select date range", "选择时间范围")) style=(panel_anchor)>
                <div class="gr-date-range-fields">
                    <label for=(from_id.as_str())><span>(language.select("Start time", "开始时间"))</span><input id=(from_id.as_str()) name=(from_name) type="datetime-local" value=(from)></label>
                    <span class="gr-date-range-separator" aria-hidden="true">"→"</span>
                    <label for=(to_id.as_str())><span>(language.select("End time", "结束时间"))</span><input id=(to_id.as_str()) name=(to_name) type="datetime-local" value=(to)></label>
                </div>
                <div class="gr-date-range-quick" aria-label=(language.select("Quick date ranges", "快捷时间范围"))>
                    <button type="button" (quick_day)>(language.select("Last 24 hours", "最近 24 小时"))</button>
                    <button type="button" (quick_week)>(language.select("Last 7 days", "最近 7 天"))</button>
                    <button type="button" (quick_month)>(language.select("Last 30 days", "最近 30 天"))</button>
                </div>
                <footer class="gr-date-range-actions">
                    <button class="gr-button gr-button-default" type="button" @click=$(move |_e: Event| {
                        let _from = clear_from.to_owned(); let _to = clear_to.to_owned();
                        raw!("document.getElementById(${_from}.dehydrate()).value=''; document.getElementById(${_to}.dehydrate()).value=''", ());
                    })>(language.select("Clear", "清除"))</button>
                    <button class="gr-button gr-button-primary" type="submit">(language.select("Apply", "应用"))</button>
                </footer>
            </aside>
        </span>
    })
}

fn range_label(from: &str, to: &str, language: UiLanguage) -> String {
    match (from.is_empty(), to.is_empty()) {
        (true, true) => language.select("All time", "全部时间").to_owned(),
        (false, true) => match language {
            UiLanguage::English => format!("From {}", compact_time(from)),
            UiLanguage::ChineseSimplified => format!("{} 起", compact_time(from)),
        },
        (true, false) => match language {
            UiLanguage::English => format!("Until {}", compact_time(to)),
            UiLanguage::ChineseSimplified => format!("截至 {}", compact_time(to)),
        },
        (false, false) => format!("{} → {}", compact_time(from), compact_time(to)),
    }
}

fn compact_time(value: &str) -> String {
    value.replace('T', " ").chars().skip(5).collect()
}

fn recent_range_attributes(cx: &Cx, from_id: String, to_id: String, days: f64) -> Attributes {
    attributes! { cx =>
        @click=$(move |_e: Event| {
            let _from = from_id.to_owned(); let _to = to_id.to_owned(); let _days = days;
            raw!("(() => { const pad = value => String(value).padStart(2, '0'); const format = date => date.getFullYear() + '-' + pad(date.getMonth()+1) + '-' + pad(date.getDate()) + 'T' + pad(date.getHours()) + ':' + pad(date.getMinutes()); const end = new Date(); const start = new Date(end.getTime() - ${_days}.dehydrate() * 86400000); document.getElementById(${_from}.dehydrate()).value = format(start); document.getElementById(${_to}.dehydrate()).value = format(end); })()", ());
        })
    }
}

#[cfg(test)]
mod tests {
    use super::range_label;
    use crate::UiLanguage;

    #[test]
    fn labels_empty_and_complete_ranges() {
        assert_eq!(range_label("", "", UiLanguage::English), "All time");
        assert_eq!(
            range_label("", "", UiLanguage::ChineseSimplified),
            "全部时间"
        );
        assert_eq!(
            range_label("2026-09-20T10:00", "2026-09-21T10:00", UiLanguage::English),
            "09-20 10:00 → 09-21 10:00"
        );
    }
}
