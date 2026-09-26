use topcoat::{
    Result,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::ui::button::{ButtonVariant, button};

/// Suggested starting points for an empty conversation.
#[component]
pub async fn chat_prompts(
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div role="group" aria-label=(label) class=(class!("gr-chat-prompts grid gap-3 sm:grid-cols-2", attrs.remove("class"))) (attrs)>(child)</div> },
    )
}

/// One suggestion. The caller supplies the click handler through `attrs`.
#[component]
pub async fn chat_prompt(title: &str, #[default] mut attrs: Attributes) -> Result<impl View> {
    let button_class = class!(
        "rounded-xl border-[var(--gr-accent-border)] bg-[var(--gr-surface)] px-4 py-4 text-left text-[var(--gr-accent-strong)] shadow-sm hover:border-[var(--gr-accent-border)] hover:bg-[var(--gr-accent-soft)]",
        attrs.remove("class"),
    );
    Ok(view! {
        button(variant: ButtonVariant::Outline, attrs: attributes! {
            class=(button_class)
            type="button"
            (attrs)
        }, (title))
    })
}
