use serde::{Deserialize, Serialize};
use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::message::ChatMessageStatus;
use crate::UiLanguage;

/// Which side of a conversation a message belongs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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
    #[default] status: Option<ChatMessageStatus>,
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
    if let Some(status) = status {
        attrs.extend(attributes! { cx =>
            data-status=(status.as_str())
            aria-busy=(matches!(status, ChatMessageStatus::Sending | ChatMessageStatus::Streaming))
        });
    }

    Ok(view! {
        <article (attrs)>
            <span class="gr-chat-avatar grid size-8 shrink-0 place-items-center rounded-[10px] text-[11px] font-bold" aria-hidden="true">(if is_user { "U" } else { "AI" })</span>
            <div class="min-w-0 max-w-[min(76ch,88%)]">
                <p class="gr-chat-role m-0 mb-1 text-[11px] font-semibold tracking-[0.08em] text-[#8c8c8c]">(role_label)</p>
                <div class="gr-chat-content rounded-[14px] px-4 py-3 text-sm leading-6">(child)</div>
                if let Some(status) = status {
                    <p class="mb-0 mt-1.5 text-[11px] font-medium text-[#6b7c91]" role="status">(status.label(language))</p>
                }
            </div>
        </article>
    })
}
