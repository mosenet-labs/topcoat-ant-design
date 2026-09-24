# Tooltip

Show complete text on hover or keyboard focus. This suits short explanations and truncated values such as commit IDs.

```rust,ignore
use topcoat_ant_design::tooltip;

view! {
    tooltip(id: "commit-tooltip", content: "4be48fb908424803fbe041ad5a50c0bf73f4f425",
        <span>"4be48fb9…"</span>
    )
}
```

- `id`: a page-unique DOM identifier provided by the caller.
- `content`: complete plain text; the component renders it as text, never as HTML.
- `child`: visible, non-interactive text or an icon. The wrapper can receive Tab focus, so do not nest a button or link.

The tooltip uses the browser Popover top layer and is not clipped by a scrolling table. It starts above the trigger, flips below when needed, and shifts near viewport edges while keeping its arrow pointed at the trigger. Position is recalculated on scroll and resize.

The pointer may enter the tooltip to select text. It closes after the pointer leaves both the trigger and tooltip and the trigger loses focus. Escape closes it without moving focus. Event listeners and positioning callbacks are removed when it closes or is unmounted.
