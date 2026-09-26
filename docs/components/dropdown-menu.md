# Dropdown Menu 下拉菜单

官方 Dropdown Menu 基于原生 `details` 展开。组合触发元素、菜单面板与操作项，键盘可通过 Tab 到达各项。

```rust,ignore
use topcoat_ant_design::{dropdown_menu, dropdown_menu_trigger, dropdown_menu_content, dropdown_menu_item};

view! {
    dropdown_menu(
        dropdown_menu_trigger("操作")
        dropdown_menu_content(
            dropdown_menu_item("编辑")
            dropdown_menu_item("复制")
        )
    )
}
```
