use topcoat::{
    Result,
    icon::icon,
    view::{Child, View, component, view},
};

use crate::icons::DOWN_OUTLINED;

/// Expandable list of references used while preparing a response.
#[component]
pub async fn chat_sources(label: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <details class="gr-chat-sources" open="open">
            <summary class="gr-chat-sources-trigger"><span>(label)</span><span class="gr-chat-sources-caret" aria-hidden="true">icon(data: DOWN_OUTLINED, size: 14)</span></summary>
            <ul class="gr-chat-sources-list" aria-label=(label)>(child)</ul>
        </details>
    })
}

/// Linked reference with an optional summary and activity label.
/// Non-HTTP destinations are inert.
#[component]
pub async fn chat_source(
    title: &str,
    href: &str,
    #[default] summary: Option<&str>,
    #[default] activity: Option<&str>,
) -> Result<impl View> {
    let safe_href = if (href.starts_with("https://") || href.starts_with("http://"))
        && !href.chars().any(char::is_control)
    {
        Some(href)
    } else {
        None
    };
    Ok(view! {
        <li class="gr-chat-source">
            if let Some(summary) = summary {
                <div class="gr-chat-source-summary">(summary)</div>
            }
            if let Some(safe_href) = safe_href {
                <a class="gr-chat-source-link" href=(safe_href) rel="noopener noreferrer" target="_blank">
                    if let Some(activity) = activity { <span class="gr-chat-source-activity">(activity)</span> }
                    <span class="gr-chat-source-title">(title)</span>
                    <span class="gr-chat-source-arrow" aria-hidden="true">"›"</span>
                </a>
            } else {
                <span class="gr-chat-source-link gr-chat-source-link-disabled" aria-disabled="true">
                    if let Some(activity) = activity { <span class="gr-chat-source-activity">(activity)</span> }
                    <span class="gr-chat-source-title">(title)</span>
                </span>
            }
        </li>
    })
}

#[cfg(test)]
mod tests {
    use topcoat::{
        context::Cx,
        view::{ViewExt, view},
    };

    use super::chat_source;

    #[tokio::test]
    async fn source_link_rejects_script_destinations() {
        let cx = &Cx::default();
        let html = view! { cx => chat_source(title: "unsafe", href: "javascript:alert(1)") }
            .single()
            .await
            .unwrap()
            .render(cx);
        assert!(html.contains("aria-disabled=\"true\""));
        assert!(!html.contains("javascript:"));
    }
}
