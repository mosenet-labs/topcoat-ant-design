# Tabs

The official Tabs support route links or browser state. The host supplies each trigger's `active` value and the matching panel content.

```rust,ignore
use topcoat_ant_design::{tabs, tabs_list, tabs_trigger, tabs_content};

view! {
    tabs(attrs: attributes! { aria-label="Project details" },
        tabs_list(
            tabs_trigger(active: true, attrs: attributes! { href="/project/overview" }, "Overview")
            tabs_trigger(active: false, attrs: attributes! { href="/project/events" }, "Events")
        )
        tabs_content("Overview content")
    )
}
```
