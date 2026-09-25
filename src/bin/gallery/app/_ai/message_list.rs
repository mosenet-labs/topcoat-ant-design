use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};
use topcoat_ant_design::{ChatBubbleRole, chat_bubble, chat_message_list};

use crate::{app::page_header, demo::component_example, locale::Locale};

const EXAMPLE_SOURCE: &str = r#"use topcoat::view::view;
use topcoat_ant_design::{ChatBubbleRole, chat_bubble, chat_message_list};

view! {
    chat_message_list(label: "Conversation messages",
        chat_bubble(role: ChatBubbleRole::User, "What does the list own?")
        chat_bubble(role: ChatBubbleRole::Assistant,
            "It arranges messages. The page owns conversation data."
        )
    )
}"#;

#[page]
pub(in crate::app) async fn message_list_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        page_header(
            eyebrow: "AI COMPONENTS",
            title: "ChatMessageList",
            description: locale.select(
                "A labelled conversation region that arranges messages and announces updates to assistive technology.",
                "带标签的会话区域，负责排列消息，并向辅助技术播报更新。",
            ),
        )
        component_example(
            id: "chat-message-list-preview",
            title: locale.select("Conversation messages", "会话消息"),
            description: locale.select("The caller provides the messages and keeps their data.", "调用方提供消息并管理会话数据。"),
            source: EXAMPLE_SOURCE,
            <div class="bg-[#f8faff] p-6 max-[520px]:p-4">
                chat_message_list(label: locale.select("Conversation messages", "会话消息"),
                    chat_bubble(role: ChatBubbleRole::User, language: locale.ui(),
                        (locale.select("What does the list own?", "消息列表负责什么？"))
                    )
                    chat_bubble(role: ChatBubbleRole::Assistant, language: locale.ui(),
                        (locale.select("It arranges messages. The page owns conversation data.", "它负责排列消息；会话数据由页面管理。"))
                    )
                )
            </div>
        )
    })
}
