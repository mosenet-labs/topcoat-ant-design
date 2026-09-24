use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    runtime::{Event, signal},
    view::{View, ViewExt, attributes, component, view},
};
use topcoat_ant_design::{
    AccordionItemConfig, DEFAULT_FONT, DateTimeRangeConfig, DialogConfig, NotificationTone,
    STYLESHEET, accordion_item, collapse, collapse_trigger_attributes, data_table,
    date_time_range_filter, dialog, dialog_close_attributes, dialog_trigger_attributes,
    embedded_stylesheet, icons::PROJECT_OUTLINED, notification, popconfirm,
    popconfirm_trigger_attributes, table_page_size_select, table_pagination,
};

#[component]
async fn notification_fixture(cx: &Cx, tone: NotificationTone) -> Result<impl View> {
    let message = signal(cx, || "保存成功".to_owned());
    Ok(view! {
        notification(
            message: &message,
            title: "操作结果",
            tone: tone,
            attrs: attributes! { class="notification-fixture" data-source="test" },
        )
    })
}

#[component]
async fn collapse_fixture(cx: &Cx) -> Result<impl View> {
    let open = signal(cx, || true);
    let trigger = collapse_trigger_attributes(cx, "fixture-content", &open);

    Ok(view! {
        <button type="button" (trigger)>"切换"</button>
        collapse(
            id: "fixture-content",
            open: &open,
            attrs: attributes! { class="collapse-fixture" aria-labelledby="fixture-trigger" },
            <p>"可折叠内容"</p>
        )
    })
}

#[component]
async fn accordion_fixture(cx: &Cx) -> Result<impl View> {
    let active = signal(cx, || "fixture-comment".to_owned());
    let selected = signal(cx, || 2.0);

    Ok(view! {
        accordion_item(
            config: AccordionItemConfig::new("fixture-comment", "评论 Trigger", "评论入口").with_badge("OR"),
            active: &active,
            selected_count: Some(&selected),
            attrs: attributes! { class="accordion-fixture" data-source="test" },
            <p>"第一组内容"</p>
        )
        accordion_item(
            config: AccordionItemConfig::new("fixture-event", "自动事件 Trigger", "事件入口"),
            active: &active,
            <p>"第二组内容"</p>
        )
    })
}

#[component]
async fn dialog_fixture(cx: &Cx) -> Result<impl View> {
    let busy = topcoat::runtime::signal(cx, || false);
    let trigger = dialog_trigger_attributes(cx, "edit-connection");
    let cancel = dialog_close_attributes(cx, "edit-connection");

    Ok(view! {
        <button type="button" (trigger)>"编辑"</button>
        dialog(
            config: DialogConfig::new("edit-connection", "编辑连接")
                .with_eyebrow("CONNECTION"),
            busy: &busy,
            attrs: attributes! { class="dialog-fixture" data-source="test" },
            <form>
                <p>"Dialog 内容"</p>
                <button type="button" (cancel)>"取消"</button>
            </form>
        )
    })
}

async fn render_notification(tone: NotificationTone) -> String {
    let cx = &Cx::default();
    view! { cx => notification_fixture(tone: tone) }
        .single()
        .await
        .unwrap()
        .render(cx)
}

#[test]
fn theme_uses_jetbrains_mono_and_declares_its_stylesheet() {
    assert_eq!(DEFAULT_FONT.family(), "JetBrains Mono");
    assert_eq!(DEFAULT_FONT.faces().len(), 4);
    assert_ne!(STYLESHEET.id().as_u64(), 0);
}

#[test]
fn stylesheet_contains_reversible_collapse_motion() {
    let stylesheet = embedded_stylesheet();

    assert!(stylesheet.contains(".gr-collapse"), "{stylesheet}");
    assert!(stylesheet.contains("grid-template-rows"), "{stylesheet}");
    assert!(
        stylesheet.contains("prefers-reduced-motion"),
        "{stylesheet}"
    );
}

#[tokio::test]
async fn icon_catalog_renders_through_topcoat_with_accessible_semantics() {
    let cx = &Cx::default();
    let html = view! { cx => icon(data: PROJECT_OUTLINED, label: "GitLab 项目") }
        .single()
        .await
        .unwrap()
        .render(cx);

    assert!(html.contains("<svg"), "{html}");
    assert!(html.contains("role=\"img\""), "{html}");
    assert!(html.contains("aria-label=\"GitLab 项目\""), "{html}");
    assert!(html.contains("currentColor"), "{html}");
}

#[tokio::test]
async fn notification_renders_polite_success_semantics_and_runtime_controls() {
    let html = render_notification(NotificationTone::Success).await;

    assert!(html.contains("gr-notification-success"), "{html}");
    assert!(html.contains("role=\"status\""), "{html}");
    assert!(html.contains("aria-live=\"polite\""), "{html}");
    assert!(html.contains("Close notification"), "{html}");
    assert!(html.contains("popover=\"manual\""), "{html}");
    assert!(html.contains("showPopover"), "{html}");
    assert!(html.contains("hidePopover"), "{html}");
    assert!(html.contains("data-topcoat-on:animationend"), "{html}");
    assert!(html.contains("保存成功"), "{html}");
    assert!(html.contains("notification-fixture"), "{html}");
    assert!(html.contains("data-source=\"test\""), "{html}");
}

#[tokio::test]
async fn notification_renders_assertive_error_semantics() {
    let html = render_notification(NotificationTone::Error).await;

    assert!(html.contains("gr-notification-error"), "{html}");
    assert!(html.contains("role=\"alert\""), "{html}");
    assert!(html.contains("aria-live=\"assertive\""), "{html}");
}

#[tokio::test]
async fn popconfirm_connects_trigger_dialog_and_confirm_action() {
    let cx = &Cx::default();
    let trigger = popconfirm_trigger_attributes(cx, "remove-project");
    let html = view! { cx =>
        <button type="button" (trigger)>"停用"</button>
        popconfirm(
            id: "remove-project",
            title: "确认停用？",
            description: Some("停用后不再处理任务。"),
            attrs: attributes! { class="popconfirm-fixture" data-source="test" },
            <button type="button">"确认"</button>
        )
    }
    .single()
    .await
    .unwrap()
    .render(cx);

    assert!(html.contains("popovertarget=\"remove-project\""), "{html}");
    assert!(html.contains("popover=\"auto\""), "{html}");
    assert!(html.contains("role=\"alertdialog\""), "{html}");
    assert!(
        html.contains("aria-labelledby=\"remove-project-title\""),
        "{html}"
    );
    assert!(
        html.contains("aria-describedby=\"remove-project-description\""),
        "{html}"
    );
    assert!(html.contains("anchor-name: --gr-remove-project"), "{html}");
    assert!(
        html.contains("position-anchor: --gr-remove-project"),
        "{html}"
    );
    assert!(html.contains("gr-popconfirm-arrow"), "{html}");
    assert!(html.contains("aria-hidden=\"true\""), "{html}");
    assert!(html.contains("data-topcoat-on:toggle"), "{html}");
    assert!(html.contains("data-topcoat-on:click"), "{html}");
    assert!(html.contains("popconfirm-fixture"), "{html}");
    assert!(html.contains("data-source=\"test\""), "{html}");
    assert!(html.contains("停用后不再处理任务。"), "{html}");
}

#[tokio::test]
async fn date_time_range_keeps_native_form_and_popover_semantics() {
    let cx = &Cx::default();
    let html = view! { cx =>
        <form>
            date_time_range_filter(
                config: DateTimeRangeConfig::new("run-range", "2026-09-20T10:00", "2026-09-21T10:00"),
                attrs: attributes! { data-source="test" },
            )
        </form>
    }
    .single()
    .await
    .unwrap()
    .render(cx);

    assert!(html.contains("popovertarget=\"run-range\""), "{html}");
    assert!(html.contains("popover=\"auto\""), "{html}");
    assert!(html.contains("name=\"from\""), "{html}");
    assert!(html.contains("name=\"to\""), "{html}");
    assert!(html.contains("09-20 10:00 → 09-21 10:00"), "{html}");
    assert!(html.contains("Last 24 hours"), "{html}");
    assert!(html.contains("data-source=\"test\""), "{html}");
}

#[tokio::test]
async fn dialog_connects_native_controls_semantics_and_forwarded_attributes() {
    let cx = &Cx::default();
    let html = view! { cx => dialog_fixture() }
        .single()
        .await
        .unwrap()
        .render(cx);

    assert!(html.contains("commandfor=\"edit-connection\""), "{html}");
    assert!(html.contains("command=\"show-modal\""), "{html}");
    assert!(html.contains("command=\"close\""), "{html}");
    assert!(html.contains("aria-haspopup=\"dialog\""), "{html}");
    assert!(html.contains("<dialog"), "{html}");
    assert!(html.contains("role=\"dialog\""), "{html}");
    assert!(html.contains("aria-modal=\"true\""), "{html}");
    assert!(
        html.contains("aria-labelledby=\"edit-connection-title\""),
        "{html}"
    );
    assert!(html.contains("data-topcoat-on:cancel"), "{html}");
    assert!(html.contains("aria-label=\"Close\""), "{html}");
    assert!(html.contains("dialog-fixture"), "{html}");
    assert!(html.contains("data-source=\"test\""), "{html}");
    assert!(html.contains("Dialog 内容"), "{html}");
}

#[tokio::test]
async fn collapse_connects_trigger_state_content_and_forwarded_attributes() {
    let cx = &Cx::default();
    let html = view! { cx => collapse_fixture() }
        .single()
        .await
        .unwrap()
        .render(cx);

    assert!(html.contains("aria-controls=\"fixture-content\""), "{html}");
    assert!(html.contains("aria-expanded=\"true\""), "{html}");
    assert!(html.contains("data-topcoat-on:click"), "{html}");
    assert!(html.contains("id=\"fixture-content\""), "{html}");
    assert!(html.contains("data-state=\"open\""), "{html}");
    assert!(html.contains("aria-hidden=\"false\""), "{html}");
    assert!(html.contains("collapse-fixture"), "{html}");
    assert!(html.contains("gr-collapse-inner"), "{html}");
    assert!(html.contains("可折叠内容"), "{html}");
}

#[tokio::test]
async fn accordion_uses_one_active_item_and_keeps_content_rendered() {
    let cx = &Cx::default();
    let html = view! { cx => accordion_fixture() }
        .single()
        .await
        .unwrap()
        .render(cx);

    assert!(html.contains("class=\"gr-accordion-item"), "{html}");
    assert!(html.contains("aria-controls=\"fixture-comment\""), "{html}");
    assert!(html.contains("aria-expanded=\"true\""), "{html}");
    assert!(html.contains("aria-controls=\"fixture-event\""), "{html}");
    assert!(html.contains("aria-expanded=\"false\""), "{html}");
    assert!(
        html.contains("<!--::topcoat::expr::start") && html.contains(" enabled"),
        "{html}"
    );
    assert!(html.contains("第一组内容"), "{html}");
    assert!(html.contains("第二组内容"), "{html}");
    assert!(html.contains("accordion-fixture"), "{html}");
    assert!(html.contains("data-source=\"test\""), "{html}");
}

#[tokio::test]
async fn table_pagination_supports_a_reusable_page_size_selector() {
    let cx = &Cx::default();
    let html = view! { cx =>
        data_table(label: "审计事件",
            <thead><tr><th>"操作"</th></tr></thead>
            <tbody><tr><td>"登录"</td></tr></tbody>
        )
        table_pagination(summary: "共 1 条事件", label: "审计日志分页",
            table_page_size_select(
                id: "audit-page-size",
                label: "每页记录",
                attrs: attributes! { class="page-size-fixture" data-source="test" },
                <option value="10">"10"</option>
                <option value="30">"30"</option>
            )
            <button type="button" disabled="">"上一页"</button>
            <span aria-current="page">"1 / 1"</span>
            <button type="button" disabled="">"下一页"</button>
        )
    }
    .single()
    .await
    .unwrap()
    .render(cx);

    assert!(html.contains("gr-data-table"), "{html}");
    assert!(html.contains("gr-table-pagination"), "{html}");
    assert!(html.contains("gr-table-page-size"), "{html}");
    assert!(html.contains("id=\"audit-page-size\""), "{html}");
    assert!(html.contains("aria-label=\"每页记录\""), "{html}");
    assert!(html.contains("page-size-fixture"), "{html}");
    assert!(html.contains("data-source=\"test\""), "{html}");
    assert!(html.contains("上一页"), "{html}");
    assert!(html.contains("下一页"), "{html}");
}

#[component]
async fn controlled_dialog_fixture(
    cx: &Cx,
    initially_open: bool,
    pending: bool,
) -> Result<impl View> {
    let open = signal(cx, || initially_open);
    let busy = signal(cx, || pending);
    let title = signal(cx, || "编辑 Provider".to_owned());
    let secret = signal(cx, || "".to_owned());
    Ok(view! {
        dialog(
            config: DialogConfig::new("controlled-provider", "静态备用标题"),
            busy: &busy,
            open: Some(&open),
            title: Some(&title),
            attrs: attributes! { @close=$(|_event: Event| secret.set("".to_owned())) },
            <form method="post" action="/providers/save">
                <input type="password" :value=$(secret.get())>
            </form>
        )
    })
}

#[tokio::test]
async fn controlled_dialog_renders_signal_state_without_non_modal_open_attribute() {
    for (initially_open, state) in [(false, "closed"), (true, "open")] {
        let cx = &Cx::default();
        let html = view! { cx => controlled_dialog_fixture(initially_open: initially_open, pending: false) }
            .single().await.unwrap().render(cx);
        assert!(html.contains(&format!("data-state=\"{state}\"")), "{html}");
        assert!(
            !html.contains(" open=\""),
            "SSR must wait for showModal: {html}"
        );
        assert!(html.contains("data-topcoat-bind:data-state"));
        assert!(html.contains("queueMicrotask"));
        // Topcoat prefixes raw expressions with `return `: a leading line
        // terminator silently returns undefined instead of running the adapter.
        let binding = html
            .split_once("data-topcoat-bind:data-state=\"")
            .expect("dialog state binding")
            .1
            .split('"')
            .next()
            .unwrap();
        let returned = binding.split_once("return ").expect("raw return").1;
        let whitespace = &returned[..returned.len() - returned.trim_start().len()];
        assert!(
            !whitespace.contains(['\n', '\r', '\u{2028}', '\u{2029}']),
            "a newline after return skips the dialog adapter"
        );
        assert!(html.contains("target.showModal()"));
        assert!(html.contains("target.close()"));
        assert!(html.contains("编辑 Provider"));
        assert!(!html.contains("静态备用标题"));
    }
}

#[tokio::test]
async fn controlled_dialog_keeps_caller_close_cleanup_and_guards_busy_cancellation() {
    let cx = &Cx::default();
    let html = view! { cx => controlled_dialog_fixture(initially_open: true, pending: true) }
        .single()
        .await
        .unwrap()
        .render(cx);
    assert!(
        html.contains("data-topcoat-on:close"),
        "caller cleanup must survive"
    );
    assert!(
        html.contains("data-topcoat-on:cancel"),
        "Escape uses the busy guard"
    );
    assert!(
        html.contains("state.open.toggle()"),
        "native close synchronizes the signal"
    );
    assert!(
        html.contains("state.busy.get()"),
        "native close retains busy protection"
    );
    assert!(
        html.contains(" disabled"),
        "busy disables the header close button"
    );
}
