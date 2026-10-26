use crate::language::t;
use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, Signal, signal},
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::{
    UiLanguage,
    ui::{button, label, textarea},
};

/// Chat input with a browser-owned draft and caller-owned submit behavior.
///
/// Attach an `@submit` handler to `submit_attrs` and call `prevent_default()`
/// before submitting through a Topcoat procedure or another host integration.
#[component]
pub async fn chat_sender(
    cx: &Cx,
    id: &str,
    draft: &Signal<String>,
    mut submit_attrs: Attributes,
    #[default] busy: Option<&Signal<bool>>,
    #[default] language: UiLanguage,
    #[default] placeholder: Option<&str>,
    #[default] max_length: Option<usize>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let draft = draft.clone();
    let busy = busy.cloned().unwrap_or_else(|| signal(cx, || false));
    let caller_class = attrs.remove("class");
    let root_class = class!("gr-chat-sender", caller_class);
    attrs.extend(attributes! { cx => class=(root_class) });
    let form_class = class!("gr-chat-sender-form", submit_attrs.remove("class"));
    let default_placeholder = language.resolve(t!(ask_a_question_or_describe_a_task));
    let input_label = placeholder
        .map(str::to_owned)
        .unwrap_or(default_placeholder);
    let send_label = language.resolve(t!(send_message));
    let textarea_id = id.to_owned();

    Ok(view! {
        <div (attrs)>
            <form class=(form_class) (submit_attrs)>
                (child)
                label::label(
                    attrs: attributes! { class="sr-only" for=(id) },
                    (input_label.as_str())
                )
                textarea::textarea(
                    attrs: attributes! {
                        cx =>
                        class="gr-chat-sender-input"
                        id=(id)
                        name="message"
                        rows="2"
                        maxlength=(max_length)
                        placeholder=(input_label.as_str())
                        :value=$(draft.get())
                        @input=$(|event: Event| draft.set(event.target.value))
                        @keydown=$(|event: Event| {
                            if event.key == "Enter" {
                                if !event.shift_key {
                                    if !event.is_composing {
                                        event.prevent_default();
                                        raw!(
                                            "document.getElementById(${textarea_id}.dehydrate())?.form?.requestSubmit()",
                                            (),
                                        );
                                    }
                                }
                            }
                        })
                    }
                )
                <div class="gr-chat-sender-actions">
                    <span>
                        (language.resolve(t!(enter_to_send_shift_enter_for_a_new_line)))
                    </span>
                    button::button(
                        attrs: attributes! {
                            cx =>
                            class="gr-chat-send-button"
                            type="submit"
                            :disabled=$(if busy.get() {
                                true
                            } else {
                                draft.get().trim().is_empty()
                            })
                            aria-label=(send_label)
                        },
                        (language.resolve(t!(send)))
                        <span aria-hidden="true">"↗"</span>
                    )
                </div>
            </form>
        </div>
    })
}
