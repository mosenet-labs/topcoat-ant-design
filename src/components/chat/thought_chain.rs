use topcoat::{
    Result,
    view::{Child, View, component, view},
};

use crate::UiLanguage;

/// Visual state of one process step.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChatThoughtStatus {
    Loading,
    Success,
    Error,
    Abort,
}

impl ChatThoughtStatus {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Loading => "loading",
            Self::Success => "success",
            Self::Error => "error",
            Self::Abort => "abort",
        }
    }

    const fn label(self, language: UiLanguage) -> &'static str {
        match self {
            Self::Loading => language.select("In progress", "进行中"),
            Self::Success => language.select("Completed", "已完成"),
            Self::Error => language.select("Failed", "失败"),
            Self::Abort => language.select("Stopped", "已中止"),
        }
    }
}

/// Ordered steps in a user-visible process summary.
#[component]
pub async fn chat_thought_chain(label: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! { <ol class="gr-chat-thought-chain" aria-label=(label)>(child)</ol> })
}

/// A single step inside `chat_thought_chain`.
#[component]
pub async fn chat_thought_step(
    title: &str,
    #[default] status: Option<ChatThoughtStatus>,
    #[default] language: UiLanguage,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let state = status.map_or("default", ChatThoughtStatus::as_str);
    Ok(view! {
        <li class="gr-chat-thought-step" data-status=(state)>
            <span class="gr-chat-thought-step-marker" aria-hidden="true">
                if status.is_none() { <span class="gr-chat-thought-step-number"></span> }
                else if status == Some(ChatThoughtStatus::Loading) { <span class="gr-chat-thought-step-spinner"></span> }
                else if status == Some(ChatThoughtStatus::Success) { "✓" }
                else if status == Some(ChatThoughtStatus::Error) { "×" }
                else { "−" }
            </span>
            <div class="gr-chat-thought-step-body">
                if let Some(status) = status { <span class="sr-only">(status.label(language)) " · "</span> }
                <strong class="gr-chat-thought-step-title">(title)</strong>
                <div class="gr-chat-thought-step-description">(child)</div>
            </div>
        </li>
    })
}
