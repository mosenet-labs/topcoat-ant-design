# Tabs 标签页

官方 Tabs 可与真实路由或浏览器状态配合。宿主提供每个触发项的 `active` 状态和对应面板内容。

```rust,ignore
use topcoat_ant_design::{tabs, tabs_list, tabs_trigger, tabs_content};

view! {
    tabs(attrs: attributes! { aria-label="项目详情" },
        tabs_list(
            tabs_trigger(active: true, attrs: attributes! { href="/project/overview" }, "概览")
            tabs_trigger(active: false, attrs: attributes! { href="/project/events" }, "事件")
        )
        tabs_content("概览内容")
    )
}
```
