# Accordion

The library exports Topcoat's official Accordion directly. Native `details` elements keep their content in the DOM. Give items the same `name` to allow one open item at a time.

```rust,ignore
use topcoat_ant_design::{accordion, accordion_item, accordion_trigger, accordion_content};

view! {
    accordion(
        accordion_item(attrs: attributes! { name="settings" open="" },
            accordion_trigger("Comments")
            accordion_content("Comment notifications are enabled.")
        )
        accordion_item(attrs: attributes! { name="settings" },
            accordion_trigger("Events")
            accordion_content("Event notifications are enabled.")
        )
    )
}
```
