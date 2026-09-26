use crate::locale::{Locale, text};
use topcoat::{
    Result,
    router::page,
    view::{View, component, view},
};

#[page]
pub(super) async fn permissions_page() -> Result<impl View> {
    Ok(view! {})
}

#[component]
pub(super) async fn content(locale: Locale) -> Result<impl View> {
    Ok(view! {
        <section aria-labelledby="tabs-permissions-heading">
            <p class="m-0 text-xs font-bold tracking-[0.1em] text-primary">"ACCESS"</p>
            <h3 class="mb-2 mt-2 text-base font-semibold" id="tabs-permissions-heading">(text(locale, "访问权限"))</h3>
            <p class="m-0 text-sm leading-6 text-muted-foreground">(text(locale, "确认当前用户是否有权查看项目配置和事件请求正文。"))</p>
        </section>
    })
}
