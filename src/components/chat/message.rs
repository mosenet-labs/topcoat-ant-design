use serde::{Deserialize, Serialize};

use super::ChatBubbleRole;
use crate::UiLanguage;

/// Rendering state of a Chat message. The host owns transitions between states.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatMessageStatus {
    Sending,
    Streaming,
    Complete,
    Failed,
    Cancelled,
}

impl ChatMessageStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sending => "sending",
            Self::Streaming => "streaming",
            Self::Complete => "complete",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn label(self, language: UiLanguage) -> &'static str {
        match self {
            Self::Sending => language.select("Sending", "发送中"),
            Self::Streaming => language.select("Streaming", "生成中"),
            Self::Complete => language.select("Complete", "已完成"),
            Self::Failed => language.select("Failed", "失败"),
            Self::Cancelled => language.select("Cancelled", "已取消"),
        }
    }
}

/// Minimal message contract shared by the Gallery and host integrations.
///
/// IDs remain stable while content and status change. Host applications own
/// storage, authentication, and the model request that produces content.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: ChatBubbleRole,
    pub status: ChatMessageStatus,
    pub content: String,
}

impl ChatMessage {
    pub fn new(
        id: impl Into<String>,
        role: ChatBubbleRole,
        status: ChatMessageStatus,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            role,
            status,
            content: content.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ChatBubbleRole, ChatMessage, ChatMessageStatus};

    #[test]
    fn message_contract_round_trips_with_stable_role_and_status_names() {
        let message = ChatMessage::new(
            "turn-42",
            ChatBubbleRole::Assistant,
            ChatMessageStatus::Streaming,
            "partial",
        );
        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("\"id\":\"turn-42\""));
        assert!(json.contains("\"role\":\"assistant\""));
        assert!(json.contains("\"status\":\"streaming\""));
        assert_eq!(serde_json::from_str::<ChatMessage>(&json).unwrap(), message);
    }
}
