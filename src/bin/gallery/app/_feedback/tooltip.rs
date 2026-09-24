use crate::locale::Locale;
use crate::locale::text;
use topcoat::{
    Result,
    router::page,
    view::{View, view},
};
use topcoat_ant_design::tooltip;

use crate::{
    app::page_header,
    demo::component_example,
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
pub(in crate::app) async fn tooltip_page(cx: &topcoat::context::Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(TOOLTIP_DOC_EN, TOOLTIP_DOC_ZH);
    let source = rust_code_block(document, 0);
    Ok(view! {
        page_header(eyebrow: "TOOLTIP", title: text(locale, "Tooltip 文字提示"), description: text(locale, "悬停或聚焦查看完整文本，气泡与箭头自动适应视口边缘。"))
        <div class="grid gap-6">
            component_example(id: "tooltip-preview", title: text(locale, "组件预览"), description: text(locale, "鼠标移入提示可选择文本；Tab 聚焦也可显示，Escape 关闭。滚动页面或缩窄窗口可验证翻转和箭头偏移。"), source: source,
                <div class="flex flex-wrap items-center justify-between gap-8 p-6">
                    tooltip(id: "gallery-tooltip-left", content: text(locale, "左侧空间不足时，提示向右偏移，箭头仍然指向触发元素。"), <span>(text(locale, "左侧提示"))</span>)
                    tooltip(id: "gallery-tooltip-commit", content: "4be48fb908424803fbe041ad5a50c0bf73f4f425", <span>"commit 4be48fb9…"</span>)
                    tooltip(id: "gallery-tooltip-right", content: text(locale, "右侧空间不足时，提示向左偏移，箭头仍然指向触发元素。"), <span>(text(locale, "右侧提示"))</span>)
                </div>
            )
            markdown_document(source: document)
        </div>
    })
}
