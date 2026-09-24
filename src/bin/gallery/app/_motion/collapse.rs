use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    router::page,
    runtime::signal,
    view::{View, attributes, view},
};
use topcoat_ant_design::{collapse, collapse_trigger_attributes, icons::DOWN_OUTLINED};

use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const COLLAPSE_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/collapse.md"
));

#[page]
pub(in crate::app) async fn collapse_page(cx: &Cx) -> Result<impl View> {
    let basic_open = signal(cx, || true);
    let detail_open = signal(cx, || false);
    let basic_trigger = collapse_trigger_attributes(cx, "collapse-basic-content", &basic_open);
    let detail_trigger = collapse_trigger_attributes(cx, "collapse-detail-content", &detail_open);
    let basic_chevron = attributes! { cx =>
        :class=$(if basic_open.get() { "rotate-180 transition-transform duration-200" } else { "transition-transform duration-200" })
    };
    let detail_chevron = attributes! { cx =>
        :class=$(if detail_open.get() { "rotate-180 transition-transform duration-200" } else { "transition-transform duration-200" })
    };
    let basic_panel = attributes! {
        aria-labelledby="collapse-basic-trigger"
    };
    let detail_panel = attributes! {
        aria-labelledby="collapse-detail-trigger"
    };
    let example_source = rust_code_block(COLLAPSE_DOC, 0);

    Ok(view! {
        page_header(
            eyebrow: "MOTION",
            title: "Collapse 折叠动画",
            description: "使用 Topcoat component、signal 和响应式属性，为未知高度内容提供平滑、可逆的展开与收起。",
        )
        <div class="grid gap-6">
            component_example(id: "collapse-preview", title: "组件预览", description: "可以连续快速点击，观察动画从当前位置自然反向。", source: example_source,
                <div class="grid grid-cols-2 gap-5 p-6 max-[760px]:grid-cols-1">
                    <article class="overflow-hidden rounded-lg border border-[#d9d9d9] bg-white">
                        <button id="collapse-basic-trigger" class="flex min-h-12 w-full cursor-pointer items-center justify-between border-0 bg-[#fafafa] px-4 py-3 font-mono text-sm font-semibold text-[#262626] hover:bg-[#f5f5f5]" type="button" (basic_trigger)><span>"基础折叠"</span>icon(data: DOWN_OUTLINED, size: 16, attrs: basic_chevron)</button>
                        collapse(id: "collapse-basic-content", open: &basic_open, attrs: basic_panel,
                            <div class="border-t border-[#edf0f4] px-4 py-4"><p class="m-0 text-sm leading-6 text-[#595959]">"内容高度无需预先计算。Grid 轨道会从 0fr 过渡到 1fr，关闭时按同一路径反向执行。"</p></div>
                        )
                    </article>
                    <article class="overflow-hidden rounded-lg border border-[#d9d9d9] bg-white">
                        <button id="collapse-detail-trigger" class="flex min-h-12 w-full cursor-pointer items-center justify-between border-0 bg-[#fafafa] px-4 py-3 font-mono text-sm font-semibold text-[#262626] hover:bg-[#f5f5f5]" type="button" (detail_trigger)><span>"多段内容"</span>icon(data: DOWN_OUTLINED, size: 16, attrs: detail_chevron)</button>
                        collapse(id: "collapse-detail-content", open: &detail_open, attrs: detail_panel,
                            <div class="border-t border-[#edf0f4] px-4 py-4"><ul class="my-0 grid gap-2 pl-5 text-sm leading-6 text-[#595959]"><li>"透明度与高度同步过渡。"</li><li>"关闭后内容不可见且不会响应鼠标。"</li><li>"系统减少动态效果时自动取消过渡。"</li></ul><button class="mt-4 h-8 cursor-pointer rounded-md border border-[#d9d9d9] bg-white px-3 font-mono text-sm text-[#262626] hover:border-[#1677ff] hover:text-[#1677ff]" type="button">"内部操作"</button></div>
                        )
                    </article>
                </div>
            )
            markdown_document(source: COLLAPSE_DOC)
        </div>
    })
}
