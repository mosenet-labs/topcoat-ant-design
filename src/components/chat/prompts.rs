use topcoat::{
    Result,
    view::{Attributes, Child, View, component, view},
};

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
    Ok(
        view! { <button class="rounded-xl border border-[#dbe8f7] bg-white px-4 py-4 text-left text-sm font-medium text-[#315a85] shadow-sm hover:border-[#91caff] hover:bg-[#f8fbff] focus-visible:outline-2 focus-visible:outline-[#1677ff]" type="button" (attrs)>(title)</button> },
    )
}
