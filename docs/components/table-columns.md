# 表格列设置

`table_column_settings` 提供勾选显示列、固定必要列和恢复默认设置。与 `data_table` 组合使用，继续保留原生表格语义。它不请求数据，不改变过滤条件、分页或排序。

```rust,ignore
use topcoat::{runtime::signal, view::view};
use topcoat_ant_design::{
    TableColumn, UiLanguage, data_table, table_column_attributes,
    table_column_settings, table_default_hidden_columns, table_toolbar,
};

let columns = [
    TableColumn::new("name", "名称").required(),
    TableColumn::new("namespace", "Namespace"),
    TableColumn::new("labels", "Label").hidden(),
];
let hidden = signal(cx, || table_default_hidden_columns(&columns));

view! {
    table_toolbar(title: "Pods",
        table_column_settings(id: "pod-columns", columns: &columns,
            hidden: &hidden, language: UiLanguage::ChineseSimplified)
    )
    data_table(label: "Pods",
        <colgroup>
            for column in &columns { <col (table_column_attributes(cx, column, &hidden))> }
        </colgroup>
        <thead><tr>
            for column in &columns {
                <th (table_column_attributes(cx, column, &hidden))>(column.label.as_str())</th>
            }
        </tr></thead>
        <tbody><tr>
            <td (table_column_attributes(cx, &columns[0], &hidden))>"api-7b8fc"</td>
            <td (table_column_attributes(cx, &columns[1], &hidden))>"production"</td>
            <td (table_column_attributes(cx, &columns[2], &hidden))>"app=api"</td>
        </tr></tbody>
    )
}
```

## API 与状态

- `TableColumn::new(key, label)` 默认显示并允许隐藏；`hidden()` 默认隐藏；`required()` 强制显示并禁用勾选框。建议名称列使用 `required()`，避免隐藏全部列。
- 同一表格内 `key` 必须唯一且稳定，不能使用显示标题或列下标作为业务标识。
- `hidden: &Signal<String>` 保存隐藏列 key 的 JSON 字符串数组。初始化使用 `table_default_hidden_columns(&columns)`，如 `["labels"]`；空集合使用 `"[]"`。
- `table_column_attributes(cx, column, &hidden)` 必须同时应用到该列的表头、每行单元格以及可选的 `col`，以便隐藏列宽、表头和数据。返回的 `hidden` 是 Topcoat 响应式属性，首屏与浏览器状态一致。
- 若空状态或汇总行使用 `colspan`，宿主应根据可见列数设置跨度；不要为业务列保留固定总宽度，否则隐藏后仍可能留白。
- `id` 必须是宿主提供的可信 DOM 标识，同一页面每个设置面板使用不同 ID。原生 Popover 在表格滚动区域外展示，按 Escape 或点击外部关闭；勾选时保持打开。
- `language` 默认英文，可传 `UiLanguage::ChineseSimplified`；`attrs` 转发到设置控件外层。
- 设置状态由宿主 signal 控制，重建页面后恢复初始值。组件不内置 localStorage、拖拽排序和远程偏好保存。

## Kubernetes 页面接入约定

UI 库发布后，Pods、Namespaces 页面应共享该控件，配置 `Label` 列默认隐藏。列设置只影响展示，名称、Namespace、Label、Fields 等搜索条件继续按原查询流程处理。
