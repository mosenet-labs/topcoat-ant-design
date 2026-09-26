use topcoat::{
    Result,
    view::{Child, View, component, view},
};

/// Attachments selected for a draft. The host owns upload and removal.
#[component]
pub async fn chat_attachment_tray(label: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(
        view! { <div class="gr-chat-attachment-tray flex flex-wrap gap-2" role="group" aria-label=(label)>(child)</div> },
    )
}

/// File metadata shown inside a message or attachment tray.
#[component]
pub async fn chat_file(name: &str, detail: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="gr-chat-file inline-flex min-w-0 items-center gap-3 rounded-lg border border-[#dbe8f7] bg-white px-3 py-2 text-sm">
            <span class="grid size-8 shrink-0 place-items-center rounded-md bg-[#e6f4ff] font-bold text-[#1677ff]" aria-hidden="true">"F"</span>
            <span class="min-w-0"><strong class="block truncate font-semibold">(name)</strong><small class="block text-[#8c8c8c]">(detail)</small></span>
            (child)
        </div>
    })
}
