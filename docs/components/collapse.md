为未知高度内容提供可逆的展开与收起过渡。

Collapse 是 Topcoat `#[component]`。调用方维护 `Signal<bool>`，组件负责响应式状态属性和内容结构；`collapse_trigger_attributes` 负责触发控件的 `aria-controls`、`aria-expanded` 与点击切换。

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `id` | `&str` | 折叠内容的 DOM 标识。 |
| `open` | `&Signal<bool>` | 展开状态。 |
| `attrs` | `Attributes` | 可选；转发到根元素，调用方 class 会与组件 class 合并。 |
| `child` | `Child` | 任意高度的内容。 |

## 使用示例

```rust,ignore
use topcoat_ant_design::{collapse, collapse_trigger_attributes};
use topcoat::{
    runtime::signal,
    view::{attributes, view},
};

// 根布局已经调用 topcoat_ant_design::head_assets()。
let open = signal(cx, || true);
let trigger = collapse_trigger_attributes(cx, "advanced-options", &open);

Ok(view! {
    <button type="button" (trigger)>"展开/收起"</button>
    collapse(
        id: "advanced-options",
        open: &open,
        attrs: attributes! { aria-label="高级选项" },
        <div>"任意高度的内容"</div>
    )
})
```

## 交互行为

- Topcoat `signal` 是唯一的开关状态，触发属性和内容属性读取同一个 signal。
- `collapse` 自动输出 `data-state`、`aria-hidden` 以及动画所需结构。
- `collapse_trigger_attributes` 使用 Topcoat `attributes!`、绑定属性和 `@click`，调用方不需要重复状态逻辑。
- 组件通过 CSS Grid 的 `0fr`/`1fr` 轨道处理未知内容高度，不执行 JavaScript 测量。
- 动画中途再次切换状态时，会从当前位置自然反向。
- 关闭完成后内容不可见，也不会响应鼠标。
- 用户启用减少动态效果时，`prefers-reduced-motion` 会关闭过渡。
