use topcoat::{
    Result,
    runtime::Expr,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::native_ui::sidebar;

/// Navigation region for host-provided conversation routes.
#[component]
pub async fn chat_conversation_list(
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <nav aria-label=(label) class=(class!("native-ui gr-chat-conversation-list grid gap-1", attrs.remove("class"))) (attrs)>(child)</nav> },
    )
}

/// One conversation link. Its active state comes from the current route.
#[component]
pub async fn chat_conversation_item(
    title: &str,
    href: &str,
    #[into] active: Expr<bool>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let item_class = class!("native-ui gr-chat-conversation-item", attrs.remove("class"));
    Ok(view! {
        sidebar::sidebar_menu_button(active: active, href: Some(href), attrs: attributes! { class=(item_class) (attrs) },
            <span>(title)</span>
        )
    })
}
