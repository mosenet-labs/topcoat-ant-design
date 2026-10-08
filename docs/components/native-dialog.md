# 原生模态弹窗

`native_dialog` 封装浏览器 `<dialog>` 的模态模式，提供焦点约束、Escape 关闭和提交中禁止关闭。用 `NativeDialogConfig::new(id, title)` 配置弹窗，传入 `open: Signal<bool>` 与 `busy: Signal<bool>`；取消按钮使用 `native_dialog_close_attributes(cx, id)`。`id` 必须由应用生成，不能直接采用未经校验的用户输入。

弹窗状态由 Topcoat Signal 管理。组件库内部负责调用浏览器的 `showModal` 和 `close`，业务页面无需自行操作 DOM。
