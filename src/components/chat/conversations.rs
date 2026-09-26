use topcoat::{
    Result,
    view::{Child, View, attributes, component, view},
};

use crate::native_ui::sidebar;

/// Navigation region for host-provided conversation routes.
#[component]
pub async fn chat_conversation_list(label: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(
        view! { <nav class="native-ui gr-chat-conversation-list grid gap-1" aria-label=(label)>(child)</nav> },
    )
}

/// One conversation link. Its active state comes from the current route.
#[component]
pub async fn chat_conversation_item(title: &str, href: &str, active: bool) -> Result<impl View> {
    Ok(view! {
        sidebar::sidebar_menu_button(active: active, href: Some(href), attrs: attributes! { class="native-ui gr-chat-conversation-item" },
            <span>(title)</span>
        )
    })
}
