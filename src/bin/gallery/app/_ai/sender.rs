use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::{Event, signal},
    view::{View, attributes, view},
};
use topcoat_ant_design::chat_sender;

use crate::{app::page_header, demo::component_example, locale::Locale};

const EXAMPLE_SOURCE: &str = r#"use topcoat::{runtime::{Event, signal}, view::{attributes, view}};
use topcoat_ant_design::chat_sender;

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
    chat_sender(id: "example-draft", draft: &draft, submit_attrs: submit)
    <p>$(submitted.get())</p>
}"#;

#[page]
pub(in crate::app) async fn sender_page(cx: &Cx) -> Result<impl View> {
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
    };

    Ok(view! {
        page_header(
            eyebrow: "AI COMPONENTS",
            title: "ChatSender",
            description: locale.select(
                "A reactive draft field with a caller-provided submit handler.",
                "响应式消息输入区，发送行为由调用方提供。",
            ),
        )
        component_example(
            id: "chat-sender-preview",
            title: locale.select("Compose and send", "编写并发送"),
            description: locale.select("Enter a message to see the local submit behavior. No model is contacted.", "输入消息，体验本地发送行为；不会请求模型。"),
            source: EXAMPLE_SOURCE,
            <div class="grid gap-4 bg-background p-6 max-[520px]:p-4">
                chat_sender(id: "gallery-sender-draft", draft: &draft, submit_attrs: submit, language: locale.ui())
                <p class="m-0 rounded-lg border border-border bg-card p-4 text-sm text-foreground" (latest_message)>
                    <span class="mb-1 block text-xs font-semibold text-primary">(locale.select("Last sent message", "最近发送的消息"))</span>
                    $(submitted.get())
                </p>
            </div>
        )
    })
}
