在触发按钮附近显示轻量二次确认气泡。

组件负责打开与关闭、箭头定位、视口边缘偏移和上下翻转。调用方通过 child 提供确认按钮，并在按钮事件中执行具体业务操作。

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `id` | `&str` | 气泡 DOM 标识，同时用于关联触发按钮。 |
| `title` | `&str` | 确认问题。 |
| `description` | `Option<&str>` | 可选的影响说明。 |
| `attrs` | `Attributes` | 可选；转发到气泡根元素，调用方 class 会与组件 class 合并。 |
| `child` | `Child` | 调用方提供的确认操作区域。 |

## 使用示例

```rust,ignore
use topcoat_ant_design::{popconfirm, popconfirm_trigger_attributes};
use topcoat::{
    Result,
    context::Cx,
    view::{View, attributes, component, view},
};

#[component]
async fn disable_example(cx: &Cx) -> Result<impl View> {
    let trigger = popconfirm_trigger_attributes(cx, "disable-instance");

    Ok(view! {
        <button type="button" (trigger)>"停用"</button>
        popconfirm(
            id: "disable-instance",
            title: "确认停用此实例？",
            description: Some("停用后将不再处理任务。"),
            attrs: attributes! { data-scope="gitlab-instance" },
            <button type="button" @click=$(|_e| disable_instance())>
                "确认"
            </button>
        )
    })
}
```

## 交互行为

- 使用 `popconfirm_trigger_attributes` 把触发按钮与气泡关联。
- 点击取消、气泡外部或按下 `Escape` 时关闭。
- 点击调用方提供的确认控件后，组件立即恢复隐藏状态，再继续执行业务事件。
- 靠近左右边缘时气泡会平移；下方空间不足时会翻转到按钮上方。

`id` 会参与 DOM 标识和 CSS 锚点名称，只能使用由代码生成的可信值，不能直接使用未经校验的用户输入。

## 可访问性

气泡使用 `role="alertdialog"`，并通过 `aria-labelledby` 和可选的 `aria-describedby` 关联标题与说明。触发按钮会获得 `aria-haspopup="dialog"` 和 `aria-controls`。
