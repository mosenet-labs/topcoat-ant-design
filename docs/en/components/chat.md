# Chat components

`topcoat-ant-design` Chat components own layout, semantics, local browser interaction, and content presentation. The host application owns conversation data, routes, authentication, storage, model requests, and cancellation.

## Components and modules

| `src/components/chat/` module | Public components or types | Purpose |
| --- | --- | --- |
| `message.rs` | `ChatMessage`, `ChatMessageStatus` | Stable `id`, `role`, `status`, and `content` contract |
| `bubble.rs`, `message_list.rs` | `chat_bubble`, `chat_message_list` | One message and an accessible conversation log |
| `sender.rs` | `chat_sender` | Draft, send button, Enter to send, and Shift+Enter for a new line |
| `conversations.rs` | `chat_conversation_list`, `chat_conversation_item` | Conversation links whose active state follows the route |
| `markdown.rs` | `chat_markdown` | Safe rendering of chat Markdown |
| `think.rs`, `thought_chain.rs` | `chat_think`, `chat_thought_chain`, `chat_thought_step`, `ChatThoughtStatus` | Expandable process summary, numbered steps, and node states |
| `sources.rs`, `actions.rs` | `chat_sources`, `chat_source`, `chat_actions` | Citations and host-owned actions |
| `attachments.rs`, `prompts.rs` | `chat_attachment_tray`, `chat_file`, `chat_prompts`, `chat_prompt` | Draft attachments, file metadata, and suggestions |

All are exported from the crate root. `ChatMessageStatus` covers `Sending`, `Streaming`, `Complete`, `Failed`, and `Cancelled`. `chat_bubble` can display a status and set `aria-busy`; `chat_message_list` supplies a labelled `role="log"` region. Give each message a stable ID and use Topcoat 0.9's `#[key(message.id.clone())]` in the `view!` message loop so browser state stays with the same message after list updates.

Every rendered Chat component accepts optional `attrs` for its root element. Caller classes are appended to component classes, matching the native Topcoat UI convention. `chat_conversation_item` also accepts a boolean or reactive `Expr<bool>` for `active`.
Custom Chat colors use the `--gr-*` theme variables and follow the `.dark` theme class.

`chat_think` uses a compact triangle and brain icon before the reply text to reveal process details. The Gallery's “Copy reply” action sits beneath the reply and copies only that assistant message's text, then shows success feedback. The page supplies the action; `chat_actions` provides its container and layout.

`chat_thought_chain` uses quiet numbered nodes and a thin connector. A `chat_thought_step` renders its child content as a muted description. Pass `ChatThoughtStatus::{Loading, Success, Error, Abort}` to emphasize execution state, and set `language` for an accessible status label. The visual hierarchy follows [Ant Design X ThoughtChain](https://x.ant.design/components/thought-chain/).

`chat_sources` uses a native disclosure to show research references. Each `chat_source` is a two-row card with an optional `summary` and an `activity` label. The Gallery places sources below the message bubble to avoid nested cards. The host still supplies trusted HTTP/HTTPS destinations.

## Composition

```rust,ignore
let draft = signal(cx, String::new);
let busy = signal(cx, || false);
let submit = attributes! { cx =>
    @submit=$(|event: Event| {
        event.prevent_default();
        // The page appends a user message and a pending assistant message.
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

`chat_sender` disables Send when the draft is empty or `busy` is true. Set `max_length` to limit the draft length. The page clears the draft, restores focus, and scrolls the message region as new messages arrive. Attachment content can be composed inside the sender; the host owns file selection and upload.

## Gallery routes

- `/chat`, `/chat/notes`: two sample conversations on real routes.
- `/chat/new`: empty state, suggestions, multiple turns, and message actions.
- `/bubble`, `/message-list`, `/sender`: individual base components.
- `/chat/states`, `/chat/markdown`, `/chat/think`, `/chat/thought-chain`, `/chat/sources`, `/chat/actions`, `/chat/attachments`, `/chat/files`, `/chat/prompts`, `/chat/conversations`: content and navigation examples.

Gallery's `gallery_reply` and `gallery_action` are validated demo `#[procedure]` functions. They do not call a model or persist conversations; cancel and retry only acknowledge demo actions. Sending appends a user message and an assistant message in `Sending` state. The chunk control updates only the active assistant bubble's browser signal. Completion, failure, and cancellation commit the final content and status to the message-list shard.
The Gallery uses Topcoat 0.9's explicit endpoint paths at `/_gallery/chat/reply`, `/_gallery/chat/action`, and `/_gallery/chat/messages` so requests and route registration are easier to inspect.

Real hosts should implement send, retry, authentication, and input validation through their own `#[procedure]` functions or endpoints. A streaming endpoint updates the active assistant message; cancellation must stop the real host request. The route identifies the current conversation while the server owns history and persistence. Treat browser-supplied message content, roles, states, and IDs as untrusted.

## Content safety

`chat_markdown` escapes raw HTML as text, permits safe Markdown link targets, and omits Markdown images. `chat_source` accepts only HTTP/HTTPS destinations; the host should still assess source trust. Gallery's general Markdown document renderer accepts trusted repository files only and is unsuitable for external chat content. Attachment components display metadata; the host validates type, size, download access, and uploads.
