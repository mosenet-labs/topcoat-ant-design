# Navigation Menu 导航菜单

由普通链接和可展开面板组成的站点导航。鼠标悬停、键盘聚焦或触屏操作即可打开面板，无需 JavaScript。使用普通 Tab 导航，不提供方向键导航和 Escape 关闭。

```rust,ignore
use topcoat::runtime::{link_attrs, prefetch_mode};
use topcoat_ant_design::{navigation_menu, navigation_menu_list, navigation_menu_item,
    navigation_menu_trigger, navigation_menu_content, navigation_menu_link};

view! {
    navigation_menu(
        navigation_menu_list(
            navigation_menu_item(
                navigation_menu_trigger("指南")
                navigation_menu_content(
                    navigation_menu_link(attrs: link_attrs(cx, "/docs", prefetch_mode(cx)), "文档")
                )
            )
        )
    )
}
```

通过 `active` 标记当前页面链接，顶层链接使用 `NavigationMenuLinkVariant::Trigger`。[交互示例](/topcoat-ui?lang=zh#navigation_menu_card)。
