use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::signal,
    view::{View, view},
};
use topcoat_ant_design::{SearchOption, search_multi_select};

use crate::{
    app::page_header,
    demo::component_example,
    locale::Locale,
    markdown::{markdown_document, rust_code_block},
};

const SEARCH_MULTI_SELECT_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/search-multi-select.md"
));
const SEARCH_MULTI_SELECT_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/search-multi-select.md"
));

#[page]
pub(in crate::app) async fn search_multi_select_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(SEARCH_MULTI_SELECT_DOC_EN, SEARCH_MULTI_SELECT_DOC_ZH);
    let basic_source = rust_code_block(document, 0);
    let disabled_source = rust_code_block(document, 1);
    let options = vec![
        SearchOption {
            value: "gpt-4.1".into(),
            label: "GPT-4.1".into(),
        },
        SearchOption {
            value: "o3".into(),
            label: "o3".into(),
        },
        SearchOption {
            value: "claude-sonnet-4".into(),
            label: "Claude Sonnet 4".into(),
        },
        SearchOption {
            value: "gemini-2.5-pro".into(),
            label: "Gemini 2.5 Pro".into(),
        },
    ];
    let selected = signal(cx, || r#"["gpt-4.1"]"#.to_owned());
    let search = signal(cx, String::new);
    let open = signal(cx, || false);
    let disabled_selected = signal(cx, || r#"["o3"]"#.to_owned());
    let disabled_search = signal(cx, String::new);
    let disabled_open = signal(cx, || false);

    Ok(view! {
        page_header(
            eyebrow: "DATA ENTRY",
            title: locale.select("SearchMultiSelect", "SearchMultiSelect 搜索多选"),
            description: locale.select(
                "Search options, choose multiple values, and remove selections without leaving the form.",
                "在表单内搜索选项、选择多个值，并随时移除已选项。",
            ),
        )
        <div class="grid gap-6">
            component_example(
                id: "search-multi-select-basic",
                title: locale.select("Search and select models", "搜索并选择模型"),
                description: locale.select("Search by model name or ID, then select or remove multiple entries.", "按模型名称或 ID 搜索，然后选择或移除多个选项。"),
                source: basic_source,
                <div class="grid max-w-[640px] gap-4 bg-background p-6 max-[520px]:p-4">
                    search_multi_select(id: "gallery-models", options: &options, selected: &selected, search: &search, open: &open, label: Some(locale.select("Models", "模型")), placeholder: Some(locale.select("Search models", "搜索模型")), language: locale.ui())
                    <p class="m-0 rounded-lg border border-border bg-card px-3 py-2 text-xs text-muted-foreground"><span class="font-semibold text-foreground">(locale.select("Selected values", "已选值")) ": "</span><code class="break-all">$(selected.get())</code></p>
                </div>
            )
            component_example(
                id: "search-multi-select-disabled",
                title: locale.select("Disabled state", "禁用状态"),
                description: locale.select("The selected values remain visible while editing is disabled.", "禁用编辑时仍显示已选值。"),
                source: disabled_source,
                <div class="max-w-[640px] bg-background p-6 max-[520px]:p-4">
                    search_multi_select(id: "gallery-models-disabled", options: &options, selected: &disabled_selected, search: &disabled_search, open: &disabled_open, label: Some(locale.select("Models", "模型")), disabled: true, language: locale.ui())
                </div>
            )
            markdown_document(source: document)
        </div>
    })
}
