在有限空间里展示多组可展开内容。多个条目共用一个 Topcoat `Signal<String>`，因此同一时间最多展开一组；点击已展开的标题会收起全部。

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `config` | `AccordionItemConfig` | 唯一 ID、标题、说明，可附加简短标记。 |
| `active` | `&Signal<String>` | 当前展开条目的 ID；空字符串表示全部收起。 |
| `selected_count` | `Option<&Signal<f64>>` | 可选；在标题上实时显示已启用项数量。 |
| `language` | `UiLanguage` | 可选；组件自带文案默认英文，传入 `ChineseSimplified` 切换中文。 |
| `attrs` | `Attributes` | 可选；转发到根元素，调用方 class 会与组件 class 合并。 |
| `child` | `Child` | 折叠内容。 |

## 使用示例

```rust,ignore
use topcoat_ant_design::{AccordionItemConfig, accordion_item};
use topcoat::{runtime::signal, view::view};

let active = signal(cx, String::new); // 初始全部收起
let selected = signal(cx, || 2.0);

Ok(view! {
    accordion_item(
        config: AccordionItemConfig::new("review-triggers", "评论 Trigger", "选择评论入口"),
        active: &active,
        selected_count: Some(&selected),
        <div class="p-4">"评论入口配置"</div>
    )
    accordion_item(
        config: AccordionItemConfig::new("event-triggers", "自动事件 Trigger", "选择事件入口"),
        active: &active,
        <div class="p-4">"事件入口配置"</div>
    )
})
```

标题是原生按钮，带有 `aria-controls` 和随状态变化的 `aria-expanded`；内容区带有对应的 `aria-labelledby`。展开动画复用 Collapse 的 Grid 过渡，并遵守系统减少动态效果设置。折叠只隐藏内容，不卸载输入控件，因此表单编辑状态会保留。
