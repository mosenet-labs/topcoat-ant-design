use crate::locale::Locale;
use topcoat::{
    Result,
    context::Cx,
    view::{View, attributes, component, view},
};
use topcoat_ant_design::{
    ButtonSize, ButtonVariant, button_variants, card, card_content, card_description, card_header,
    card_title,
};

#[component]
pub(in crate::app) async fn overview_content(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let native_url = locale.link("/topcoat-ui");
    let start_url = locale.link("/");
    let chat_url = locale.link("/chat");
    let native_examples = [
        ("Accordion", "/accordion"),
        ("Dialog", "/dialog"),
        ("Dropdown Menu", "/dropdown-menu"),
        ("Tabs", "/tabs/webhook"),
        ("Tooltip", "/tooltip"),
    ];
    let composed_examples = [
        ("Notification", "/notification"),
        ("Popconfirm", "/popconfirm"),
        ("Drawer", "/drawer"),
        ("FormField", "/form-field"),
        ("DateTimeRange", "/date-time-range"),
        ("Table", "/table"),
    ];
    let chat_examples = [
        ("ChatBubble", "/bubble"),
        ("ChatMessageList", "/message-list"),
        ("ChatSender", "/sender"),
        ("ChatMarkdown", "/chat/markdown"),
        ("ChatConversationList", "/chat/conversations"),
    ];

    Ok(view! {
        <div class="space-y-9">
            <section class="relative overflow-hidden rounded-2xl border border-border bg-card px-8 py-10 shadow-sm max-[640px]:px-5" aria-labelledby="overview-title">
                <div class="pointer-events-none absolute -right-16 -top-32 size-80 rounded-full bg-[var(--gr-accent-soft)] blur-3xl" aria-hidden="true"></div>
                <div class="relative">
                    <p class="m-0 text-xs font-bold uppercase tracking-[0.16em] text-primary">"Topcoat Ant Design / Library"</p>
                    <h1 id="overview-title" class="mb-3 mt-4 max-w-[750px] text-[clamp(32px,5vw,52px)] font-bold leading-[1.1] tracking-[-0.045em] text-foreground">(locale.select("Components for real applications", "面向真实应用的组件"))</h1>
                    <p class="m-0 max-w-[720px] text-[15px] leading-7 text-muted-foreground">(locale.select("Explore 31 official Topcoat UI modules and the composed components built for data workflows and AI conversations. Every example uses the same light and dark theme.", "浏览 31 个 Topcoat 官方 UI 模块，以及面向数据流程和 AI 对话的组合组件。所有示例共用明暗主题。"))</p>
                    <div class="mt-7 flex flex-wrap gap-3">
                        <a href=(native_url.as_str()) class=(button_variants(ButtonVariant::Primary, ButtonSize::Md))>(locale.select("Explore official components", "查看官方组件")) " →"</a>
                        <a href=(start_url.as_str()) class=(button_variants(ButtonVariant::Outline, ButtonSize::Md))>(locale.select("Read the quick start", "阅读快速开始"))</a>
                    </div>
                </div>
            </section>

            <section class="grid gap-4 md:grid-cols-3" aria-label=(locale.select("Library at a glance", "组件库概览"))>
                <div class="rounded-xl border border-border bg-card p-5"><strong class="block text-3xl font-bold tracking-tight text-primary">"31"</strong><span class="mt-1 block text-sm text-muted-foreground">(locale.select("Official UI modules", "官方 UI 模块"))</span></div>
                <div class="rounded-xl border border-border bg-card p-5"><strong class="block text-3xl font-bold tracking-tight text-primary">"2"</strong><span class="mt-1 block text-sm text-muted-foreground">(locale.select("Coordinated themes", "统一的明暗主题"))</span></div>
                <div class="rounded-xl border border-border bg-card p-5"><strong class="block text-3xl font-bold tracking-tight text-primary">"1"</strong><span class="mt-1 block text-sm text-muted-foreground">(locale.select("Direct crate-level API", "统一的 crate 根级 API"))</span></div>
            </section>

            <section class="grid gap-5 lg:grid-cols-2" aria-label=(locale.select("Component groups", "组件分类"))>
                card(
                    card_header(
                        card_title((locale.select("Official primitives", "官方基础组件")))
                        card_description((locale.select("Imported directly from Topcoat UI and exported at the crate root.", "直接导入 Topcoat UI，并从当前 crate 根级导出。")))
                    )
                    card_content(
                        <div class="flex flex-wrap gap-2">
                            for (name, path) in native_examples {
                                <a href=(locale.link(path)) class="rounded-md border border-border bg-background px-3 py-2 text-sm font-medium text-foreground no-underline transition-colors hover:border-primary hover:text-primary">(name) " ↗"</a>
                            }
                        </div>
                        <a href=(native_url.as_str()) class="mt-5 inline-flex text-sm font-semibold text-primary no-underline hover:underline">(locale.select("See the complete registry →", "查看完整组件目录 →"))</a>
                    )
                )
                card(
                    card_header(
                        card_title((locale.select("Composed controls", "组合组件")))
                        card_description((locale.select("Application patterns assembled from official primitives.", "基于官方基础组件构建的应用交互模式。")))
                    )
                    card_content(
                        <div class="flex flex-wrap gap-2">
                            for (name, path) in composed_examples {
                                <a href=(locale.link(path)) class="rounded-md border border-border bg-background px-3 py-2 text-sm font-medium text-foreground no-underline transition-colors hover:border-primary hover:text-primary">(name) " ↗"</a>
                            }
                        </div>
                    )
                )
            </section>

            card(attrs: attributes! { class="border-[var(--gr-accent-border)] bg-[var(--gr-accent-soft)]" },
                card_header(
                    card_title((locale.select("AI conversation components", "AI 对话组件")))
                    card_description((locale.select("Compose a chat interface from messages, input, markdown, and conversation navigation.", "用消息、输入、Markdown 和会话导航组合聊天界面。")))
                )
                card_content(
                    <a href=(chat_url.as_str()) class=(button_variants(ButtonVariant::Primary, ButtonSize::Md))>(locale.select("Open chat demo", "打开聊天演示")) " →"</a>
                    <div class="mt-5 flex flex-wrap gap-2">
                        for (name, path) in chat_examples {
                            <a href=(locale.link(path)) class="rounded-md border border-[var(--gr-accent-border)] bg-card px-3 py-2 text-sm font-medium text-foreground no-underline hover:text-primary">(name)</a>
                        }
                    </div>
                )
            )

            <p class="m-0 border-t border-border pt-6 text-sm text-muted-foreground">(locale.select("Complete the five integration steps in the quick start, then choose a component from the catalog.", "先完成五步接入，再从组件目录选择需要的组件。"))</p>
        </div>
    })
}
