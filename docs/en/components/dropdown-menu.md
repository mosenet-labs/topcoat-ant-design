# Dropdown Menu

The official Dropdown Menu uses a native `details` disclosure. Compose its trigger, panel, and items; browser Tab navigation reaches the actions.

```rust,ignore
use topcoat_ant_design::{dropdown_menu, dropdown_menu_trigger, dropdown_menu_content, dropdown_menu_item};

view! {
    dropdown_menu(
        dropdown_menu_trigger("Actions")
        dropdown_menu_content(
            dropdown_menu_item("Edit")
            dropdown_menu_item("Duplicate")
        )
    )
}
```
