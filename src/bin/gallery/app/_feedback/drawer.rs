use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::signal,
    view::{View, view},
};
use topcoat_ant_design::{DrawerConfig, drawer};

use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const DRAWER_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/drawer.md"
));

#[page]
pub(in crate::app) async fn drawer_page(cx: &Cx) -> Result<impl View> {
    let open = signal(cx, || false);
    let example_source = rust_code_block(DRAWER_DOC, 0);

    Ok(view! {
        page_header(
            eyebrow: "FEEDBACK",
            title: "Drawer 抽屉",
            description: "在不离开列表的情况下查看一条记录的完整详情。",
        )
        <div class="grid gap-6">
            component_example(id: "drawer-preview", title: "组件预览", description: "打开后可点击遮罩、关闭按钮或按 Escape。", source: example_source,
                <div class="p-6"><button class="h-9 cursor-pointer rounded-md border border-[#1677ff] bg-[#1677ff] px-4 font-mono text-sm text-white hover:bg-[#4096ff]" type="button" @click=$(|_e| open.set(true))>"查看事件详情"</button></div>
            )
            markdown_document(source: DRAWER_DOC)
        </div>
        drawer(config: DrawerConfig::new("gallery-event-drawer", "Webhook 事件详情"), open: &open,
            <div class="grid gap-5">
                <section class="rounded-lg border border-[#f0f0f0] p-4"><h3 class="mb-3 mt-0 text-sm font-semibold">"事件信息"</h3><dl class="m-0 grid grid-cols-[140px_minmax(0,1fr)] gap-x-4 gap-y-3 text-sm"><dt class="text-[#8c8c8c]">"事件类型"</dt><dd class="m-0">"Note Hook"</dd><dt class="text-[#8c8c8c]">"处理状态"</dt><dd class="m-0 text-[#389e0d]">"处理成功"</dd></dl></section>
                <section class="rounded-lg border border-[#f0f0f0] p-4"><h3 class="mb-3 mt-0 text-sm font-semibold">"Request Body"</h3><pre class="m-0 overflow-auto rounded-md bg-[#0d1117] p-4 text-xs leading-6 text-[#d6deeb]">"{\n  \"object_kind\": \"note\",\n  \"project_id\": 2788\n}"</pre></section>
            </div>
        )
    })
}
