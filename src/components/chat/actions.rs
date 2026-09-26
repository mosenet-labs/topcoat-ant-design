use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// Host-owned actions such as copy, retry, and cancel beneath a message.
#[component]
pub async fn chat_actions(
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div role="group" aria-label=(label) class=(class!("gr-chat-actions flex flex-wrap gap-2 pt-2", attrs.remove("class"))) (attrs)>(child)</div> },
    )
}
