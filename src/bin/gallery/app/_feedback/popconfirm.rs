use crate::locale::Locale;
use crate::locale::text;
use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::{Event, signal},
    view::{View, view},
};
use topcoat_ant_design::{
    NotificationTone, notification, popconfirm, popconfirm_trigger_attributes,
};

use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const POPCONFIRM_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/popconfirm.md"
));
const POPCONFIRM_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/popconfirm.md"
));

#[page]
pub(in crate::app) async fn popconfirm_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(POPCONFIRM_DOC_EN, POPCONFIRM_DOC_ZH);
    let success = signal(cx, String::new);
    let warning = signal(cx, String::new);
    let normal_trigger = popconfirm_trigger_attributes(cx, "gallery-standard-confirm");
    let danger_trigger = popconfirm_trigger_attributes(cx, "gallery-danger-confirm");
    let edge_trigger = popconfirm_trigger_attributes(cx, "gallery-edge-confirm");
    let example_source = rust_code_block(document, 0);
    let enabled_message = text(locale, "示例实例已启用。");
    let disabled_message = text(locale, "示例实例已停用。");
    let edge_message = text(locale, "边缘定位验证完成。");

    Ok(view! {
        page_header(
            eyebrow: "CONFIRMATION",
            title: text(locale, "Popconfirm 气泡确认框"),
            description: text(locale, "用于上下文明确的轻量二次确认。下方说明与 popconfirm 方法的 Rustdoc 来自同一份 Markdown。"),
        )
        <div class="grid gap-6">
            component_example(id: "popconfirm-preview", title: text(locale, "组件预览"), description: text(locale, "确认后气泡立即恢复隐藏状态，业务回调继续执行。"), source: example_source,
                <div class="grid grid-cols-2 gap-4 p-6 max-[680px]:grid-cols-1">
                    <article class="rounded-lg border border-border bg-background p-5"><h3 class="m-0 text-base font-semibold">(text(locale, "普通操作"))</h3><p class="mb-5 mt-2 min-h-11 text-sm leading-6 text-muted-foreground">(text(locale, "适用于启用、重新验证等可恢复操作。"))</p><button class="cursor-pointer border-0 bg-transparent p-0 font-mono text-sm text-primary hover:underline" type="button" (normal_trigger)>(text(locale, "启用实例"))</button>
                        popconfirm(language: locale.ui(), id: "gallery-standard-confirm", title: text(locale, "确认启用此实例？"), description: Some(text(locale, "启用前仍会验证当前 Token。")), <button class="gr-button gr-button-primary" type="button" @click=$(|_e: Event| { warning.set("".to_owned()); success.set(enabled_message.to_owned()); })>(text(locale, "确认"))</button>)
                    </article>
                    <article class="rounded-lg border border-border bg-background p-5"><h3 class="m-0 text-base font-semibold">(text(locale, "危险操作"))</h3><p class="mb-5 mt-2 min-h-11 text-sm leading-6 text-muted-foreground">(text(locale, "危险样式由调用方的确认按钮决定。"))</p><button class="cursor-pointer border-0 bg-transparent p-0 font-mono text-sm text-[var(--gr-error)] hover:underline" type="button" (danger_trigger)>(text(locale, "停用实例"))</button>
                        popconfirm(language: locale.ui(), id: "gallery-danger-confirm", title: text(locale, "确认停用此实例？"), description: Some(text(locale, "停用后将不再处理该实例的 Webhook。")), <button class="gr-button gr-button-danger" type="button" @click=$(|_e: Event| { success.set("".to_owned()); warning.set(disabled_message.to_owned()); })>(text(locale, "停用"))</button>)
                    </article>
                    <article class="relative col-span-2 rounded-lg border border-border bg-background p-5 max-[680px]:col-span-1"><div class="flex items-end justify-between gap-8"><div><h3 class="m-0 text-base font-semibold">(text(locale, "边缘偏移"))</h3><p class="mb-0 mt-2 text-sm leading-6 text-muted-foreground">(text(locale, "气泡靠近视口边缘时保持可见，箭头继续指向触发按钮。"))</p></div><button class="shrink-0 cursor-pointer border-0 bg-transparent p-0 font-mono text-sm text-primary hover:underline" type="button" (edge_trigger)>(text(locale, "边缘操作"))</button></div>
                        popconfirm(language: locale.ui(), id: "gallery-edge-confirm", title: text(locale, "确认执行边缘操作？"), description: Some(text(locale, "缩窄窗口或滚动页面可以观察偏移与翻转。")), <button class="gr-button gr-button-primary" type="button" @click=$(|_e: Event| { warning.set("".to_owned()); success.set(edge_message.to_owned()); })>(text(locale, "确认"))</button>)
                    </article>
                </div>
            )
            markdown_document(source: document)
            notification(language: locale.ui(), message: &success, title: text(locale, "操作成功"), tone: NotificationTone::Success)
            notification(language: locale.ui(), message: &warning, title: text(locale, "请注意"), tone: NotificationTone::Warning)
        </div>
    })
}
