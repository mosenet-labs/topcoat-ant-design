use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::{Event, signal},
    view::{View, attributes, view},
};
use topcoat_ant_design::{
    ButtonVariant, button, dialog, dialog_content, dialog_description, dialog_footer,
    dialog_header, dialog_title, input, label,
};

use crate::{
    app::page_header,
    demo::component_example,
    locale::Locale,
    markdown::{markdown_document, rust_code_block},
};

const DIALOG_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/dialog.md"
));
const DIALOG_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/dialog.md"
));

#[page]
pub(in crate::app) async fn dialog_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(DIALOG_DOC_EN, DIALOG_DOC_ZH);
    let source = rust_code_block(document, 0);
    let open = signal(cx, || false);

    Ok(view! {
        page_header(
            eyebrow: "FEEDBACK / OFFICIAL",
            title: locale.select("Dialog", "Dialog 对话框"),
            description: locale.select("The official dialog groups a title, description, form fields and actions.", "使用官方 Dialog 组织标题、说明、表单字段与操作。"),
        )
        <div class="grid gap-6">
            component_example(
                id: "dialog-preview",
                title: locale.select("Component preview", "组件预览"),
                description: locale.select("Open the panel, edit its field, then close it.", "打开面板、编辑字段并关闭。"),
                source: source,
                <div class="p-6">
                    button(attrs: attributes! { type="button" @click=$(|_e: Event| open.set(true)) }, (locale.select("Edit connection", "编辑连接")))
                </div>
            )
            markdown_document(source: document)
        </div>
        dialog(open: $(open.get()), attrs: attributes! { aria-labelledby="gallery-dialog-title" aria-describedby="gallery-dialog-description" },
            dialog_content(
                dialog_header(
                    dialog_title(attrs: attributes! { id="gallery-dialog-title" }, (locale.select("Edit example connection", "编辑示例连接")))
                    dialog_description(attrs: attributes! { id="gallery-dialog-description" }, (locale.select("Change the connection name in this preview.", "在预览中修改连接名称。")))
                )
                <div class="grid gap-2 py-2">
                    label(attrs: attributes! { for="gallery-connection-name" }, (locale.select("Connection name", "连接名称")))
                    input(attrs: attributes! { id="gallery-connection-name" value="Example GitLab" autocomplete="off" })
                </div>
                dialog_footer(
                    button(variant: ButtonVariant::Outline, attrs: attributes! { type="button" @click=$(|_e: Event| open.set(false)) }, (locale.select("Cancel", "取消")))
                    button(attrs: attributes! { type="button" @click=$(|_e: Event| open.set(false)) }, (locale.select("Save", "保存")))
                )
            )
        )
    })
}
