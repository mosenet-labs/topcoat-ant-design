use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    runtime::{Event, Signal},
    view::{Attributes, View, attributes, class, component, view},
};

use crate::{UiLanguage, anchored_menu_trigger_attributes, checkbox, icons::SETTING_OUTLINED};

/// Stable identity and initial visibility of a table column.
#[derive(Clone, Debug)]
pub struct TableColumn {
    pub key: String,
    pub label: String,
    pub default_visible: bool,
    pub hideable: bool,
}

impl TableColumn {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            default_visible: true,
            hideable: true,
        }
    }

    /// Hide this column until selected in the settings panel.
    pub fn hidden(mut self) -> Self {
        self.default_visible = false;
        self
    }

    /// Keep an essential column visible and disable its settings checkbox.
    pub fn required(mut self) -> Self {
        self.hideable = false;
        self.default_visible = true;
        self
    }
}

/// Initial value for the host's `Signal<String>` containing hidden column keys as JSON.
pub fn table_default_hidden_columns(columns: &[TableColumn]) -> String {
    serde_json::to_string(
        &columns
            .iter()
            .filter(|c| c.hideable && !c.default_visible)
            .map(|c| c.key.as_str())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// Apply the same attributes to a column's `th`, each `td`, and optional `col`.
pub fn table_column_attributes(
    cx: &Cx,
    column: &TableColumn,
    hidden: &Signal<String>,
) -> Attributes {
    let key = column.key.clone();
    if !column.hideable {
        return attributes! { cx => data-table-column=(key.as_str()) };
    }
    let initially_hidden = serde_json::from_str::<Vec<String>>(&hidden.get_untracked())
        .unwrap_or_default()
        .contains(&key);
    let hidden = hidden.clone();
    attributes! { cx =>
        data-table-column=(key.as_str())
        :hidden=$(if hidden.get().is_empty() { false } else {
            raw!("JSON.parse(${hidden}.get().dehydrate()).includes(${key}.dehydrate())", initially_hidden)
        })
    }
}

#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/en/components/table-columns.md"))]
#[component]
pub async fn table_column_settings(
    cx: &Cx,
    id: &str,
    columns: &[TableColumn],
    hidden: &Signal<String>,
    #[default] language: UiLanguage,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let defaults = table_default_hidden_columns(columns);
    let chosen: Vec<String> = serde_json::from_str(&hidden.get_untracked()).unwrap_or_default();
    let label = language.select("Columns", "列设置");
    let reset = language.select("Reset", "恢复默认");
    let caller_class = attrs.remove("class");
    attrs.extend(attributes! { cx => class=(class!("gr-table-column-settings", caller_class)) });
    let panel_style = format!("position-anchor: --gr-{id}");
    Ok(view! {
        <div (attrs)>
            <button type="button" class="gr-table-columns-trigger" aria-label=(label) title=(label) (anchored_menu_trigger_attributes(cx, id))>
                icon(data: SETTING_OUTLINED, size: 16)
            </button>
            <div id=(id) class="gr-table-columns-panel" popover="auto" role="group" aria-label=(label) style=(panel_style)>
                <div class="gr-table-columns-heading">
                    <strong>(label)</strong>
                    <button type="button" class="gr-table-columns-reset" @click=$(|_event: Event| hidden.set(defaults.clone()))>(reset)</button>
                </div>
                <div class="gr-table-columns-options">
                    #[key(column.key.clone())] for column in columns {
                        let key = column.key.clone();
                        let is_visible = !column.hideable || !chosen.contains(&key);
                        let required = !column.hideable;
                        <label class="gr-table-columns-option">
                            checkbox(attrs: attributes! {
                                :checked=$(if required { true } else if hidden.get().is_empty() { true } else {
                                    raw!("!JSON.parse(${hidden}.get().dehydrate()).includes(${key}.dehydrate())", is_visible)
                                })
                                disabled=(required)
                                @change=$(|_event: Event| {
                                    let _key = key.clone();
                                    raw!("(() => { const values = JSON.parse(${hidden}.get().dehydrate()); const key = ${_key}.dehydrate(); ${hidden}.set(cx.hydrate(JSON.stringify(values.includes(key) ? values.filter(value => value !== key) : [...values, key]))); })()", ());
                                })
                            })
                            <span>(column.label.as_str())</span>
                            if required { <small>(language.select("Required", "固定"))</small> }
                        </label>
                    }
                </div>
            </div>
        </div>
    })
}
