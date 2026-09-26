use std::sync::Mutex;

use tokio::sync::broadcast;
use topcoat::{
    Result,
    context::{Cx, app_context},
    router::page,
    runtime::{Event, connected, procedure, shard, signal},
    view::{View, attributes, emit, live, view},
};
use topcoat_ant_design::{
    ChatBubbleRole, ChatMessage, ChatMessageStatus, chat_bubble, chat_message_list, chat_sender,
};

use crate::{app::page_header, demo::component_example, locale::Locale};

const MAX_MESSAGES: usize = 100;
const MAX_MESSAGE_BYTES: usize = 8_192;

#[derive(Default)]
struct RoomMessages {
    next_id: u64,
    items: Vec<ChatMessage>,
}

/// Shared only by this Gallery process. A real host supplies its own room,
/// authorization, persistence, and model response in this same boundary.
pub(in crate::app) struct LiveChatRoom {
    messages: Mutex<RoomMessages>,
    changed: broadcast::Sender<()>,
}

impl Default for LiveChatRoom {
    fn default() -> Self {
        Self {
            messages: Mutex::new(RoomMessages::default()),
            changed: broadcast::channel(32).0,
        }
    }
}

impl LiveChatRoom {
    fn subscribe(&self) -> broadcast::Receiver<()> {
        self.changed.subscribe()
    }

    fn messages(&self) -> Vec<ChatMessage> {
        self.messages
            .lock()
            .expect("live chat room mutex")
            .items
            .clone()
    }

    fn send(&self, message: &str, language: &str) -> std::io::Result<()> {
        let message = message.trim();
        if message.is_empty() || message.len() > MAX_MESSAGE_BYTES {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "A non-empty message of at most 8192 bytes is required",
            ));
        }

        {
            let mut state = self.messages.lock().expect("live chat room mutex");
            for (role, content) in [
                (ChatBubbleRole::User, message.to_owned()),
                (
                    ChatBubbleRole::Assistant,
                    if language == "zh-CN" {
                        "已收到。这条会话消息会同步到所有打开的标签页。".to_owned()
                    } else {
                        "Received. This conversation updates in every open tab.".to_owned()
                    },
                ),
            ] {
                state.next_id += 1;
                let id = format!("live-{}", state.next_id);
                state.items.push(ChatMessage::new(
                    id,
                    role,
                    ChatMessageStatus::Complete,
                    content,
                ));
            }
            let overflow = state.items.len().saturating_sub(MAX_MESSAGES);
            if overflow > 0 {
                state.items.drain(..overflow);
            }
        }
        let _ = self.changed.send(());
        Ok(())
    }
}

#[procedure("/_gallery/chat/live/send")]
pub(in crate::app) async fn send_live_message(
    cx: &Cx,
    message: String,
    language: String,
) -> std::io::Result<bool> {
    app_context::<LiveChatRoom>(cx).send(&message, &language)?;
    Ok(true)
}

/// Subscribe before the first read so changes between rendering and waiting
/// cannot be lost. The initial HTTP render finishes without a WebSocket;
/// connected browsers keep receiving updates through Topcoat's live shard.
#[shard("/_gallery/chat/live/messages")]
pub(in crate::app) async fn live_message_region(cx: &Cx, language: String) -> Result<impl View> {
    Ok(live! {
        let room = app_context::<LiveChatRoom>(cx);
        let mut changed = room.subscribe();
        let locale = if language == "zh-CN" { Locale::Zh } else { Locale::En };
        loop {
            let messages = room.messages();
            let token = emit! {
                chat_message_list(label: locale.select("Shared conversation messages", "共享会话消息"),
                    if messages.is_empty() {
                        <p class="m-0 rounded-xl border border-dashed border-border bg-card px-5 py-8 text-center text-sm text-muted-foreground">(locale.select("Send a message to see it appear in every open tab.", "发送消息后，所有打开的标签页都会看到更新。"))</p>
                    }
                    #[key(message.id.clone())]
                    for message in messages {
                        chat_bubble(role: message.role, status: Some(message.status), language: locale.ui(),
                            <p class="m-0 whitespace-pre-wrap">(message.content)</p>
                        )
                    }
                )
            }?;
            if !connected(cx) {
                break Ok(token);
            }
            changed.recv().await.ok();
        }
    })
}

const EXAMPLE_SOURCE: &str = r#"// The host owns shared messages and notifies subscribers.
#[shard]
async fn message_region(cx: &Cx) -> Result<impl View> {
    Ok(live! {
        let chat = app_context::<ChatRoom>(cx);
        let mut changed = chat.subscribe();
        loop {
            let token = emit! {
                chat_message_list(label: "Messages",
                    #[key(message.id.clone())]
                    for message in chat.messages() {
                        chat_bubble(role: message.role, (message.content))
                    }
                )
            }?;
            if !connected(cx) { break Ok(token); }
            changed.recv().await.ok();
        }
    })
}
// chat_sender submits through a #[procedure] that stores the message
// and notifies every subscriber. The host registers ChatRoom with
// RouterBuilder::app_context and enables .runtime()."#;

#[page]
pub(in crate::app) async fn chat_live_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let draft = signal(cx, String::new);
    let language = locale.html_lang().to_owned();
    let submit = attributes! { cx =>
        @submit=$(async |event: Event| {
            event.prevent_default();
            let message = draft.get();
            if !message.trim().is_empty() {
                let _sent = send_live_message(message, language.clone()).await;
                draft.set("".to_owned());
            }
        })
    };

    Ok(view! {
        page_header(
            eyebrow: "AI COMPONENTS",
            title: locale.select("Live Chat", "实时 Chat"),
            description: locale.select(
                "Topcoat 0.9 server push composed with the existing Chat components. Open this page in two tabs to see both update.",
                "用现有 Chat 组件组合 Topcoat 0.9 的服务端推送。在两个标签页打开此页面即可观察同步更新。",
            ),
        )
        component_example(
            id: "chat-live-preview",
            title: locale.select("Shared demo room", "共享示例会话"),
            description: locale.select("Messages are shared with everyone viewing this Gallery process and are cleared when it restarts. Do not enter private information.", "消息会与查看此 Gallery 进程的所有人共享，重启后清空。请勿输入隐私信息。"),
            source: EXAMPLE_SOURCE,
            <div class="flex min-h-[420px] flex-col bg-background">
                <div class="min-h-0 flex-1 px-6 py-7 max-[520px]:px-4">
                    live_message_region(language: locale.html_lang().to_owned())
                </div>
                <div class="border-t border-border bg-card/80 px-5 pb-5 pt-4 max-[520px]:px-4">
                    chat_sender(id: "gallery-live-chat-draft", draft: &draft, submit_attrs: submit, max_length: Some(2000), language: locale.ui())
                </div>
            </div>
        )
    })
}

#[cfg(test)]
mod tests {
    use super::LiveChatRoom;

    #[test]
    fn shared_room_notifies_subscribers_and_rejects_invalid_messages() {
        let room = LiveChatRoom::default();
        let mut subscriber = room.subscribe();
        assert!(room.send("  Hello  ", "en").is_ok());
        assert!(subscriber.try_recv().is_ok());
        let messages = room.messages();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].content, "Hello");
        assert_ne!(messages[0].id, messages[1].id);
        assert!(room.send(" ", "en").is_err());
        assert!(room.send(&"x".repeat(8_193), "en").is_err());
        assert!(subscriber.try_recv().is_err());
    }
}
