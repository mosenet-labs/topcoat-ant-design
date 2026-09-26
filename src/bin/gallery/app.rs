use crate::locale::text;
mod _ai;
mod _data_display;
mod _data_entry;
mod _feedback;
mod _guide;
mod _motion;
mod _native;
mod _navigation;
mod _overview;
pub(in crate::app) use _overview::overview_content;

use topcoat::{
    Result,
    asset::{AssetConfig, RouterBuilderAssetExt},
    context::Cx,
    router::{Router, Slot, href, layout, page, request::uri},
    runtime::{Event, RouterBuilderRuntimeExt, signal},
    view::{View, attributes, class, component, view},
};
use topcoat_ant_design::{
    RouterBuilderUiExt, SidebarCollapsible, SidebarVariant, button, head_assets, sidebar,
    sidebar_content, sidebar_header, sidebar_inset, sidebar_menu_button, sidebar_provider,
    sidebar_trigger,
};

use crate::{assets::GALLERY_STYLESHEET, locale::Locale};

const THEME_COOKIE: &str = "topcoat-ant-theme=dark";

fn theme_is_dark(cx: &Cx) -> bool {
    topcoat::router::request::headers(cx)
        .get("cookie")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|cookies| {
            cookies
                .split(';')
                .any(|cookie| cookie.trim() == THEME_COOKIE)
        })
}

/// 使用 Topcoat 模块路由构造 Gallery。
///
/// `module_router!` 发现模块路由；静态资源、过程函数与 shard 通过 `.route()` 注册。
pub(crate) fn router(app_assets: AssetConfig) -> Router {
    topcoat::router::module_router!()
        .route(crate::assets::component_css)
        .route(crate::assets::gallery_css)
        .route(crate::assets::topcoat_runtime_js)
        .route(crate::assets::native_ui_portrait)
        .page(_native::native_ui_page)
        .route(_ai::chat::flow::gallery_reply)
        .route(_ai::chat::flow::gallery_action)
        .route(_ai::chat::flow::message_region)
        .page(_ai::extras::chat_states_page)
        .page(_ai::extras::chat_markdown_page)
        .page(_ai::extras::chat_think_page)
        .page(_ai::extras::chat_thought_chain_page)
        .page(_ai::extras::chat_sources_page)
        .page(_ai::extras::chat_actions_page)
        .page(_ai::extras::chat_attachments_page)
        .page(_ai::extras::chat_files_page)
        .page(_ai::extras::chat_prompts_page)
        .page(_ai::extras::chat_conversations_page)
        .runtime()
        .topcoat_ant_design()
        .assets(app_assets)
        .build()
}

#[component]
async fn gallery_nav_link(href: &str, badge: &str, label: &str, active: bool) -> Result<impl View> {
    let badge_class = class!(
        "grid size-7 shrink-0 place-items-center rounded-md border text-[11px] font-bold",
        "border-primary bg-primary text-primary-foreground" if active,
        "border-sidebar-border bg-background text-muted-foreground" if !active,
    );

    Ok(view! {
        sidebar_menu_button(href: Some(href), active: active, attrs: attributes! { class="min-h-10 gap-3 px-3 py-2 text-sidebar-foreground" },
            <span class=(badge_class) aria-hidden="true">(badge)</span>
            <span>(label)</span>
        )
    })
}

/// Gallery 所有页面共用的文档标题。
#[component]
pub(in crate::app) async fn page_header(
    eyebrow: &str,
    title: &str,
    description: &str,
) -> Result<impl View> {
    Ok(view! {
        <header class="mb-8 border-b border-border pb-7">
            <p class="m-0 text-xs font-bold tracking-[0.12em] text-primary">(eyebrow)</p>
            <h1 class="mb-3 mt-2 text-[clamp(30px,5vw,42px)] font-bold leading-[1.15] tracking-[-0.025em] text-foreground">(title)</h1>
            <p class="m-0 max-w-[760px] text-[15px] leading-7 text-muted-foreground">(description)</p>
        </header>
    })
}

/// 根布局只声明一次文档外壳，并自动包裹 `app` 模块下的所有页面。
#[layout]
async fn gallery_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let locale = Locale::current(cx);
    let sidebar_open = signal(cx, || true);
    let mobile_open = signal(cx, || false);
    let light_theme_label = locale.select("Light theme", "浅色主题");
    let dark_theme_label = locale.select("Dark theme", "深色主题");
    let english_url = uri(cx).path();
    let chinese_url = format!("{}?lang=zh", uri(cx).path());
    let overview_link = href!(_guide::overview_page);
    let getting_started_link = href!(home);
    let icons_link = href!(_guide::icons_page);
    let native_ui_link = href!(_native::native_ui_page);
    let notification_link = href!(_feedback::notification_page);
    let tag_link = href!(_feedback::tag_page);
    let tag_active = tag_link.is_current(cx);
    let tooltip_link = href!(_feedback::tooltip_page);
    let popconfirm_link = href!(_feedback::popconfirm_page);
    let dialog_link = href!(_feedback::dialog_page);
    let drawer_link = href!(_feedback::drawer_page);
    let collapse_link = href!(_motion::collapse_page);
    let accordion_link = href!(_motion::accordion_page);
    let dropdown_menu_link = href!(_navigation::dropdown_menu_page);
    let tabs_link = href!(_navigation::tabs_page);
    let table_link = href!(_data_display::table_page);
    let form_field_link = href!(_data_entry::form_field_page);
    let date_time_range_link = href!(_data_entry::date_time_range_page);
    let chat_link = href!(_ai::chat_page);
    let bubble_link = href!(_ai::bubble_page);
    let message_list_link = href!(_ai::message_list_page);
    let sender_link = href!(_ai::sender_page);

    let overview_active = overview_link.is_current(cx);
    let getting_started_active =
        getting_started_link.is_current(cx) || href!(_guide::getting_started_page).is_current(cx);
    let icons_active = icons_link.is_current(cx);
    let native_ui_active = native_ui_link.is_current(cx);
    let dark = signal(cx, || theme_is_dark(cx));
    let notification_active = notification_link.is_current(cx);
    let tooltip_active = tooltip_link.is_current(cx);
    let popconfirm_active = popconfirm_link.is_current(cx);
    let dialog_active = dialog_link.is_current(cx);
    let drawer_active = drawer_link.is_current(cx);
    let collapse_active = collapse_link.is_current(cx);
    let accordion_active = accordion_link.is_current(cx);
    let dropdown_menu_active = dropdown_menu_link.is_current(cx);
    let tabs_active = tabs_link.is_current(cx) || uri(cx).path().starts_with("/tabs/");
    let table_active = table_link.is_current(cx);
    let form_field_active = form_field_link.is_current(cx);
    let date_time_range_active = date_time_range_link.is_current(cx);
    let chat_active = chat_link.is_current(cx)
        || href!(_ai::chat_notes_page).is_current(cx)
        || href!(_ai::chat_new_page).is_current(cx);
    let bubble_active = bubble_link.is_current(cx);
    let message_list_active = message_list_link.is_current(cx);
    let sender_active = sender_link.is_current(cx);
    let ai_extra_title = match uri(cx).path() {
        "/chat/states" => Some(locale.select("Message states", "消息状态")),
        "/chat/markdown" => Some("ChatMarkdown"),
        "/chat/think" => Some("ChatThink"),
        "/chat/thought-chain" => Some("ChatThoughtChain"),
        "/chat/sources" => Some("ChatSources"),
        "/chat/actions" => Some("ChatActions"),
        "/chat/attachments" => Some("ChatAttachmentTray"),
        "/chat/files" => Some("ChatFile"),
        "/chat/prompts" => Some("ChatPrompts"),
        "/chat/conversations" => Some("ChatConversationList"),
        _ => None,
    };
    let document_title = if getting_started_active {
        text(locale, "快速开始")
    } else if overview_active {
        text(locale, "组件概览")
    } else if icons_active {
        text(locale, "Icons 图标")
    } else if native_ui_active {
        locale.select("Official Topcoat components", "Topcoat 官方组件")
    } else if chat_active {
        locale.select("Chat interface", "Chat 聊天界面")
    } else if bubble_active {
        "ChatBubble"
    } else if message_list_active {
        "ChatMessageList"
    } else if sender_active {
        "ChatSender"
    } else if let Some(title) = ai_extra_title {
        title
    } else if notification_active {
        text(locale, "Notification 通知提醒框")
    } else if tag_active {
        text(locale, "Tag 标签")
    } else if tooltip_active {
        text(locale, "Tooltip 文字提示")
    } else if popconfirm_active {
        text(locale, "Popconfirm 气泡确认框")
    } else if dialog_active {
        text(locale, "Dialog 模态对话框")
    } else if drawer_active {
        text(locale, "Drawer 抽屉")
    } else if collapse_active {
        text(locale, "Collapse 折叠动画")
    } else if accordion_active {
        text(locale, "Accordion 手风琴")
    } else if dropdown_menu_active {
        locale.select("Dropdown Menu", "Dropdown 下拉菜单")
    } else if tabs_active {
        text(locale, "Tabs 路由页签")
    } else if table_active {
        text(locale, "Table 数据表格")
    } else if form_field_active {
        text(locale, "FormField 表单字段")
    } else if date_time_range_active {
        text(locale, "DateTimeRange 时间范围")
    } else {
        text(locale, "组件概览")
    };

    let overview_url = locale.link(&overview_link.resolve(cx));
    let getting_started_url = locale.link(&getting_started_link.resolve(cx));
    let icons_url = locale.link(&icons_link.resolve(cx));
    let native_ui_url = locale.link(&native_ui_link.resolve(cx));
    let notification_url = locale.link(&notification_link.resolve(cx));
    let tag_url = locale.link(&tag_link.resolve(cx));
    let tooltip_url = locale.link(&tooltip_link.resolve(cx));
    let popconfirm_url = locale.link(&popconfirm_link.resolve(cx));
    let dialog_url = locale.link(&dialog_link.resolve(cx));
    let drawer_url = locale.link(&drawer_link.resolve(cx));
    let collapse_url = locale.link(&collapse_link.resolve(cx));
    let accordion_url = locale.link(&accordion_link.resolve(cx));
    let dropdown_menu_url = locale.link(&dropdown_menu_link.resolve(cx));
    let tabs_url = locale.link(&tabs_link.resolve(cx));
    let table_url = locale.link(&table_link.resolve(cx));
    let form_field_url = locale.link(&form_field_link.resolve(cx));
    let date_time_range_url = locale.link(&date_time_range_link.resolve(cx));
    let chat_url = locale.link(&chat_link.resolve(cx));
    let bubble_url = locale.link(&bubble_link.resolve(cx));
    let message_list_url = locale.link(&message_list_link.resolve(cx));
    let sender_url = locale.link(&sender_link.resolve(cx));
    let ai_extra_links = [
        (
            "/chat/states",
            "St",
            locale.select("Message states", "消息状态"),
        ),
        ("/chat/markdown", "Md", "ChatMarkdown"),
        ("/chat/think", "Th", "ChatThink"),
        ("/chat/thought-chain", "Tc", "ChatThoughtChain"),
        ("/chat/sources", "So", "ChatSources"),
        ("/chat/actions", "Ac", "ChatActions"),
        ("/chat/attachments", "At", "ChatAttachmentTray"),
        ("/chat/files", "Fi", "ChatFile"),
        ("/chat/prompts", "Pr", "ChatPrompts"),
        ("/chat/conversations", "Co", "ChatConversationList"),
    ]
    .map(|(path, badge, label)| (locale.link(path), badge, label, uri(cx).path() == path));
    let ai_extra_mobile_links = ai_extra_links.clone();

    Ok(view! {
        <!DOCTYPE html>
        <html lang=(locale.html_lang()) :class=$(if dark.get() { "dark" } else { "" })>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <meta name="color-scheme" content="light dark">
                <title>(document_title) " · Topcoat Ant Design"</title>
                head_assets()
                <link rel="stylesheet" href=(GALLERY_STYLESHEET)>
                topcoat::runtime::script()
                topcoat::dev::script(status_indicator: false)
            </head>
            <body class="m-0 min-w-80 bg-background font-mono text-foreground antialiased">
                sidebar_provider(attrs: attributes! { class="gallery-shell" },
                        sidebar(
                            open: $(sidebar_open.get()),
                            mobile_open: $(mobile_open.get()),
                            variant: SidebarVariant::Sidebar,
                            collapsible: SidebarCollapsible::Offcanvas,
                            sheet_attrs: attributes! { id="gallery-sidebar" aria-label=(text(locale, "组件导航")) },
                            sidebar_header(attrs: attributes! { class="[&]:h-auto gap-4 px-5 py-5" },
                                <a class="flex items-center gap-3 text-sidebar-foreground no-underline" href=(getting_started_url.as_str())>
                                    <span class="grid size-9 place-items-center rounded-lg bg-primary text-sm font-bold text-primary-foreground shadow-sm">"AD"</span>
                                    <span><strong class="block text-[15px] font-semibold leading-5">"Topcoat Ant Design"</strong><small class="mt-0.5 block text-xs text-muted-foreground">(locale.select("Topcoat components", "Topcoat 组件"))</small></span>
                                </a>
                                <nav class="flex gap-2" aria-label=(text(locale, "语言"))>
                                    <a class="rounded-md border border-border px-3 py-1 text-xs text-sidebar-foreground no-underline aria-[current=page]:border-primary aria-[current=page]:bg-sidebar-accent aria-[current=page]:text-sidebar-accent-foreground" href=(english_url) data-language-switch="" aria-current=(if locale == Locale::En { Some("page") } else { None })>"English"</a>
                                    <a class="rounded-md border border-border px-3 py-1 text-xs text-sidebar-foreground no-underline aria-[current=page]:border-primary aria-[current=page]:bg-sidebar-accent aria-[current=page]:text-sidebar-accent-foreground" href=(chinese_url.as_str()) data-language-switch="" aria-current=(if locale == Locale::Zh { Some("page") } else { None })>"中文"</a>
                                </nav>
                                button(attrs: attributes! { type="button" class="md:hidden" @click=$(|_e: Event| mobile_open.set(false)) }, (locale.select("Close navigation", "关闭导航")))
                            )
                            sidebar_content(
                                <nav class="grid gap-6 px-4 py-5" aria-label=(text(locale, "组件导航"))>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-muted-foreground">(text(locale, "开始"))</p>
                                <div class="grid gap-1">
                                    gallery_nav_link(href: getting_started_url.as_str(), badge: "→", label: text(locale, "快速开始"), active: getting_started_active)
                                    gallery_nav_link(href: overview_url.as_str(), badge: "01", label: text(locale, "组件概览"), active: overview_active)
                                    gallery_nav_link(href: icons_url.as_str(), badge: "I", label: text(locale, "Icons 图标"), active: icons_active)
                                    gallery_nav_link(href: native_ui_url.as_str(), badge: "UI", label: locale.select("Official Topcoat components", "Topcoat 官方组件"), active: native_ui_active)
                                </div>
                            </section>
                            <section>
                                <details class="rounded-lg border border-border bg-card open:shadow-sm" open=(native_ui_active)>
                                    <summary class="cursor-pointer px-3 py-3 text-[11px] font-bold tracking-[0.1em] text-muted-foreground">(locale.select("Official UI · 31 components", "官方 UI · 31 个组件"))</summary>
                                    <div class="grid grid-cols-2 gap-1 border-t border-border p-2">
                                        for name in _native::REGISTRY_COMPONENTS {
                                            let target = format!("{}#{}", native_ui_url, _native::registry_target(name));
                                            <a class="truncate rounded px-2 py-1.5 font-mono text-[11px] text-sidebar-foreground no-underline hover:bg-sidebar-accent hover:text-primary" href=(target.as_str())>(name)</a>
                                        }
                                    </div>
                                </details>
                            </section>
                            <section class="max-[899px]:hidden">
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-muted-foreground">(locale.select("AI Components", "AI 组件"))</p>
                                <div class="grid gap-1">
                                    gallery_nav_link(href: chat_url.as_str(), badge: "AI", label: locale.select("Chat interface", "Chat 聊天界面"), active: chat_active)
                                    gallery_nav_link(href: bubble_url.as_str(), badge: "B", label: "ChatBubble", active: bubble_active)
                                    gallery_nav_link(href: message_list_url.as_str(), badge: "L", label: "ChatMessageList", active: message_list_active)
                                    gallery_nav_link(href: sender_url.as_str(), badge: "S", label: "ChatSender", active: sender_active)
                                    for (url, badge, label, active) in ai_extra_links {
                                        gallery_nav_link(href: url.as_str(), badge: badge, label: label, active: active)
                                    }
                                </div>
                            </section>
                            <section class="min-[900px]:hidden">
                                <details class="rounded-lg border border-border bg-card open:shadow-sm">
                                    <summary class="cursor-pointer px-3 py-3 text-[11px] font-bold uppercase tracking-[0.1em] text-muted-foreground">(locale.select("AI Components", "AI 组件"))</summary>
                                    <div class="grid gap-1 border-t border-border p-2">
                                        gallery_nav_link(href: chat_url.as_str(), badge: "AI", label: locale.select("Chat interface", "Chat 聊天界面"), active: chat_active)
                                        gallery_nav_link(href: bubble_url.as_str(), badge: "B", label: "ChatBubble", active: bubble_active)
                                        gallery_nav_link(href: message_list_url.as_str(), badge: "L", label: "ChatMessageList", active: message_list_active)
                                        gallery_nav_link(href: sender_url.as_str(), badge: "S", label: "ChatSender", active: sender_active)
                                        for (url, badge, label, active) in ai_extra_mobile_links {
                                            gallery_nav_link(href: url.as_str(), badge: badge, label: label, active: active)
                                        }
                                    </div>
                                </details>
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-muted-foreground">(text(locale, "反馈"))</p>
                                <div class="grid gap-1">
                                    gallery_nav_link(href: notification_url.as_str(), badge: "N", label: "Notification", active: notification_active)
                                    gallery_nav_link(href: tag_url.as_str(), badge: "Ta", label: text(locale, "Tag 标签"), active: tag_active)
                                    gallery_nav_link(href: tooltip_url.as_str(), badge: "T", label: "Tooltip", active: tooltip_active)
                                    gallery_nav_link(href: popconfirm_url.as_str(), badge: "P", label: "Popconfirm", active: popconfirm_active)
                                    gallery_nav_link(href: dialog_url.as_str(), badge: "Di", label: "Dialog", active: dialog_active)
                                    gallery_nav_link(href: drawer_url.as_str(), badge: "D", label: "Drawer", active: drawer_active)
                                </div>
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-muted-foreground">(text(locale, "导航"))</p>
                                gallery_nav_link(href: dropdown_menu_url.as_str(), badge: "Dd", label: locale.select("Dropdown Menu", "Dropdown 下拉菜单"), active: dropdown_menu_active)
                                gallery_nav_link(href: tabs_url.as_str(), badge: "T", label: "Tabs", active: tabs_active)
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-muted-foreground">(text(locale, "数据录入"))</p>
                                gallery_nav_link(href: form_field_url.as_str(), badge: "F", label: "FormField", active: form_field_active)
                                gallery_nav_link(href: date_time_range_url.as_str(), badge: "R", label: "DateTimeRange", active: date_time_range_active)
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-muted-foreground">(text(locale, "数据展示"))</p>
                                gallery_nav_link(href: table_url.as_str(), badge: "Tb", label: "Table", active: table_active)
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-muted-foreground">(text(locale, "动效"))</p>
                                gallery_nav_link(href: collapse_url.as_str(), badge: "C", label: "Collapse", active: collapse_active)
                                gallery_nav_link(href: accordion_url.as_str(), badge: "A", label: "Accordion", active: accordion_active)
                            </section>
                                </nav>
                            )
                        )
                        sidebar_inset(
                            sidebar_header(attrs: attributes! { class="[&]:h-16 [&]:bg-card px-6" },
                                sidebar_trigger(open: $(sidebar_open.get()), attrs: attributes! { class="max-md:hidden" aria-label=(locale.select("Toggle sidebar", "切换侧边栏")) aria-controls="gallery-sidebar" @click=$(|_e: Event| sidebar_open.toggle()) })
                                sidebar_trigger(open: $(mobile_open.get()), attrs: attributes! { class="md:hidden" aria-label=(locale.select("Toggle sidebar", "切换侧边栏")) aria-controls="gallery-sidebar" @click=$(|_e: Event| mobile_open.toggle()) })
                                <span class="h-5 w-px bg-border" aria-hidden="true"></span>
                                <span class="min-w-0 flex-1 truncate text-sm font-semibold text-foreground">(document_title)</span>
                                button(attrs: attributes! { type="button" class="shrink-0" @click=$(|_e: Event| {
                                    let next = !dark.get();
                                    dark.set(next);
                                    if next {
                                        raw!("document.cookie = 'topcoat-ant-theme=dark; Path=/; Max-Age=31536000; SameSite=Lax'", ());
                                    } else {
                                        raw!("document.cookie = 'topcoat-ant-theme=light; Path=/; Max-Age=31536000; SameSite=Lax'", ());
                                    }
                                }) },
                                    $(if dark.get() { light_theme_label } else { dark_theme_label })
                                )
                            )
                            <div class="mx-auto box-border w-full min-w-0 max-w-[1240px] px-8 pb-24 pt-12 max-[640px]:px-4 max-[640px]:pt-8">(slot)</div>
                        )
                    )
            </body>
        </html>
    })
}

/// `app.rs` 是模块路由根，因此无路径的首页自然映射到 `/`。
#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let _ = cx;
    Ok(view! { _guide::getting_started_content() })
}

#[cfg(test)]
mod tests {
    use topcoat::router::{Body, Route, StatusCode, request::Request, to_bytes};

    use super::router;

    #[tokio::test]
    async fn native_ui_gallery_renders_registry_components_and_theme() {
        let router = router(crate::assets::config().expect("Gallery assets should be valid"));
        let response = router
            .handle(
                Request::builder()
                    .uri("/topcoat-ui")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let html = String::from_utf8(body.to_vec()).unwrap();
        for marker in [
            "Topcoat Ant Design",
            "Official Topcoat components",
            "Component index",
            "Topcoat 0.9.0 · 31 / 31",
            "Show example code",
            "async fn buttons_card",
            "async fn sidebar_card",
        ] {
            assert!(html.contains(marker), "missing {marker}");
        }
        for name in super::_native::REGISTRY_COMPONENTS {
            let target = super::_native::registry_target(name);
            assert!(
                html.contains(&format!("href=\"/topcoat-ui#{target}\"")),
                "missing link for {name}"
            );
            assert!(
                html.contains(&format!("id=\"{target}\"")),
                "missing example for {name}"
            );
        }

        let response = router
            .handle(
                Request::builder()
                    .uri("/topcoat-ui?lang=zh")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await;
        let chinese = String::from_utf8(
            to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(chinese.contains("Topcoat 官方组件"));
        assert!(chinese.contains("组件索引"));
        assert!(chinese.contains("显示示例代码"));
    }

    #[tokio::test]
    async fn selected_theme_is_restored_on_every_gallery_route() {
        let router = router(crate::assets::config().expect("Gallery assets should be valid"));

        for path in ["/overview", "/notification", "/topcoat-ui"] {
            let response = router
                .handle(
                    Request::builder()
                        .uri(path)
                        .header("cookie", "session=demo; topcoat-ant-theme=dark")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await;
            assert_eq!(response.status(), StatusCode::OK, "{path}");
            let html = String::from_utf8(
                to_bytes(response.into_body(), usize::MAX)
                    .await
                    .unwrap()
                    .to_vec(),
            )
            .unwrap();
            assert!(html.contains("class=\"dark\""), "{path}");
        }
    }

    #[tokio::test]
    async fn chat_endpoints_have_stable_paths_and_reply_is_served() {
        use crate::app::_ai::chat::flow::{gallery_action, gallery_reply, message_region};

        assert_eq!(
            gallery_reply.path().to_matchit_path(),
            "/_gallery/chat/reply"
        );
        assert_eq!(
            gallery_action.path().to_matchit_path(),
            "/_gallery/chat/action"
        );
        assert_eq!(
            message_region.path().to_matchit_path(),
            "/_gallery/chat/messages"
        );

        let router = router(crate::assets::config().expect("Gallery assets should be valid"));
        let response = router
            .handle(
                Request::builder()
                    .method("POST")
                    .uri("/_gallery/chat/reply")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"["Hello","en"]"#))
                    .unwrap(),
            )
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert!(
            String::from_utf8_lossy(&body)
                .contains("The demo service received your question: Hello")
        );
    }

    #[tokio::test]
    async fn accordion_gallery_shows_the_component_and_shared_usage_document() {
        let router = router(crate::assets::config().expect("Gallery assets should be valid"));
        let response = router
            .handle(
                Request::builder()
                    .uri("/accordion")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let html = String::from_utf8(body.to_vec()).unwrap();
        assert!(html.contains("Accordion"));
        assert!(html.contains("<details"));
        assert!(html.contains("Show example code"));
        assert!(html.contains("name=\"gallery-accordion\""));
    }

    #[tokio::test]
    async fn english_is_default_and_chinese_can_be_selected() {
        let router = router(crate::assets::config().expect("Gallery assets should be valid"));
        for (uri, language, heading, document_text) in [
            ("/", "en", "Quick start", "Add the Cargo dependency"),
            ("/?lang=zh", "zh-CN", "快速开始", "添加 Cargo 依赖"),
            ("/overview?lang=zh", "zh-CN", "组件概览", "先完成五步接入"),
            (
                "/dropdown-menu?lang=zh",
                "zh-CN",
                "Dropdown 下拉菜单",
                "暂不可用",
            ),
            (
                "/chat/new?lang=zh",
                "zh-CN",
                "Chat 聊天界面",
                "今天想探索什么？",
            ),
        ] {
            let response = router
                .handle(Request::builder().uri(uri).body(Body::empty()).unwrap())
                .await;
            assert_eq!(response.status(), StatusCode::OK, "{uri}");
            let html = String::from_utf8(
                to_bytes(response.into_body(), usize::MAX)
                    .await
                    .unwrap()
                    .to_vec(),
            )
            .unwrap();
            assert!(
                html.contains(&format!("<html lang=\"{language}\"")),
                "{uri}"
            );
            assert!(html.contains(heading), "{uri}");
            assert!(html.contains(document_text), "{uri}");
            assert!(html.contains("data-language-switch"), "{uri}");
            if language == "zh-CN" {
                assert!(html.contains("href=\"/notification?lang=zh\""), "{uri}");
                assert!(html.contains("href=\"/tag?lang=zh\""), "{uri}");
                assert!(html.contains("href=\"/dropdown-menu?lang=zh\""), "{uri}");
            } else {
                assert!(html.contains("href=\"/notification\""), "{uri}");
                assert!(html.contains("href=\"/tag\""), "{uri}");
                assert!(html.contains("href=\"/dropdown-menu\""), "{uri}");
            }
        }

        for (uri, label) in [
            ("/date-time-range", "Last 24 hours"),
            ("/date-time-range?lang=zh", "最近 24 小时"),
        ] {
            let response = router
                .handle(Request::builder().uri(uri).body(Body::empty()).unwrap())
                .await;
            let html = String::from_utf8(
                to_bytes(response.into_body(), usize::MAX)
                    .await
                    .unwrap()
                    .to_vec(),
            )
            .unwrap();
            assert!(html.contains(label), "{uri}");
        }

        let redirect = router
            .handle(
                Request::builder()
                    .uri("/tabs?lang=zh")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await;
        assert_eq!(redirect.status(), StatusCode::TEMPORARY_REDIRECT);
        assert_eq!(
            redirect.headers().get("location").unwrap(),
            "/tabs/webhook?lang=zh"
        );
    }

    #[tokio::test]
    async fn module_tree_serves_every_gallery_page_through_the_root_layout() {
        let router = router(crate::assets::config().expect("Gallery assets should be valid"));

        for (path, expected_heading, tab_content_id) in [
            ("/", "Quick start", None),
            ("/getting-started", "Quick start", None),
            ("/overview", "Component overview", None),
            ("/icons", "Icons", None),
            ("/chat", "Chat interface", None),
            ("/chat/notes", "Chat interface", None),
            ("/chat/new", "Chat interface", None),
            ("/bubble", "ChatBubble", None),
            ("/message-list", "ChatMessageList", None),
            ("/sender", "ChatSender", None),
            ("/chat/states", "Message states", None),
            ("/chat/markdown", "ChatMarkdown", None),
            ("/chat/think", "ChatThink", None),
            ("/chat/thought-chain", "ChatThoughtChain", None),
            ("/chat/sources", "ChatSources", None),
            ("/chat/actions", "ChatActions", None),
            ("/chat/attachments", "ChatAttachmentTray", None),
            ("/chat/files", "ChatFile", None),
            ("/chat/prompts", "ChatPrompts", None),
            ("/chat/conversations", "ChatConversationList", None),
            ("/notification", "Notification", None),
            ("/tag", "Tag", None),
            ("/tooltip", "Tooltip", None),
            ("/popconfirm", "Popconfirm", None),
            ("/dropdown-menu", "Dropdown Menu", None),
            ("/dialog", "Dialog", None),
            ("/drawer", "Drawer", None),
            ("/table", "Table", None),
            ("/form-field", "FormField", None),
            ("/date-time-range", "DateTimeRange", None),
            ("/collapse", "Collapse", None),
            ("/accordion", "Accordion", None),
            (
                "/tabs/webhook",
                "Webhook configuration",
                Some("tabs-webhook-heading"),
            ),
            ("/tabs/events", "Event history", Some("tabs-events-heading")),
            (
                "/tabs/permissions",
                "Access permissions",
                Some("tabs-permissions-heading"),
            ),
        ] {
            let response = router
                .handle(Request::builder().uri(path).body(Body::empty()).unwrap())
                .await;

            assert_eq!(response.status(), StatusCode::OK, "{path}");
            let body = to_bytes(response.into_body(), usize::MAX)
                .await
                .expect("Gallery response should be readable");
            let html = String::from_utf8(body.to_vec()).expect("Gallery should return UTF-8 HTML");
            assert!(html.contains(expected_heading), "{path}: {html}");
            if path == "/" {
                assert!(html.contains("<title>Quick start · Topcoat Ant Design</title>"));
                assert!(html.contains("Add the Cargo dependency"));
                assert!(html.contains("href=\"/overview\""));
            }
            if path == "/overview" {
                assert!(html.contains("<title>Component overview · Topcoat Ant Design</title>"));
                assert!(html.contains("Complete the five integration steps"));
                assert!(html.contains("href=\"/\""));
                assert!(html.contains("href=\"/chat\""));
                assert!(html.contains("href=\"/bubble\""));
                assert!(html.contains("href=\"/message-list\""));
                assert!(html.contains("href=\"/sender\""));
                assert!(html.contains("href=\"/chat/markdown\""));
                assert!(html.contains("href=\"/chat/conversations\""));
                assert!(html.contains("href=\"/dropdown-menu\""));
            }
            if path == "/dropdown-menu" {
                assert!(html.contains("<details"));
                assert!(html.contains("<summary"));
            }
            if matches!(path, "/chat" | "/chat/notes" | "/chat/new") {
                assert!(html.contains("<title>Chat interface · Topcoat Ant Design</title>"));
                assert!(html.contains("gr-chat-sender"));
                assert!(html.contains("AI Components"));
            }
            if matches!(path, "/chat" | "/chat/notes") {
                assert!(html.contains("gr-chat-bubble"));
            }
            if path == "/chat/new" {
                assert!(html.contains("gr-chat-prompts"));
            }
            if path == "/bubble" {
                assert!(html.contains("gr-chat-bubble-user"));
                assert!(html.contains("data-role=\"assistant\""));
            }
            if path == "/message-list" {
                assert!(html.contains("gr-chat-message-list"));
                assert!(html.contains("role=\"log\""));
            }
            if path == "/sender" {
                assert!(html.contains("gr-chat-sender"));
                assert!(html.contains("gallery-sender-draft"));
            }
            assert!(
                !html
                    .replace("中文", "")
                    .chars()
                    .any(|character| ('\u{4e00}'..='\u{9fff}').contains(&character)),
                "{path}: unexpected Chinese copy on the English page"
            );
            assert!(
                html.contains("Topcoat Ant Design"),
                "{path}: root layout missing"
            );
            if path == "/drawer" {
                assert!(html.contains(
                    "class=\"min-h-0 flex-1 overflow-y-auto px-6 py-5\"><div class=\"grid gap-5\""
                ));
                assert!(html.contains("data-topcoat-bind:inert"));
            }
            if path == "/table" {
                assert!(html.contains("gr-data-table-compact"));
                assert!(html.contains("aria-label=\"Example pagination\""));
                assert!(html.contains("aria-current=\"page\""));
            }
            if matches!(
                path,
                "/notification"
                    | "/chat"
                    | "/chat/notes"
                    | "/chat/new"
                    | "/bubble"
                    | "/message-list"
                    | "/sender"
                    | "/chat/states"
                    | "/chat/markdown"
                    | "/chat/think"
                    | "/chat/thought-chain"
                    | "/chat/sources"
                    | "/chat/actions"
                    | "/chat/attachments"
                    | "/chat/files"
                    | "/chat/prompts"
                    | "/chat/conversations"
                    | "/tag"
                    | "/tooltip"
                    | "/popconfirm"
                    | "/dropdown-menu"
                    | "/dialog"
                    | "/drawer"
                    | "/collapse"
                    | "/accordion"
                    | "/table"
                    | "/form-field"
                    | "/date-time-range"
                    | "/tabs/webhook"
                    | "/tabs/events"
                    | "/tabs/permissions"
            ) {
                assert!(html.contains("gr-gallery-demo"), "{path}: {html}");
                assert!(html.contains("Show example code"), "{path}: {html}");
                assert!(
                    !html.contains("No Rust example is available in this document"),
                    "{path}: {html}"
                );
            }
            if let Some(content_id) = tab_content_id {
                let tabs = html.find("aria-label=\"Project details example\"").unwrap();
                let tabs_end = tabs + html[tabs..].find("</div>").unwrap();
                let tabs_html = &html[tabs..tabs_end];
                assert!(tabs_html.contains("href=\"/tabs/webhook\""));
                assert!(
                    html.contains(&format!("id=\"{content_id}\"")),
                    "{path}: {html}"
                );

                let active_href = tabs_html.find(&format!("href=\"{path}\"")).unwrap();
                let active_link_start = tabs_html[..active_href].rfind("<a").expect("tab start");
                let active_link_end =
                    active_href + tabs_html[active_href..].find("</a>").expect("tab end");
                assert!(
                    tabs_html[active_link_start..active_link_end].contains("aria-current=\"page\""),
                    "{path}: current tab missing from {}",
                    &tabs_html[active_link_start..active_link_end]
                );
            }
        }

        let tabs_root = router
            .handle(Request::builder().uri("/tabs").body(Body::empty()).unwrap())
            .await;
        assert_eq!(tabs_root.status(), StatusCode::TEMPORARY_REDIRECT);
        assert_eq!(
            tabs_root.headers().get("location").unwrap(),
            "/tabs/webhook"
        );
    }
}
