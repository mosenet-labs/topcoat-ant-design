use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    runtime::{expr, signal},
    view::{View, ViewExt, attributes, component, view},
};
use topcoat_ant_design::{
    ButtonVariant, DEFAULT_FONT, DateTimeRangeConfig, NotificationTone, STYLESHEET, button, card,
    card_content, chat_actions, chat_conversation_item, chat_prompt, chat_source, chat_think,
    collapse, collapse_trigger_attributes, data_table, date_time_range_filter, embedded_stylesheet,
    icons::PROJECT_OUTLINED, notification, popconfirm, popconfirm_trigger_attributes,
    table_page_size_select, table_pagination, tabs_trigger, tooltip, tooltip_content,
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

#[test]
fn custom_stylesheet_includes_light_and_dark_theme_tokens() {
    let stylesheet = embedded_stylesheet();
    assert!(stylesheet.contains("--gr-surface:"));
    assert!(stylesheet.contains(".dark{"));
    assert!(stylesheet.contains("var(--gr-accent)"));
    assert!(!stylesheet.contains(".native-ui"));
}

#[tokio::test]
async fn official_components_render_from_crate_root_with_shared_styles() {
    let cx = &Cx::default();
    let html = view! { cx =>
        card(
            card_content(
                button(variant: ButtonVariant::Primary, "Save")
            )
        )
    }
    .single()
    .await
    .unwrap()
    .render(cx);

    assert!(html.contains("bg-primary"), "{html}");
    assert!(html.contains("bg-card"), "{html}");
    assert!(!html.contains("native-ui"), "{html}");
}

#[component]
async fn custom_components_fixture(cx: &Cx) -> Result<impl View> {
    let open = signal(cx, || true);
    Ok(view! {
        chat_actions(label: "Actions", attrs: attributes! { class="extra-actions" data-audit="actions" },
            <button type="button">"Copy"</button>
        )
        chat_source(title: "Documentation", href: "https://example.com", attrs: attributes! { class="extra-source" data-audit="source" })
        chat_prompt(title: "Try this", attrs: attributes! { class="extra-prompt" data-audit="prompt" })
        chat_think(id: "audit-think", open: &open, attrs: attributes! { class="extra-think" }, "Reasoning")
        chat_conversation_item(title: "Draft", href: "/chat/new", active: expr!(open.get()), attrs: attributes! { class="extra-conversation" })
        tabs_trigger(active: expr!(open.get()), attrs: attributes! { href="/details" class="extra-tab" }, "Details")
        tooltip(attrs: attributes! { class="extra-tooltip" }, <span>"Hint trigger"</span> tooltip_content(attrs: attributes! { id="audit-tooltip" }, "Hint"))
        date_time_range_filter(
            config: DateTimeRangeConfig::new("audit-range", "", ""),
            from_attrs: attributes! { data-audit="from" },
            to_attrs: attributes! { data-audit="to" },
        )
    })
}

#[tokio::test]
async fn custom_components_forward_attributes_and_accept_reactive_active_state() {
    let cx = &Cx::default();
    let html = view! { cx => custom_components_fixture() }
        .single()
        .await
        .unwrap()
        .render(cx);

    for marker in [
        "extra-actions",
        "extra-source",
        "extra-prompt",
        "extra-think",
        "extra-conversation",
        "extra-tab",
        "extra-tooltip",
        "data-audit=\"from\"",
        "data-audit=\"to\"",
    ] {
        assert!(html.contains(marker), "missing {marker}: {html}");
    }
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
