use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};
use topcoat_ant_design::{ChatBubbleRole, chat_bubble};

use crate::{app::page_header, demo::component_example, locale::Locale};

const EXAMPLE_SOURCE: &str = r#"use topcoat::view::view;
use topcoat_ant_design::{ChatBubbleRole, chat_bubble};

view! {
    chat_bubble(role: ChatBubbleRole::User, "How do I compose a Chat interface?")
    chat_bubble(role: ChatBubbleRole::Assistant,
        <p>Start with a bubble, message list, and sender.</p>
        <span>Content can include more than plain text.</span>
    )
}"#;

#[page]
pub(in crate::app) async fn bubble_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        page_header(
            eyebrow: "AI COMPONENTS",
            title: "ChatBubble",
            description: locale.select(
                "A single message with role-specific placement and a slot for any message content.",
                "展示单条消息，根据角色排列，并通过内容槽位承载不同类型的消息内容。",
            ),
        )
        component_example(
            id: "chat-bubble-preview",
            title: locale.select("User and assistant bubbles", "用户与助手消息气泡"),
            description: locale.select("Compare the two roles and the content slot.", "对比两种角色布局，并查看内容槽位的用法。"),
            source: EXAMPLE_SOURCE,
            <div class="grid gap-6 bg-[#f8faff] p-6 max-[520px]:p-4">
                chat_bubble(role: ChatBubbleRole::User, language: locale.ui(),
                    (locale.select("How do I compose a Chat interface?", "怎样组合 Chat 聊天界面？"))
                )
                chat_bubble(role: ChatBubbleRole::Assistant, language: locale.ui(),
                    <p class="m-0">(locale.select("Start with a bubble, message list, and sender.", "先从消息气泡、消息列表和输入区开始。"))</p>
                    <span class="mt-2 inline-block rounded-md bg-white/70 px-2 py-1 text-xs text-[#3173bc]">(locale.select("Rich content slot", "丰富内容槽位"))</span>
                )
            </div>
        )
    })
}
