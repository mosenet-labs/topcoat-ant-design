# AI 组件与 Chat 界面需求

## 目标与边界

- 借助 Topcoat 的服务端渲染、浏览器响应式状态、路由和服务端交互能力，逐步实现受 Ant Design X 启发的 AI 界面组件。
- 从直观的 Chat 聊天界面开始，先验证组件拆分和组合方式，再扩展到思考过程、附件、引用等能力。
- `topcoat-ant-design` 提供通用 UI、状态呈现和浏览器交互；会话持久化、模型请求、鉴权和文件存储由宿主应用负责。
- Gallery 单列「AI 组件」分类，同时展示 Chat 组合界面和各基础组件的独立状态与用法。

## Chat 界面拆分

```text
ChatPage（会话数据、路由、请求和流式响应）
├── ConversationSidebar（会话列表与切换）
└── ChatPanel（界面组合）
    ├── ChatHeader（标题与会话操作）
    ├── Welcome + Prompts（空会话）
    ├── MessageList
    │   └── MessageBubble
    │       ├── MessageContent（文本、Markdown 或其他内容）
    │       └── MessageActions（复制、重试等）
    └── ChatComposer
        └── AttachmentTray（附件）
```

可复用组件负责布局、语义、样式和局部交互。页面持有消息列表、当前会话与请求状态，并提供发送、取消、重试等业务行为。消息应有稳定 `id`，基础数据至少区分 `role`、`status` 和 `content`；内容区域保留组合入口，以便放入 Markdown、工具结果和引用。

## Topcoat 能力的使用方向

| 需求 | 建议使用的能力 |
| --- | --- |
| 初始页面与组件结构 | `#[component]`、`view!` 服务端渲染 |
| 输入草稿、按钮状态、展开收起 | `Signal`、事件处理和响应式属性 |
| 会话列表或历史的局部刷新 | `#[shard]` 与真实路由 |
| 发送、重试等服务端操作 | 宿主应用的 `#[procedure]` |
| 模型增量输出 | 宿主应用的流式端点；只更新正在生成的消息 |
| 样式、图标和字体 | 现有 Topcoat Asset、Tailwind 和 Iconify 接入 |

组件库不固定模型协议。宿主处理请求、认证、错误和取消，并校验来自浏览器的参数。聊天内容可能来自外部，Markdown 或 HTML 渲染需要安全处理；Gallery 中只渲染仓库可信文档的 Markdown 代码不能直接用于聊天消息。

## 当前已落地

- Gallery 的「AI 组件」分类提供 [`/chat`](../src/bin/gallery/app/_ai/chat.rs) 组合页，以及 [`/bubble`](../src/bin/gallery/app/_ai/bubble.rs)、[`/message-list`](../src/bin/gallery/app/_ai/message_list.rs)、[`/sender`](../src/bin/gallery/app/_ai/sender.rs) 三个独立组件示例页；每页可展开示例代码。
- [`chat_bubble`、`chat_message_list`、`chat_sender`](../src/components/chat/mod.rs) 已作为公共组件导出，各自位于 `chat/` 下的独立子模块。
- Chat 页面展示用户与助手样例消息，草稿由浏览器端 signal 管理；发送后会显示最近一次提交的消息。
- 独立示例分别展示两种消息角色、消息列表的可访问区域，以及输入区的本地发送交互。
- 预览支持中英文，不调用模型、不保存会话，也尚未追加完整的多轮历史。

## 后续需求与验收

1. **完善 Chat 基础交互**：空会话、发送中、完成、失败和取消状态；多轮消息追加；输入框键盘行为；滚动与焦点处理。每种状态都能在 Gallery 独立查看。
2. **完善会话管理**：会话列表、当前项、创建与切换；由路由表达当前会话，宿主负责历史数据。
3. **接入流式输出**：消息使用稳定 ID，增量更新当前助手消息；完成、失败和取消状态保持一致，避免逐 token 重渲染整页。
4. **扩展内容组件**：Markdown、Think、ThoughtChain、Sources、Actions，以及附件、文件和建议输入；复用 Chat 界面的消息与输入结构。
5. **保持组件库质量**：可访问的标签和状态、窄屏布局、中英文文案、减少动态效果支持；每个新组件在 Gallery 有可操作示例。

本文件记录当前方向与验收范围。具体公共 API 会随 Chat 交互实现继续收敛。
