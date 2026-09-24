use crate::locale::Locale;
use crate::locale::text;
mod events;
mod permissions;
mod webhook;

use topcoat::{
    Result,
    context::Cx,
    router::{Slot, error::redirect, href, layout, page, request::uri},
    view::{View, view},
};
use topcoat_ant_design::{tab_link, tabs};

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

/// Tabs 示例的共用外壳；当前路由同时决定激活态和预览内容。
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

    Ok(view! {
        page_header(
            eyebrow: "NAVIGATION",
            title: text(locale, "Tabs 路由页签"),
            description: text(locale, "使用真实链接组织同一对象下的多个页面，并由 Topcoat 路由决定当前状态。"),
        )
        <div class="grid gap-6">
            component_example(id: "tabs-preview", title: text(locale, "组件预览"), description: text(locale, "切换页签后，路由、激活态和下方内容会一起更新。"), source: example_source,
                <div class="p-6">
                    tabs(label: text(locale, "项目详情示例"),
                        tab_link(href: webhook_localized_url.as_str(), active: current_path == webhook_url.as_str(), (text(locale, "Webhook 配置")))
                        tab_link(href: events_localized_url.as_str(), active: current_path == events_url.as_str(), (text(locale, "事件记录")))
                        tab_link(href: permissions_localized_url.as_str(), active: current_path == permissions_url.as_str(), (text(locale, "访问权限")))
                    )
                    <div class="min-h-40 rounded-b-lg bg-[#fafafa] px-5 py-7">(slot)</div>
                </div>
            )
            markdown_document(source: document)
        </div>
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
