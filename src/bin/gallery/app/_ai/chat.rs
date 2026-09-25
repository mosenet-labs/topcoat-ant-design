use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::{Event, signal},
    view::{View, attributes, view},
};
use topcoat_ant_design::{ChatBubbleRole, chat_bubble, chat_message_list, chat_sender};

use crate::{app::page_header, demo::component_example, locale::Locale};

const EXAMPLE_SOURCE: &str = r#"use topcoat::{runtime::{Event, signal}, view::{attributes, view}};
use topcoat_ant_design::{ChatBubbleRole, chat_bubble, chat_message_list, chat_sender};

let draft = signal(cx, String::new);
let submitted = signal(cx, String::new);
let submit = attributes! { cx =>
    @submit=$(|event: Event| {
        event.prevent_default();
        if !draft.get().trim().is_empty() {
            submitted.set(draft.get());
            draft.set("".to_owned());
        }
    })
};

view! {
    chat_message_list(label: "Conversation",
        chat_bubble(role: ChatBubbleRole::Assistant, "How can I help?")
        chat_bubble(role: ChatBubbleRole::User, $(submitted.get()))
    )
    chat_sender(id: "chat-draft", draft: &draft, submit_attrs: submit)
}"#;

#[page]
pub(in crate::app) async fn chat_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let draft = signal(cx, String::new);
    let submitted = signal(cx, String::new);
    let submit = attributes! { cx =>
        @submit=$(|event: Event| {
            event.prevent_default();
            if !draft.get().trim().is_empty() {
                submitted.set(draft.get());
                draft.set("".to_owned());
            }
        })
    };
    let latest_message = attributes! { cx =>
        :hidden=$(submitted.get().is_empty())
        id="gallery-chat-latest-message"
    };

    Ok(view! {
        page_header(
            eyebrow: "AI COMPONENTS",
            title: locale.select("Chat interface", "Chat 聊天界面"),
            description: locale.select(
                "A first composition of Bubble, Message List, and Sender. Type a message to see their browser-side interaction.",
                "用消息气泡、消息列表和输入区组合聊天界面。输入并发送一条消息，体验浏览器本地交互。",
            ),
        )
        <div class="grid gap-6">
            component_example(
                id: "chat-preview",
                title: locale.select("Chat composition", "聊天界面组合"),
                description: locale.select("This preview keeps the draft in the browser and does not contact a model.", "此预览在浏览器保存草稿，不会请求模型。"),
                source: EXAMPLE_SOURCE,
                <div class="grid min-h-[630px] grid-cols-[210px_minmax(0,1fr)] bg-[#f8faff] max-[720px]:grid-cols-1">
                    <aside class="flex flex-col border-r border-[#e7edf6] bg-[#fbfcff] p-4 max-[720px]:border-b max-[720px]:border-r-0" aria-label=(locale.select("Conversation example", "会话示例"))>
                        <div class="mb-5 flex items-center gap-2 px-2 py-1">
                            <span class="grid size-8 place-items-center rounded-[9px] bg-[#1677ff] text-[11px] font-bold text-white">"AI"</span>
                            <span class="text-sm font-bold tracking-[-0.02em] text-[#1f3451]">"Topcoat X"</span>
                        </div>
                        <p class="mb-2 mt-0 px-2 text-[10px] font-bold uppercase tracking-[0.13em] text-[#9aa6b6]">(locale.select("Workspace", "工作区"))</p>
                        <div class="rounded-lg border border-[#d7e8ff] bg-[#ecf5ff] px-3 py-3 text-[12px] font-semibold text-[#0958d9]" aria-current="page">(locale.select("Designing a Chat interface", "设计 Chat 界面"))</div>
                        <div class="mt-auto border-t border-[#e8edf5] px-2 pt-4 text-[11px] leading-5 text-[#8b98aa] max-[720px]:hidden">(locale.select("A component composition built with Topcoat.", "使用 Topcoat 构建的组件组合。"))</div>
                    </aside>
                    <section class="flex min-h-[630px] min-w-0 flex-col max-[720px]:min-h-[560px]" aria-label=(locale.select("Chat preview", "聊天预览"))>
                        <header class="flex min-h-[72px] items-center justify-between gap-4 border-b border-[#e7edf6] bg-white/90 px-6 max-[520px]:px-4">
                            <div><p class="m-0 text-[11px] font-bold uppercase tracking-[0.12em] text-[#1677ff]">"CHAT / 01"</p><h2 class="m-0 mt-1 text-base font-semibold text-[#233449]">(locale.select("Designing a Chat interface", "设计 Chat 界面"))</h2></div>
                            <span class="rounded-full border border-[#dbe7f6] bg-[#f8faff] px-3 py-1 text-[11px] font-medium text-[#6a7b91]">(locale.select("Local preview", "本地预览"))</span>
                        </header>
                        <div class="min-h-0 flex-1 overflow-y-auto px-6 py-7 max-[520px]:px-4">
                            chat_message_list(label: locale.select("Conversation messages", "会话消息"),
                                chat_bubble(role: ChatBubbleRole::User, language: locale.ui(),
                                    (locale.select("How should we split a Chat interface into reusable components?", "Chat 聊天界面应该怎样拆分为可复用组件？"))
                                )
                                chat_bubble(role: ChatBubbleRole::Assistant, language: locale.ui(),
                                    <p class="m-0">(locale.select("Start with three pieces: a message bubble, a message list, and a sender. The page owns the conversation data and composes them.", "先从消息气泡、消息列表和输入区三个部分开始。页面持有会话数据，并负责组合这些组件。"))</p>
                                    <div class="mt-3 flex flex-wrap gap-2"><span class="rounded-md border border-[#d9e9ff] bg-white px-2 py-0.5 text-[11px] text-[#3173bc]">"Bubble"</span><span class="rounded-md border border-[#d9e9ff] bg-white px-2 py-0.5 text-[11px] text-[#3173bc]">"Message List"</span><span class="rounded-md border border-[#d9e9ff] bg-white px-2 py-0.5 text-[11px] text-[#3173bc]">"Sender"</span></div>
                                )
                                chat_bubble(role: ChatBubbleRole::User, language: locale.ui(), attrs: latest_message,
                                    $(submitted.get())
                                )
                            )
                        </div>
                        <div class="border-t border-[#e7edf6] bg-white/80 px-5 pb-5 pt-4 max-[520px]:px-4">
                            chat_sender(id: "gallery-chat-draft", draft: &draft, submit_attrs: submit, language: locale.ui())
                            <p class="mb-0 mt-2.5 text-center text-[11px] text-[#9ba7b6]">(locale.select("UI preview · messages stay on this page", "UI 预览 · 消息只保留在当前页面"))</p>
                        </div>
                    </section>
                </div>
            )
            <section class="grid grid-cols-3 gap-4 max-[720px]:grid-cols-1" aria-label=(locale.select("Component boundaries", "组件职责"))>
                <article class="rounded-xl border border-[#e8eaee] bg-white p-5"><span class="text-[11px] font-bold tracking-[0.1em] text-[#1677ff]">"01 / BUBBLE"</span><h2 class="mb-2 mt-3 text-base font-semibold">"ChatBubble"</h2><p class="m-0 text-sm leading-6 text-[#667085]">(locale.select("Role placement and a content slot for text or richer output.", "负责角色位置和内容槽位，可放入文本或更丰富的输出。"))</p></article>
                <article class="rounded-xl border border-[#e8eaee] bg-white p-5"><span class="text-[11px] font-bold tracking-[0.1em] text-[#1677ff]">"02 / LIST"</span><h2 class="mb-2 mt-3 text-base font-semibold">"ChatMessageList"</h2><p class="m-0 text-sm leading-6 text-[#667085]">(locale.select("A labelled live region that arranges messages without owning their data.", "排列消息并提供可访问的实时区域，会话数据仍由页面持有。"))</p></article>
                <article class="rounded-xl border border-[#e8eaee] bg-white p-5"><span class="text-[11px] font-bold tracking-[0.1em] text-[#1677ff]">"03 / SENDER"</span><h2 class="mb-2 mt-3 text-base font-semibold">"ChatSender"</h2><p class="m-0 text-sm leading-6 text-[#667085]">(locale.select("The draft stays reactive in the browser; the host supplies submit behavior.", "草稿在浏览器内响应式更新；发送行为由宿主提供。"))</p></article>
            </section>
        </div>
    })
}
