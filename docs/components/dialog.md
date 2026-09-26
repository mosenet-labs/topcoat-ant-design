# Dialog 对话框

官方 Dialog 通过 `open` 表达式控制显示，内容由标题、说明和操作区组合。宿主负责打开状态和所需的模态焦点行为。

```rust,ignore
use topcoat_ant_design::{dialog, dialog_content, dialog_description, dialog_footer, dialog_header, dialog_title, button};

let open = signal(cx, || false);
view! {
    button(attrs: attributes! { @click=$(|_event: Event| open.set(true)) }, "编辑")
    dialog(open: $(open.get()), attrs: attributes! { aria-labelledby="edit-title" },
        dialog_content(
            dialog_header(
                dialog_title(attrs: attributes! { id="edit-title" }, "编辑连接")
                dialog_description("更新连接配置。")
            )
            dialog_footer(
                button(attrs: attributes! { @click=$(|_event: Event| open.set(false)) }, "关闭")
            )
        )
    )
}
```
