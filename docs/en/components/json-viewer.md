# JSONViewer

Render a `serde_json::Value` as a tree with native, keyboard-accessible `details` nodes, type annotations, escaped text, and light/dark theme colors.

```rust,ignore
use topcoat_ant_design::json_viewer;

view! {
    json_viewer(value: &data)
}
```

| Parameter | Type | Description |
| --- | --- | --- |
| `value` | `&serde_json::Value` | Data to display; the host loads it. |
| `root_label` | `&str` | Root name, default `root`. |
| `expanded_depth` | `usize` | Initial expanded levels, default `1` (root only); `0` collapses everything. |
| `language` | `UiLanguage` | English by default; supports Simplified Chinese. |
| `attrs` | `Attributes` | Root attributes, including classes and accessible name. |

Hover or focus a collapsed nonempty object or array to reveal its copy and expand-all icon buttons; touch devices always show both.

- **Copy node JSON** writes that node's complete, pretty-printed JSON without expanding it and reports success or failure. The Clipboard API has a fallback that also works inside native dialogs.
- **Expand all descendants** opens the current node and every descendant object or array, leaving other branches unchanged. Focus moves to the current node's summary for continued keyboard navigation. Clicking the summary or its arrow still toggles only the current node.

Empty objects, empty arrays, and primitives display directly. The viewer preserves all fields. Domain-specific filtering, such as removing Kubernetes `metadata.managedFields`, belongs to the host. Expansion is owned by the current DOM; `expanded_depth` is an initial setting.
