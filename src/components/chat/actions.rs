use topcoat::{
    Result,
    view::{Child, View, component, view},
};

/// Host-owned actions such as copy, retry, and cancel beneath a message.
#[component]
pub async fn chat_actions(label: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(
        view! { <div class="gr-chat-actions flex flex-wrap gap-2 pt-2" role="group" aria-label=(label)>(child)</div> },
    )
}
