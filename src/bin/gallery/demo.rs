use crate::locale::text;
use topcoat::{
    Result,
    context::Cx,
    runtime::signal,
    view::{Child, View, component, view},
};
use topcoat_ant_design::{collapse, collapse_trigger_attributes};

/// Gallery 共用的组件示例外壳，负责预览说明和代码展开交互。
#[component]
pub(crate) async fn component_example(
    cx: &Cx,
    id: &str,
    title: &str,
    description: &str,
    source: &str,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let locale = crate::locale::Locale::current(cx);
    let code_open = signal(cx, || false);
    let code_id = format!("{id}-source");
    let title_id = format!("{id}-title");
    let trigger = collapse_trigger_attributes(cx, code_id.as_str(), &code_open);
    let hide_code_label = text(locale, "收起示例代码");
    let show_code_label = text(locale, "显示示例代码");

    Ok(view! {
        <section class="gr-gallery-demo overflow-hidden rounded-xl border border-[#e8eaee] bg-white shadow-sm" aria-labelledby=(title_id.as_str())>
            <header class="border-b border-[#edf0f4] px-6 py-5">
                <h2 class="m-0 text-lg font-semibold" id=(title_id.as_str())>(title)</h2>
                <p class="mb-0 mt-1.5 text-sm leading-6 text-[#8c8c8c]">(description)</p>
            </header>
            <div class="min-w-0">(child)</div>
            <div class="flex min-h-12 items-center justify-center border-t border-[#edf0f4]">
                <button class="group inline-flex size-8 cursor-pointer items-center justify-center rounded-md border-0 bg-transparent p-0 font-mono text-[13px] font-semibold text-[#8c8c8c] transition-colors duration-150 hover:bg-[#f5f5f5] hover:text-[#1677ff] focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-[#91caff]" type="button" (trigger) :title=$(if code_open.get() { hide_code_label } else { show_code_label })>
                    <span aria-hidden="true" :hidden=$(code_open.get())>"<>"</span>
                    <span aria-hidden="true" :hidden=$(!code_open.get())>"</>"</span>
                    <span class="sr-only">$(if code_open.get() { hide_code_label } else { show_code_label })</span>
                </button>
            </div>
            collapse(id: code_id.as_str(), open: &code_open,
                <div class="border-t border-[#edf0f4] bg-[#fafafa]">
                    <div class="flex items-center justify-between px-5 py-2.5 text-xs text-[#8c8c8c]"><span>(text(locale, "示例代码"))</span><span>"Rust"</span></div>
                    <pre class="gallery-code-block"><code class="language-rust">(source)</code></pre>
                </div>
            )
        </section>
    })
}
