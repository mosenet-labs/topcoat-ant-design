use crate::locale::{Locale, text};
use topcoat::{
    Result,
    router::page,
    view::{View, component, view},
};

#[page]
pub(super) async fn events_page() -> Result<impl View> {
    Ok(view! {})
}

#[component]
pub(super) async fn content(locale: Locale) -> Result<impl View> {
    Ok(view! {
        <section aria-labelledby="tabs-events-heading">
            <p class="m-0 text-xs font-bold tracking-[0.1em] text-primary">"DELIVERIES"</p>
            <h3 class="mb-2 mt-2 text-base font-semibold" id="tabs-events-heading">(text(locale, "事件记录"))</h3>
            <p class="m-0 text-sm leading-6 text-muted-foreground">(text(locale, "查看该项目收到的 Webhook 事件、支持状态和处理结果。"))</p>
        </section>
    })
}
