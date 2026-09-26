# Tooltip 文字提示

官方 Tooltip 在悬停或聚焦时显示简短说明。触发元素需有可访问名称，并用 `aria-describedby` 关联提示。

```rust,ignore
use topcoat_ant_design::{tooltip, tooltip_content};

view! {
    tooltip(
        <button aria-describedby="copy-hint" type="button">"复制"</button>
        tooltip_content(attrs: attributes! { id="copy-hint" }, "复制链接")
    )
}
```
