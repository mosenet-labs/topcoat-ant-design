Provide a data table with horizontal overflow, density options, and a composable pagination area.

`data_table` preserves native `table`, `thead`, `tbody`, `th`, and `td` semantics. It standardizes presentation and container behavior while the host owns columns, data, sorting, and requests.

`table_pagination` is composed separately. The host can use real links for server or cursor pagination, or Topcoat `@click` buttons for browser-side pagination. `table_page_size_select` supplies a consistent page-size selector; query state still belongs to the host.

## Parameters

### `data_table`

| Parameter | Type | Description |
| --- | --- | --- |
| `label` | `&str` | Accessible name of the table. |
| `density` | `DataTableDensity` | Optional; defaults to `Default`. Use `Compact` for logs and event lists. |
| `attrs` | `Attributes` | Optional native `table` attributes; classes are merged. |
| `child` | `Child` | Native `thead`, `tbody`, and optional `colgroup`. |

### `table_pagination`

| Parameter | Type | Description |
| --- | --- | --- |
| `summary` | `&str` | Record count or pagination description on the left. |
| `label` | `&str` | Accessible name of pagination navigation. |
| `attrs` | `Attributes` | Optional root `footer` attributes. |
| `child` | `Child` | Previous, current, and next links, buttons, or disabled text. |

### `table_page_size_select`

| Parameter | Type | Description |
| --- | --- | --- |
| `id` | `&str` | Stable ID of the native `select`. |
| `label` | `&str` | Visible and accessible label. |
| `attrs` | `Attributes` | Optional native `select` attributes, including Topcoat `:value` and `@change`. |
| `child` | `Child` | Native `option` elements. |

## Numbered pagination

This example updates Topcoat signals in the browser without a full-page navigation:

```rust,ignore
use topcoat_ant_design::{
    DataTableDensity, data_table, table_page_size_select, table_pagination,
};
use topcoat::{runtime::{Event, signal}, view::{attributes, view}};

let page = signal(cx, || "1".to_owned());
let page_size = signal(cx, || "10".to_owned());

view! {
    data_table(label: "Projects", density: DataTableDensity::Compact,
        <thead><tr><th>"Project"</th><th>"Status"</th></tr></thead>
        <tbody>
            <tr><td>"devops/review"</td><td>"Active"</td></tr>
        </tbody>
    )
    table_pagination(summary: "18 projects", label: "Project pagination",
        table_page_size_select(id: "project-page-size", label: "Rows per page",
            attrs: attributes! {
                :value=$(page_size.get())
                @change=$(|event: Event| {
                    page_size.set(event.target.value);
                    page.set("1".to_owned());
                })
            },
            <option value="10">"10"</option>
            <option value="20">"20"</option>
        )
        <button type="button" :disabled=$(page.get() == "1") @click=$(|_e| page.set("1".to_owned()))>"Previous"</button>
        <span aria-current="page">$(page.get()) " / 2"</span>
        <button type="button" :disabled=$(page.get() == "2") @click=$(|_e| page.set("2".to_owned()))>"Next"</button>
    )
}
```

## Default density

Omit `density` for taller rows that contain supporting text or actions:

```rust,ignore
data_table(label: "Users",
    <thead><tr><th>"Account"</th><th>"Role"</th><th>"Status"</th></tr></thead>
    <tbody>
        <tr><td>"demo-admin"</td><td>"Admin"</td><td>"Enabled"</td></tr>
    </tbody>
)
```

## Server or cursor pagination

The caller constructs a URL containing filters and the cursor. The component preserves real links:

```rust,ignore
table_pagination(summary: "Stable time cursor", label: "Event pagination",
    if let Some(next_url) = next_url {
        <a href=(next_url)>"Next"</a>
    } else {
        <span aria-disabled="true">"No more results"</span>
    }
)
```

## Conventions

- Mark the current page with `aria-current="page"`.
- Use native `disabled` on unavailable buttons and `aria-disabled="true"` on unavailable text.
- Reset to page one when page size changes to avoid an out-of-range empty page.
- Keep row navigation as a real link in a cell rather than making `tr` act as a button.
- The table scrolls horizontally in a narrow container. The host may set a business-specific minimum width through `attrs`.
