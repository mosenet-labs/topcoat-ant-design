use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, attributes, view},
};
use topcoat_ant_design::{tooltip, tooltip_content};

use crate::{
    app::page_header,
    demo::component_example,
    locale::Locale,
    markdown::{markdown_document, rust_code_block},
};

const TOOLTIP_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/tooltip.md"
));
const TOOLTIP_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/tooltip.md"
));

#[page]
pub(in crate::app) async fn tooltip_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(TOOLTIP_DOC_EN, TOOLTIP_DOC_ZH);
    let source = rust_code_block(document, 0);
    Ok(view! {
        page_header(
            eyebrow: "FEEDBACK / OFFICIAL",
            title: locale.select("Tooltip", "Tooltip 文字提示"),
            description: locale.select("The official tooltip appears on hover or keyboard focus.", "官方 Tooltip 在悬停或键盘聚焦时显示提示。"),
        )
        <div class="grid gap-6">
            component_example(
                id: "tooltip-preview",
                title: locale.select("Component preview", "组件预览"),
                description: locale.select("Hover or focus each trigger.", "悬停或聚焦下方触发元素。"),
                source: source,
                <div class="flex flex-wrap items-center gap-8 p-6">
                    tooltip(
                        <button class="rounded-lg border border-border bg-card px-3 py-2 text-foreground" type="button" aria-describedby="gallery-tooltip-copy">(locale.select("Copy link", "复制链接"))</button>
                        tooltip_content(attrs: attributes! { id="gallery-tooltip-copy" }, (locale.select("Copy a link to this page", "复制当前页面链接")))
                    )
                    tooltip(
                        <span tabindex="0" aria-describedby="gallery-tooltip-commit" class="cursor-help rounded-md border-b border-dashed border-border text-foreground">"commit 4be48fb9…"</span>
                        tooltip_content(attrs: attributes! { id="gallery-tooltip-commit" }, "4be48fb908424803fbe041ad5a50c0bf73f4f425")
                    )
                </div>
            )
            markdown_document(source: document)
        </div>
    })
}
