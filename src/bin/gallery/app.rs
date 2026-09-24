mod _data_display;
mod _data_entry;
mod _feedback;
mod _guide;
mod _motion;
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

use crate::assets::GALLERY_STYLESHEET;

/// 使用 Topcoat 模块路由构造 Gallery。
///
/// 页面和布局由 `module_router!` 按 `app` 模块树发现；静态资源路由不属于
/// 页面层级，因此在这里逐项注册，让每条路由的来源保持明确。
pub(crate) fn router(app_assets: AssetConfig) -> Router {
    topcoat::router::module_router!()
        .route(crate::assets::component_css)
        .route(crate::assets::gallery_css)
        .route(crate::assets::topcoat_runtime_js)
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
    let overview_link = href!(home);
    let getting_started_link = href!(_guide::getting_started_page);
    let icons_link = href!(_guide::icons_page);
    let notification_link = href!(_feedback::notification_page);
    let tag_link = href!(_feedback::tag_page);
    let tag_active = tag_link.is_current(cx);
    let tag_url = tag_link.resolve(cx);
    let tooltip_link = href!(_feedback::tooltip_page);
    let popconfirm_link = href!(_feedback::popconfirm_page);
    let dialog_link = href!(_feedback::dialog_page);
    let drawer_link = href!(_feedback::drawer_page);
    let collapse_link = href!(_motion::collapse_page);
    let accordion_link = href!(_motion::accordion_page);
    let tabs_link = href!(_navigation::tabs_page);
    let table_link = href!(_data_display::table_page);
    let form_field_link = href!(_data_entry::form_field_page);
    let date_time_range_link = href!(_data_entry::date_time_range_page);

    let overview_active = overview_link.is_current(cx);
    let getting_started_active = getting_started_link.is_current(cx);
    let icons_active = icons_link.is_current(cx);
    let notification_active = notification_link.is_current(cx);
    let tooltip_active = tooltip_link.is_current(cx);
    let popconfirm_active = popconfirm_link.is_current(cx);
    let dialog_active = dialog_link.is_current(cx);
    let drawer_active = drawer_link.is_current(cx);
    let collapse_active = collapse_link.is_current(cx);
    let accordion_active = accordion_link.is_current(cx);
    let tabs_active = tabs_link.is_current(cx) || uri(cx).path().starts_with("/tabs/");
    let table_active = table_link.is_current(cx);
    let form_field_active = form_field_link.is_current(cx);
    let date_time_range_active = date_time_range_link.is_current(cx);
    let document_title = if getting_started_active {
        "使用组件库"
    } else if icons_active {
        "Icons 图标"
    } else if notification_active {
        "Notification 通知提醒框"
    } else if tag_active {
        "Tag 标签"
    } else if tooltip_active {
        "Tooltip 文字提示"
    } else if popconfirm_active {
        "Popconfirm 气泡确认框"
    } else if dialog_active {
        "Dialog 模态对话框"
    } else if drawer_active {
        "Drawer 抽屉"
    } else if collapse_active {
        "Collapse 折叠动画"
    } else if accordion_active {
        "Accordion 手风琴"
    } else if tabs_active {
        "Tabs 路由页签"
    } else if table_active {
        "Table 数据表格"
    } else if form_field_active {
        "FormField 表单字段"
    } else if date_time_range_active {
        "DateTimeRange 时间范围"
    } else {
        "组件概览"
    };

    let overview_url = overview_link.resolve(cx);
    let getting_started_url = getting_started_link.resolve(cx);
    let icons_url = icons_link.resolve(cx);
    let notification_url = notification_link.resolve(cx);
    let tooltip_url = tooltip_link.resolve(cx);
    let popconfirm_url = popconfirm_link.resolve(cx);
    let dialog_url = dialog_link.resolve(cx);
    let drawer_url = drawer_link.resolve(cx);
    let collapse_url = collapse_link.resolve(cx);
    let accordion_url = accordion_link.resolve(cx);
    let tabs_url = tabs_link.resolve(cx);
    let table_url = table_link.resolve(cx);
    let form_field_url = form_field_link.resolve(cx);
    let date_time_range_url = date_time_range_link.resolve(cx);

    Ok(view! {
        <!DOCTYPE html>
        <html lang="zh-CN">
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
                            <a class="flex items-center gap-3 text-[#262626] no-underline" href=(overview_url.as_str())>
                                <span class="grid size-9 place-items-center rounded-lg bg-[#1677ff] text-sm font-bold text-white shadow-[0_4px_12px_rgba(22,119,255,0.22)]">"AD"</span>
                                <span><strong class="block text-[15px] font-semibold leading-5">"Topcoat Ant Design"</strong><small class="mt-0.5 block text-xs text-[#8c8c8c]">"Topcoat components"</small></span>
                            </a>
                        </div>
                        <nav class="grid gap-6 px-4 py-5 max-[899px]:grid-cols-3 max-[640px]:grid-cols-1" aria-label="组件导航">
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">"开始"</p>
                                <div class="grid gap-1">
                                    gallery_nav_link(href: getting_started_url.as_str(), badge: "→", label: "快速开始", active: getting_started_active)
                                    gallery_nav_link(href: overview_url.as_str(), badge: "01", label: "组件概览", active: overview_active)
                                    gallery_nav_link(href: icons_url.as_str(), badge: "I", label: "Icons 图标", active: icons_active)
                                </div>
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">"反馈"</p>
                                <div class="grid gap-1">
                                    gallery_nav_link(href: notification_url.as_str(), badge: "N", label: "Notification", active: notification_active)
                                    gallery_nav_link(href: tag_url.as_str(), badge: "Ta", label: "Tag 标签", active: tag_active)
                                    gallery_nav_link(href: tooltip_url.as_str(), badge: "T", label: "Tooltip", active: tooltip_active)
                                    gallery_nav_link(href: popconfirm_url.as_str(), badge: "P", label: "Popconfirm", active: popconfirm_active)
                                    gallery_nav_link(href: dialog_url.as_str(), badge: "Di", label: "Dialog", active: dialog_active)
                                    gallery_nav_link(href: drawer_url.as_str(), badge: "D", label: "Drawer", active: drawer_active)
                                </div>
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">"导航"</p>
                                gallery_nav_link(href: tabs_url.as_str(), badge: "T", label: "Tabs", active: tabs_active)
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">"数据录入"</p>
                                gallery_nav_link(href: form_field_url.as_str(), badge: "F", label: "FormField", active: form_field_active)
                                gallery_nav_link(href: date_time_range_url.as_str(), badge: "R", label: "DateTimeRange", active: date_time_range_active)
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">"数据展示"</p>
                                gallery_nav_link(href: table_url.as_str(), badge: "Tb", label: "Table", active: table_active)
                            </section>
                            <section>
                                <p class="mb-2 mt-0 px-3 text-[11px] font-bold tracking-[0.1em] text-[#8c8c8c]">"动效"</p>
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
async fn home() -> Result<impl View> {
    Ok(view! {
        page_header(
            eyebrow: "GETTING STARTED",
            title: "组件概览",
            description: "独立浏览每个组件的真实样式、Topcoat 交互和共用的 API 文档。",
        )
        <div class="grid gap-6">
            <a class="group flex items-center justify-between gap-6 rounded-xl border border-[#91caff] bg-[#e6f4ff] p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#1677ff] hover:shadow-md max-[620px]:block" href=(href!(_guide::getting_started_page))><div><span class="text-xs font-bold tracking-[0.1em] text-[#0958d9]">"第一次使用"</span><h2 class="mb-2 mt-2 text-xl font-semibold">"先完成五步接入"</h2><p class="m-0 text-sm leading-6 text-[#595959]">"查看依赖、页面资源、AssetBundle、Router 和第一个组件的完整示例。"</p></div><span class="shrink-0 text-sm font-semibold text-[#1677ff] max-[620px]:mt-5 max-[620px]:inline-block">"打开快速开始 →"</span></a>
            <section class="grid grid-cols-3 gap-5 max-[760px]:grid-cols-1">
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(href!(_feedback::notification_page))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#e6f4ff] font-bold text-[#1677ff]">"N"</span><h2 class="m-0 text-lg font-semibold">"Notification"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">"在视口右上角反馈操作结果，支持四种语义状态。"</p><span class="mt-6 inline-block text-sm text-[#1677ff]">"查看组件 →"</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(href!(_feedback::popconfirm_page))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#fff7e6] font-bold text-[#d46b08]">"P"</span><h2 class="m-0 text-lg font-semibold">"Popconfirm"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">"贴近操作入口完成轻量确认，并处理边缘偏移和翻转。"</p><span class="mt-6 inline-block text-sm text-[#1677ff]">"查看组件 →"</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(href!(_feedback::dialog_page))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#e6f4ff] font-bold text-[#1677ff]">"Di"</span><h2 class="m-0 text-lg font-semibold">"Dialog"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">"使用原生模态语义承载表单和集中操作。"</p><span class="mt-6 inline-block text-sm text-[#1677ff]">"查看组件 →"</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(href!(_motion::collapse_page))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#f6ffed] font-bold text-[#389e0d]">"C"</span><h2 class="m-0 text-lg font-semibold">"Collapse"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">"为未知高度内容提供可逆的展开与收起过渡。"</p><span class="mt-6 inline-block text-sm text-[#1677ff]">"查看组件 →"</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(href!(_feedback::drawer_page))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#f9f0ff] font-bold text-[#722ed1]">"D"</span><h2 class="m-0 text-lg font-semibold">"Drawer"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">"在当前列表上方查看完整记录详情。"</p><span class="mt-6 inline-block text-sm text-[#1677ff]">"查看组件 →"</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(href!(_navigation::tabs_page))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#e6f4ff] font-bold text-[#1677ff]">"T"</span><h2 class="m-0 text-lg font-semibold">"Tabs"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">"用真实路由组织对象详情页面。"</p><span class="mt-6 inline-block text-sm text-[#1677ff]">"查看组件 →"</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(href!(_data_display::table_page))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#f9f0ff] font-bold text-[#722ed1]">"Tb"</span><h2 class="m-0 text-lg font-semibold">"Table"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">"用统一密度展示数据，并组合页码或游标分页。"</p><span class="mt-6 inline-block text-sm text-[#1677ff]">"查看组件 →"</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(href!(_data_entry::form_field_page))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#fff7e6] font-bold text-[#d46b08]">"F"</span><h2 class="m-0 text-lg font-semibold">"FormField"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">"统一表单字段标签、说明与错误反馈。"</p><span class="mt-6 inline-block text-sm text-[#1677ff]">"查看组件 →"</span></a>
                <a class="group rounded-xl border border-[#e8eaee] bg-white p-6 text-[#262626] no-underline shadow-sm transition-[border-color,box-shadow,transform] duration-200 hover:-translate-y-0.5 hover:border-[#91caff] hover:shadow-md" href=(href!(_data_entry::date_time_range_page))><span class="mb-5 grid size-10 place-items-center rounded-lg bg-[#e6f4ff] font-bold text-[#0958d9]">"R"</span><h2 class="m-0 text-lg font-semibold">"DateTimeRange"</h2><p class="mb-0 mt-2 text-sm leading-6 text-[#595959]">"在弹出层中选择开始和结束时间。"</p><span class="mt-6 inline-block text-sm text-[#1677ff]">"查看组件 →"</span></a>
            </section>
        </div>
    })
}

#[cfg(test)]
mod tests {
    use topcoat::router::{Body, StatusCode, request::Request, to_bytes};

    use super::router;

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
        assert!(html.contains("Accordion 手风琴"));
        assert!(html.contains("gr-accordion-item"));
        assert!(html.contains("显示示例代码"));
        assert!(html.contains("aria-controls=\"gallery-comment\""));
    }

    #[tokio::test]
    async fn module_tree_serves_every_gallery_page_through_the_root_layout() {
        let router = router(crate::assets::config().expect("Gallery assets should be valid"));

        for (path, expected_heading, tab_content_id) in [
            ("/", "组件概览", None),
            ("/getting-started", "使用组件库", None),
            ("/icons", "Icons 图标", None),
            ("/notification", "Notification 通知提醒框", None),
            ("/tag", "Tag 标签", None),
            ("/tooltip", "Tooltip 文字提示", None),
            ("/popconfirm", "Popconfirm 气泡确认框", None),
            ("/dialog", "Dialog 模态对话框", None),
            ("/drawer", "Drawer 抽屉", None),
            ("/table", "Table 数据表格", None),
            ("/form-field", "FormField 表单字段", None),
            ("/date-time-range", "DateTimeRange 时间范围", None),
            ("/collapse", "Collapse 折叠动画", None),
            ("/accordion", "Accordion 手风琴", None),
            (
                "/tabs/webhook",
                "Webhook 配置",
                Some("tabs-webhook-heading"),
            ),
            ("/tabs/events", "事件记录", Some("tabs-events-heading")),
            (
                "/tabs/permissions",
                "访问权限",
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
                assert!(html.contains("aria-label=\"组件示例分页\""));
                assert!(html.contains("aria-current=\"page\""));
            }
            if matches!(
                path,
                "/notification"
                    | "/tag"
                    | "/tooltip"
                    | "/popconfirm"
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
                assert!(html.contains("显示示例代码"), "{path}: {html}");
                assert!(
                    !html.contains("当前文档没有可展示的 Rust 示例"),
                    "{path}: {html}"
                );
            }
            if let Some(content_id) = tab_content_id {
                let tabs = html.find("aria-label=\"项目详情示例\"").unwrap();
                let first_link = html.find("href=\"/tabs/webhook\"").unwrap();
                let tabs_end = tabs + html[tabs..].find("</nav>").unwrap();
                assert!(first_link > tabs && first_link < tabs_end);
                assert!(
                    html.contains(&format!("id=\"{content_id}\"")),
                    "{path}: {html}"
                );

                let tabs_html = &html[tabs..tabs_end];
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
