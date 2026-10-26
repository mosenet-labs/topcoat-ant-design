# Floating

Shared official CSS positioning for Popover, Dropdown Menu, Tooltip and Hover Card. `FloatingSide` selects the direction; `FloatingAlign` selects alignment. `FloatingAnchor::Invoker` follows a native popover trigger, while `Scope` follows an ancestor with `FLOATING_SCOPE`.

```rust,ignore
use topcoat::view::{class, element_id};
use topcoat_ant_design::{FloatingAlign, FloatingAnchor, FloatingSide, floating_variants};

view! {
    let panel = element_id(cx);
    <button type="button" popovertarget=(&panel)>"Open panel"</button>
    <div id=(&panel) popover="auto" class=(class!(
        floating_variants(FloatingAnchor::Invoker, FloatingSide::Right, FloatingAlign::Start),
        "rounded-lg border border-border bg-popover p-4 text-popover-foreground",
    ))>"Panel content"</div>
}
```

The browser tries other positions when space is insufficient. Use modern browsers with Popover and CSS anchor positioning support. [Live example](/topcoat-ui#floating_card).
