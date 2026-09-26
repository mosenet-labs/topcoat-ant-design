# Accordion 手风琴

组件库直接导出 Topcoat 官方 Accordion。每项使用原生 `details`，折叠时内容仍保留在 DOM 中；相同的 `name` 让分组一次只展开一项。

```rust,ignore
use topcoat_ant_design::{accordion, accordion_item, accordion_trigger, accordion_content};

view! {
    accordion(
        accordion_item(attrs: attributes! { name="settings" open="" },
            accordion_trigger("评论")
            accordion_content("评论通知已启用。")
        )
        accordion_item(attrs: attributes! { name="settings" },
            accordion_trigger("事件")
            accordion_content("事件通知已启用。")
        )
    )
}
```
