use crate::locale::Locale;
use crate::locale::text;
use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const TAG_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/tag.md"
));
const TAG_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/tag.md"
));
use topcoat::{
    Result,
    router::page,
    view::{View, view},
};
use topcoat_ant_design::{TagTone, tag};

#[page]
pub(in crate::app) async fn tag_page(cx: &topcoat::context::Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(TAG_DOC_EN, TAG_DOC_ZH);
    let source = rust_code_block(document, 0);
    Ok(view! {
        page_header(eyebrow: "TAG", title: text(locale, "Tag 标签"), description: text(locale, "以文字和语义颜色区分状态。"))
        <div class="grid gap-6">
            component_example(id: "tag-preview", title: text(locale, "组件预览"), description: text(locale, "查看不同语义颜色的标签。"), source: source,
                <div class="flex flex-wrap gap-3 p-6">
                    tag(tone: TagTone::Success, (text(locale, "已启用")))
                    tag(tone: TagTone::Default, (text(locale, "已停用")))
                    tag(tone: TagTone::Processing, (text(locale, "运行中")))
                    tag(tone: TagTone::Warning, (text(locale, "待确认")))
                    tag(tone: TagTone::Error, (text(locale, "失败")))
                </div>
            )
            markdown_document(source: document)
        </div>
    })
}
