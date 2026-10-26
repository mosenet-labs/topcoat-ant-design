# Dropdown Menu

The official menu uses the browser Popover API. Outside click and Escape close it; Tab moves between actions. Arrow-key navigation is not provided. Use a stable element ID for the trigger and panel, and `close` to dismiss the menu after an action.

```rust,ignore
use topcoat::view::element_id;
use topcoat_ant_design::{FloatingAlign, dropdown_menu, dropdown_menu_trigger, dropdown_menu_content, dropdown_menu_item};

view! {
    let menu = element_id(cx);
    dropdown_menu(
        dropdown_menu_trigger(target: &menu, "Actions")
        dropdown_menu_content(id: &menu, align: FloatingAlign::Start,
            dropdown_menu_item(close: Some(&menu), "Edit")
            dropdown_menu_item(close: Some(&menu), "Duplicate")
        )
    )
}
```
