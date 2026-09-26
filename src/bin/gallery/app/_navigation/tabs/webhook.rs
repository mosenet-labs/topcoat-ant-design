use crate::locale::{Locale, text};
use topcoat::{
    Result,
    router::page,
    view::{View, component, view},
};

#[page]
pub(super) async fn webhook_page() -> Result<impl View> {
    Ok(view! {})
}

#[component]
pub(super) async fn content(locale: Locale) -> Result<impl View> {
    Ok(view! {
        <section aria-labelledby="tabs-webhook-heading">
            <p class="m-0 text-xs font-bold tracking-[0.1em] text-primary">"WEBHOOK"</p>
            <h3 class="mb-2 mt-2 text-base font-semibold" id="tabs-webhook-heading">(text(locale, "Webhook 配置"))</h3>
            <p class="m-0 text-sm leading-6 text-muted-foreground">(text(locale, "设置回调地址和 Secret，并检查最近一次事件是否验证成功。"))</p>
        </section>
    })
}
