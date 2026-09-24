use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::signal,
    view::{View, view},
};
use topcoat_ant_design::{
    DialogConfig, dialog, dialog_close_attributes, dialog_trigger_attributes,
};

use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const DIALOG_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/dialog.md"
));

#[page]
pub(in crate::app) async fn dialog_page(cx: &Cx) -> Result<impl View> {
    let busy = signal(cx, || false);
    let trigger = dialog_trigger_attributes(cx, "gallery-connection-dialog");
    let cancel = dialog_close_attributes(cx, "gallery-connection-dialog");
    let save = dialog_close_attributes(cx, "gallery-connection-dialog");
    let example_source = rust_code_block(DIALOG_DOC, 0);

    Ok(view! {
        page_header(
            eyebrow: "FEEDBACK",
            title: "Dialog 模态对话框",
            description: "用于编辑连接、填写配置等需要集中处理的任务。下方说明与 dialog 方法的 Rustdoc 来自同一份 Markdown。",
        )
        <div class="grid gap-6">
            component_example(id: "dialog-preview", title: "组件预览", description: "原生模态行为包含焦点约束、Escape 关闭和统一标题栏。", source: example_source,
                <div class="p-6">
                    <button class="h-9 cursor-pointer rounded-md border border-[#1677ff] bg-[#1677ff] px-4 font-mono text-sm text-white hover:bg-[#4096ff]" type="button" (trigger)>"编辑连接"</button>
                </div>
            )
            markdown_document(source: DIALOG_DOC)
        </div>
        dialog(
            config: DialogConfig::new("gallery-connection-dialog", "编辑示例连接")
                .with_eyebrow("CONNECTION"),
            busy: &busy,
            <form class="flex min-h-0 flex-1 flex-col">
                <div class="min-h-0 flex-1 overflow-y-auto px-6 py-5 max-[640px]:px-[18px]">
                    <p class="mb-5 mt-0 text-sm leading-6 text-[#595959]">"修改连接信息后保存。这里仅演示组件，不会发送请求。"</p>
                    <label class="grid gap-2 text-sm font-medium" for="gallery-connection-name">"连接名称"</label>
                    <input class="mt-2 h-10 rounded-md border border-[#d9d9d9] px-3 font-mono text-sm outline-none focus:border-[#1677ff]" id="gallery-connection-name" value="示例 GitLab" autocomplete="off">
                </div>
                <footer class="flex shrink-0 justify-end gap-2.5 border-t border-[#f0f0f0] bg-[#fafafa] px-6 py-4 max-[640px]:px-[18px]">
                    <button class="h-8 cursor-pointer rounded-md border border-[#d9d9d9] bg-white px-4 font-mono text-sm text-[#262626]" type="button" (cancel)>"取消"</button>
                    <button class="h-8 cursor-pointer rounded-md border border-[#1677ff] bg-[#1677ff] px-4 font-mono text-sm text-white" type="button" (save)>"保存"</button>
                </footer>
            </form>
        )
    })
}
