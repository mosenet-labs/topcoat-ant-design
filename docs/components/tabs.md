# Tabs 标签页

官方 Tabs 可与真实路由或浏览器状态配合。下面的示例用 Topcoat signal 在浏览器中切换面板，不刷新页面；`href` 保留为可直接访问的地址。

```rust,ignore
use topcoat::{runtime::{Event, signal}, view::{attributes, view}};
use topcoat_ant_design::{tabs, tabs_list, tabs_trigger, tabs_content};

let selected = signal(cx, || "overview".to_owned());

view! {
    tabs(attrs: attributes! { aria-label="项目详情" },
        tabs_list(
            tabs_trigger(active: $(selected.get() == "overview"), attrs: attributes! {
                href="/project/overview"
                @click=$(|e: Event| { e.prevent_default(); selected.set("overview".to_owned()); })
            }, "概览")
            tabs_trigger(active: $(selected.get() == "events"), attrs: attributes! {
                href="/project/events"
                @click=$(|e: Event| { e.prevent_default(); selected.set("events".to_owned()); })
            }, "事件")
        )
        tabs_content(attrs: attributes! { :hidden=$(selected.get() != "overview") }, "概览内容")
        tabs_content(attrs: attributes! { :hidden=$(selected.get() != "events") }, "事件内容")
    )
}
```
