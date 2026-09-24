use crate::locale::Locale;
use crate::locale::text;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};
use topcoat_ant_design::{DateTimeRangeConfig, date_time_range_filter};

use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const DATE_TIME_RANGE_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/date-time-range.md"
));
const DATE_TIME_RANGE_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/date-time-range.md"
));

#[page]
pub(in crate::app) async fn date_time_range_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(DATE_TIME_RANGE_DOC_EN, DATE_TIME_RANGE_DOC_ZH);
    let source = rust_code_block(document, 0);
    Ok(view! {
        page_header(
            eyebrow: "DATA ENTRY",
            title: text(locale, "DateTimeRange 时间范围"),
            description: text(locale, "把两个日期时间字段收纳为紧凑的弹出筛选控件，并保留原生表单提交能力。"),
        )
        <div class="grid gap-6">
            component_example(id: "date-time-range-basic", title: text(locale, "时间范围筛选"), description: text(locale, "支持快捷范围、清除和应用；示例不会提交到业务接口。"), source: source,
                <form class="flex min-h-52 items-start gap-3 p-6" method="get">
                    if locale == Locale::Zh {
                        <input type="hidden" name="lang" value="zh">
                    }
                    date_time_range_filter(language: locale.ui(), config: DateTimeRangeConfig::new("gallery-date-range", "2026-09-20T10:00", "2026-09-21T10:00"))
                    <button class="gr-button gr-button-primary h-10" type="submit">(text(locale, "筛选"))</button>
                </form>
            )
            markdown_document(source: document)
        </div>
    })
}
