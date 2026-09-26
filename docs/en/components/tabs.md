# Tabs

The official Tabs support route links or browser state. This example uses a Topcoat signal to switch panels without reloading. Each `href` remains a directly accessible URL.

```rust,ignore
use topcoat::{runtime::{Event, signal}, view::{attributes, view}};
use topcoat_ant_design::{tabs, tabs_list, tabs_trigger, tabs_content};

let selected = signal(cx, || "overview".to_owned());

view! {
    tabs(attrs: attributes! { aria-label="Project details" },
        tabs_list(
            tabs_trigger(active: $(selected.get() == "overview"), attrs: attributes! {
                href="/project/overview"
                @click=$(|e: Event| { e.prevent_default(); selected.set("overview".to_owned()); })
            }, "Overview")
            tabs_trigger(active: $(selected.get() == "events"), attrs: attributes! {
                href="/project/events"
                @click=$(|e: Event| { e.prevent_default(); selected.set("events".to_owned()); })
            }, "Events")
        )
        tabs_content(attrs: attributes! { :hidden=$(selected.get() != "overview") }, "Overview content")
        tabs_content(attrs: attributes! { :hidden=$(selected.get() != "events") }, "Events content")
    )
}
```
