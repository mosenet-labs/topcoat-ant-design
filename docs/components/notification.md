在视口右上角显示一条操作结果通知。组件进入浏览器 Top Layer；原生模态 Dialog 打开时，通知会临时挂载到当前 Dialog 内，因此仍显示在弹窗上方并可正常交互。

调用方维护消息 `Signal<String>`。消息非空时组件显示；计时结束、用户点击关闭按钮或调用方清空消息后，组件隐藏。组件不会保存业务状态，也不会发起服务端请求。

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `message` | `&Signal<String>` | 通知正文，也是组件的显示状态；空字符串表示隐藏。 |
| `title` | `&str` | 简短标题，例如“操作成功”。 |
| `tone` | `NotificationTone` | `Success`、`Info`、`Warning` 或 `Error`。 |
| `attrs` | `Attributes` | 可选；转发到通知根元素，调用方 class 会与组件 class 合并。 |

## 使用示例

```rust,ignore
use topcoat_ant_design::{NotificationTone, notification};
use topcoat::{
    Result,
    context::Cx,
    runtime::signal,
    view::{View, attributes, component, view},
};

#[component]
async fn save_example(cx: &Cx) -> Result<impl View> {
    let notice = signal(cx, String::new);

    Ok(view! {
        <button @click=$(|_e| notice.set("保存成功。".to_owned()))>
            "保存"
        </button>
        notification(
            message: &notice,
            title: "操作成功",
            tone: NotificationTone::Success,
            attrs: attributes! { data-source="save-form" },
        )
    })
}
```

## 交互行为

- 默认约 4.5 秒后自动关闭。
- 鼠标悬停或通知内部获得焦点时暂停关闭计时。
- 手动关闭和计时结束都会清空调用方传入的消息。
- 同一个消息 signal 只控制一条通知；组件不维护通知队列。

## 可访问性

`Success`、`Info` 和 `Warning` 使用 `role="status"` 与 `aria-live="polite"`。`Error` 使用 `role="alert"` 与 `aria-live="assertive"`，适合需要立即关注的失败结果。
