# Topcoat 0.10 组件核对记录

核对日期：2026-10-08。依据：[v0.10.0 官方发布说明](https://github.com/tokio-rs/topcoat/releases/tag/v0.10.0)。

范围：`src/components` 的 30 个组件、其公开类型和属性辅助函数，`src/ui` 的 31 个官方组件模块及其子组件，以及 Gallery 的布局、展示页、procedure 和 shard。检查新特性的适用性不代表每个组件都需要修改。

## 逐项特性

| v0.10 特性 | 核对与采用结果 |
| --- | --- |
| 客户端导航 | Gallery 主导航、语言切换、概览链接、官方组件索引和 `chat_conversation_item` 使用 `link` / `link_attrs`。Drawer 的关闭路由也使用运行时链接，Escape 触发同一关闭链接。 |
| 页面预取 | 普通导航使用 `prefetch_mode(cx)`，遵循宿主配置，默认 Intent。Drawer 的关闭链接使用 Never。Gallery 的页面渲染不写入共享会话；发送操作由 procedure 执行。 |
| `#[record]` | Chat Gallery 的 `ReplyPreview` 将内容和生成状态放入一个类型化 Signal，通过一次写入更新或重置两者。重试显式清空预览状态，避免稳定消息 ID 保留上次生成的分段。公共组件接受现有信号或渲染参数；带借用生命周期的配置、Chrono 日期与枚举模型不适合直接声明为 record。 |
| 元组运行时支持 | 核对了 tuple 在循环、日期配置和 Chat 消息组合中的用法：现有 tuple 仅在服务端组织数据，没有需要捕获到浏览器的关联状态。无需为展示组件引入 tuple Signal。 |
| 共享 WebSocket / 独立 shard 更新 | `live_message_region` 已调用 `connected(cx)`，升级后自动使用文档共享连接，API 不变。`message_region` 和官方表格示例的 shard 按已有信号重新渲染；它们没有持续服务端推送，不额外启用 connected。 |
| `topcoat ui add --all` | 库已安装全部 31 个模块。上游 v0.9.0 和 v0.10.0 的 registry 实现相同，批量安装不提供新组件。与 v0.10 官方源直接比对：25 个本地文件完全相同，6 个文件仅有现有样式修正，见下表。 |
| `topcoat fmt --rustfmt` / `--check` | 这是 CLI 工具能力，不改变组件运行。当前构建和检查使用 Cargo；本次执行 `cargo fmt --check`。没有因新增格式化命令重写官方宏源码。 |
| CLI 版本与 `default-run` | 不属于组件 API。项目只有一个 Gallery 可执行目标，现有启动命令明确指定 `--bin` 和 `--features gallery`；不存在默认目标歧义。 |
| 动态模块路由兼容性 | 项目没有 `path_param!` 动态模块声明，无需迁移为 `module_param!`。现有路由检查覆盖展示页面和显式 Chat 端点。 |
| 字符串 `.len()` 与脚本兼容性 | 没有运行时字符串长度的浮点比较或浮点接收类型。Gallery 已使用 `runtime::script()`，没有手写运行时脚本标签。原生 `maxlength` 和服务器字节长度校验保持各自语义。 |

## 自定义组件

下表每行覆盖对应组件及其同模块公开辅助函数。没有导航目标的组件由宿主通过 `attrs` / `child` 提供交互；它们不自行选择路由、预取或服务器连接。

| 组件 | 新特性结论 |
| --- | --- |
| `anchored_menu` | 内容由宿主提供；关闭使用原生 Popover API。触发属性函数不创建导航或结构化状态。 |
| `calendar` | 日期和事件在服务器计算、渲染；`CalendarEvent` 包含 Chrono 日期和枚举，无 record 迁移需求。 |
| `chat_actions` | 操作容器；procedure、取消、复制等由宿主提供。 |
| `chat_attachment_tray` | 附件容器；上传和状态由宿主提供。 |
| `chat_bubble` | 消息角色与最终状态由参数提供；Gallery 用 record 管理正在生成的内容与指示状态。 |
| `chat_conversation_item` | 已使用运行时导航，保留官方 Sidebar 的 active 与属性转发；预取遵循上下文。 |
| `chat_conversation_list` | 导航容器；运行时链接行为由 item 提供。 |
| `chat_file` | 文件元信息展示；没有自身下载路由或信号模型。 |
| `chat_markdown` | 渲染用户/模型文本并限制链接协议；当前示例引用外部资料，保留普通链接。组件不对任意文本链接执行应用页面预取。 |
| `chat_message_list` | 消息日志容器；Chat 示例由 shard 渲染消息，按稳定 ID 保留浏览器组件状态。 |
| `chat_prompt` | 原生按钮；点击行为来自调用方属性。 |
| `chat_prompts` | 建议输入容器；没有自身状态。 |
| `chat_sender` | 接受 draft、busy 和 submit 属性；提交由宿主 procedure 完成，Enter 调用原生 form.requestSubmit。公开信号无需改成 record。 |
| `chat_source` | 经校验的 HTTP/HTTPS 链接在新标签打开，遵循浏览器正常行为。 |
| `chat_sources` | 官方 Accordion 组成的引用容器；无连接或路由状态。 |
| `chat_think` | 单一 bool Signal 管理展开；record 不会减少状态或脚本。 |
| `chat_thought_chain` | 有序步骤容器；无浏览器数据模型。 |
| `chat_thought_step` | 枚举步骤状态由宿主传入，使用官方 Spinner；无 record 迁移需求。 |
| `collapse` | 与触发属性共享一个 bool Signal；保持展开动画和无障碍状态。 |
| `data_table` | 官方 Table 组合，数据由宿主提供；无需将通用表格绑定到某种 record。 |
| `date_time_range_filter` | 原生 datetime-local 输入；日期计算与 DOM input/change 事件仍需浏览器适配。配置带生命周期，不是运行时 record。 |
| `drawer` | 已接入运行时关闭链接；关闭路由不预取；未配置路由时仍写回单一 open Signal。 |
| `form_field` | 官方 Field 组合；配置与错误信息是渲染参数，实际表单状态由宿主提供。 |
| `native_dialog` | open/busy 信号与原生 modal dialog 同步，保留焦点限制和忙碌时取消约束。v0.10 没有新增对应 DOM 方法封装。 |
| `notification` | 一个消息 Signal 驱动原生 Popover 和计时；不创建服务端更新任务。 |
| `popconfirm` | 原生 Popover + 官方 Button；业务操作由 child 提供，无路由状态。 |
| `search_multi_select` | 选中值沿用 JSON String、搜索与展开沿用已有信号契约。`SearchOption` 只用于服务器生成选项；增加 record 注解不会消除既有 JSON 契约。 |
| `table_page_size_select` | 官方 Select 组合；响应式值与 change 行为通过 attrs 转发。 |
| `table_pagination` | 官方 Pagination 容器；中英文服务端分页示例改用 `pagination_next(attrs: link_attrs(...))`。 |
| `tag` | 官方 Badge 的展示组合；没有状态或路由。 |

`ChatMessage` 继续使用稳定的序列化契约及枚举角色、状态。Gallery 对浏览器传入的消息数量、ID、内容长度和重复 ID 的校验继续生效。`ReplyPreview` 仅包含公开展示的内容与生成标志，不替代这个输入边界。

## 官方组件模块

所有模块均继续从 crate 根导出。表中的“属性接入”表示调用方可以将 `link_attrs` 转发给具体链接，而无需修改官方组件实现。

| 官方模块（含其子组件） | 核对结论 |
| --- | --- |
| `accordion` | 原生 details 展开，源码与 v0.10 一致，无新状态 API 迁移。 |
| `alert` | 展示组件；现有本地差异为标题段落添加 `m-0`。 |
| `alert_dialog` | 接受 open 表达式，动作来自 child；源码一致。 |
| `avatar` | 图片和 fallback 展示；源码一致。 |
| `badge` | 展示组件；源码一致。 |
| `breadcrumb` | breadcrumb_link 支持运行时链接属性接入；当前示例指向外部文档。源码一致。 |
| `button` | 动作来自 attrs；现有本地差异为 Outline/Ghost 明确设置透明背景。 |
| `card` | 展示组合；现有本地差异为标题/描述添加 `m-0`。 |
| `checkbox` | 原生表单控件，通过 attrs 绑定状态；源码一致。 |
| `dialog` | 接受 open 表达式，焦点行为由宿主控制；现有本地差异为标题/描述添加 `m-0`。 |
| `dropdown_menu` | 原生 details 与菜单组合；现有本地差异为 label 添加 `m-0`。 |
| `field` | 表单语义组合；现有本地差异为描述段落添加 `m-0`。 |
| `hover_card` | 原生 Popover 展示；触发链接来自 child，源码一致。 |
| `input` | 原生输入，通过 attrs 绑定状态；源码一致。 |
| `kbd` | 快捷键展示；源码一致。 |
| `label` | 原生表单标签；源码一致。 |
| `pagination` | 链接支持 `link_attrs` 属性接入；Gallery 本地分页由 Signal 更新，不请求新路由。源码一致。 |
| `progress` | 进度表达由参数和 attrs 提供；源码一致。 |
| `radio_group` | 原生表单控件；源码一致。 |
| `select` | 原生 select；值与事件通过 attrs 提供，源码一致。 |
| `separator` | 视觉/语义分隔；源码一致。 |
| `sheet` | 接受 open 表达式；Drawer 使用它组合运行时关闭行为，源码一致。 |
| `sidebar` | menu_button/sub_button 支持运行时属性转发；Gallery 和 Chat 调用方已接入，源码一致。 |
| `skeleton` | 加载占位展示；源码一致。 |
| `spinner` | 加载指示展示；源码一致。 |
| `switch` | 原生表单开关；源码一致。 |
| `table` | 原生表格展示；Gallery 表格 shard 负责更新，源码一致。 |
| `tabs` | 支持路由链接属性或浏览器信号；现有示例通过 prevent_default 和 selected Signal 即时切换。源码一致。 |
| `textarea` | 原生多行输入；ChatSender 通过 attrs 管理 draft，源码一致。 |
| `toggle` | 原生 checkbox/radio 实现切换；源码一致。 |
| `tooltip` | CSS/原生提示展示；源码一致。 |

## 验证

- 全功能测试包含全部展示页的路由、主题、官方组件和 Chat 输入边界。
- 浏览器验证官方索引跨页导航，以及 record 驱动的 Chat 分段、完成、失败、重试和取消。
- 精简功能构建、格式检查与差异检查。

这里记录的是组件与展示用法的审查结果，不是对所有宿主应用组合的运行时保证。宿主仍负责路由、数据获取、鉴权和输入校验。
