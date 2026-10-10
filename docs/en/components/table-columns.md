# Table column settings

Compose `table_column_settings` with `data_table` to choose visible columns, keep essential columns visible, and reset defaults. Queries, pagination, sorting, and data loading remain with the host.

```rust,ignore
use topcoat::{runtime::signal, view::view};
use topcoat_ant_design::{
    TableColumn, data_table, table_column_attributes,
    table_column_settings, table_default_hidden_columns, table_toolbar,
};

let columns = [
    TableColumn::new("name", "Name").required(),
    TableColumn::new("namespace", "Namespace"),
    TableColumn::new("labels", "Label").hidden(),
];
let hidden = signal(cx, || table_default_hidden_columns(&columns));

view! {
    table_toolbar(title: "Pods",
        table_column_settings(id: "pod-columns", columns: &columns, hidden: &hidden)
    )
    data_table(label: "Pods",
        <colgroup>
            for column in &columns { <col (table_column_attributes(cx, column, &hidden))> }
        </colgroup>
        <thead><tr>
            for column in &columns {
                <th (table_column_attributes(cx, column, &hidden))>(column.label.as_str())</th>
            }
        </tr></thead>
        <tbody><tr>
            <td (table_column_attributes(cx, &columns[0], &hidden))>"api-7b8fc"</td>
            <td (table_column_attributes(cx, &columns[1], &hidden))>"production"</td>
            <td (table_column_attributes(cx, &columns[2], &hidden))>"app=api"</td>
        </tr></tbody>
    )
}
```

## API and state

- `TableColumn::new(key, label)` starts visible and hideable. Use `hidden()` for an initially hidden column and `required()` for a permanently visible column with a disabled checkbox. Keep at least an identity column required.
- Keys must be unique and stable within a table. Do not derive them from translated titles or positions.
- `hidden: &Signal<String>` contains hidden column keys as a JSON string array. Initialize it with `table_default_hidden_columns(&columns)`; an empty selection is `"[]"`.
- Apply `table_column_attributes(cx, column, &hidden)` to the corresponding header, every cell, and optional `col`. Server-rendered visibility and browser reactive bindings agree.
- Update empty or summary row `colspan` to the visible count. Fixed total table widths may leave extra space after hiding columns.
- `id` is a trusted, unique DOM identifier supplied by the host. Native Popover avoids table overflow clipping; Escape or an outside click closes it, while checkbox changes keep it open.
- `language` defaults to English and supports `UiLanguage::ChineseSimplified`. `attrs` forwards attributes to the outer control.
- The host owns state and its lifetime. This component does not add localStorage, remote preferences, or drag reordering.

For Kubernetes lists, configure Label as initially hidden in the host after upgrading to the published library. Visibility does not affect Label or Fields filters.
