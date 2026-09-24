Display several expandable sections in a small area. Items share one Topcoat `Signal<String>`, so only one item can be open at a time. Clicking its heading again closes all items.

## Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `config` | `AccordionItemConfig` | Unique ID, title, description, and optional short badge. |
| `active` | `&Signal<String>` | ID of the open item; an empty string closes all items. |
| `selected_count` | `Option<&Signal<f64>>` | Optional live count displayed in the heading. |
| `language` | `UiLanguage` | Optional labels supplied by the component; defaults to English. Use `ChineseSimplified` for Chinese. |
| `attrs` | `Attributes` | Optional root attributes; caller and component classes are merged. |
| `child` | `Child` | Collapsible content. |

## Example

```rust,ignore
use topcoat_ant_design::{AccordionItemConfig, accordion_item};
use topcoat::{runtime::signal, view::view};

let active = signal(cx, String::new); // All items start closed.
let selected = signal(cx, || 2.0);

Ok(view! {
    accordion_item(
        config: AccordionItemConfig::new("review-triggers", "Comment trigger", "Choose comment entry points"),
        active: &active,
        selected_count: Some(&selected),
        <div class="p-4">"Comment entry settings"</div>
    )
    accordion_item(
        config: AccordionItemConfig::new("event-triggers", "Automatic event trigger", "Choose event entry points"),
        active: &active,
        <div class="p-4">"Event entry settings"</div>
    )
})
```

Headings are native buttons with `aria-controls` and a stateful `aria-expanded`. Content has the matching `aria-labelledby`. The animation reuses the Collapse grid transition and respects reduced-motion preferences. Closing an item hides its content without unmounting inputs, so edited form values remain intact.
