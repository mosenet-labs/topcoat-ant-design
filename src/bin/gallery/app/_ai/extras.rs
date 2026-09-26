use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    router::page,
    runtime::{Event, signal},
    view::{View, attributes, view},
};
use topcoat_ant_design::{
    ChatBubbleRole, ChatMessageStatus, ChatThoughtStatus, chat_actions, chat_attachment_tray,
    chat_bubble, chat_conversation_item, chat_conversation_list, chat_file, chat_markdown,
    chat_prompt, chat_prompts, chat_source, chat_sources, chat_think, chat_thought_chain,
    chat_thought_step, icons::COPY_OUTLINED,
};

use crate::{app::page_header, demo::component_example, locale::Locale};

#[page("/chat/states")]
pub(in crate::app) async fn chat_states_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        page_header(eyebrow: "AI COMPONENTS", title: locale.select("Message states", "消息状态"), description: locale.select("The same bubble shows every request state with accessible status text.", "同一消息气泡展示全部请求状态和可访问的状态文本。"))
        component_example(id: "chat-states-preview", title: locale.select("Request lifecycle", "请求生命周期"), description: locale.select("Compare sending, streaming, complete, failed, and cancelled.", "对比发送中、生成中、完成、失败与取消。"), source: "chat_bubble(role: ChatBubbleRole::Assistant, status: Some(ChatMessageStatus::Streaming), \"Partial reply\")",
            <div class="grid gap-5 bg-background p-6 max-[520px]:p-4">
                chat_bubble(role: ChatBubbleRole::Assistant, status: Some(ChatMessageStatus::Sending), language: locale.ui(), (locale.select("Waiting for the reply…", "等待回复…")))
                chat_bubble(role: ChatBubbleRole::Assistant, status: Some(ChatMessageStatus::Streaming), language: locale.ui(), (locale.select("This response is arriving in parts…", "这条回复正在分段生成…")))
                chat_bubble(role: ChatBubbleRole::Assistant, status: Some(ChatMessageStatus::Complete), language: locale.ui(), (locale.select("The response is complete.", "回复已完成。")))
                chat_bubble(role: ChatBubbleRole::Assistant, status: Some(ChatMessageStatus::Failed), language: locale.ui(), (locale.select("The request failed; retry is available.", "请求失败，可重试。")))
                chat_bubble(role: ChatBubbleRole::Assistant, status: Some(ChatMessageStatus::Cancelled), language: locale.ui(), (locale.select("The request was cancelled.", "请求已取消。")))
            </div>
        )
    })
}

#[page("/chat/markdown")]
pub(in crate::app) async fn chat_markdown_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let sample = locale.select(
        "**Markdown** supports lists, `code`, and [safe links](https://docs.rs/topcoat).\n\n- One component\n- Several content types\n\nRaw HTML stays text: <script>alert(1)</script>",
        "**Markdown** 支持列表、`代码`和[安全链接](https://docs.rs/topcoat)。\n\n- 一个组件\n- 多种内容\n\n原始 HTML 会保留为文本：<script>alert(1)</script>",
    );
    Ok(view! {
        page_header(eyebrow: "AI COMPONENTS", title: "ChatMarkdown", description: locale.select("Render untrusted message Markdown while escaping raw HTML and unsafe URLs.", "安全渲染聊天 Markdown，转义原始 HTML 和不安全链接。"))
        component_example(id: "chat-markdown-preview", title: locale.select("Formatted message", "格式化消息"), description: locale.select("Raw HTML is displayed as text; image embeds are omitted.", "原始 HTML 作为文本显示；不渲染图片嵌入。"), source: "chat_markdown(source: \"**Markdown** and [docs](https://docs.rs/topcoat)\")",
            <div class="bg-background p-6 max-[520px]:p-4">chat_bubble(role: ChatBubbleRole::Assistant, language: locale.ui(), chat_markdown(source: sample))</div>
        )
    })
}

#[page("/chat/think")]
pub(in crate::app) async fn chat_think_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let open = signal(cx, || false);
    Ok(view! {
        page_header(eyebrow: "AI COMPONENTS", title: "ChatThink", description: locale.select("Show or hide a user-visible process summary.", "展开或收起面向用户的过程摘要。"))
        component_example(id: "chat-think-preview", title: locale.select("Expandable detail", "可展开详情"), description: locale.select("The trigger exposes its expanded state and controls the detail region.", "触发按钮标明展开状态，并控制详情区域。"), source: "let open = signal(cx, || false);\nchat_think(id: \"thinking\", open: &open, \"Checking the available components…\")",
            <div class="bg-background p-6 max-[520px]:p-4">chat_think(id: "gallery-chat-thinking", open: &open, language: locale.ui(), <p class="m-0">(locale.select("Checked the reusable components and selected a message layout.", "检查了可复用组件，并选择了消息布局。"))</p>)</div>
        )
    })
}

#[page("/chat/thought-chain")]
pub(in crate::app) async fn chat_thought_chain_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        page_header(eyebrow: "AI COMPONENTS", title: "ChatThoughtChain", description: locale.select("Track a process with numbered nodes, quiet connectors, and concise descriptions.", "用编号节点、细连接线和简短说明展示过程。"))
        component_example(id: "chat-thought-chain-preview", title: locale.select("Process steps", "过程步骤"), description: locale.select("A neutral numbered chain keeps each title and detail easy to scan.", "中性的编号思维链让标题和说明更易浏览。"), source: "chat_thought_chain(label: \"Process\",\n    chat_thought_step(title: \"Inspect\", \"Read the components\")\n    chat_thought_step(title: \"Compose\", \"Build the preview\")\n)",
            <div class="bg-background p-6 max-[520px]:p-4">chat_thought_chain(label: locale.select("Process", "过程"),
                chat_thought_step(title: locale.select("Inspect", "检查"), (locale.select("Read the component contracts.", "读取组件契约。")))
                chat_thought_step(title: locale.select("Compose", "组合"), (locale.select("Connect message, sources, and actions.", "连接消息、来源和操作。")))
                chat_thought_step(title: locale.select("Review", "检查结果"), (locale.select("Verify the final preview.", "验证最终预览。")))
                chat_thought_step(title: locale.select("Deliver", "交付"), (locale.select("Prepare the reusable Chat interface.", "准备可复用的 Chat 界面。")))
            )</div>
        )
        component_example(id: "chat-thought-step-preview", title: "ChatThoughtStep", description: locale.select("Optional statuses distinguish progress, success, failure, and interruption.", "可选状态区分进行中、成功、失败和中止。"), source: "chat_thought_step(title: \"Tool call\", status: Some(ChatThoughtStatus::Success), \"Completed\")",
            <div class="bg-background p-6 max-[520px]:p-4">chat_thought_chain(label: locale.select("Step statuses", "步骤状态"),
                chat_thought_step(title: locale.select("Query knowledge", "查询知识库"), status: Some(ChatThoughtStatus::Success), language: locale.ui(), (locale.select("The reference was found.", "已找到参考资料。")))
                chat_thought_step(title: locale.select("Call tool", "调用工具"), status: Some(ChatThoughtStatus::Loading), language: locale.ui(), (locale.select("Waiting for the tool response.", "正在等待工具返回。")))
                chat_thought_step(title: locale.select("Open result", "打开结果"), status: Some(ChatThoughtStatus::Error), language: locale.ui(), (locale.select("The resource could not be opened.", "无法打开该资源。")))
                chat_thought_step(title: locale.select("Stop task", "停止任务"), status: Some(ChatThoughtStatus::Abort), language: locale.ui(), (locale.select("The remaining work was stopped.", "剩余任务已中止。")))
            )</div>
        )
    })
}

#[page("/chat/sources")]
pub(in crate::app) async fn chat_sources_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    Ok(view! {
        page_header(eyebrow: "AI COMPONENTS", title: "ChatSources", description: locale.select("Show expandable research references beneath a response.", "在回复下方展示可展开的资料来源。"))
        component_example(id: "chat-sources-preview", title: locale.select("Research references", "检索来源"), description: locale.select("Expand or collapse the references. Each card combines a short summary with a destination.", "展开或收起来源；每张卡片包含摘要和目标链接。"), source: "chat_sources(label: \"Searched the web\",\n    chat_source(title: \"Topcoat documentation\", href: \"https://docs.rs/topcoat\", summary: Some(\"Reviewed the component API.\"), activity: Some(\"Searched the web\"))\n)",
            <div class="bg-background p-6 max-[520px]:p-4">chat_sources(label: locale.select("Searched the web", "已搜索网页"),
                chat_source(title: locale.select("Topcoat component documentation", "Topcoat 组件文档"), href: "https://docs.rs/topcoat", summary: Some(locale.select("Reviewed the component API used in this reply.", "查看了本条回复所用组件的 API 文档。")), activity: Some(locale.select("Searched the web", "已搜索网页")))
                chat_source(title: locale.select("Ant Design documentation", "Ant Design 文档"), href: "https://ant.design", summary: Some(locale.select("Compared the layout and interaction patterns.", "对照了布局和交互方式。")), activity: Some(locale.select("Searched the web", "已搜索网页")))
            )</div>
        )
        component_example(id: "chat-source-preview", title: "ChatSource", description: locale.select("A source works on its own; invalid destinations stay disabled.", "单张来源卡片可以独立使用；无效链接保持禁用。"), source: "<ul>chat_source(title: \"Topcoat\", href: \"https://docs.rs/topcoat\", summary: Some(\"Checked the API.\"))</ul>",
            <ul class="gr-chat-sources-list bg-background p-6 max-[520px]:p-4">chat_source(title: "Topcoat", href: "https://docs.rs/topcoat", summary: Some(locale.select("Checked the public API.", "查看了公开 API。")), activity: Some(locale.select("Searched the web", "已搜索网页"))) chat_source(title: locale.select("Disabled destination", "禁用的链接"), href: "javascript:alert(1)", summary: Some(locale.select("Unsafe URL is unavailable.", "不安全的链接不可使用。")), activity: Some(locale.select("Searched the web", "已搜索网页")))</ul>
        )
    })
}

#[page("/chat/actions")]
pub(in crate::app) async fn chat_actions_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let action = signal(cx, || locale.select("No action yet", "尚未操作").to_owned());
    let copied = locale.select("Copied", "已复制").to_owned();
    let retried = locale.select("Retry requested", "已请求重试").to_owned();
    Ok(view! {
        page_header(eyebrow: "AI COMPONENTS", title: "ChatActions", description: locale.select("Place host-owned actions beneath a message.", "在消息下方放置由宿主处理的操作。"))
        component_example(id: "chat-actions-preview", title: locale.select("Message actions", "消息操作"), description: locale.select("Use the buttons to see local action feedback.", "点击按钮查看本地操作反馈。"), source: "chat_actions(label: \"Message actions\",\n    <button type=\"button\" aria-label=\"Copy reply text\" @click=...>\"Copy reply\"</button>\n    <button type=\"button\" @click=...>\"Retry\"</button>\n)",
            <div class="grid gap-3 bg-background p-6 max-[520px]:p-4">chat_actions(label: locale.select("Message actions", "消息操作"),
                <button class="gr-chat-action gr-chat-action-copy" type="button" aria-label=(locale.select("Copy reply text", "复制回复文本")) title=(locale.select("Copy reply text", "复制回复文本")) @click=$(|_e: Event| action.set(copied.clone()))>icon(data: COPY_OUTLINED, size: 14)<span>(locale.select("Copy reply", "复制回复"))</span></button>
                <button class="gr-chat-action" type="button" @click=$(|_e: Event| action.set(retried.clone()))>(locale.select("Retry", "重试"))</button>
            )<p class="m-0 text-xs text-muted-foreground" role="status">$(action.get())</p></div>
        )
    })
}

#[page("/chat/attachments")]
pub(in crate::app) async fn chat_attachments_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let attached = signal(cx, || true);
    Ok(view! {
        page_header(eyebrow: "AI COMPONENTS", title: "ChatAttachmentTray", description: locale.select("Review selected files before sending a message.", "发送消息前查看已选文件。"))
        component_example(id: "chat-attachments-preview", title: locale.select("Draft attachments", "草稿附件"), description: locale.select("Add and remove a local example file. Upload remains a host responsibility.", "添加或移除本地示例文件；上传由宿主负责。"), source: "chat_attachment_tray(label: \"Selected attachments\",\n    chat_file(name: \"example.txt\", detail: \"Local file\")\n)",
            <div class="grid gap-4 bg-background p-6 max-[520px]:p-4">
                <button class="gr-chat-action w-fit" type="button" @click=$(|_e: Event| attached.set(true))>(locale.select("Add sample file", "添加示例文件"))</button>
                <div :hidden=$(!attached.get())>chat_attachment_tray(label: locale.select("Selected attachments", "已选附件"),
                    chat_file(name: "example.txt", detail: locale.select("Local example", "本地示例"),
                        <button class="gr-chat-action" type="button" @click=$(|_e: Event| attached.set(false))>(locale.select("Remove", "移除"))</button>
                    )
                )</div>
            </div>
        )
    })
}

#[page("/chat/files")]
pub(in crate::app) async fn chat_files_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let visible = signal(cx, || true);
    Ok(view! {
        page_header(eyebrow: "AI COMPONENTS", title: "ChatFile", description: locale.select("Show safe file metadata in a message or attachment tray.", "在消息或附件区展示文件元数据。"))
        component_example(id: "chat-files-preview", title: locale.select("File item", "文件项"), description: locale.select("The host decides whether the file can be opened or removed.", "宿主决定文件是否可打开或移除。"), source: "chat_file(name: \"notes.txt\", detail: \"12 KB\")",
            <div class="grid gap-3 bg-background p-6 max-[520px]:p-4">
                <div :hidden=$(!visible.get())>chat_file(name: "notes.txt", detail: "12 KB", <button class="gr-chat-action" type="button" @click=$(|_e: Event| visible.set(false))>(locale.select("Remove", "移除"))</button>)</div>
                <button class="gr-chat-action w-fit" type="button" :hidden=$(visible.get()) @click=$(|_e: Event| visible.set(true))>(locale.select("Restore example", "恢复示例"))</button>
            </div>
        )
    })
}

#[page("/chat/prompts")]
pub(in crate::app) async fn chat_prompts_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let selected = signal(cx, String::new);
    let first = locale
        .select("Plan a Chat interface", "规划 Chat 界面")
        .to_owned();
    let second = locale
        .select("Explain component boundaries", "解释组件边界")
        .to_owned();
    let first_attrs = attributes! { cx => @click=$(|_e: Event| selected.set(first.clone())) };
    let second_attrs = attributes! { cx => @click=$(|_e: Event| selected.set(second.clone())) };
    Ok(view! {
        page_header(eyebrow: "AI COMPONENTS", title: "ChatPrompts", description: locale.select("Suggest useful starting points for an empty conversation.", "为新的空会话提供建议输入。"))
        component_example(id: "chat-prompts-preview", title: locale.select("Suggested prompts", "建议输入"), description: locale.select("Select a prompt to see the host-owned choice.", "选择一个建议，查看宿主管理的选择结果。"), source: "chat_prompts(label: \"Suggested prompts\",\n    chat_prompt(title: \"Plan a Chat interface\", attrs: click_attrs)\n)",
            <div class="grid gap-4 bg-background p-6 max-[520px]:p-4">chat_prompts(label: locale.select("Suggested prompts", "建议输入"),
                chat_prompt(title: first.as_str(), attrs: first_attrs)
                chat_prompt(title: second.as_str(), attrs: second_attrs)
            )<p class="m-0 text-xs text-muted-foreground" role="status">$(selected.get())</p></div>
        )
        component_example(id: "chat-prompt-preview", title: "ChatPrompt", description: locale.select("A single prompt exposes a click handler supplied by the host.", "单条建议使用宿主提供的点击处理函数。"), source: "chat_prompt(title: \"Plan a Chat interface\", attrs: click_attrs)",
            <div class="grid gap-3 bg-background p-6 max-[520px]:p-4">chat_prompt(title: first.as_str(), attrs: attributes! { cx => @click=$(|_e: Event| selected.set(first.clone())) })<p class="m-0 text-xs text-muted-foreground" role="status">$(selected.get())</p></div>
        )
    })
}

#[page("/chat/conversations")]
pub(in crate::app) async fn chat_conversations_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let design = locale.link("/chat");
    let notes = locale.link("/chat/notes");
    let new = locale.link("/chat/new");
    Ok(view! {
        page_header(eyebrow: "AI COMPONENTS", title: "ChatConversationList", description: locale.select("Navigate between host-provided conversation routes.", "在宿主提供的会话路由间切换。"))
        component_example(id: "chat-conversations-preview", title: locale.select("Conversation navigation", "会话导航"), description: locale.select("Links open real Chat routes; the current route controls the active item.", "链接打开真实 Chat 路由，当前路由决定激活项。"), source: "chat_conversation_list(label: \"Conversations\",\n    chat_conversation_item(title: \"Design\", href: \"/chat\", active: true)\n)",
            <div class="max-w-[360px] bg-background p-6 max-[520px]:p-4">chat_conversation_list(label: locale.select("Conversations", "会话"),
                chat_conversation_item(title: locale.select("Designing a Chat interface", "设计 Chat 界面"), href: design.as_str(), active: true)
                chat_conversation_item(title: locale.select("Component boundaries", "组件边界"), href: notes.as_str(), active: false)
                chat_conversation_item(title: locale.select("New conversation", "新建会话"), href: new.as_str(), active: false)
            )</div>
        )
        component_example(id: "chat-conversation-item-preview", title: "ChatConversationItem", description: locale.select("A conversation item links to its route and exposes the active page.", "单条会话链接到对应路由，并标识当前页面。"), source: "chat_conversation_item(title: \"Design\", href: \"/chat\", active: true)",
            <div class="max-w-[360px] bg-background p-6 max-[520px]:p-4">chat_conversation_item(title: locale.select("Designing a Chat interface", "设计 Chat 界面"), href: design.as_str(), active: true)</div>
        )
    })
}
