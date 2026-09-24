use topcoat::{
    Result,
    router::page,
    view::{View, view},
};

use crate::{app::page_header, markdown::markdown_document};

const GETTING_STARTED_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/getting-started.md"
));

#[page]
pub(in crate::app) async fn getting_started_page() -> Result<impl View> {
    Ok(view! {
        page_header(
            eyebrow: "GETTING STARTED",
            title: "使用组件库",
            description: "把 Topcoat Ant Design 接入 Topcoat 应用，需要完成依赖、页面资源和路由注册，再按需调用组件。",
        )
        <div class="grid gap-6">
            <section class="rounded-xl border border-[#91caff] bg-[#e6f4ff] p-6 shadow-sm" aria-labelledby="integration-flow">
                <div class="mb-5 flex items-start justify-between gap-5 max-[620px]:block">
                    <div><p class="m-0 text-xs font-bold tracking-[0.1em] text-[#0958d9]">"接入流程"</p><h2 class="mb-0 mt-2 text-xl font-semibold" id="integration-flow">"三个显式接入点"</h2></div>
                    <span class="rounded-full border border-[#91caff] bg-white px-3 py-1 text-xs text-[#0958d9] max-[620px]:mt-4 max-[620px]:inline-block">"无需复制 CSS"</span>
                </div>
                <ol class="m-0 grid list-none grid-cols-3 gap-4 p-0 max-[760px]:grid-cols-1">
                    <li class="rounded-lg border border-[#bae0ff] bg-white p-4"><span class="mb-3 grid size-7 place-items-center rounded-full bg-[#1677ff] text-xs font-bold text-white">"1"</span><strong class="block text-sm">"添加依赖"</strong><span class="mt-1.5 block text-xs leading-5 text-[#595959]">"工作区引用 path，外部项目固定 Git commit。"</span></li>
                    <li class="rounded-lg border border-[#bae0ff] bg-white p-4"><span class="mb-3 grid size-7 place-items-center rounded-full bg-[#1677ff] text-xs font-bold text-white">"2"</span><strong class="block text-sm">"加载页面资源"</strong><span class="mt-1.5 block text-xs leading-5 text-[#595959]">"根布局调用 head_assets() 加载样式和字体。"</span></li>
                    <li class="rounded-lg border border-[#bae0ff] bg-white p-4"><span class="mb-3 grid size-7 place-items-center rounded-full bg-[#1677ff] text-xs font-bold text-white">"3"</span><strong class="block text-sm">"注册服务端资源"</strong><span class="mt-1.5 block text-xs leading-5 text-[#595959]">"由宿主 Router 暴露字体与静态资源。"</span></li>
                </ol>
            </section>
            markdown_document(source: GETTING_STARTED_DOC)
        </div>
    })
}
