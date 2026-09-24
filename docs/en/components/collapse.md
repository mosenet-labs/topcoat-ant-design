Provide a reversible expand-and-collapse transition for content of unknown height.

Collapse is a Topcoat `#[component]`. The caller owns a `Signal<bool>`; the component renders reactive state attributes and content structure. `collapse_trigger_attributes` supplies `aria-controls`, `aria-expanded`, and click handling to a trigger.

## Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `id` | `&str` | DOM identifier of the collapsible content. |
| `open` | `&Signal<bool>` | Whether the content is expanded. |
| `attrs` | `Attributes` | Optional root attributes; caller and component classes are merged. |
| `child` | `Child` | Content of any height. |

## Example

```rust,ignore
use topcoat_ant_design::{collapse, collapse_trigger_attributes};
use topcoat::{
    runtime::signal,
    view::{attributes, view},
};

// The root layout has already called topcoat_ant_design::head_assets().
let open = signal(cx, || true);
let trigger = collapse_trigger_attributes(cx, "advanced-options", &open);

Ok(view! {
    <button type="button" (trigger)>"Expand / collapse"</button>
    collapse(
        id: "advanced-options",
        open: &open,
        attrs: attributes! { aria-label="Advanced options" },
        <div>"Content of any height"</div>
    )
})
```

## Behavior

- The Topcoat signal is the sole source of open state; the trigger and content read the same signal.
- `collapse` emits `data-state`, `aria-hidden`, and the animation structure.
- `collapse_trigger_attributes` uses Topcoat `attributes!`, bound attributes, and `@click`; callers do not duplicate the state logic.
- A CSS Grid `0fr`/`1fr` track handles unknown height without JavaScript measurement.
- Toggling mid-transition reverses naturally from the current position.
- Closed content becomes invisible and does not receive pointer events.
- `prefers-reduced-motion` disables the transition when requested by the user.
