use topcoat::{
    Result,
    context::Cx,
    runtime::Signal,
    view::{Child, View, component, view},
};

use crate::{UiLanguage, collapse};

/// Expandable reasoning or process details. Only pass content intended for users.
#[component]
pub async fn chat_think(
    cx: &Cx,
    id: &str,
    open: &Signal<bool>,
    #[default] language: UiLanguage,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let _ = cx;
    let open = open.clone();
    let title = language.select("Thinking process", "思考过程");
    Ok(view! {
        <section class="gr-chat-think" :data-state=$(if open.get() { "open" } else { "closed" })>
            <button type="button" class="gr-chat-think-trigger" :aria-expanded=$(open.get()) aria-controls=(id) @click=$(|_e| open.toggle())>
                <span class="gr-chat-think-caret" aria-hidden="true">"▶"</span>
                <span class="gr-chat-think-brain" aria-hidden="true">"🧠"</span>
                <span>(title)</span>
            </button>
            collapse(id: id, open: &open,
                <div class="gr-chat-think-detail">(child)</div>
            )
        </section>
    })
}
