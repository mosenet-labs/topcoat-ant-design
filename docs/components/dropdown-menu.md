在触发按钮旁显示一组紧凑操作。菜单使用浏览器 Popover API 和 CSS 锚点定位；点击外部或按 Escape 关闭。调用方提供普通按钮或表单，负责具体操作。

通过 `dropdown_menu_trigger_attributes(cx, id)` 关联触发按钮。`id` 同时用于 DOM ID 和 CSS 锚点名称，应由可信代码生成。

## 示例

```rust,ignore
use topcoat::runtime::signal;
use topcoat_ant_design::{dropdown_menu, dropdown_menu_trigger_attributes};

let last_action = signal(cx, String::new);
let trigger = dropdown_menu_trigger_attributes(cx, "project-actions");

Ok(view! {
    <button type="button" (trigger)>"操作"</button>
    dropdown_menu(id: "project-actions", label: "项目操作",
        <button type="button" @click=$(|_e| last_action.set("编辑".to_owned()))>"编辑"</button>
        <button type="button" @click=$(|_e| last_action.set("复制".to_owned()))>"复制"</button>
        <button type="button" disabled="">"暂不可用"</button>
    )
})
```
