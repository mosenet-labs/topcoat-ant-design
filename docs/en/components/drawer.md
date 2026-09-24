Show supplementary details from the right edge without leaving the current list, for example an event or audit record.

The caller provides a `Signal<bool>` for open state. By default, close actions set it to `false`. If a close URL is configured, clicking the backdrop or close button or pressing Escape navigates there instead. Scrolling stays inside the panel; it is up to 720 px wide on desktop and fills the viewport on mobile.

## Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `config` | `DrawerConfig<'_>` | Trusted DOM ID, title, and optional close URL. |
| `open` | `&Signal<bool>` | Open state, set to `false` by default close actions. |
| `language` | `UiLanguage` | Optional labels supplied by the component; defaults to English. Use `ChineseSimplified` for Chinese. |
| `attrs` | `Attributes` | Optional root attributes; caller and component classes are merged. |
| `child` | `Child` | Detail content. |

## Example

```rust,ignore
use topcoat_ant_design::{DrawerConfig, drawer};
use topcoat::{runtime::signal, view::view};

let open = signal(cx, || false);

Ok(view! {
    <button type="button" @click=$(|_e| open.set(true))>"View details"</button>
    drawer(config: DrawerConfig::new("event-detail", "Event details"), open: &open,
        <p>"Drawer content"</p>
    )
})
```

## Behavior

- Topcoat signal synchronizes `data-state` and accessibility state.
- The backdrop, close button, and Escape all close the drawer.
- `DrawerConfig::with_close_href` changes close behavior to navigation; otherwise `open` becomes `false`.
- Open and close transitions respect reduced-motion preferences.
- A closed drawer is invisible and does not receive pointer events.

The drawer does not construct browser URLs. For route-based details, the host should generate a URL without the current detail parameter and pass it to `with_close_href`.
