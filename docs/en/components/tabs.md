Provide route-based tabs for console detail pages.

`tabs` renders the navigation container and `tab_link` uses real links. The host route determines the active page, so refresh, browser history, and copied URLs all restore the correct content.

## Parameters

### `tabs`

| Parameter | Type | Description |
| --- | --- | --- |
| `label` | `&str` | Accessible name of the navigation region. |
| `attrs` | `Attributes` | Optional root `nav` attributes; caller and component classes are merged. |
| `child` | `Child` | A set of `tab_link` items. |

### `tab_link`

| Parameter | Type | Description |
| --- | --- | --- |
| `href` | `&str` | Real route for this tab. |
| `active` | `bool` | Whether this is the current page. |
| `attrs` | `Attributes` | Optional link attributes. |
| `child` | `Child` | Tab text or icon. |

## Example

```rust,ignore
use topcoat_ant_design::{tab_link, tabs};
use topcoat::router::request::uri;
use topcoat::view::view;

let webhook_url = format!("/projects/{project_id}/webhook");
let events_url = format!("/projects/{project_id}/events");
let current_path = uri(cx).path();

Ok(view! {
    tabs(label: "Project details",
        tab_link(
            href: webhook_url.as_str(),
            active: current_path == webhook_url,
            "Webhook configuration",
        )
        tab_link(
            href: events_url.as_str(),
            active: current_path == events_url,
            "Event history",
        )
    )
})
```

## Behavior

- Tabs are real `<a>` elements; routing is not hidden in temporary browser state.
- The active link emits `aria-current="page"`.
- Tabs may scroll horizontally on narrow screens.
- The host sets `active` from the current Topcoat route.
