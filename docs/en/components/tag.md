# Tag

Display a status or category with a soft background, subtle border, and semantic color inspired by Ant Design Tag.

```rust
use topcoat_ant_design::{TagTone, tag};

view! {
    tag(tone: TagTone::Success, "Enabled")
    tag(tone: TagTone::Default, "Disabled")
}
```

`tone` supports `Default` (gray), `Success` (green), `Warning` (gold), `Error` (red), and `Processing` (blue). Pass additional attributes through `attrs`; the child supplies the label. Always include explicit status text instead of relying on color alone. This version is for static display and has no close or multi-select behavior.
