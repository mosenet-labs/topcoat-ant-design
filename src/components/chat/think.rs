use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, Signal},
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::{UiLanguage, collapse, ui::button};

/// Expandable reasoning or process details. Only pass content intended for users.
#[component]
pub async fn chat_think(
    cx: &Cx,
    id: &str,
    open: &Signal<bool>,
    #[default] language: UiLanguage,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let open = open.clone();
    let title = language.select("Thinking process", "思考过程");
    Ok(view! {
        <section class=(class!("gr-chat-think", attrs.remove("class"))) :data-state=$(if open.get() { "open" } else { "closed" }) (attrs)>
            button::button(variant: button::ButtonVariant::Ghost, attrs: attributes! { cx => type="button" class="gr-chat-think-trigger" :aria-expanded=$(open.get()) aria-controls=(id) @click=$(|_e: Event| open.toggle()) },
                <span class="gr-chat-think-caret" aria-hidden="true">"▶"</span>
                <span class="gr-chat-think-brain" aria-hidden="true">"🧠"</span>
                <span>(title)</span>
            )
            collapse(id: id, open: &open,
                <div class="gr-chat-think-detail">(child)</div>
            )
        </section>
    })
}
