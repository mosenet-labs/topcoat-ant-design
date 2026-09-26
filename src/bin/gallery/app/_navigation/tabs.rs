use crate::locale::Locale;
use crate::locale::text;
mod events;
mod permissions;
mod webhook;

use topcoat::{
    Result,
    context::Cx,
    router::{Slot, error::redirect, href, layout, page, request::uri},
    runtime::{Event, signal},
    view::{View, attributes, view},
};
use topcoat_ant_design::{tabs, tabs_content, tabs_list, tabs_trigger};

use crate::{
    app::page_header,
    demo::component_example,
    markdown::{markdown_document, rust_code_block},
};

const TABS_DOC_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/tabs.md"
));
const TABS_DOC_ZH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/tabs.md"
));

/// Tabs 示例的共用外壳；路由决定初始页签，浏览器状态负责后续切换。
#[layout]
async fn tabs_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(TABS_DOC_EN, TABS_DOC_ZH);
    let current_path = uri(cx).path();
    let webhook_link = href!(webhook::webhook_page);
    let events_link = href!(events::events_page);
    let permissions_link = href!(permissions::permissions_page);
    let webhook_url = webhook_link.resolve(cx);
    let events_url = events_link.resolve(cx);
    let permissions_url = permissions_link.resolve(cx);
    let webhook_localized_url = locale.link(&webhook_url);
    let events_localized_url = locale.link(&events_url);
    let permissions_localized_url = locale.link(&permissions_url);
    let example_source = rust_code_block(document, 0);
    let initial_tab = if current_path == events_url.as_str() {
        "events"
    } else if current_path == permissions_url.as_str() {
        "permissions"
    } else {
        "webhook"
    };
    let selected = signal(cx, || initial_tab.to_owned());

    Ok(view! {
        page_header(
            eyebrow: "NAVIGATION",
            title: text(locale, "Tabs 路由页签"),
            description: text(locale, "页签在浏览器中即时切换；直接打开某个页签地址时，会显示对应的初始内容。"),
        )
        <div class="grid gap-6">
            component_example(id: "tabs-preview", title: text(locale, "组件预览"), description: text(locale, "点击页签即可切换内容，无需刷新页面。"), source: example_source,
                <div class="p-6">
                    tabs(attrs: attributes! { aria-label=(text(locale, "项目详情示例")) },
                        tabs_list(
                            tabs_trigger(active: $(selected.get() == "webhook"), attrs: attributes! {
                                href=(webhook_localized_url.as_str())
                                @click=$(|e: Event| { e.prevent_default(); selected.set("webhook".to_owned()); })
                            }, (text(locale, "Webhook 配置")))
                            tabs_trigger(active: $(selected.get() == "events"), attrs: attributes! {
                                href=(events_localized_url.as_str())
                                @click=$(|e: Event| { e.prevent_default(); selected.set("events".to_owned()); })
                            }, (text(locale, "事件记录")))
                            tabs_trigger(active: $(selected.get() == "permissions"), attrs: attributes! {
                                href=(permissions_localized_url.as_str())
                                @click=$(|e: Event| { e.prevent_default(); selected.set("permissions".to_owned()); })
                            }, (text(locale, "访问权限")))
                        )
                        tabs_content(attrs: attributes! { class="min-h-40 rounded-lg bg-background px-5 py-7" :hidden=$(selected.get() != "webhook") }, webhook::content(locale: locale))
                        tabs_content(attrs: attributes! { class="min-h-40 rounded-lg bg-background px-5 py-7" :hidden=$(selected.get() != "events") }, events::content(locale: locale))
                        tabs_content(attrs: attributes! { class="min-h-40 rounded-lg bg-background px-5 py-7" :hidden=$(selected.get() != "permissions") }, permissions::content(locale: locale))
                    )
                </div>
            )
            markdown_document(source: document)
        </div>
        (slot)
    })
}

/// `/tabs` 保留为组件入口，并跳转到第一个真实页签。
#[page]
pub(in crate::app) async fn tabs_page(cx: &Cx) -> Result<()> {
    let target = if Locale::current(cx) == Locale::Zh {
        "/tabs/webhook?lang=zh"
    } else {
        "/tabs/webhook"
    };
    Err(redirect(target).into())
}
