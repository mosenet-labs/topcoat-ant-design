从视口右侧展示补充详情，适合事件、审计记录等不需要离开当前列表的内容。

Drawer 使用调用方提供的 `Signal<bool>` 表达开关状态。默认关闭交互会把状态恢复为 `false`；配置关闭路由后，遮罩、关闭按钮和 Escape 会导航到该地址。内容滚动限制在面板内部，桌面端宽度最大为 720px，移动端占满视口。

## 参数

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `config` | `DrawerConfig<'_>` | Drawer 的可信 DOM 标识、标题和可选关闭路由。 |
| `open` | `&Signal<bool>` | 打开状态；关闭交互会自动写回 `false`。 |
| `language` | `UiLanguage` | 可选；组件自带文案默认英文，传入 `ChineseSimplified` 切换中文。 |
| `attrs` | `Attributes` | 可选；转发到根元素，调用方 class 会与组件 class 合并。 |
| `child` | `Child` | Drawer 的详情内容。 |

## 使用示例

```rust,ignore
use topcoat_ant_design::{DrawerConfig, drawer};
use topcoat::{runtime::signal, view::view};

let open = signal(cx, || false);

Ok(view! {
    <button type="button" @click=$(|_e| open.set(true))>"查看详情"</button>
    drawer(config: DrawerConfig::new("event-detail", "事件详情"), open: &open,
        <p>"Drawer 内容"</p>
    )
})
```

## 交互行为

- 由 Topcoat signal 同步 `data-state` 和无障碍状态。
- 点击遮罩、关闭按钮或按 Escape 都会关闭。
- 通过 `DrawerConfig::with_close_href` 设置关闭地址后执行导航；未设置时写回 `open = false`。
- 打开和关闭均有平滑过渡；系统要求减少动态效果时取消过渡。
- 关闭后根元素不可见且不接收指针事件。

Drawer 不自行拼接浏览器 URL。路由型详情应由宿主生成不含当前详情参数的地址，并通过 `DrawerConfig::with_close_href` 设置。
