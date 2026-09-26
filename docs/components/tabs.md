提供适合控制台详情页的路由型页签。

`tabs` 负责导航容器，`tab_link` 使用真实链接表达当前页。当前页由宿主路由判断，因此刷新、浏览器前进后退和复制地址都能恢复正确内容。

## 参数

### `tabs`

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `label` | `&str` | 导航区域的无障碍名称。 |
| `attrs` | `Attributes` | 可选；转发到根 `nav`，调用方 class 会与组件 class 合并。 |
| `child` | `Child` | 一组 `tab_link`。 |

### `tab_link`

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `href` | `&str` | 页签对应的真实路由。 |
| `active` | `Expr<bool>`（可直接传 `bool`） | 是否为当前页面，也可随 signal 响应式更新。 |
| `attrs` | `Attributes` | 可选；转发到链接。 |
| `child` | `Child` | 页签文字或图标。 |

## 使用示例

```rust,ignore
use topcoat_ant_design::{tab_link, tabs};
use topcoat::router::request::uri;
use topcoat::view::view;

let webhook_url = format!("/projects/{project_id}/webhook");
let events_url = format!("/projects/{project_id}/events");
let current_path = uri(cx).path();

Ok(view! {
    tabs(label: "项目详情",
        tab_link(
            href: webhook_url.as_str(),
            active: current_path == webhook_url,
            "Webhook 配置",
        )
        tab_link(
            href: events_url.as_str(),
            active: current_path == events_url,
            "事件记录",
        )
    )
})
```

## 交互行为

- 组件使用真实 `<a>`，不把页面路由隐藏在浏览器临时状态中。
- 当前链接自动输出 `aria-current="page"`。
- 窄屏时页签可以水平滚动。
- 宿主负责根据当前 Topcoat 路由传入 `active`。
