use crate::locale::Locale;
use crate::locale::text;
use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    router::page,
    runtime::signal,
    view::{View, attributes, view},
};
use topcoat_ant_design::{collapse, collapse_trigger_attributes, icons::DOWN_OUTLINED};

use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const COLLAPSE_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/collapse.md"
));
const COLLAPSE_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/collapse.md"
));

#[page]
pub(in crate::app) async fn collapse_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(COLLAPSE_DOC_EN, COLLAPSE_DOC_ZH);
    let basic_open = signal(cx, || true);
    let detail_open = signal(cx, || false);
    let basic_trigger = collapse_trigger_attributes(cx, "collapse-basic-content", &basic_open);
    let detail_trigger = collapse_trigger_attributes(cx, "collapse-detail-content", &detail_open);
    let basic_chevron = attributes! { cx =>
        :class=$(if basic_open.get() { "rotate-180 transition-transform duration-200" } else { "transition-transform duration-200" })
    };
    let detail_chevron = attributes! { cx =>
        :class=$(if detail_open.get() { "rotate-180 transition-transform duration-200" } else { "transition-transform duration-200" })
    };
    let basic_panel = attributes! {
        aria-labelledby="collapse-basic-trigger"
    };
    let detail_panel = attributes! {
        aria-labelledby="collapse-detail-trigger"
    };
    let example_source = rust_code_block(document, 0);

    Ok(view! {
        page_header(
            eyebrow: "MOTION",
            title: text(locale, "Collapse 折叠动画"),
            description: text(locale, "使用 Topcoat component、signal 和响应式属性，为未知高度内容提供平滑、可逆的展开与收起。"),
        )
        <div class="grid gap-6">
            component_example(id: "collapse-preview", title: text(locale, "组件预览"), description: text(locale, "可以连续快速点击，观察动画从当前位置自然反向。"), source: example_source,
                <div class="grid grid-cols-2 gap-5 p-6 max-[760px]:grid-cols-1">
                    <article class="overflow-hidden rounded-lg border border-border bg-card">
                        <button id="collapse-basic-trigger" class="flex min-h-12 w-full cursor-pointer items-center justify-between border-0 bg-background px-4 py-3 font-mono text-sm font-semibold text-foreground hover:bg-background" type="button" (basic_trigger)><span>(text(locale, "基础折叠"))</span>icon(data: DOWN_OUTLINED, size: 16, attrs: basic_chevron)</button>
                        collapse(id: "collapse-basic-content", open: &basic_open, attrs: basic_panel,
                            <div class="border-t border-border px-4 py-4"><p class="m-0 text-sm leading-6 text-muted-foreground">(text(locale, "内容高度无需预先计算。Grid 轨道会从 0fr 过渡到 1fr，关闭时按同一路径反向执行。"))</p></div>
                        )
                    </article>
                    <article class="overflow-hidden rounded-lg border border-border bg-card">
                        <button id="collapse-detail-trigger" class="flex min-h-12 w-full cursor-pointer items-center justify-between border-0 bg-background px-4 py-3 font-mono text-sm font-semibold text-foreground hover:bg-background" type="button" (detail_trigger)><span>(text(locale, "多段内容"))</span>icon(data: DOWN_OUTLINED, size: 16, attrs: detail_chevron)</button>
                        collapse(id: "collapse-detail-content", open: &detail_open, attrs: detail_panel,
                            <div class="border-t border-border px-4 py-4"><ul class="my-0 grid gap-2 pl-5 text-sm leading-6 text-muted-foreground"><li>(text(locale, "透明度与高度同步过渡。"))</li><li>(text(locale, "关闭后内容不可见且不会响应鼠标。"))</li><li>(text(locale, "系统减少动态效果时自动取消过渡。"))</li></ul><button class="mt-4 h-8 cursor-pointer rounded-md border border-border bg-card px-3 font-mono text-sm text-foreground hover:border-primary hover:text-primary" type="button">(text(locale, "内部操作"))</button></div>
                        )
                    </article>
                </div>
            )
            markdown_document(source: document)
        </div>
    })
}
