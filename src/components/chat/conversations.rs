use topcoat::{
    Result,
    view::{Child, View, class, component, view},
};

/// Navigation region for host-provided conversation routes.
#[component]
pub async fn chat_conversation_list(label: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! { <nav class="gr-chat-conversation-list grid gap-1" aria-label=(label)>(child)</nav> })
}

/// One conversation link. Its active state comes from the current route.
#[component]
pub async fn chat_conversation_item(title: &str, href: &str, active: bool) -> Result<impl View> {
    let classes = class!(
        "gr-chat-conversation-item rounded-lg border px-3 py-3 text-sm font-medium no-underline transition-colors focus-visible:outline-2 focus-visible:outline-[#1677ff]",
        "border-[#d7e8ff] bg-[#ecf5ff] text-[#0958d9]" if active,
        "border-transparent text-[#526174] hover:bg-[#f3f7fc]" if !active,
    );
    Ok(view! {
        <a class=(classes) href=(href) aria-current=(active.then_some("page"))>
            (title)
        </a>
    })
}
