# Floating 浮层定位

Popover、Dropdown Menu、Tooltip 和 Hover Card 共用的官方 CSS 定位基础。`FloatingSide` 选择方向，`FloatingAlign` 选择对齐。`FloatingAnchor::Invoker` 跟随原生 Popover 触发按钮，`Scope` 跟随带有 `FLOATING_SCOPE` 的祖先元素。

```rust,ignore
use topcoat::view::{class, element_id};
use topcoat_ant_design::{FloatingAlign, FloatingAnchor, FloatingSide, floating_variants};

view! {
    let panel = element_id(cx);
    <button type="button" popovertarget=(&panel)>"打开面板"</button>
    <div id=(&panel) popover="auto" class=(class!(
        floating_variants(FloatingAnchor::Invoker, FloatingSide::Right, FloatingAlign::Start),
        "rounded-lg border border-border bg-popover p-4 text-popover-foreground",
    ))>"面板内容"</div>
}
```

视口空间不足时浏览器尝试其他位置。需要支持 Popover 和 CSS 锚点定位的现代浏览器。[交互示例](/topcoat-ui?lang=zh#floating_card)。
