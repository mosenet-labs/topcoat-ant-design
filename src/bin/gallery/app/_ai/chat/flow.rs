use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    runtime::{Event, Signal, procedure, shard, signal},
    view::{View, attributes, component, view},
};
use topcoat_ant_design::{
    ChatBubbleRole, ChatMessage, ChatMessageStatus, chat_actions, chat_attachment_tray,
    chat_bubble, chat_conversation_item, chat_conversation_list, chat_file, chat_markdown,
    chat_message_list, chat_prompt, chat_prompts, chat_sender, chat_source, chat_sources,
    chat_think, chat_thought_chain, chat_thought_step, icons::COPY_OUTLINED,
};

use crate::{app::page_header, demo::component_example, locale::Locale};

/// Gallery-only stand-in for a host procedure. It validates browser input and
/// returns deterministic text; applications replace this with their own model.
#[procedure("/_gallery/chat/reply")]
pub(in crate::app) async fn gallery_reply(
    prompt: String,
    language: String,
) -> std::result::Result<String, std::io::Error> {
    let prompt = prompt.trim();
    if prompt.is_empty() || prompt.len() > 8_192 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "A non-empty prompt of at most 8192 bytes is required",
        ));
    }
    Ok(if language == "zh-CN" {
        format!("示例服务已收到你的问题：{prompt}")
    } else {
        format!("The demo service received your question: {prompt}")
    })
}

/// Acknowledges Gallery cancel/retry controls through the same boundary a
/// host would use to cancel a stream or retry a request.
#[procedure("/_gallery/chat/action")]
pub(in crate::app) async fn gallery_action(
    action: String,
    message_id: String,
) -> std::result::Result<String, std::io::Error> {
    if !matches!(action.as_str(), "cancel" | "retry")
        || message_id.is_empty()
        || message_id.len() > 100
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid Chat action or message ID",
        ));
    }
    Ok(action)
}

const EXAMPLE_SOURCE: &str = r#"// The page owns a JSON-encoded Vec<ChatMessage> signal.
// A Topcoat shard parses and renders the current messages. Each message
// has a stable ID, role, status, and content. The host replaces the
// Gallery's local response controls with a procedure and stream.
let messages = signal(cx, || serde_json::to_string(&initial_messages).unwrap());
let draft = signal(cx, String::new);
view! {
    message_region(messages: $(messages), draft: $(draft), language: "en".to_owned())
    chat_sender(id: "chat-draft", draft: &draft, submit_attrs: submit)
}"#;

fn initial_messages(session: &str, locale: Locale) -> Vec<ChatMessage> {
    match session {
        "notes" => vec![
            ChatMessage::new("notes-user", ChatBubbleRole::User, ChatMessageStatus::Complete, locale.select("Summarize the component boundaries.", "总结一下组件边界。")),
            ChatMessage::new("notes-assistant", ChatBubbleRole::Assistant, ChatMessageStatus::Complete, locale.select("**Components** render the interface. The **page** owns conversation data, requests, and routing.", "**组件**负责界面呈现，**页面**负责会话数据、请求和路由。")),
        ],
        "new" => Vec::new(),
        _ => vec![
            ChatMessage::new("design-user", ChatBubbleRole::User, ChatMessageStatus::Complete, locale.select("How should we split a Chat interface?", "Chat 界面应该怎样拆分？")),
            ChatMessage::new("design-assistant", ChatBubbleRole::Assistant, ChatMessageStatus::Complete, locale.select("Start with **Bubble**, **MessageList**, and **Sender**. The page composes them and owns the conversation.", "先从 **Bubble**、**MessageList** 和 **Sender** 开始。页面组合它们并管理会话。")),
        ],
    }
}

fn parse_messages(raw: &str) -> Vec<ChatMessage> {
    if raw.len() > 131_072 {
        return Vec::new();
    }
    let mut seen_ids = std::collections::HashSet::new();
    serde_json::from_str::<Vec<ChatMessage>>(raw)
        .unwrap_or_default()
        .into_iter()
        .take(100)
        .filter(|message| {
            !message.id.is_empty()
                && message.id.len() <= 100
                && message.content.len() <= 8_192
                && seen_ids.insert(message.id.clone())
        })
        .collect()
}

#[component]
pub(super) async fn chat_flow(cx: &Cx, session: &str) -> Result<impl View> {
    let locale = Locale::current(cx);
    let initial = serde_json::to_string(&initial_messages(session, locale))?;
    let messages = signal(cx, || initial);
    let draft = signal(cx, String::new);
    let busy = signal(cx, || false);
    let attached = signal(cx, || false);
    let language = locale.html_lang().to_owned();
    let design_url = locale.link("/chat");
    let notes_url = locale.link("/chat/notes");
    let new_url = locale.link("/chat/new");
    let live_url = locale.link("/chat/live");
    let submit = attributes! { cx =>
        @submit=$(|event: Event| {
            event.prevent_default();
            let text = draft.get();
            if !text.trim().is_empty() {
                raw!(r#"(() => {
                    const items = JSON.parse(${messages}.get().dehydrate());
                    const text = ${text}.dehydrate().trim();
                    items.push({ id: crypto.randomUUID(), role: 'user', status: 'complete', content: text });
                    items.push({ id: crypto.randomUUID(), role: 'assistant', status: 'sending', content: '' });
                    ${messages}.set(cx.hydrate(JSON.stringify(items)));
                    const pane = document.getElementById('gallery-chat-scroll');
                    if (pane) {
                        const observer = new MutationObserver(() => { pane.scrollTop = pane.scrollHeight; observer.disconnect(); });
                        observer.observe(pane, { childList: true, subtree: true });
                        setTimeout(() => observer.disconnect(), 1500);
                    }
                    document.getElementById('gallery-chat-draft')?.focus();
                })();"#, ());
                draft.set("".to_owned());
                busy.set(true);
            }
        })
    };

    Ok(view! {
        page_header(
            eyebrow: "AI COMPONENTS",
            title: locale.select("Chat interface", "Chat 聊天界面"),
            description: locale.select(
                "Compose conversations, message states, and reusable content components. This is a local interaction preview.",
                "组合会话、消息状态与可复用内容组件。这里是浏览器本地交互预览。",
            ),
        )
        component_example(
            id: "chat-preview",
            title: locale.select("Chat composition", "聊天界面组合"),
            description: locale.select("Send several messages, then advance, finish, fail, cancel, or retry a local reply.", "连续发送消息，再通过本地按钮推进、完成、失败、取消或重试回复。"),
            source: EXAMPLE_SOURCE,
            <div class="grid min-h-[650px] grid-cols-[210px_minmax(0,1fr)] bg-background max-[720px]:grid-cols-1">
                <aside class="flex flex-col border-r border-border bg-background p-4 max-[720px]:border-b max-[720px]:border-r-0" aria-label=(locale.select("Conversations", "会话"))>
                    <div class="mb-5 flex items-center gap-2 px-2 py-1"><span class="grid size-8 place-items-center rounded-[9px] bg-primary text-[11px] font-bold text-primary-foreground">"AI"</span><span class="text-sm font-bold text-foreground">"Topcoat X"</span></div>
                    <p class="mb-2 mt-0 px-2 text-[10px] font-bold uppercase tracking-[0.13em] text-muted-foreground">(locale.select("Workspace", "工作区"))</p>
                    chat_conversation_list(label: locale.select("Demo conversations", "示例会话"),
                        chat_conversation_item(title: locale.select("Designing a Chat interface", "设计 Chat 界面"), href: design_url.as_str(), active: session == "design")
                        chat_conversation_item(title: locale.select("Component boundaries", "组件边界"), href: notes_url.as_str(), active: session == "notes")
                        chat_conversation_item(title: locale.select("New conversation", "新建会话"), href: new_url.as_str(), active: session == "new")
                        chat_conversation_item(title: locale.select("Live shared room", "实时共享会话"), href: live_url.as_str(), active: false)
                    )
                    <p class="mt-auto border-t border-border px-2 pt-4 text-[11px] leading-5 text-muted-foreground max-[720px]:hidden">(locale.select("Routes select sample conversations. Messages on this page stay in browser memory.", "路由切换示例会话；本页新消息仅保留在浏览器内存中。"))</p>
                </aside>
                <section class="flex min-h-[650px] min-w-0 flex-col max-[720px]:min-h-[560px]" aria-label=(locale.select("Chat preview", "聊天预览"))>
                    <header class="flex min-h-[70px] items-center justify-between gap-4 border-b border-border bg-card/90 px-6 max-[520px]:px-4">
                        <div><p class="m-0 text-[11px] font-bold uppercase tracking-[0.12em] text-primary">"CHAT / 01"</p><h2 class="m-0 mt-1 text-base font-semibold text-foreground">(match session { "notes" => locale.select("Component boundaries", "组件边界"), "new" => locale.select("New conversation", "新建会话"), _ => locale.select("Designing a Chat interface", "设计 Chat 界面") })</h2></div>
                        <span class="rounded-full border border-border bg-background px-3 py-1 text-[11px] font-medium text-muted-foreground">(locale.select("Local preview", "本地预览"))</span>
                    </header>
                    <div class="min-h-0 flex-1 overflow-y-auto px-6 py-7 max-[520px]:px-4" id="gallery-chat-scroll">
                        message_region(messages: $(messages), draft: $(draft), busy: $(busy), language: $(language.clone()))
                    </div>
                    <div class="border-t border-border bg-card/80 px-5 pb-5 pt-4 max-[520px]:px-4">
                        chat_sender(id: "gallery-chat-draft", draft: &draft, submit_attrs: submit, busy: Some(&busy), max_length: Some(2000), language: locale.ui(),
                            <div class="flex items-center gap-2 px-4 pt-3">
                                <button type="button" class="rounded-md border border-border bg-background px-2 py-1 text-xs text-primary" @click=$(|_e| attached.toggle())>(locale.select("Attach sample file", "添加示例文件"))</button>
                                <div :hidden=$(!attached.get())>
                                    chat_attachment_tray(label: locale.select("Selected attachments", "已选附件"),
                                        chat_file(name: "example.txt", detail: locale.select("Local example · no upload", "本地示例 · 未上传"),
                                            <button type="button" class="text-xs text-[var(--gr-error)]" aria-label=(locale.select("Remove example file", "移除示例文件")) @click=$(|_e| attached.set(false))>"×"</button>
                                        )
                                    )
                                </div>
                            </div>
                        )
                        <p class="mb-0 mt-2.5 text-center text-[11px] text-muted-foreground">(locale.select("Demo procedure · no model or file service", "示例过程函数 · 不连接模型或文件服务"))</p>
                    </div>
                </section>
            </div>
        )
    })
}

#[shard("/_gallery/chat/messages")]
pub(in crate::app) async fn message_region(
    messages: Signal<String>,
    draft: Signal<String>,
    busy: Signal<bool>,
    language: String,
) -> Result<impl View> {
    let parsed = parse_messages(&messages.get());
    let locale = if language == "zh-CN" {
        Locale::Zh
    } else {
        Locale::En
    };
    let first_prompt = locale
        .select("Plan a reusable Chat interface", "规划可复用的 Chat 界面")
        .to_owned();
    let second_prompt = locale
        .select("Explain the component boundaries", "解释组件边界")
        .to_owned();
    let first_attrs = attributes! { @click=$(|_e: Event| draft.set(first_prompt.clone())) };
    let second_attrs = attributes! { @click=$(|_e: Event| draft.set(second_prompt.clone())) };
    let mut last_user = String::new();
    let rendered = parsed
        .into_iter()
        .map(|message| {
            if message.role == ChatBubbleRole::User {
                last_user = message.content.clone();
            }
            (message, last_user.clone())
        })
        .collect::<Vec<_>>();
    Ok(view! {
        if rendered.is_empty() {
            <div class="mx-auto flex min-h-[340px] max-w-[620px] flex-col justify-center py-6">
                <span class="mb-5 grid size-12 place-items-center rounded-2xl bg-[var(--gr-accent-soft)] text-lg font-bold text-primary">"AI"</span>
                <h3 class="m-0 text-[25px] font-semibold tracking-[-0.03em] text-foreground">(locale.select("What shall we explore?", "今天想探索什么？"))</h3>
                <p class="mb-7 mt-2 text-sm leading-6 text-muted-foreground">(locale.select("Choose a starting point or write your own message below.", "选择一个起点，或在下方输入自己的消息。"))</p>
                chat_prompts(label: locale.select("Suggested prompts", "建议输入"),
                    chat_prompt(title: first_prompt.as_str(), attrs: first_attrs)
                    chat_prompt(title: second_prompt.as_str(), attrs: second_attrs)
                )
            </div>
        } else {
            chat_message_list(label: locale.select("Conversation messages", "会话消息"),
                #[key(message.id.clone())]
                for (message, prompt) in rendered {
                    demo_message(message: message, prompt: prompt, messages: messages.clone(), busy: busy.clone(), language: language.clone())
                }
            )
        }
    })
}

#[component]
async fn demo_message(
    cx: &Cx,
    message: ChatMessage,
    prompt: String,
    messages: Signal<String>,
    busy: Signal<bool>,
    language: String,
) -> Result<impl View> {
    let locale = if language == "zh-CN" {
        Locale::Zh
    } else {
        Locale::En
    };
    let message_id = message.id.clone();
    let finish_id = message.id.clone();
    let fail_id = message.id.clone();
    let cancel_id = message.id.clone();
    let retry_id = message.id.clone();
    let copy_content = message.content.clone();
    let finish_state = messages.clone();
    let fail_state = messages.clone();
    let cancel_state = messages.clone();
    let retry_state = messages.clone();
    let service_state = messages.clone();
    let service_id = message.id.clone();
    let service_prompt = prompt.clone();
    let service_language = language.clone();
    let busy_for_service = busy.clone();
    let busy_for_finish = busy.clone();
    let busy_for_fail = busy.clone();
    let busy_for_cancel = busy.clone();
    let busy_for_retry = busy.clone();
    let live_content = signal(cx, || message.content.clone());
    let live_status = signal(cx, || message.status.as_str().to_owned());
    let next_content = live_content.clone();
    let next_status = live_status.clone();
    let finish_content = live_content.clone();
    let fail_content = live_content.clone();
    let cancel_content = live_content.clone();
    let waiting_label = locale
        .select("Waiting for a reply…", "等待回复…")
        .to_owned();
    let sending_label = locale.select("Sending", "发送中").to_owned();
    let streaming_label = locale.select("Streaming", "生成中").to_owned();
    let first_chunk = locale
        .select(
            "This response is arriving in parts. ",
            "这条回复正在分段生成。 ",
        )
        .to_owned();
    let second_chunk = locale
        .select(
            "The page owns messages and request state. ",
            "页面持有消息与请求状态。 ",
        )
        .to_owned();
    let third_chunk = locale
        .select(
            "Reusable components render each part.",
            "可复用组件负责呈现各部分。 ",
        )
        .to_owned();
    let pending = matches!(
        message.status,
        ChatMessageStatus::Sending | ChatMessageStatus::Streaming
    );
    let bubble_attrs = if pending {
        let status = live_status.clone();
        attributes! { cx => id=(message_id.as_str()) :data-status=$(status.get()) aria-busy="true" }
    } else {
        attributes! { cx => id=(message_id.as_str()) }
    };
    let think_open = signal(cx, || false);
    let copied = signal(cx, || false);
    let copied_state = copied.clone();
    let copy_label = locale.select("Copy reply text", "复制回复文本");
    let copied_label = locale.select("Copied", "已复制");
    let has_sources = message.id == "design-assistant";

    Ok(view! {
        <div class="gr-chat-message-entry">
        chat_bubble(role: message.role, status: if pending { None } else { Some(message.status) }, language: locale.ui(), attrs: bubble_attrs,
            if has_sources {
                chat_think(id: "gallery-design-thinking", open: &think_open, language: locale.ui(),
                    chat_thought_chain(label: locale.select("Design steps", "设计步骤"),
                        chat_thought_step(title: locale.select("Split", "拆分"), (locale.select("Identify reusable message and input pieces.", "识别可复用的消息和输入部分。")))
                        chat_thought_step(title: locale.select("Compose", "组合"), (locale.select("Keep conversation state in the page.", "在页面管理会话状态。")))
                    )
                )
            }
            if pending {
                <p class="m-0" :hidden=$(live_content.get().is_empty())>$(live_content.get())</p>
                <p class="m-0 text-muted-foreground" :hidden=$(!live_content.get().is_empty())>(waiting_label.as_str())</p>
                <p class="mb-0 mt-2 text-[11px] font-medium text-muted-foreground" role="status">$(if live_status.get() == "sending" { sending_label.clone() } else { streaming_label.clone() })</p>
            } else if message.content.is_empty() {
                <p class="m-0 text-muted-foreground">(waiting_label.as_str())</p>
            } else {
                chat_markdown(source: message.content.as_str())
            }
            if message.role == ChatBubbleRole::Assistant {
                chat_actions(label: locale.select("Message actions", "消息操作"),
                    if message.status == ChatMessageStatus::Sending || message.status == ChatMessageStatus::Streaming {
                        <button type="button" class="gr-chat-action" @click=$(async |_e: Event| {
                            let _answer = gallery_reply(service_prompt.clone(), service_language.clone()).await;
                            raw!(r#"(() => {
                                    const items = JSON.parse(${service_state}.get().dehydrate());
                                    const item = items.find(value => value.id === ${service_id}.dehydrate());
                                    if (!item || (item.status !== 'sending' && item.status !== 'streaming')) return;
                                    item.content = ${_answer}.dehydrate();
                                    item.status = 'complete';
                                    ${service_state}.set(cx.hydrate(JSON.stringify(items)));
                                    ${busy_for_service}.set(cx.hydrate(false));
                                })();"#, ());
                        })>(locale.select("Demo service", "示例服务"))</button>
                        <button type="button" class="gr-chat-action" @click=$(|_e| {
                            raw!(r#"(() => {
                                const current = ${next_content}.get().dehydrate();
                                const chunks = [${first_chunk}.dehydrate(), ${second_chunk}.dehydrate(), ${third_chunk}.dehydrate()];
                                const next = chunks.find(chunk => !current.includes(chunk));
                                if (next) ${next_content}.set(cx.hydrate(current + next));
                                ${next_status}.set(cx.hydrate('streaming'));
                            })()"#, ());
                        })>(locale.select("Next chunk", "下一段"))</button>
                        <button type="button" class="gr-chat-action" @click=$(|_e| {
                            raw!(r#"(() => {
                                const items = JSON.parse(${finish_state}.get().dehydrate());
                                const item = items.find(value => value.id === ${finish_id}.dehydrate());
                                if (!item) return;
                                item.content = ${finish_content}.get().dehydrate() || 'Done.';
                                item.status = 'complete';
                                ${finish_state}.set(cx.hydrate(JSON.stringify(items)));
                                ${busy_for_finish}.set(cx.hydrate(false));
                            })()"#, ());
                        })>(locale.select("Finish", "完成"))</button>
                        <button type="button" class="gr-chat-action" @click=$(|_e| {
                            raw!(r#"(() => { const items = JSON.parse(${fail_state}.get().dehydrate()); const item = items.find(value => value.id === ${fail_id}.dehydrate()); if (item) { item.content = ${fail_content}.get().dehydrate(); item.status = 'failed'; ${fail_state}.set(cx.hydrate(JSON.stringify(items))); ${busy_for_fail}.set(cx.hydrate(false)); } })()"#, ());
                        })>(locale.select("Fail", "失败"))</button>
                        <button type="button" class="gr-chat-action" @click=$(async |_e: Event| {
                            let _ack = gallery_action("cancel".to_owned(), cancel_id.clone()).await;
                            raw!(r#"(() => { const items = JSON.parse(${cancel_state}.get().dehydrate()); const item = items.find(value => value.id === ${cancel_id}.dehydrate()); if (item) { item.content = ${cancel_content}.get().dehydrate(); item.status = 'cancelled'; ${cancel_state}.set(cx.hydrate(JSON.stringify(items))); ${busy_for_cancel}.set(cx.hydrate(false)); } })()"#, ());
                        })>(locale.select("Cancel", "取消"))</button>
                    }
                    if message.status == ChatMessageStatus::Failed || message.status == ChatMessageStatus::Cancelled {
                        <button type="button" class="gr-chat-action" @click=$(async |_e: Event| {
                            let _ack = gallery_action("retry".to_owned(), retry_id.clone()).await;
                            raw!(r#"(() => { const items = JSON.parse(${retry_state}.get().dehydrate()); const item = items.find(value => value.id === ${retry_id}.dehydrate()); if (item) { item.content = ''; item.status = 'sending'; ${retry_state}.set(cx.hydrate(JSON.stringify(items))); ${busy_for_retry}.set(cx.hydrate(true)); } })()"#, ());
                        })>(locale.select("Retry", "重试"))</button>
                    }
                    if message.status == ChatMessageStatus::Complete {
                        <button type="button" class="gr-chat-action gr-chat-action-copy" :aria-label=$(if copied.get() { copied_label } else { copy_label }) :title=$(if copied.get() { copied_label } else { copy_label }) @click=$(|_e| {
                            raw!("navigator.clipboard?.writeText(${copy_content}.dehydrate()).then(() => ${copied_state}.set(cx.hydrate(true)))", ());
                        })>icon(data: COPY_OUTLINED, size: 14)<span>(locale.select("Copy reply", "复制回复"))</span></button>
                        <span class="gr-chat-copy-feedback" role="status" :hidden=$(!copied.get())>(copied_label)</span>
                    }
                )
            }
        )
        if has_sources {
            chat_sources(label: locale.select("Searched the web", "已搜索网页"),
                chat_source(
                    title: locale.select("Topcoat component documentation", "Topcoat 组件文档"),
                    href: "https://docs.rs/topcoat",
                    summary: Some(locale.select("Reviewed the component API used in this reply.", "查看了本条回复所用组件的 API 文档。")),
                    activity: Some(locale.select("Searched the web", "已搜索网页")),
                )
            )
        }
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::parse_messages;

    #[test]
    fn rejects_oversized_or_invalid_browser_message_state() {
        assert!(parse_messages("not JSON").is_empty());
        assert!(parse_messages(&"x".repeat(131_073)).is_empty());
        let repeated = r#"[{"id":"same","role":"user","status":"complete","content":"first"},{"id":"same","role":"assistant","status":"complete","content":"second"}]"#;
        assert_eq!(parse_messages(repeated).len(), 1);
    }
}
