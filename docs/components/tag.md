# Tag 标签

用于展示状态或分类。采用浅色背景、细边框和语义颜色，参考 Ant Design Tag。

```rust
use topcoat_ant_design::{TagTone, tag};

view! {
    tag(tone: TagTone::Success, "已启用")
    tag(tone: TagTone::Default, "已停用")
}
```

`tone` 支持 Default（灰）、Success（绿）、Warning（金）、Error（红）、Processing（蓝）。
通过 `attrs` 传入额外属性，子节点承载标签内容。必须同时提供明确的状态文字，不能只用颜色表达含义。
首版用于静态展示，不包含关闭或多选交互。
