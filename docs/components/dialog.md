使用浏览器原生模态对话框承载需要集中填写或确认的内容。

Dialog 统一提供标题栏、关闭按钮、遮罩、进入动画与可访问性语义。调用方负责内容、底部操作区和具体业务提交；执行异步操作时通过 `busy` 锁定 Escape 和关闭按钮，避免请求中途丢失当前状态。

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `config` | `DialogConfig<'_>` | Dialog 的可信 DOM 标识、标题和可选眉题。 |
| `busy` | `&Signal<bool>` | 操作状态；为 `true` 时禁止关闭按钮和 Escape，受控关闭会等待操作结束。 |
| `open` | `Option<&Signal<bool>>` | 可选受控开关；默认 `None`，保留原生 command 开关方式。 |
| `title` | `Option<&Signal<String>>` | 可选动态标题；默认使用 `DialogConfig` 的静态标题。 |
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


## 单个受控弹窗

传入 `open` 后，通过同一个 Topcoat signal 打开与关闭弹窗；传入 `title` 可以让同一个弹窗承载新建、编辑等操作。表单字段和提交方式仍由宿主决定，可以直接使用原生 HTML POST 表单与服务端校验。

```rust,ignore
use topcoat_ant_design::{DialogConfig, UiLanguage, dialog};
use topcoat::{runtime::{Event, signal}, view::{attributes, view}};

let open = signal(cx, || false);
let busy = signal(cx, || false);
let title = signal(cx, || "新建 Provider".to_owned());
let name = signal(cx, String::new);
let api_key = signal(cx, String::new);

Ok(view! {
    <button type="button" @click=$(|_event: Event| {
        title.set("新建 Provider".to_owned());
        name.set("".to_owned());
        api_key.set("".to_owned());
        open.set(true);
    })>"新建"</button>
    dialog(
        config: DialogConfig::new("provider-editor", "Provider"),
        busy: &busy,
        open: Some(&open),
        title: Some(&title),
        language: UiLanguage::ChineseSimplified,
        attrs: attributes! { @close=$(|_event: Event| api_key.set("".to_owned())) },
        <form method="post" action="/providers/save">
            <input name="name" :value=$(name.get()) @input=$(|event: Event| name.set(event.target.value))>
            <input name="api_key" type="password" :value=$(api_key.get()) @input=$(|event: Event| api_key.set(event.target.value))>
            <button type="button" :disabled=$(busy.get()) @click=$(|_event| open.set(false))>"取消"</button>
            <button type="submit" :disabled=$(busy.get())>"保存"</button>
        </form>
    )
})
```

- 原生关闭按钮和 Escape 会把 `open` 同步为 `false`；调用方提供的 `@close` 仍会执行，可用于清理敏感输入。
- `busy` 为 `true` 时，`open = false` 暂不关闭原生弹窗；恢复 `false` 后再完成关闭。自定义取消按钮也应绑定 `:disabled=$(busy.get())`。
- 初始 `open = true` 适用于服务端校验失败后的重新渲染。组件等待 Topcoat 完成绑定后调用 `showModal()`，保证背景不可交互、原生焦点管理及 Escape 行为；不会用 HTML `open` 属性创建非模态弹窗。
- 浏览器原生 Dialog 的 `showModal` / `close` 尚无 Topcoat Rust facade，因此仅此 DOM 适配封装于组件内部。宿主不需要手写脚本、获取 DOM 或重新 hydration。
- 受控模式下，使用 signal 设置开关；不再混用 `dialog_trigger_attributes` 改变原生状态。省略 `open` 的现有调用保持原来的原生 command 行为。
