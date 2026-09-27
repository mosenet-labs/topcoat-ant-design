# SearchMultiSelect 搜索多选

`search_multi_select` 将搜索输入、可多选列表和可移除的已选标签组合在一起。调用方提供全部选项，并管理三个 Topcoat signal：保存所选值的 JSON 数组、搜索文字和展开状态。组件不会自行获取选项。

```rust,ignore
use topcoat::runtime::signal;
use topcoat_ant_design::{SearchOption, search_multi_select, UiLanguage};

let options = vec![
    SearchOption { value: "gpt-4.1".into(), label: "GPT-4.1".into() },
    SearchOption { value: "o3".into(), label: "o3".into() },
];
let selected = signal(cx, || r#"["gpt-4.1"]"#.to_owned());
let search = signal(cx, String::new);
let open = signal(cx, || false);

view! {
    search_multi_select(
        id: "models", options: &options, selected: &selected,
        search: &search, open: &open, label: Some("模型"),
        language: UiLanguage::ChineseSimplified,
    )
    <output>$(selected.get())</output>
}
```

选择选项或移除标签时，组件会更新 `selected`，并从根元素发出可冒泡的 `selectionchange` 事件，供宿主同步其他表单字段。搜索同时匹配选项标签和值。组件支持 `language`、`placeholder`、`disabled` 和根元素 `attrs`；颜色使用当前库的明暗主题变量。

只读场景可保留已选 JSON 数组并设置 `disabled: true`：

```rust,ignore
search_multi_select(
    id: "locked-models", options: &options, selected: &selected,
    search: &search, open: &open, disabled: true,
)
```
