# Popover 弹出面板

官方面板不需要 JavaScript 即可打开，点击外部或 Escape 关闭。触发器和面板共享稳定 ID，可指定方向与对齐，浏览器会在视口空间不足时尝试其他位置。

```rust,ignore
use topcoat::view::element_id;
use topcoat_ant_design::{FloatingAlign, popover, popover_trigger, popover_content};

view! {
    let panel = element_id(cx);
    popover(
        popover_trigger(target: &panel, "筛选条件")
        popover_content(id: &panel, align: FloatingAlign::End,
            <p>"在这里放置表单或操作控件。"</p>
        )
    )
}
```

`attrs` 可转发样式与属性。通过 `side: FloatingSide::Top` 或 `align: FloatingAlign::Start` 改变定位。所有组件直接从 `topcoat_ant_design` 导出。[交互示例](/topcoat-ui?lang=zh#popover_card)。
