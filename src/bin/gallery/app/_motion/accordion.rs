use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::{Event, signal},
    view::{View, attributes, view},
};
use topcoat_ant_design::{accordion, accordion_content, accordion_item, accordion_trigger, badge};

use crate::{
    app::page_header,
    demo::component_example,
    locale::Locale,
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
    let source = rust_code_block(document, 0);
    let selected_count = signal(cx, || 1usize);

    Ok(view! {
        page_header(
            eyebrow: "MOTION / OFFICIAL",
            title: locale.select("Accordion", "Accordion 手风琴"),
            description: locale.select("Official details elements share a name so one section stays open at a time.", "官方 Accordion 通过同名 details 控制单项展开。"),
        )
        <div class="grid gap-6">
            component_example(
                id: "accordion-preview",
                title: locale.select("Component preview", "组件预览"),
                description: locale.select("Open either section and change the checkbox value.", "切换分组并修改复选框值。"),
                source: source,
                <div class="p-6">
                    accordion(attrs: attributes! { class="rounded-xl border border-border bg-card px-5" },
                        accordion_item(attrs: attributes! { name="gallery-accordion" open="" },
                            accordion_trigger(
                                <span>(locale.select("Comment trigger", "评论 Trigger"))</span>
                                badge("OR")
                                <span class="text-xs text-muted-foreground">$(selected_count.get())</span>
                            )
                            accordion_content(
                                <label class="flex items-center gap-2">
                                    <input type="checkbox" checked="" @change=$(|event: Event| selected_count.set(if event.target.checked { 1 } else { 0 }))>
                                    (locale.select("Merge Request comments", "Merge Request 评论"))
                                </label>
                            )
                        )
                        accordion_item(attrs: attributes! { name="gallery-accordion" },
                            accordion_trigger((locale.select("Automatic events", "自动事件 Trigger")))
                            accordion_content((locale.select("For example, a branch push or Merge Request change.", "例如分支 Push 或 Merge Request 变化。")))
                        )
                    )
                </div>
            )
            markdown_document(source: document)
        </div>
    })
}
