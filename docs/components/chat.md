# Chat 组件

`topcoat-ant-design` 的 Chat 组件负责布局、语义、局部浏览器交互和内容呈现。宿主应用负责会话数据、路由、鉴权、存储、模型请求和取消机制。

## 组件与模块

| `src/components/chat/` 模块 | 公共组件或类型 | 用途 |
| --- | --- | --- |
| `message.rs` | `ChatMessage`、`ChatMessageStatus` | 消息约定：稳定 `id`、`role`、`status`、`content` |
| `bubble.rs`、`message_list.rs` | `chat_bubble`、`chat_message_list` | 单条消息与可访问的会话日志 |
| `sender.rs` | `chat_sender` | 草稿、发送按钮、回车发送和 Shift+回车换行 |
| `conversations.rs` | `chat_conversation_list`、`chat_conversation_item` | 由真实路由决定激活项的会话导航 |
| `markdown.rs` | `chat_markdown` | 聊天 Markdown 安全渲染 |
| `think.rs`、`thought_chain.rs` | `chat_think`、`chat_thought_chain`、`chat_thought_step`、`ChatThoughtStatus` | 可展开的过程摘要、编号步骤与节点状态 |
| `sources.rs`、`actions.rs` | `chat_sources`、`chat_source`、`chat_actions` | 引用来源和宿主提供的操作 |
| `attachments.rs`、`prompts.rs` | `chat_attachment_tray`、`chat_file`、`chat_prompts`、`chat_prompt` | 草稿附件、文件元数据和空会话建议 |

这些组件都从 crate 根导出。`ChatMessageStatus` 有 `Sending`、`Streaming`、`Complete`、`Failed`、`Cancelled` 五种状态。`chat_bubble` 可以通过 `status` 显示状态并设置 `aria-busy`；`chat_message_list` 提供带标签的 `role="log"` 区域。页面应为消息提供稳定 ID，并在 `view!` 的消息循环中使用 Topcoat 0.9 的 `#[key(message.id.clone())]`，使消息组件的浏览器状态在列表更新后仍对应同一条消息。

`chat_think` 使用轻量的三角形与脑图标入口，在回复正文前展开过程摘要。Gallery 的“复制回复”操作位于回复底部，只复制该条助手消息的文本内容；复制成功后显示反馈。操作本身由页面提供，`chat_actions` 只负责承载与布局。

`chat_thought_chain` 使用中性的编号节点和细连接线；`chat_thought_step` 将子内容作为浅灰说明展示。需要强调执行结果时，可传入 `ChatThoughtStatus::{Loading, Success, Error, Abort}`，并用 `language` 指定状态的可访问文案。视觉层级参考 [Ant Design X ThoughtChain](https://x.ant.design/components/thought-chain-cn/)。

`chat_sources` 使用原生可展开区域展示检索记录；每个 `chat_source` 是一张两行来源卡片，可用 `summary` 提供摘要、`activity` 标识检索动作。Gallery 将来源放在消息气泡下方，避免卡片嵌套。来源链接仍须由宿主提供可信的 HTTP/HTTPS 地址。

## 组合方式

```rust,ignore
let draft = signal(cx, String::new);
let busy = signal(cx, || false);
let submit = attributes! { cx =>
    @submit=$(|event: Event| {
        event.prevent_default();
        // 页面在此追加带稳定 ID 的用户消息与待回复消息。
    })
};

view! {
    chat_message_list(label: "Conversation messages",
        chat_bubble(role: ChatBubbleRole::Assistant,
            status: Some(ChatMessageStatus::Complete),
            chat_markdown(source: "**Hello** from Topcoat")
        )
    )
    chat_sender(id: "chat-draft", draft: &draft, busy: Some(&busy), submit_attrs: submit)
}
```

`chat_sender` 在输入为空或 `busy` 为真时禁用发送，可用 `max_length` 限制草稿长度。发送后由页面清空草稿并把焦点还给输入框；新消息到来时，由页面滚动消息区域。附件区可以作为 `chat_sender` 的子内容放入表单，由宿主处理文件选择和上传。

## Gallery 验证路径

- `/chat`、`/chat/notes`：两个通过真实路由切换的示例会话。
- `/chat/new`：空会话、建议输入、多轮消息和状态操作。
- `/bubble`、`/message-list`、`/sender`：基础组件独立示例。
- `/chat/states`、`/chat/markdown`、`/chat/think`、`/chat/thought-chain`、`/chat/sources`、`/chat/actions`、`/chat/attachments`、`/chat/files`、`/chat/prompts`、`/chat/conversations`：内容和导航组件示例。

Gallery 的 `gallery_reply` 与 `gallery_action` 是有输入校验的演示 `#[procedure]`。它们不调用模型，也不保存会话；取消与重试只确认示例操作。发送时，页面先追加用户消息和 `Sending` 状态的助手消息。分段按钮只修改当前助手气泡的浏览器 signal；完成、失败或取消时，才把最终内容和状态提交到消息列表 shard。新消息追加、会话列表和历史的局部刷新由宿主按同样方式接入。

实际应用应以宿主的 `#[procedure]` 或端点处理发送、重试、鉴权与参数校验，并以流式端点更新当前助手消息。取消操作还需要停止宿主的真实请求。路由表达当前会话；服务端负责会话历史和持久化。不要把浏览器传来的消息内容、角色、状态或 ID 当作已验证的服务端事实。

## 内容安全

`chat_markdown` 把原始 HTML 作为文本转义，只允许安全的 Markdown 链接目标，不渲染 Markdown 图片。`chat_source` 仅接受 HTTP/HTTPS 目标；宿主仍应审查来源是否可信。Gallery 的通用 Markdown 文档渲染器只处理仓库内可信文档，不能用于外部聊天内容。附件组件只展示文件元数据；文件大小、类型、下载权限和上传流程由宿主校验。
