use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::{Event, signal},
    view::{View, attributes, element_id, view},
};
use topcoat_ant_design::{
    ButtonSize, ButtonVariant, button_variants, dropdown_menu, dropdown_menu_content,
    dropdown_menu_item, dropdown_menu_trigger,
};

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
    let menu_id = element_id(cx);
    let action = signal(cx, || 0usize);
    let none = locale.select("None", "无");
    let edit = locale.select("Edit", "编辑");
    let duplicate = locale.select("Duplicate", "复制");
    let export = locale.select("Export", "导出");

    Ok(view! {
        page_header(
            eyebrow: "NAVIGATION / OFFICIAL",
            title: locale.select("Dropdown Menu", "Dropdown 下拉菜单"),
            description: locale.select(
                "A native popover menu for related actions.",
                "使用原生 Popover 组织相关操作。",
            )
        )
        <div class="grid gap-6">
            component_example(
                id: "dropdown-menu-preview",
                title: locale.select("Component preview", "组件预览"),
                description: locale.select(
                    "Open the menu and choose an action.",
                    "展开菜单并选择操作。",
                ),
                source: source,
                <div class="rounded-lg border border-border bg-card p-6">
                    <p class="mb-4 mt-0 text-sm text-muted-foreground">
                        (locale.select("Last action:", "最近操作："))
                        " "
                        <strong class="text-foreground">
                            $(if action.get() == 1usize { edit } else if action.get() == 2usize { duplicate } else if action.get() == 3usize { export } else { none })
                        </strong>
                    </p>
                    dropdown_menu(
                        dropdown_menu_trigger(
                            target: &menu_id,
                            attrs: attributes! {
                                class=(button_variants(
                                    ButtonVariant::Outline,
                                    ButtonSize::Md,
                                ))
                            },
                            (locale.select("Actions", "操作"))
                        )
                        dropdown_menu_content(
                            id: &menu_id,
                            dropdown_menu_item(
                                close: Some(&menu_id),
                                attrs: attributes! {
                                    type="button"
                                    @click=$(|_e: Event| action.set(1usize))
                                },
                                (edit)
                            )
                            dropdown_menu_item(
                                close: Some(&menu_id),
                                attrs: attributes! {
                                    type="button"
                                    @click=$(|_e: Event| action.set(2usize))
                                },
                                (duplicate)
                            )
                            dropdown_menu_item(
                                close: Some(&menu_id),
                                attrs: attributes! {
                                    type="button"
                                    @click=$(|_e: Event| action.set(3usize))
                                },
                                (export)
                            )
                            dropdown_menu_item(
                                close: Some(&menu_id),
                                attrs: attributes! { type="button" disabled="" },
                                (locale.select("Unavailable", "暂不可用"))
                            )
                        )
                    )
                </div>
            )
            markdown_document(source: document)
        </div>
    })
}
