# SearchMultiSelect

`search_multi_select` combines a search field, a multi-select listbox, and removable selected tags. The caller supplies all options and owns three Topcoat signals: a JSON array of selected values, the search text, and the open state. It does not fetch options.

```rust,ignore
use topcoat::runtime::signal;
use topcoat_ant_design::{SearchOption, search_multi_select};

let options = vec![
    SearchOption { value: "gpt-4.1".into(), label: "GPT-4.1".into() },
    SearchOption { value: "o3".into(), label: "o3".into() },
];
let selected = signal(cx, || r#"["gpt-4.1"]"#.to_owned());
let search = signal(cx, String::new);
let open = signal(cx, || false);

view! {
    search_multi_select(
        id: "models", options: &options, selected: &selected,
        search: &search, open: &open, label: Some("Models"),
    )
    <output>$(selected.get())</output>
}
```

The component updates `selected` when an option is chosen or a tag is removed. It also dispatches a bubbling `selectionchange` event from the root element, so the host can synchronize related fields. Search matches both option labels and values. The component accepts `language`, `placeholder`, `disabled`, and root `attrs`; its colors follow the library's light and dark theme tokens.

For a read-only field, pass `disabled: true` while keeping the selected JSON array:

```rust,ignore
search_multi_select(
    id: "locked-models", options: &options, selected: &selected,
    search: &search, open: &open, disabled: true,
)
```
