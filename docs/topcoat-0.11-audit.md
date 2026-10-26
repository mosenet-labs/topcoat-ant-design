# Topcoat 0.11.0 组件核对记录

核对日期：2026-10-26。基线：官方 `v0.11.0` 标签，提交 `0a77100`。

范围：全部 33 个公开自定义组件、公开辅助类型/属性函数，全部 34 个官方模块及子组件，以及 Gallery 的导航、输入、procedure、shard、样式和翻译。逐项审查不意味着每个展示容器都必须改写。

## 新特性与兼容性

| 项目 | 处理结果 |
| --- | --- |
| Rust / 依赖 | 最低 Rust 1.99；直接依赖、build/dev 依赖及锁文件中的 Topcoat 系列统一为 0.11.0。Fluent 元数据直接链接 inventory，以保证不启用 Gallery/discover 时仍可构建。 |
| Fluent | 自定义组件的内置渲染标签迁入 `src/l10n/{en,zh}.ftl`。Gallery 新示例有独立翻译目录。旧的显式 `language` 属性仍可用、仍默认英文；新增 `UiLanguage::current(cx)` / `locale()` 供宿主接入官方上下文。公开的静态状态标签方法保留兼容。 |
| Gallery 请求语言 | 根 `#[layer]` 在布局和页面前设置官方 Locale。保留原有 `?lang=zh` 与默认英文 URL 契约；旧页面的 `Locale.select` 仍是兼容适配器，没有宣称全部历史文档段落已改写为 FTL。 |
| 数字/日期 | JSONViewer 用 Fluent 复数与数量参数；Calendar 日/周标题使用 DateTime；新增示例同时展示 Number、DateTime、Fluent 数量消息。 |
| 稳定元素 ID | JSONViewer 动作、Notification 自动 ID、Tooltip 关联和新增官方组件示例改用 `element_id`。公开传入的 ID 继续保留，以满足宿主的标签/表单/触发器契约。 |
| Popover / Floating | 集成官方实现，迁移 AnchoredMenu、Popconfirm、DateTimeRange、TableColumnSettings；共享定位回退，保留当前主题和业务组合语义；主题在菜单根下限定重置按钮默认边框、背景和列表缩进，适配未启用 Preflight 的宿主。 |
| Dropdown Menu | 全部调用迁移为 target/id/close；移除旧 details open Signal 适配，子菜单复用官方父子关闭行为。 |
| 表单值保留 | ChatSender 和 SearchMultiSelect 继续使用 Signal 受控输入与显式清除；日期输入保留原生编辑状态。新增 shard 示例对比普通 value 和 `Expr::dynamic()` 的重渲染行为。 |
| 错误传播 | 没有自定义 `#[memoize(as_ref)]` 函数，无需为新借用 Error 转换修改组件。 |
| CLI | 不创建新应用，不调用 `topcoat new`。在临时目录安装匹配 CLI；项目加入 edition=2024 的 rustfmt 配置。翻译测试使用与 `topcoat l10n check` 相同的官方 Check 引擎读取已编译 Gallery，兼容本项目通过 gallery feature 启动的方式。 |
| 框架修复 | 导航、动态文本、含 `<` 的信号 hydration 等随 runtime 升级生效；新版内置服务器超时由框架提供。 |

## 全部自定义组件

| 组件 | 核对与优化 |
| --- | --- |
| `anchored_menu` | 改由官方 `popover_content` 承载；复用 Floating 的方向、对齐和边缘回退，保留操作后关闭与 attrs 转发。 |
| `calendar` | 日/周标题采用官方 `DateTime` 本地化；空状态进入 Fluent。月份网格、日期类型和宿主事件数据不变。 |
| `chat_actions` | 宿主提供操作与文字，无内置文案、自动 ID 或输入状态；核对 Button 组合与 attrs 转发。 |
| `chat_attachment_tray` | 附件容器，文字与上传状态由宿主提供；无需新增框架状态。 |
| `chat_bubble` | 角色及渲染中的状态标签进入 Fluent；复用更新后的 Avatar，保留公开枚举与 `ChatMessageStatus::label` 的静态字符串接口。 |
| `chat_conversation_item` | 复用更新后的 Sidebar，保留客户端导航、上下文预取和响应式 active。 |
| `chat_conversation_list` | 导航容器；内容及标签由宿主传入，链接行为在 item 中实现。 |
| `chat_file` | 文件元信息由宿主传入，不引入翻译键或浏览器数据请求。 |
| `chat_markdown` | 保留文本转义与协议限制；浏览器含 `<` 的消息 hydration 由新版 runtime 修复，Markdown HTML 策略继续由库限制。 |
| `chat_message_list` | 日志容器保留稳定消息身份、aria-live 和 shard 组合；新版文本更新与 hydration 修复自动生效。 |
| `chat_prompt` | 继续组合官方 Button；标题与点击过程由宿主提供。 |
| `chat_prompts` | 建议输入容器，无自有文案或表单值。 |
| `chat_sender` | 内置文案进入 Fluent；draft 保持 Signal 的受控绑定，发送后显式清空，不依赖重渲染重置。 |
| `chat_source` | 保留 HTTP/HTTPS 外链协议校验及新标签语义，不将外部资料纳入应用预取。 |
| `chat_sources` | 组合新版 Accordion；引用内容及标签由宿主提供。 |
| `chat_think` | 标题进入 Fluent；展开状态继续共享一个 bool Signal，宿主 ID 仍供 Collapse 与触发器共同使用。 |
| `chat_thought_chain` | 有序容器，无默认文案或独立状态；核对属性合并。 |
| `chat_thought_step` | 内部步骤状态标签进入 Fluent；复用新版 Spinner，保留状态枚举。 |
| `collapse` | 保留一个 open Signal 与可逆动画；宿主 ID 是公开关联契约，无需强制自动生成。 |
| `data_table` | 复用新版官方 Table；行与数据由宿主提供，不引入多余状态。 |
| `date_time_range_filter` | 面板改用官方 Popover/Floating；内置文案进入 Fluent。日期输入保留原生已编辑值，快捷范围与清除仍显式更新 DOM 并派发 input/change。 |
| `drawer` | 关闭文案进入 Fluent；保留 Sheet、Signal 关闭和路由关闭两种模式及焦点/键盘语义。 |
| `form_field` | 复用新版 Field/Label；公开表单 ID 与 help ID 的关联保持兼容，字段值由调用方控制。 |
| `json_viewer` | 展开按钮用 `element_id`；内置文案与条目数量进入 Fluent/复数规则；复制状态保存语言无关的状态码，切换语言时重新翻译；保留递归展开、焦点恢复、复制及安全文本渲染。 |
| `native_dialog` | 关闭文案进入 Fluent；保留原生 showModal、焦点约束和 busy 时的取消限制；默认值变化不代替宿主的显式重置。 |
| `notification` | 用 `element_id(cx)` 替代全局原子计数 ID，重渲染时稳定；关闭标签进入 Fluent，计时和手动 Popover 行为不变。 |
| `popconfirm` | 复用官方 `popover_content` 和 Floating，保留 alertdialog、箭头和确认动作；取消标签进入 Fluent。 |
| `search_multi_select` | 默认标签、提示和移除文案进入 Fluent；搜索和选中状态仍为受控 Signal，公开 JSON 契约及事件保留。 |
| `table_column_settings` | 改为官方 Popover/Trigger/Content 组合，保留列显隐、required 列和恢复默认；标签进入 Fluent。 |
| `table_page_size_select` | 复用新版 Select；value/change 由宿主响应式属性控制，不以重渲染清除用户选择。 |
| `table_pagination` | 复用新版 Pagination，保留运行时链接属性组合和真实链接回退。 |
| `table_toolbar` | 展示标题及宿主操作，无内置文案或状态。 |
| `tag` | 继续组合新版 Badge；标签来自 child，不替换宿主业务文本。 |

## 全部官方模块

全部从 crate 根导出，也可通过 `ui` 模块访问。`components.toml` 保存 v0.11.0 原始文件哈希，便于辨认本地样式修正。

与上游逐字节比较：28 个完全相同；6 个保留已有的段落 `m-0` 或按钮透明背景修正。

| 模块 | 核对结果 |
| --- | --- |
| `accordion` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `alert` | 同步新版，保留已有局部样式修正。 |
| `alert_dialog` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `avatar` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `badge` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `breadcrumb` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `button` | 同步新版，保留已有局部样式修正。 |
| `card` | 同步新版，保留已有局部样式修正。 |
| `checkbox` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `dialog` | 同步新版，保留已有局部样式修正。 |
| `dropdown_menu` | 同步官方 Popover 重构，迁移全部示例和子菜单；保留 label 间距修正。 |
| `field` | 同步新版，保留已有局部样式修正。 |
| `floating` | 新增：根导出方向、对齐、锚点和定位函数；独立四方向示例。 |
| `hover_card` | 同步官方 Floating 定位；展示页移除旧手动定位干预。 |
| `input` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `kbd` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `label` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `navigation_menu` | 新增：根导出全部组件与 variant；双语导航示例，链接接入 runtime。 |
| `pagination` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `popover` | 新增：根导出全部组件；双语资料编辑面板示例，outside click / Escape / 显式关闭。 |
| `progress` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `radio_group` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `select` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `separator` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `sheet` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `sidebar` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `skeleton` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `spinner` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `switch` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `table` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `tabs` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `textarea` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `toggle` | 与官方 v0.11.0 完全一致；更新实现或核对无变化。 |
| `tooltip` | 同步官方 Floating 定位；展示页移除旧手动定位干预。 |

## 验证范围

- 默认功能构建与全功能测试：全部组件渲染、路由、双语、主题、表单、Chat、官方新增模块的根导出与稳定 ID。
- 使用官方 Fluent Check 引擎检查组件与 Gallery 两个 namespace 的中英文消息和变量。
- 浏览器：Dropdown/子菜单关闭、Popover、NavigationMenu、Floating、输入重渲染与显式重置，及复用面板的表格列设置、日期范围、确认气泡。
- 生成并同步 docs.rs 使用的预生成 CSS，检查格式、差异与归档构建。

最终自动化结果：`cargo test --all-features --locked --offline` 通过 35 项测试与 1 项文档测试；`cargo check --no-default-features --locked --offline`、`cargo fmt --check`、`git diff --check` 通过。`cargo package --locked --offline` 完成归档构建。浏览器确认复制/菜单操作状态随语言切换重新翻译，控制台无 warning/error。

公开组件仍由宿主负责请求、鉴权、数据获取和输入验证；本记录不等同于对所有宿主组合的运行时保证。
