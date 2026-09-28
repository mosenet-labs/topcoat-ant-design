use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, Signal},
    view::{Attributes, View, class, component, view},
};

use crate::UiLanguage;

/// A value and label offered by [`search_multi_select`].
#[derive(Clone)]
pub struct SearchOption {
    pub value: String,
    pub label: String,
}

/// Searchable multi-select backed by a JSON array in `selected`.
///
/// The application supplies options and may handle the root `selectionchange` event to
/// synchronize related form fields. The component does not fetch options.
#[component]
pub async fn search_multi_select(
    cx: &Cx,
    id: &str,
    options: &[SearchOption],
    selected: &Signal<String>,
    search: &Signal<String>,
    open: &Signal<bool>,
    #[default] disabled: bool,
    #[default] language: UiLanguage,
    #[default] label: Option<&str>,
    #[default] placeholder: Option<&str>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let _ = cx;
    let chosen: Vec<String> = serde_json::from_str(&selected.get_untracked()).unwrap_or_default();
    let query = search.get_untracked().to_lowercase();
    let _root_id = id.to_owned();
    let input_id = format!("{id}-input");
    let list_id = format!("{id}-options");
    let label = label.unwrap_or(language.select("Options", "选项"));
    let placeholder = placeholder.unwrap_or(language.select("Search options", "搜索选项"));
    let remove_label = language.select("Remove", "移除");
    let caller_class = attrs.remove("class");
    Ok(view! {
        <div id=(id) class=(class!("group relative min-w-0", caller_class)) :data-open=$(open.get()) (attrs)
            @focusout=$(|_event: Event| {
                let _root_id = _root_id.to_owned();
                raw!("setTimeout(() => { const root = document.getElementById(${_root_id}.dehydrate()); if (root && !root.contains(document.activeElement)) ${open}.set(cx.hydrate(false)); }, 0);", ());
            })>
            <div class="flex min-h-9 flex-wrap items-center gap-1 rounded-md border border-border bg-card px-2 py-1 text-foreground focus-within:border-primary focus-within:ring-2 focus-within:ring-primary/10">
                #[key(option.value.clone())] for option in options {
                    let value = option.value.clone();
                    let value_for_click = option.value.clone();
                    let is_chosen = chosen.contains(&option.value);
                    <span class="inline-flex max-w-full items-center gap-1 rounded border border-primary/30 bg-sidebar-accent px-1.5 py-0.5 text-xs text-sidebar-accent-foreground" :hidden=$(if selected.get().is_empty() { true } else { raw!("!JSON.parse(${selected}.get().dehydrate()).includes(${value}.dehydrate())", !is_chosen) })>
                        <span class="truncate">(option.label.as_str())</span>
                        <button class="border-0 bg-transparent p-0 text-sidebar-accent-foreground hover:text-primary" type="button" aria-label=(format!("{remove_label} {}", option.label)) :disabled=(disabled) @click=$(|event: Event| {
                            event.stop_propagation();
                            let _value = value_for_click.to_owned();
                            let _root_id = _root_id.to_owned();
                            raw!("(() => { const values = JSON.parse(${selected}.get().dehydrate()).filter(item => item !== ${_value}.dehydrate()); ${selected}.set(cx.hydrate(JSON.stringify(values))); document.getElementById(${_root_id}.dehydrate())?.dispatchEvent(new CustomEvent('selectionchange', {bubbles:true})); })();", ());
                        })>"×"</button>
                    </span>
                }
                <input id=(input_id.as_str()) class="h-6! min-w-[120px] flex-1 border-0! bg-transparent! px-1! py-0! text-sm! shadow-none! outline-none! ring-0!" type="search" role="combobox" aria-label=(label) aria-autocomplete="list" aria-controls=(list_id.as_str()) :aria-expanded=$(open.get()) :value=$(search.get()) placeholder=(placeholder) :disabled=(disabled)
                    @focus=$(|_event: Event| open.set(true))
                    @input=$(|event: Event| {
                        let _value = event.target.value;
                        raw!("${search}.set(cx.hydrate(${_value}.dehydrate().toLowerCase()));", ());
                        open.set(true);
                    })
                    @keydown=$(|_event: Event| {
                        let _list_id = list_id.to_owned();
                        raw!("{ const key = ${_event}.key.dehydrate(); if (key === 'ArrowDown' || key === 'Enter') { if (key === 'Enter') ${_event}.prevent_default(); const first = [...(document.getElementById(${_list_id}.dehydrate())?.querySelectorAll('button[role=option]') ?? [])].find(option => !option.disabled && !option.hidden); if (first) { if (key === 'ArrowDown') ${_event}.prevent_default(); if (key === 'Enter') first.click(); else first.focus(); } } else if (key === 'Escape') { ${open}.set(cx.hydrate(false)); } }", ());
                    })>
            </div>
            <div id=(list_id.as_str()) class="absolute z-20 mt-1 hidden max-h-48 w-full overflow-y-auto rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-sm group-data-[open=true]:block" role="listbox" aria-label=(label) aria-multiselectable="true">
                #[key(option.value.clone())] for option in options {
                    let value = option.value.clone();
                    let haystack = format!("{} {}", option.value, option.label).to_lowercase();
                    let is_chosen = chosen.contains(&option.value);
                    let is_filtered = !haystack.contains(&query);
                    <button class="flex w-full items-center justify-between gap-2 rounded-md border-0! bg-transparent! px-3 py-2 text-left text-sm leading-5 text-popover-foreground shadow-none! hover:bg-sidebar-accent! focus:bg-sidebar-accent! focus:outline-none disabled:cursor-default! disabled:text-primary! disabled:opacity-100!" type="button" role="option" :aria-selected=$(if selected.get().is_empty() { false } else { raw!("JSON.parse(${selected}.get().dehydrate()).includes(${value}.dehydrate())", is_chosen) }) :disabled=$(if selected.get().is_empty() { false } else { raw!("JSON.parse(${selected}.get().dehydrate()).includes(${value}.dehydrate())", is_chosen) }) :hidden=$(if search.get().is_empty() { false } else { raw!("!${haystack}.dehydrate().includes(${search}.get().dehydrate().toLowerCase())", is_filtered) })
                        @click=$(|_event: Event| {
                            let _value = value.to_owned();
                            let _root_id = _root_id.to_owned();
                            raw!("(() => { const values = JSON.parse(${selected}.get().dehydrate()); if (!values.includes(${_value}.dehydrate())) { values.push(${_value}.dehydrate()); ${selected}.set(cx.hydrate(JSON.stringify(values))); document.getElementById(${_root_id}.dehydrate())?.dispatchEvent(new CustomEvent('selectionchange', {bubbles:true})); } })();", ());
                            search.set("".to_owned());
                            open.set(false);
                        })><span class="min-w-0 truncate">(option.label.as_str())</span><span aria-hidden="true" :hidden=$(if selected.get().is_empty() { true } else { raw!("!JSON.parse(${selected}.get().dehydrate()).includes(${value}.dehydrate())", !is_chosen) })>"✓"</span></button>
                }
            </div>
        </div>
    })
}
