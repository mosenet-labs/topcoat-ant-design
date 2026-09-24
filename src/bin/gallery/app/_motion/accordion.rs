use crate::locale::Locale;
use crate::locale::text;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::{Event, signal},
    view::{View, view},
};
use topcoat_ant_design::{AccordionItemConfig, accordion_item};

use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const ACCORDION_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/accordion.md"
));
const ACCORDION_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/accordion.md"
));

#[page]
pub(in crate::app) async fn accordion_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(ACCORDION_DOC_EN, ACCORDION_DOC_ZH);
    let active = signal(cx, String::new);
    let comment_count = signal(cx, || 1.0);
    let example_source = rust_code_block(document, 0);

    Ok(view! {
        page_header(
            eyebrow: "MOTION",
            title: text(locale, "Accordion 手风琴"),
            description: text(locale, "多个区块共用展开状态；切换时自动收起另一组，折叠中的表单值保持不变。"),
        )
        <div class="grid gap-6">
            component_example(id: "accordion-preview", title: text(locale, "组件预览"), description: text(locale, "试着切换两组，并勾选评论入口观察标题计数。"), source: example_source,
                <div class="grid gap-4 p-6">
                    accordion_item(
                        language: locale.ui(),
                        config: AccordionItemConfig::new("gallery-comment", text(locale, "评论 Trigger"), text(locale, "选择由评论指令触发的入口")).with_badge("OR"),
                        active: &active,
                        selected_count: Some(&comment_count),
                        <div class="grid gap-3 p-4 text-sm">
                            <label class="flex items-center gap-2"><input type="checkbox" checked="" @change=$(|event: Event| comment_count.set(if event.target.checked { 1.0 } else { 0.0 }))>(text(locale, "Merge Request 评论"))</label>
                            <span class="text-[#8c8c8c]">(text(locale, "未展开的分组不会清除选择状态。"))</span>
                        </div>
                    )
                    accordion_item(
                        language: locale.ui(),
                        config: AccordionItemConfig::new("gallery-event", text(locale, "自动事件 Trigger"), text(locale, "符合条件的事件直接创建任务")).with_badge("OR"),
                        active: &active,
                        <div class="p-4 text-sm text-[#595959]">(text(locale, "例如 Merge Request 变化或分支 Push。"))</div>
                    )
                </div>
            )
            markdown_document(source: document)
        </div>
    })
}
