use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, Signal},
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::UiLanguage;

/// Which side of a conversation a message belongs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChatBubbleRole {
    Assistant,
    User,
}

/// A message bubble with caller-provided content.
///
/// Keep message content in the child slot so plain text, Markdown, tool output,
/// and other renderers can share the same conversation layout.
#[component]
pub async fn chat_bubble(
    cx: &Cx,
    role: ChatBubbleRole,
    #[default] language: UiLanguage,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let is_user = role == ChatBubbleRole::User;
    let role_name = if is_user { "user" } else { "assistant" };
    let role_label = if is_user {
        language.select("You", "你")
    } else {
        language.select("Assistant", "助手")
    };
    let caller_class = attrs.remove("class");
    let root_class = class!(
        "gr-chat-bubble flex items-start gap-3",
        "gr-chat-bubble-user flex-row-reverse" if is_user,
        caller_class,
    );
    attrs.extend(attributes! { cx =>
        class=(root_class)
        data-role=(role_name)
        aria-label=(role_label)
    });

    Ok(view! {
        <article (attrs)>
            <span class="gr-chat-avatar grid size-8 shrink-0 place-items-center rounded-[10px] text-[11px] font-bold" aria-hidden="true">(if is_user { "U" } else { "AI" })</span>
            <div class="min-w-0 max-w-[min(76ch,88%)]">
                <p class="gr-chat-role m-0 mb-1 text-[11px] font-semibold tracking-[0.08em] text-[#8c8c8c]">(role_label)</p>
                <div class="gr-chat-content rounded-[14px] px-4 py-3 text-sm leading-6">(child)</div>
            </div>
        </article>
    })
}

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
