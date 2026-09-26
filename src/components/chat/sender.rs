use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, Signal, signal},
    view::{Attributes, Child, View, attributes, class, component, view},
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
    #[default] busy: Option<&Signal<bool>>,
    #[default] language: UiLanguage,
    #[default] placeholder: Option<&str>,
    #[default] max_length: Option<usize>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let draft = draft.clone();
    let busy = busy.cloned().unwrap_or_else(|| signal(cx, || false));
    let caller_class = attrs.remove("class");
    let root_class = class!("gr-chat-sender", caller_class);
    attrs.extend(attributes! { cx => class=(root_class) });
    let default_placeholder = language.select(
        "Ask a question or describe a task...",
        "提问或描述一个任务...",
    );
    let input_label = placeholder.unwrap_or(default_placeholder);
    let send_label = language.select("Send message", "发送消息");
    let textarea_id = id.to_owned();

    Ok(view! {
        <div (attrs)>
            <form class="gr-chat-sender-form" (submit_attrs)>
                (child)
                <label class="sr-only" for=(id)>(input_label)</label>
                <textarea
                    class="gr-chat-sender-input"
                    id=(id)
                    name="message"
                    rows="2"
                    maxlength=(max_length)
                    placeholder=(input_label)
                    :value=$(draft.get())
                    @input=$(|event: Event| draft.set(event.target.value))
                    @keydown=$(|event: Event| {
                        if event.key == "Enter" {
                            if !event.shift_key {
                                if !event.is_composing {
                                    event.prevent_default();
                                    raw!("document.getElementById(${textarea_id}.dehydrate())?.form?.requestSubmit()", ());
                                }
                            }
                        }
                    })
                ></textarea>
                <div class="gr-chat-sender-actions">
                    <span>(language.select("Enter to send · Shift+Enter for a new line", "回车发送 · Shift+回车换行"))</span>
                    <button class="gr-chat-send-button" type="submit" :disabled=$(if busy.get() { true } else { draft.get().trim().is_empty() }) aria-label=(send_label)>(language.select("Send", "发送"))<span aria-hidden="true">"↗"</span></button>
                </div>
            </form>
        </div>
    })
}
