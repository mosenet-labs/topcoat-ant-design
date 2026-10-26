# Dropdown Menu 下拉菜单

官方菜单使用浏览器 Popover API。点击外部或 Escape 关闭，Tab 在操作间移动；不提供方向键导航。用 `element_id(cx)` 生成触发器和面板共享的 ID，`close` 指定点击后关闭菜单。

```rust,ignore
use topcoat::view::element_id;
use topcoat_ant_design::{FloatingAlign, dropdown_menu, dropdown_menu_trigger, dropdown_menu_content, dropdown_menu_item};

view! {
    let menu = element_id(cx);
    dropdown_menu(
        dropdown_menu_trigger(target: &menu, "操作")
        dropdown_menu_content(id: &menu, align: FloatingAlign::Start,
            dropdown_menu_item(close: Some(&menu), "编辑")
            dropdown_menu_item(close: Some(&menu), "复制")
        )
    )
}
```
