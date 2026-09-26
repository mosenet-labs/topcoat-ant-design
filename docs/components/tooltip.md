# Tooltip 文字提示

鼠标悬停或键盘聚焦时显示完整文本。用于短说明、被截断的提交 ID 等内容。

```rust,ignore
use topcoat_ant_design::tooltip;

view! {
    tooltip(id: "commit-tooltip", content: "4be48fb908424803fbe041ad5a50c0bf73f4f425",
        <span>"4be48fb9…"</span>
    )
}
```

- `id`：页面内唯一的 DOM 标识，由调用方生成。
- `content`：完整纯文本；组件按文本渲染，不插入 HTML。
- `child`：可见的非交互文本或图标；组件外壳可通过 Tab 聚焦，不要再嵌套按钮或链接。
- `attrs`：可选，转发到可聚焦外壳；调用方 class 会与组件 class 合并。

提示采用 Popover 顶层显示，不受表格滚动容器裁剪。默认显示在上方，空间不足时翻转至下方；左右贴近视口边缘时自动偏移，箭头继续指向触发元素。滚动、调整窗口大小时重新计算位置。

鼠标可以移入提示并选择文本；离开触发元素及提示、且触发元素失去焦点后关闭。Escape 可直接关闭，不移动焦点。关闭或组件移除时清理事件监听和定位回调。
