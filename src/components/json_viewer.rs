use crate::language::t;
use serde_json::Value;
use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    runtime::{Event, signal},
    view::{Attributes, View, ViewExt, class, component, element_id, view},
};

use crate::{
    UiLanguage,
    icons::{CHECK_OUTLINED, COPY_OUTLINED, NODE_EXPAND_OUTLINED},
};

#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/en/components/json-viewer.md"))]
#[component]
pub async fn json_viewer(
    cx: &Cx,
    value: &Value,
    #[default] language: UiLanguage,
    #[default("root")] root_label: &str,
    #[default(1)] expanded_depth: usize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let caller_class = attrs.remove("class");
    if !attrs.contains_key("aria-label") {
        attrs.insert(cx, "aria-label", &language.resolve(t!(json_data)));
    }
    let root = serde_json::to_string(root_label).unwrap();
    Ok(view! {
        <div class=(class!("gr-json-viewer", caller_class)) (attrs)>
            json_node(
                label: root.as_str(),
                value: value,
                depth: 0,
                expanded_depth: expanded_depth,
                language: language
            )
        </div>
    }
    .boxed())
}

#[component]
async fn json_node(
    label: &str,
    value: &Value,
    depth: usize,
    expanded_depth: usize,
    language: UiLanguage,
) -> Result<impl View> {
    let entries: Vec<(String, &Value)> = match value {
        Value::Object(map) => map
            .iter()
            .map(|(key, value)| (serde_json::to_string(key).unwrap(), value))
            .collect(),
        Value::Array(items) => items
            .iter()
            .enumerate()
            .map(|(index, value)| (index.to_string(), value))
            .collect(),
        _ => Vec::new(),
    };
    let (start, end) = if value.is_array() {
        ("[", "]")
    } else {
        ("{", "}")
    };
    let (kind, class) = match value {
        Value::String(_) => ("string", "gr-json-viewer-string"),
        Value::Number(_) => ("number", "gr-json-viewer-number"),
        Value::Bool(_) => ("boolean", "gr-json-viewer-boolean"),
        Value::Null => ("null", "gr-json-viewer-null"),
        _ => ("", "gr-json-viewer-bracket"),
    };
    let literal = if entries.is_empty() {
        serde_json::to_string(value).unwrap()
    } else {
        String::new()
    };
    let count = language.resolve(t!(json_items, count = entries.len()));
    Ok(view! {
        if entries.is_empty() {
            <div class="gr-json-viewer-leaf">
                <span class="gr-json-viewer-key">(label)</span>
                ": "
                if !kind.is_empty() {
                    <span class="gr-json-viewer-type">(kind)</span>
                }
                <span class=(class)>(literal.as_str())</span>
            </div>
        } else {
            <details class="gr-json-viewer-node" open=(depth < expanded_depth)>
                <summary>
                    <span class="gr-json-viewer-key">(label)</span>
                    ": "
                    <span class="gr-json-viewer-bracket">(start)</span>
                    <span class="gr-json-viewer-collapsed">
                        "…"
                        <span class="gr-json-viewer-bracket">(end)</span>
                    </span>
                    <span class="gr-json-viewer-count">(count)</span>
                    json_copy(value: value, label: label, language: language)
                    json_expand(label: label, language: language)
                </summary>
                <div class="gr-json-viewer-children">
                    #[key(key.clone())]
                    for (key, item) in &entries {
                        json_node(
                            label: key.as_str(),
                            value: item,
                            depth: depth + 1,
                            expanded_depth: expanded_depth,
                            language: language
                        )
                    }
                </div>
                <div class="gr-json-viewer-end">
                    <span class="gr-json-viewer-bracket">(end)</span>
                </div>
            </details>
        }
    }
    .boxed())
}

#[component]
async fn json_expand(cx: &Cx, label: &str, language: UiLanguage) -> Result<impl View> {
    let id = element_id(cx);
    let expand_label = language.resolve(t!(expand_all_descendants));
    Ok(view! {
        <button
            type="button"
            id=(id.as_str())
            class="gr-json-viewer-expand"
            aria-label=(format!("{expand_label}: {label}"))
            title=(expand_label)
            @click=$(|event: Event| {
                event.prevent_default();
                event.stop_propagation();
                raw!(
                    "(() => { const node = document.getElementById(${id}.dehydrate())?.closest('details.gr-json-viewer-node'); if (!node) return; node.open = true; for (const child of node.querySelectorAll('details.gr-json-viewer-node')) child.open = true; node.querySelector(':scope > summary')?.focus({ preventScroll: true }); })()",
                    (),
                );
            })
        >
            icon(data: NODE_EXPAND_OUTLINED, size: 13)
        </button>
    })
}

#[component]
async fn json_copy(cx: &Cx, value: &Value, label: &str, language: UiLanguage) -> Result<impl View> {
    let payload = serde_json::to_string_pretty(value).unwrap();
    let copy_label = language.resolve(t!(copy_node_json));
    let success = language.resolve(t!(copied));
    let failure = language.resolve(t!(copy_failed_try_again));
    let status = signal(cx, || 0usize);
    Ok(view! {
        <button
            type="button"
            class="gr-json-viewer-copy"
            aria-label=(format!("{copy_label}: {label}"))
            :title=$(if status.get() == 1usize { success.clone() } else if status.get() == 2usize { failure.clone() } else { copy_label.clone() })
            @click=$(async move |event: Event| {
                event.prevent_default();
                event.stop_propagation();
                let _value = payload.clone();
                let copied = raw!(
                    "cx.hydrate(await (async () => { const value = ${_value}.dehydrate(); try { if (globalThis.navigator?.clipboard?.writeText) { await globalThis.navigator.clipboard.writeText(value); return true; } } catch (_error) {} const active = document.activeElement; const field = document.createElement('textarea'); field.value = value; field.setAttribute('readonly', ''); field.style.cssText = 'position:fixed;left:-9999px;top:0;opacity:0;pointer-events:none'; (active?.closest('dialog') ?? document.body).appendChild(field); field.focus(); field.select(); field.setSelectionRange(0, field.value.length); let result = false; try { result = document.execCommand('copy'); } catch (_error) {} finally { field.remove(); if (active instanceof HTMLElement) active.focus({ preventScroll: true }); } return result; })())",
                    false,
                );
                status.set(if copied { 1usize } else { 2usize });
            })
        >
            <span :hidden=$(status.get() == 1usize)>
                icon(data: COPY_OUTLINED, size: 13)
            </span>
            <span :hidden=$(status.get() != 1usize)>
                icon(data: CHECK_OUTLINED, size: 13)
            </span>
            <span class="gr-json-viewer-status" role="status">
                $(if status.get() == 1usize { success.clone() } else if status.get() == 2usize { failure.clone() } else { copy_label.clone() })
            </span>
        </button>
    })
}
