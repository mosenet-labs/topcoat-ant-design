use crate::locale::Locale;
use crate::locale::text;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};
use topcoat_ant_design::{FormFieldConfig, form_field};

use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const FORM_FIELD_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/form-field.md"
));
const FORM_FIELD_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/form-field.md"
));

#[page]
pub(in crate::app) async fn form_field_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(FORM_FIELD_DOC_EN, FORM_FIELD_DOC_ZH);
    let basic_source = rust_code_block(document, 0);
    let error_source = rust_code_block(document, 1);

    Ok(view! {
        page_header(
            eyebrow: "DATA ENTRY",
            title: text(locale, "FormField 表单字段"),
            description: text(locale, "统一标签、辅助信息和错误反馈，同时保留原生输入控件与 Topcoat 绑定能力。"),
        )
        <div class="grid gap-6">
            component_example(id: "form-field-basic", title: text(locale, "基础字段"), description: text(locale, "标签会通过稳定 id 与输入控件关联；必填标记只表达界面语义。"), source: basic_source,
                <div class="max-w-[560px] p-6">
                    form_field(config: FormFieldConfig::new("gallery-automation-name", text(locale, "自动化名称")).required().with_hint(text(locale, "用于项目内识别这项自动化。")),
                        <input id="gallery-automation-name" class="h-10 rounded-md border border-[#d9d9d9] px-3 font-mono text-sm outline-none transition-colors focus:border-[#1677ff] focus:shadow-[0_0_0_2px_rgba(22,119,255,0.12)]" type="text" value=(text(locale, "连通性测试")) required="" aria-describedby="gallery-automation-name-help">
                    )
                </div>
            )
            component_example(id: "form-field-error", title: text(locale, "错误状态"), description: text(locale, "服务端校验失败时用统一位置呈现可操作的字段错误。"), source: error_source,
                <div class="max-w-[560px] p-6">
                    form_field(config: FormFieldConfig::new("gallery-command-name", text(locale, "指令")).with_error(text(locale, "指令只能包含小写字母、数字、连字符和下划线。")),
                        <input id="gallery-command-name" class="h-10 rounded-md border border-[#ff4d4f] px-3 font-mono text-sm outline-none shadow-[0_0_0_2px_rgba(255,77,79,0.08)]" type="text" value="/Test Command" aria-invalid="true" aria-describedby="gallery-command-name-help">
                    )
                </div>
            )
            markdown_document(source: document)
        </div>
    })
}
