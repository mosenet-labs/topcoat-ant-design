使用浏览器原生模态对话框承载需要集中填写或确认的内容。

Dialog 统一提供标题栏、关闭按钮、遮罩、进入动画与可访问性语义。调用方负责内容、底部操作区和具体业务提交；执行异步操作时通过 `busy` 锁定 Escape 和关闭按钮，避免请求中途丢失当前状态。

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `config` | `DialogConfig<'_>` | Dialog 的可信 DOM 标识、标题和可选眉题。 |
| `busy` | `&Signal<bool>` | 异步操作状态；为 `true` 时禁止关闭按钮和 Escape。 |
| `language` | `UiLanguage` | 可选；组件自带文案默认英文，传入 `ChineseSimplified` 切换中文。 |
| `attrs` | `Attributes` | 可选；转发到根元素，调用方 class 会与组件 class 合并。 |
| `child` | `Child` | 调用方提供的表单、内容与底部操作区。 |

## 使用示例

```rust,ignore
use topcoat_ant_design::{
    DialogConfig, dialog, dialog_close_attributes, dialog_trigger_attributes,
};
use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, signal},
    view::{View, attributes, component, view},
};

#[component]
async fn connection_dialog(cx: &Cx) -> Result<impl View> {
    let busy = signal(cx, || false);
    let trigger = dialog_trigger_attributes(cx, "connection-dialog");
    let cancel = dialog_close_attributes(cx, "connection-dialog");

    Ok(view! {
        <button type="button" (trigger)>"编辑连接"</button>
        dialog(
            config: DialogConfig::new("connection-dialog", "编辑连接")
                .with_eyebrow("CONNECTION"),
            busy: &busy,
            attrs: attributes! { @close=$(|_event: Event| clear_secret()) },
            <form @submit=$(|event: Event| event.prevent_default())>
                <div>"表单内容"</div>
                <footer>
                    <button type="button" (cancel)>"取消"</button>
                    <button type="submit">"保存"</button>
                </footer>
            </form>
        )
    })
}
```

## 交互行为

- `dialog_trigger_attributes` 使用原生 `commandfor="…" command="show-modal"` 打开模态框。
- `dialog_close_attributes` 使用原生 `command="close"` 关闭模态框。
- 标题栏关闭按钮由组件提供；关闭后的业务状态清理可通过根元素的 `@close` 处理。
- `busy = true` 时，标题栏关闭按钮不可用，按 Escape 不会中断正在执行的操作。
- 打开时使用轻量缩放和淡入动画；系统要求减少动态效果时取消动画。

`id` 会用于 DOM 标识以及标题关联，只能使用由代码生成的可信值，不能直接使用未经校验的用户输入。

## 可访问性

Dialog 使用原生 `<dialog>`，并声明 `role="dialog"`、`aria-modal="true"` 和 `aria-labelledby`。触发按钮会获得 `aria-haspopup="dialog"` 与 `aria-controls`；关闭按钮提供明确的中文标签。
