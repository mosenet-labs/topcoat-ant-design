use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, Signal},
    view::{Attributes, View, class, component, view},
};

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
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let _ = cx;
    let chosen: Vec<String> = serde_json::from_str(&selected.get_untracked()).unwrap_or_default();
    let _root_id = id.to_owned();
    let input_id = format!("{id}-input");
    let list_id = format!("{id}-options");
    let caller_class = attrs.remove("class");
    Ok(view! {
        <div id=(id) class=(class!("group relative min-w-0", caller_class)) :data-open=$(open.get()) (attrs)
            @focusout=$(|_event: Event| {
                let _root_id = _root_id.to_owned();
                raw!("setTimeout(() => { const root = document.getElementById(${_root_id}.dehydrate()); if (root && !root.contains(document.activeElement)) ${open}.set(cx.hydrate(false)); }, 0);", ());
            })>
            <div class="flex min-h-9 flex-wrap items-center gap-1 rounded-md border border-control-border bg-white px-2 py-1 focus-within:border-primary-hover focus-within:ring-2 focus-within:ring-primary/10">
                for value in &chosen {
                    let value_for_click = value.clone();
                    <span class="inline-flex max-w-full items-center gap-1 rounded border border-[#b7d7ff] bg-primary-soft px-1.5 py-0.5 text-xs text-primary">
                        <span class="truncate">(options.iter().find(|option| option.value == *value).map_or(value.as_str(), |option| option.label.as_str()))</span>
                        <button class="border-0 bg-transparent p-0 text-primary hover:text-primary-hover" type="button" aria-label=(format!("移除 {value}")) :disabled=(disabled) @click=$(|event: Event| {
                            event.stop_propagation();
                            let _value = value_for_click.to_owned();
                            let _root_id = _root_id.to_owned();
                            raw!("(() => { const values = JSON.parse(${selected}.get().dehydrate()).filter(item => item !== ${_value}.dehydrate()); ${selected}.set(cx.hydrate(JSON.stringify(values))); document.getElementById(${_root_id}.dehydrate())?.dispatchEvent(new CustomEvent('selectionchange', {bubbles:true})); })();", ());
                        })>"×"</button>
                    </span>
                }
                <input id=(input_id.as_str()) class="h-6! min-w-[120px] flex-1 border-0! bg-transparent! px-1! py-0! text-sm! shadow-none! outline-none! ring-0!" type="search" role="combobox" aria-autocomplete="list" aria-controls=(list_id.as_str()) :aria-expanded=$(open.get()) :value=$(search.get()) placeholder="搜索上游模型" :disabled=(disabled)
                    @focus=$(|_event: Event| open.set(true))
                    @input=$(|event: Event| {
                        let _value = event.target.value;
                        raw!("${search}.set(cx.hydrate(${_value}.dehydrate().toLowerCase()));", ());
                        open.set(true);
                    })
                    @keydown=$(|_event: Event| {
                        let _list_id = list_id.to_owned();
                        raw!("{ const key = ${_event}.key.dehydrate(); if (key === 'ArrowDown' || key === 'Enter') { if (key === 'Enter') ${_event}.prevent_default(); const query = ${_event}.target.value.dehydrate().toLowerCase(); const first = [...(document.getElementById(${_list_id}.dehydrate())?.querySelectorAll('button[role=option]') ?? [])].find(option => !option.disabled && option.textContent.toLowerCase().includes(query)); if (first) { if (key === 'ArrowDown') ${_event}.prevent_default(); if (key === 'Enter') first.click(); else first.focus(); } } else if (key === 'Escape') { ${open}.set(cx.hydrate(false)); } }", ());
                    })>
            </div>
            <div id=(list_id.as_str()) class="absolute z-20 mt-1 hidden max-h-48 w-full overflow-y-auto rounded-md border border-control-border bg-white p-1 shadow-[0_6px_16px_rgba(0,0,0,0.08)] group-data-[open=true]:block" role="listbox" aria-label="上游模型" aria-multiselectable="true">
                #[key(option.value)] for option in options {
                    let value = option.value.clone();
                    let is_chosen = chosen.contains(&option.value);
                    <button class="flex w-full items-center justify-between gap-2 rounded-md border-0! bg-transparent! px-3 py-2 text-left text-sm leading-5 text-heading shadow-none! hover:bg-primary-soft! focus:bg-primary-soft! focus:outline-none disabled:cursor-default! disabled:text-primary! disabled:opacity-100! data-[filtered]:hidden" type="button" role="option" :aria-selected=(is_chosen) :disabled=(is_chosen) :data-filtered=$(if search.get().is_empty() { false } else { raw!("!${value}.dehydrate().toLowerCase().includes(${search}.get().dehydrate().toLowerCase())", false) })
                        @click=$(|_event: Event| {
                            let _value = value.to_owned();
                            let _root_id = _root_id.to_owned();
                            raw!("(() => { const values = JSON.parse(${selected}.get().dehydrate()); if (!values.includes(${_value}.dehydrate())) { values.push(${_value}.dehydrate()); ${selected}.set(cx.hydrate(JSON.stringify(values))); document.getElementById(${_root_id}.dehydrate())?.dispatchEvent(new CustomEvent('selectionchange', {bubbles:true})); } })();", ());
                            search.set("".to_owned());
                            open.set(false);
                        })><span class="min-w-0 truncate">(option.label.as_str())</span>if is_chosen { <span aria-hidden="true">"✓"</span> }</button>
                }
            </div>
        </div>
    })
}
