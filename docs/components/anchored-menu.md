# 锚定操作菜单

`anchored_menu` 结合浏览器 Popover API 和 CSS anchor，使表格滚动区域内的菜单浮在容器之上。点击外部或按 Escape 可关闭，点击菜单项后也会关闭。

在触发按钮上使用 `anchored_menu_trigger_attributes(cx, id)`，菜单使用相同的 `id`。`id` 同时作为 CSS anchor 名称，必须由应用生成，不能直接采用未经校验的用户输入。
