use topcoat::{
    Result,
    view::{Attributes, Child, View, attributes, component, view},
};

use crate::native_ui::button::{ButtonVariant, button};

/// Suggested starting points for an empty conversation.
#[component]
pub async fn chat_prompts(label: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(
        view! { <div class="gr-chat-prompts grid gap-3 sm:grid-cols-2" role="group" aria-label=(label)>(child)</div> },
    )
}

/// One suggestion. The caller supplies the click handler through `attrs`.
#[component]
pub async fn chat_prompt(title: &str, #[default] attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        button(variant: ButtonVariant::Outline, attrs: attributes! {
            class="native-ui rounded-xl border-[#dbe8f7] bg-white px-4 py-4 text-left text-[#315a85] shadow-sm hover:border-[#91caff] hover:bg-[#f8fbff]"
            type="button"
            (attrs)
        }, (title))
    })
}
