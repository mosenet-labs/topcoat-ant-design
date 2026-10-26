# Popover

The official panel opens without JavaScript. Outside click or Escape closes it. Use stable IDs and choose a side/alignment; the browser tries fallback positions near window edges.

```rust,ignore
use topcoat::view::element_id;
use topcoat_ant_design::{FloatingAlign, popover, popover_trigger, popover_content};

view! {
    let panel = element_id(cx);
    popover(
        popover_trigger(target: &panel, "Filters")
        popover_content(id: &panel, align: FloatingAlign::End,
            <p>"Place your form or controls here."</p>
        )
    )
}
```

`attrs` forwards classes and attributes. Use `side: FloatingSide::Top` or `align: FloatingAlign::Start` to change positioning. Components are exported directly from `topcoat_ant_design`. [Live example](/topcoat-ui#popover_card).
