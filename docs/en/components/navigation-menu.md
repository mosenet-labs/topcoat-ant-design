# Navigation Menu

Site navigation with ordinary links and optional panels. Hover, focus, or touch reveals panels without JavaScript. Use normal Tab navigation; arrow-key navigation and Escape dismissal are not provided.

```rust,ignore
use topcoat::runtime::{link_attrs, prefetch_mode};
use topcoat_ant_design::{navigation_menu, navigation_menu_list, navigation_menu_item,
    navigation_menu_trigger, navigation_menu_content, navigation_menu_link};

view! {
    navigation_menu(
        navigation_menu_list(
            navigation_menu_item(
                navigation_menu_trigger("Guides")
                navigation_menu_content(
                    navigation_menu_link(attrs: link_attrs(cx, "/docs", prefetch_mode(cx)), "Documentation")
                )
            )
        )
    )
}
```

Use `active` for a current-page link and `NavigationMenuLinkVariant::Trigger` for a top-level link. [Live example](/topcoat-ui#navigation_menu_card).
