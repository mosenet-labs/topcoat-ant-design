use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// Attachments selected for a draft. The host owns upload and removal.
#[component]
pub async fn chat_attachment_tray(
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(
        view! { <div role="group" aria-label=(label) class=(class!("gr-chat-attachment-tray flex flex-wrap gap-2", attrs.remove("class"))) (attrs)>(child)</div> },
    )
}

/// File metadata shown inside a message or attachment tray.
#[component]
pub async fn chat_file(
    name: &str,
    detail: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("gr-chat-file inline-flex min-w-0 items-center gap-3 rounded-lg border border-[var(--gr-accent-border)] bg-[var(--gr-surface)] px-3 py-2 text-sm", attrs.remove("class"))) (attrs)>
            <span class="grid size-8 shrink-0 place-items-center rounded-md bg-[var(--gr-accent-soft)] font-bold text-[var(--gr-accent)]" aria-hidden="true">"F"</span>
            <span class="min-w-0"><strong class="block truncate font-semibold">(name)</strong><small class="block text-[var(--gr-fg-subtle)]">(detail)</small></span>
            (child)
        </div>
    })
}
