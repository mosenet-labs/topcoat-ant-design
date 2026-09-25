use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, Signal},
    view::{Attributes, View, attributes, class, component, view},
};

use crate::UiLanguage;

/// Chat input with a browser-owned draft and caller-owned submit behavior.
///
/// Attach an `@submit` handler to `submit_attrs` and call `prevent_default()`
/// before submitting through a Topcoat procedure or another host integration.
#[component]
pub async fn chat_sender(
    cx: &Cx,
    id: &str,
    draft: &Signal<String>,
    submit_attrs: Attributes,
    #[default] language: UiLanguage,
    #[default] placeholder: Option<&str>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let draft = draft.clone();
    let caller_class = attrs.remove("class");
    let root_class = class!("gr-chat-sender", caller_class);
    attrs.extend(attributes! { cx => class=(root_class) });
    let default_placeholder = language.select(
        "Ask a question or describe a task...",
        "提问或描述一个任务...",
    );
    let input_label = placeholder.unwrap_or(default_placeholder);
    let send_label = language.select("Send message", "发送消息");

    Ok(view! {
        <div (attrs)>
            <form class="gr-chat-sender-form" (submit_attrs)>
                <label class="sr-only" for=(id)>(input_label)</label>
                <textarea
                    class="gr-chat-sender-input"
                    id=(id)
                    name="message"
                    rows="2"
                    placeholder=(input_label)
                    :value=$(draft.get())
                    @input=$(|event: Event| draft.set(event.target.value))
                ></textarea>
                <div class="gr-chat-sender-actions">
                    <span>(language.select("Compose a message", "编写消息"))</span>
                    <button class="gr-chat-send-button" type="submit" :disabled=$(draft.get().trim().is_empty()) aria-label=(send_label)>(language.select("Send", "发送"))<span aria-hidden="true">"↗"</span></button>
                </div>
            </form>
        </div>
    })
}
