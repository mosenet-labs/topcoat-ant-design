Show a compact set of actions next to a trigger button.

The dropdown uses the browser Popover API and CSS anchor positioning. Outside click and Escape close it. Child controls remain ordinary buttons or forms, so the caller owns each action.

## Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `id` | `&str` | DOM ID shared by the menu and its trigger. |
| `label` | `&str` | Accessible name for the group of actions. |
| `attrs` | `Attributes` | Optional menu attributes; classes are merged. |
| `child` | `Child` | Caller-provided action controls. |

Apply `dropdown_menu_trigger_attributes(cx, id)` to the trigger button. Generate `id` from trusted code because it is also used as a CSS anchor name.

## Example

```rust,ignore
use topcoat::runtime::signal;
use topcoat_ant_design::{dropdown_menu, dropdown_menu_trigger_attributes};

let last_action = signal(cx, String::new);
let trigger = dropdown_menu_trigger_attributes(cx, "project-actions");

Ok(view! {
    <button type="button" (trigger)>"Actions"</button>
    dropdown_menu(id: "project-actions", label: "Project actions",
        <button type="button" @click=$(|_e| last_action.set("Edit".to_owned()))>"Edit"</button>
        <button type="button" @click=$(|_e| last_action.set("Duplicate".to_owned()))>"Duplicate"</button>
        <button type="button" disabled="">"Unavailable"</button>
    )
})
```
