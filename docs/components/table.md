提供带横向滚动、显示密度和分页区域的数据表格。

`data_table` 保留原生 `table`、`thead`、`tbody`、`th` 和 `td` 语义。组件只统一视觉与容器行为，列结构、业务数据、排序和数据请求仍由宿主负责。

`table_pagination` 与表格分开组合。宿主可以传入真实链接完成服务端或游标分页，也可以传入带 Topcoat `@click` 的按钮完成浏览器内分页。`table_page_size_select` 用于在分页区提供统一的页容量选择器，实际查询状态仍由宿主控制。

## 参数

### `data_table`

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `label` | `&str` | 表格的无障碍名称。 |
| `density` | `DataTableDensity` | 可选；默认为 `Default`，日志和事件列表可使用 `Compact`。 |
| `attrs` | `Attributes` | 可选；转发到原生 `table`，调用方 class 会与组件 class 合并。 |
| `child` | `Child` | 原生 `thead`、`tbody` 和可选的 `colgroup`。 |

### `table_pagination`

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `summary` | `&str` | 左侧的数据量或分页方式说明。 |
| `label` | `&str` | 分页导航的无障碍名称。 |
| `attrs` | `Attributes` | 可选；转发到根 `footer`。 |
| `child` | `Child` | 上一页、当前页和下一页的链接、按钮或禁用文本。 |

### `table_page_size_select`

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `id` | `&str` | 原生 `select` 的稳定 ID。 |
| `label` | `&str` | 可见标签，同时作为选择器的无障碍名称。 |
| `attrs` | `Attributes` | 可选；转发到原生 `select`，可传入 Topcoat `:value` 和 `@change`。 |
| `child` | `Child` | 原生 `option` 列表。 |

## 页码分页

下面的分页在浏览器中更新 Topcoat signal，不触发整页跳转：

```rust,ignore
use topcoat_ant_design::{
    DataTableDensity, data_table, table_page_size_select, table_pagination,
};
use topcoat::{runtime::{Event, expr, signal}, view::{attributes, view}};

let page = signal(cx, || "1".to_owned());
let page_size = signal(cx, || "10".to_owned());
let last_page = expr!(if page_size.get() == "20" { true } else { page.get() == "2" });

view! {
    data_table(label: "项目列表", density: DataTableDensity::Compact,
        <thead><tr><th>"项目"</th><th>"状态"</th></tr></thead>
        <tbody>
            <tr><td>"devops/review"</td><td>"已生效"</td></tr>
        </tbody>
    )
    table_pagination(summary: "共 18 个项目", label: "项目列表分页",
        table_page_size_select(id: "project-page-size", label: "每页记录",
            attrs: attributes! {
                :value=$(page_size.get())
                @change=$(|event: Event| {
                    page_size.set(event.target.value);
                    page.set("1".to_owned());
                })
            },
            <option value="10">"10"</option>
            <option value="20">"20"</option>
        )
        <button type="button" :disabled=$(page.get() == "1") @click=$(|_e| page.set("1".to_owned()))>"上一页"</button>
        <span aria-current="page">$(page.get()) " / 2"</span>
        <button type="button" :disabled=$(last_page) @click=$(|_e| page.set("2".to_owned()))>"下一页"</button>
    )
}
```

## 默认密度

省略 `density` 时使用默认行高，适合包含辅助文字或操作入口的表格：

```rust,ignore
data_table(label: "用户列表",
    <thead><tr><th>"账号"</th><th>"角色"</th><th>"状态"</th></tr></thead>
    <tbody>
        <tr><td>"demo-admin"</td><td>"管理员"</td><td>"启用"</td></tr>
    </tbody>
)
```

## 服务端或游标分页

调用方负责构造包含筛选条件和游标的地址，组件保留真实链接：

```rust,ignore
table_pagination(summary: "使用稳定时间游标分页", label: "事件列表分页",
    if let Some(next_url) = next_url {
        <a href=(next_url)>"下一页"</a>
    } else {
        <span aria-disabled="true">"没有更多"</span>
    }
)
```

## 约定

- 当前页元素使用 `aria-current="page"`。
- 不可用的按钮使用原生 `disabled`；不可用的文本使用 `aria-disabled="true"`。
- 切换页容量时应由宿主把当前页重置为第一页，避免新页容量下出现越界空页。
- 整行跳转仍由业务单元格中的真实链接表达，避免让 `tr` 模拟按钮。
- 表格会在窄容器中横向滚动；宿主可通过 `attrs` 中的 class 或 style 设置业务需要的最小宽度。
