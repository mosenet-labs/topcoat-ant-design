use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::signal,
    view::{View, view},
};
use topcoat_ant_design::{dropdown_menu, dropdown_menu_trigger_attributes};

use crate::{
    app::page_header,
    demo::component_example,
    locale::Locale,
    markdown::{markdown_document, rust_code_block},
};

const DROPDOWN_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/dropdown-menu.md"
));
const DROPDOWN_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/dropdown-menu.md"
));

#[page]
pub(in crate::app) async fn dropdown_menu_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(DROPDOWN_DOC_EN, DROPDOWN_DOC_ZH);
    let source = rust_code_block(document, 0);
    let action = signal(cx, || locale.select("None", "无").to_owned());
    let trigger = dropdown_menu_trigger_attributes(cx, "gallery-actions-menu");
    let edit = locale.select("Edit", "编辑");
    let duplicate = locale.select("Duplicate", "复制");
    let export = locale.select("Export", "导出");

    Ok(view! {
        page_header(
            eyebrow: "NAVIGATION",
            title: locale.select("Dropdown Menu", "Dropdown 下拉菜单"),
            description: locale.select(
                "Group secondary actions in a compact menu next to their trigger.",
                "把次要操作收进触发按钮旁的紧凑菜单。",
            ),
        )
        <div class="grid gap-6">
            component_example(
                id: "dropdown-menu-preview",
                title: locale.select("Component preview", "组件预览"),
                description: locale.select("Choose an action; outside click and Escape close the menu.", "选择一个操作；点击外部或按 Escape 可关闭菜单。"),
                source: source,
                <div class="p-6">
                    <div class="rounded-lg border border-[#edf0f4] bg-[#fafafa] p-5">
                        <p class="mb-4 mt-0 text-sm text-[#595959]">(locale.select("Last action:", "最近操作：")) " " <strong class="font-medium text-[#262626]">$(action.get())</strong></p>
                        <button class="gr-button gr-button-primary inline-flex items-center gap-2" type="button" (trigger)>(locale.select("Actions", "操作"))<span aria-hidden="true">"⌄"</span></button>
                        dropdown_menu(id: "gallery-actions-menu", label: locale.select("Example actions", "示例操作"),
                            <button type="button" @click=$(|_e| action.set(edit.to_owned()))>(edit)</button>
                            <button type="button" @click=$(|_e| action.set(duplicate.to_owned()))>(duplicate)</button>
                            <button type="button" @click=$(|_e| action.set(export.to_owned()))>(export)</button>
                            <button type="button" disabled="">(locale.select("Unavailable", "暂不可用"))</button>
                        )
                    </div>
                </div>
            )
            markdown_document(source: document)
        </div>
    })
}
