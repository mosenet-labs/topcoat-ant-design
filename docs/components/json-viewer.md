# JSONViewer JSON 浏览器

`json_viewer` 接收 `serde_json::Value`，以树形结构展示对象、数组和基本类型。通过原生 `details` 折叠节点，支持键盘操作；文字按 HTML 文本转义，颜色使用 UI 库的亮色、暗色主题。

```rust,ignore
use topcoat_ant_design::{UiLanguage, json_viewer};

view! {
    json_viewer(value: &data, language: UiLanguage::ChineseSimplified)
}
```

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `value` | `&serde_json::Value` | 待展示的数据；获取数据由宿主负责。 |
| `root_label` | `&str` | 根节点名称，默认 `root`。 |
| `expanded_depth` | `usize` | 初始展开层数，默认 `1`，即仅展开根节点；`0` 表示全部折叠。 |
| `language` | `UiLanguage` | 组件提示语言，默认英文，可选简体中文。 |
| `attrs` | `Attributes` | 根元素附加属性，可设置 class 和无障碍名称。 |

未展开的非空对象、数组在悬停或键盘聚焦时显示复制按钮；触屏始终显示。点击后复制该节点完整的格式化 JSON，不展开节点，并反馈成功或失败。优先使用 Clipboard API；不可用时使用兼容路径，支持在原生 dialog 内复制。

空对象、空数组和基本类型直接显示。通用组件不删除任何字段；如 Kubernetes `metadata.managedFields` 等业务字段，应由宿主在传入前处理。组件展开状态属于当前 DOM，首次展开层数不代表受控展开状态。
