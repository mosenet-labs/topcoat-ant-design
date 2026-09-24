Show a lightweight confirmation bubble near a trigger button.

The component manages opening, closing, arrow placement, viewport-edge shifting, and vertical flipping. The caller supplies the confirm button as the child and performs the application action in its event handler.

## Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `id` | `&str` | DOM ID of the bubble, also used to associate the trigger. |
| `title` | `&str` | Confirmation question. |
| `description` | `Option<&str>` | Optional explanation of the effect. |
| `language` | `UiLanguage` | Optional labels supplied by the component; defaults to English. Use `ChineseSimplified` for Chinese. |
| `attrs` | `Attributes` | Optional root attributes; caller and component classes are merged. |
| `child` | `Child` | Caller-provided confirmation action. |

## Example

```rust,ignore
use topcoat_ant_design::{popconfirm, popconfirm_trigger_attributes};
use topcoat::{
    Result,
    context::Cx,
    view::{View, attributes, component, view},
};

#[component]
async fn disable_example(cx: &Cx) -> Result<impl View> {
    let trigger = popconfirm_trigger_attributes(cx, "disable-instance");

    Ok(view! {
        <button type="button" (trigger)>"Disable"</button>
        popconfirm(
            id: "disable-instance",
            title: "Disable this instance?",
            description: Some("It will stop processing tasks."),
            attrs: attributes! { data-scope="gitlab-instance" },
            <button type="button" @click=$(|_e| disable_instance())>
                "Confirm"
            </button>
        )
    })
}
```

## Behavior and accessibility

- Associate the trigger and bubble with `popconfirm_trigger_attributes`.
- Cancel, outside click, and Escape close the bubble.
- Activating the caller's confirm control first hides the bubble, then continues the business event.
- The bubble shifts near horizontal edges and flips above the button when there is insufficient room below.
- `role="alertdialog"` is associated with its title and optional description through `aria-labelledby` and `aria-describedby`; the trigger receives `aria-haspopup="dialog"` and `aria-controls`.

The ID becomes a DOM ID and CSS anchor name. Generate it from trusted code, not unchecked user input.
