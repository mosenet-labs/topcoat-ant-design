use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

/// Scrollable conversation region. The caller owns message data and stable IDs.
#[component]
pub async fn chat_message_list(
    cx: &Cx,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let caller_class = attrs.remove("class");
    let root_class = class!("gr-chat-message-list flex flex-col gap-6", caller_class);
    attrs.extend(attributes! { cx =>
        class=(root_class)
        role="log"
        aria-label=(label)
        aria-live="polite"
        aria-relevant="additions text"
    });
    Ok(view! { <div (attrs)>(child)</div> })
}
