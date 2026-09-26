use crate::locale::text;
mod _ai;
mod _data_display;
mod _data_entry;
mod _feedback;
mod _guide;
mod _motion;
mod _native;
mod _navigation;

use topcoat::{
    Result,
    asset::{AssetConfig, RouterBuilderAssetExt},
    context::Cx,
    router::{Router, Slot, href, layout, page, request::uri},
    runtime::RouterBuilderRuntimeExt,
    view::{View, class, component, view},
};
use topcoat_ant_design::{RouterBuilderUiExt, head_assets};

use crate::{assets::GALLERY_STYLESHEET, locale::Locale};

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
    let link_class = class!(
        "group flex min-h-10 items-center gap-3 rounded-md px-3 py-2 text-sm font-medium no-underline transition-colors duration-150",
        "bg-[#e6f4ff] text-[#0958d9]" if active,
        "text-[#595959] hover:bg-[#f5f5f5] hover:text-[#262626]" if !active,
    );
    let badge_class = class!(
        "grid size-7 shrink-0 place-items-center rounded-md border text-[11px] font-bold",
        "border-[#91caff] bg-white text-[#1677ff]" if active,
        "border-[#e5e7eb] bg-[#fafafa] text-[#8c8c8c] group-hover:border-[#d9d9d9]" if !active,
    );

    Ok(view! {
        <a class=(link_class) href=(href) aria-current=(active.then_some("page"))>
            <span class=(badge_class) aria-hidden="true">(badge)</span>
            <span>(label)</span>
        </a>
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
        <header class="mb-8 border-b border-[#e8eaee] pb-7">
            <p class="m-0 text-xs font-bold tracking-[0.12em] text-[#1677ff]">(eyebrow)</p>
            <h1 class="mb-3 mt-2 text-[clamp(30px,5vw,42px)] font-bold leading-[1.15] tracking-[-0.025em]">(title)</h1>
            <p class="m-0 max-w-[760px] text-[15px] leading-7 text-[#595959]">(description)</p>
        </header>
    })
}

/// 根布局只声明一次文档外壳，并自动包裹 `app` 模块下的所有页面。
#[layout]
async fn gallery_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let locale = Locale::current(cx);
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
        locale.select("Topcoat native UI", "Topcoat 原生 UI")
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
        <html lang=(locale.html_lang())>
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <meta name="color-scheme" content="light">
                <title>(document_title) " · Topcoat Ant Design"</title>
                head_assets()
                <link rel="stylesheet" href=(GALLERY_STYLESHEET)>
                topcoat::runtime::script()
                topcoat::dev::script(status_indicator: false)
            </head>
            <body class="m-0 min-w-80 bg-[#f5f7fa] font-mono text-[#262626] antialiased">
                <div class="min-h-screen min-[900px]:grid min-[900px]:grid-cols-[248px_minmax(0,1fr)]">
                    <aside class="border-b border-[#edf0f4] bg-white min-[900px]:sticky min-[900px]:top-0 min-[900px]:h-screen min-[900px]:overflow-y-auto min-[900px]:border-b-0 min-[900px]:border-r">
                        <div class="border-b border-[#edf0f4] px-6 py-6">
                            <a class="flex items-center gap-3 text-[#262626] no-underline" href=(getting_started_url.as_str())>
                                <span class="grid size-9 place-items-center rounded-lg bg-[#1677ff] text-sm font-bold text-white shadow-[0_4px_12px_rgba(22,119,255,0.22)]">"AD"</span>
                                <span><strong class="block text-[15px] font-semibold leading-5">"Topcoat Ant Design"</strong><small class="mt-0.5 block text-xs text-[#8c8c8c]">"Topcoat components"</small></span>
                            </a>
                            <nav class="mt-5 flex gap-2" aria-label=(text(locale, "语言"))>
                                <a class="rounded-md border border-[#d9d9d9] px-3 py-1 text-xs text-[#262626] no-underline aria-[current=page]:border-[#1677ff] aria-[current=page]:bg-[#e6f4ff] aria-[current=page]:text-[#0958d9]" href=(english_url) data-language-switch="" aria-current=(if locale == Locale::En { Some("page") } else { None })>"English"</a>
                                <a class="rounded-md border border-[#d9d9d9] px-3 py-1 text-xs text-[#262626] no-underline aria-[current=page]:border-[#1677ff] aria-[current=page]:bg-[#e6f4ff] aria-[current=page]:text-[#0958d9]" href=(chinese_url.as_str()) data-language-switch="" aria-current=(if locale == Locale::Zh { Some("page") } else { None })>"中文"</a>
                            </nav>
                        </div>
                        <nav class="grid gap-6 px-4 py-5 max-[899px]:grid-cols-3 max-[640px]:grid-cols-1" aria-label=(text(locale, "组件导航"))>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">(text(locale, "开始"))</p>
                                <div class="grid gap-1">
                                    gallery_nav_link(href: getting_started_url.as_str(), badge: "→", label: text(locale, "快速开始"), active: getting_started_active)
                                    gallery_nav_link(href: overview_url.as_str(), badge: "01", label: text(locale, "组件概览"), active: overview_active)
                                    gallery_nav_link(href: icons_url.as_str(), badge: "I", label: text(locale, "Icons 图标"), active: icons_active)
                                    gallery_nav_link(href: native_ui_url.as_str(), badge: "UI", label: locale.select("Topcoat native UI", "Topcoat 原生 UI"), active: native_ui_active)
                                </div>
                            </section>
                            <section class="max-[899px]:hidden">
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">(locale.select("AI Components", "AI 组件"))</p>
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
                                <details class="rounded-lg border border-[#e8eaee] bg-white open:shadow-sm">
                                    <summary class="cursor-pointer px-3 py-3 text-[11px] font-bold uppercase tracking-[0.1em] text-[#595959]">(locale.select("AI Components", "AI 组件"))</summary>
                                    <div class="grid gap-1 border-t border-[#edf0f4] p-2">
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
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">(text(locale, "反馈"))</p>
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
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">(text(locale, "导航"))</p>
                                gallery_nav_link(href: dropdown_menu_url.as_str(), badge: "Dd", label: locale.select("Dropdown Menu", "Dropdown 下拉菜单"), active: dropdown_menu_active)
                                gallery_nav_link(href: tabs_url.as_str(), badge: "T", label: "Tabs", active: tabs_active)
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">(text(locale, "数据录入"))</p>
                                gallery_nav_link(href: form_field_url.as_str(), badge: "F", label: "FormField", active: form_field_active)
                                gallery_nav_link(href: date_time_range_url.as_str(), badge: "R", label: "DateTimeRange", active: date_time_range_active)
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">(text(locale, "数据展示"))</p>
                                gallery_nav_link(href: table_url.as_str(), badge: "Tb", label: "Table", active: table_active)
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">(text(locale, "动效"))</p>
                                gallery_nav_link(href: collapse_url.as_str(), badge: "C", label: "Collapse", active: collapse_active)
                                gallery_nav_link(href: accordion_url.as_str(), badge: "A", label: "Accordion", active: accordion_active)
                            </section>
                        </nav>
                    </aside>
                    <main class="min-w-0 px-8 pb-20 pt-12 max-[640px]:px-4 max-[640px]:pt-8">
                        <div class="mx-auto w-full max-w-[1080px]">(slot)</div>
                    </main>
                </div>
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

#[component]
pub(in crate::app) async fn overview_content(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let ai_examples = [
        ("/chat/states", locale.select("Message states", "消息状态")),
        ("/chat/markdown", "ChatMarkdown"),
        ("/chat/think", "ChatThink"),
        ("/chat/thought-chain", "ChatThoughtChain"),
        ("/chat/sources", "ChatSources"),
        ("/chat/actions", "ChatActions"),
        ("/chat/attachments", "ChatAttachmentTray"),
        ("/chat/files", "ChatFile"),
        ("/chat/prompts", "ChatPrompts"),
        ("/chat/conversations", "ChatConversationList"),
    ]
    .map(|(path, label)| (locale.link(path), label));
    Ok(view! {
        page_header(
            eyebrow: "COMPONENTS",
            title: text(locale, "组件概览"),
            description: text(locale, "独立浏览每个组件的真实样式、Topcoat 交互和共用的 API 文档。"),
        )
        <div class="grid gap-6">
            <a class="group flex items-center justify-between gap-6 rounded-xl border border-[#91caff] bg-[#e6f4ff] p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#1677ff] hover:shadow-md max-[620px]:block" href=(locale.link(&href!(home).resolve(cx)))><div><span class="text-xs font-bold tracking-[0.1em] text-[#0958d9]">(text(locale, "第一次使用"))</span><h2 class="mb-2 mt-2 text-xl font-semibold">(text(locale, "先完成五步接入"))</h2><p class="m-0 text-sm leading-6 text-[#595959]">(text(locale, "查看依赖、页面资源、AssetBundle、Router 和第一个组件的完整示例。"))</p></div><span class="shrink-0 text-sm font-semibold text-[#1677ff] max-[620px]:mt-5 max-[620px]:inline-block">(text(locale, "打开快速开始 →"))</span></a>
            <a class="group flex items-center justify-between gap-6 rounded-xl border border-[#d5d5d5] bg-[#f7f6f3] p-6 text-[#272727] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#555] hover:shadow-md max-[620px]:block" href=(locale.link(&href!(_native::native_ui_page).resolve(cx)))><div><span class="text-xs font-bold tracking-[0.1em] text-[#777]">"TOPCOAT 0.9.0"</span><h2 class="mb-2 mt-2 text-xl font-semibold">(locale.select("Native UI · 31 components", "原生 UI · 31 个组件"))</h2><p class="m-0 text-sm leading-6 text-[#666]">(locale.select("Explore the official neutral theme, reactive controls, sidebar, forms and overlays.", "查看官方 neutral 主题、响应式控件、侧栏、表单与弹层。"))</p></div><span class="shrink-0 text-sm font-semibold text-[#333] max-[620px]:mt-5 max-[620px]:inline-block">(locale.select("Open showcase →", "打开展示页 →"))</span></a>
            <section class="rounded-xl border border-[#dbe8f7] bg-[#f8fbff] p-5" aria-label=(locale.select("AI component examples", "AI 组件示例"))>
                <h2 class="m-0 mb-3 text-base font-semibold text-[#233449]">(locale.select("AI component examples", "AI 组件示例"))</h2>
                <div class="flex flex-wrap gap-2">
                    for (url, label) in ai_examples {
                        <a class="rounded-md border border-[#dbe8f7] bg-white px-3 py-1.5 text-xs font-medium text-[#0958d9] no-underline hover:border-[#91caff] hover:bg-[#e6f4ff] focus-visible:outline-2 focus-visible:outline-[#1677ff]" href=(url.as_str())>(label)</a>
                    }
                </div>
            </section>
            <section class="grid grid-cols-3 gap-5 max-[760px]:grid-cols-1">
                <a class="group rounded-xl border border-[#cbdfff] bg-[#f6faff] p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#1677ff] hover:shadow-md" href=(locale.link(&href!(_ai::chat_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#1677ff] text-xs font-bold text-white">"AI"</span><h2 class="m-0 text-lg font-semibold">(locale.select("Chat interface", "Chat 聊天界面"))</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(locale.select("Explore a composed AI conversation with reusable Topcoat components.", "用可复用的 Topcoat 组件查看完整聊天界面。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_ai::bubble_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#e6f4ff] font-bold text-[#1677ff]">"B"</span><h2 class="m-0 text-lg font-semibold">"ChatBubble"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(locale.select("Compare user and assistant message bubbles.", "查看用户与助手消息气泡。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_ai::message_list_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#e6f4ff] font-bold text-[#1677ff]">"L"</span><h2 class="m-0 text-lg font-semibold">"ChatMessageList"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(locale.select("See a labelled conversation region with messages.", "查看带标签的会话消息区域。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_ai::sender_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#e6f4ff] font-bold text-[#1677ff]">"S"</span><h2 class="m-0 text-lg font-semibold">"ChatSender"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(locale.select("Try the browser-side draft and submit behavior.", "体验浏览器本地草稿和发送交互。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_feedback::notification_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#e6f4ff] font-bold text-[#1677ff]">"N"</span><h2 class="m-0 text-lg font-semibold">"Notification"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(text(locale, "在视口右上角反馈操作结果，支持四种语义状态。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_feedback::popconfirm_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#fff7e6] font-bold text-[#d46b08]">"P"</span><h2 class="m-0 text-lg font-semibold">"Popconfirm"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(text(locale, "贴近操作入口完成轻量确认，并处理边缘偏移和翻转。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_feedback::dialog_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#e6f4ff] font-bold text-[#1677ff]">"Di"</span><h2 class="m-0 text-lg font-semibold">"Dialog"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(text(locale, "使用原生模态语义承载表单和集中操作。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_motion::collapse_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#f6ffed] font-bold text-[#389e0d]">"C"</span><h2 class="m-0 text-lg font-semibold">"Collapse"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(text(locale, "为未知高度内容提供可逆的展开与收起过渡。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_feedback::drawer_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#f9f0ff] font-bold text-[#722ed1]">"D"</span><h2 class="m-0 text-lg font-semibold">"Drawer"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(text(locale, "在当前列表上方查看完整记录详情。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_navigation::dropdown_menu_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#e6f4ff] font-bold text-[#1677ff]">"Dd"</span><h2 class="m-0 text-lg font-semibold">"Dropdown Menu"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(locale.select("Group secondary actions beside their trigger.", "把次要操作收进触发按钮旁的菜单。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_navigation::tabs_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#e6f4ff] font-bold text-[#1677ff]">"T"</span><h2 class="m-0 text-lg font-semibold">"Tabs"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(text(locale, "用真实路由组织对象详情页面。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_data_display::table_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#f9f0ff] font-bold text-[#722ed1]">"Tb"</span><h2 class="m-0 text-lg font-semibold">"Table"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(text(locale, "用统一密度展示数据，并组合页码或游标分页。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_data_entry::form_field_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#fff7e6] font-bold text-[#d46b08]">"F"</span><h2 class="m-0 text-lg font-semibold">"FormField"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(text(locale, "统一表单字段标签、说明与错误反馈。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(locale.link(&href!(_data_entry::date_time_range_page).resolve(cx)))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#e6f4ff] font-bold text-[#0958d9]">"R"</span><h2 class="m-0 text-lg font-semibold">"DateTimeRange"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">(text(locale, "在弹出层中选择开始和结束时间。"))</p><span class="mt-6 inline-block text-sm text-[#1677ff]">(text(locale, "查看组件 →"))</span></a>
            </section>
        </div>
    })
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
            "31 native components",
            "class=\"native-ui",
            "Buttons",
            "Forms",
            "Sidebar",
            "Dialog",
            "Accordion",
        ] {
            assert!(html.contains(marker), "missing {marker}");
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
        assert!(html.contains("gr-accordion-item"));
        assert!(html.contains("Show example code"));
        assert!(html.contains("aria-controls=\"gallery-comment\""));
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
                assert!(html.contains("id=\"gallery-actions-menu\""));
                assert!(html.contains("gr-dropdown-menu"));
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
                let tabs_end = tabs + html[tabs..].find("</nav>").unwrap();
                let tabs_html = &html[tabs..tabs_end];
                assert!(tabs_html.contains("href=\"/tabs/webhook\""));
                assert!(
                    html.contains(&format!("id=\"{content_id}\"")),
                    "{path}: {html}"
                );

                let active_href = tabs_html.find(&format!("href=\"{path}\"")).unwrap();
                let active_link_start = tabs_html[..active_href].rfind("<a").expect("tab start");
                let active_link_end =
                    active_href + tabs_html[active_href..].find('>').expect("tab start end");
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
