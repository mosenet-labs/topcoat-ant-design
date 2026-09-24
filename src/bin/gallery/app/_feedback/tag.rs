use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const TAG_DOC: &str = include_str!(concat!(
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
pub(in crate::app) async fn tag_page() -> Result<impl View> {
    let source = rust_code_block(TAG_DOC, 0);
    Ok(view! {
        page_header(eyebrow: "TAG", title: "Tag 标签", description: "以文字和语义颜色区分状态。")
        <div class="grid gap-6">
            component_example(id: "tag-preview", title: "组件预览", description: "查看不同语义颜色的标签。", source: source,
                <div class="flex flex-wrap gap-3 p-6">
                    tag(tone: TagTone::Success, "已启用")
                    tag(tone: TagTone::Default, "已停用")
                    tag(tone: TagTone::Processing, "运行中")
                    tag(tone: TagTone::Warning, "待确认")
                    tag(tone: TagTone::Error, "失败")
                </div>
            )
            markdown_document(source: TAG_DOC)
        </div>
    })
}
