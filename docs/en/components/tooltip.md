# Tooltip

The official Tooltip displays a short hint on hover or focus. Give the trigger an accessible name and connect it to the hint with `aria-describedby`.

```rust,ignore
use topcoat_ant_design::{tooltip, tooltip_content};

view! {
    tooltip(
        <button aria-describedby="copy-hint" type="button">"Copy"</button>
        tooltip_content(attrs: attributes! { id="copy-hint" }, "Copy link")
    )
}
```
