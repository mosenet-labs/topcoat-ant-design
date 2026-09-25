# AI Components and Chat Interface Requirements

## Goal and boundaries

- Use Topcoat's server rendering, browser reactivity, routing, and server interaction features to build AI interface components inspired by Ant Design X.
- Start with a visible Chat interface to validate component composition, then add thinking, attachments, sources, and other capabilities.
- `topcoat-ant-design` owns reusable UI, state presentation, and browser interaction. The host application owns conversation persistence, model requests, authentication, and file storage.
- Keep a separate **AI Components** section in the Gallery for the composed Chat interface and individual component examples.

## Chat interface composition

```text
ChatPage (conversation data, routes, requests, streaming)
├── ConversationSidebar (conversation list and selection)
└── ChatPanel (page composition)
    ├── ChatHeader (title and conversation actions)
    ├── Welcome + Prompts (empty conversation)
    ├── MessageList
    │   └── MessageBubble
    │       ├── MessageContent (text, Markdown, or other content)
    │       └── MessageActions (copy, retry, etc.)
    └── ChatComposer
        └── AttachmentTray (attachments)
```

Reusable components own layout, semantics, styling, and local interaction. The page owns the message list, active conversation, and request state, and supplies send, cancel, and retry behavior. Messages need stable IDs and should at least distinguish `role`, `status`, and `content`. The content area remains composable for Markdown, tool output, and sources.

## Intended use of Topcoat features

| Need | Topcoat capability |
| --- | --- |
| Initial page and component structure | Server rendering with `#[component]` and `view!` |
| Drafts, button states, expansion | `Signal`, events, and reactive attributes |
| Partial updates of history or conversation lists | `#[shard]` and real routes |
| Send and retry operations | Host-owned `#[procedure]` functions |
| Incremental model output | A host-owned streaming endpoint that updates the active message |
| Styling, icons, and fonts | Existing Topcoat Asset, Tailwind, and Iconify setup |

The component library does not prescribe a model protocol. The host handles requests, authentication, errors, cancellation, and validation of browser-supplied input. Chat content may come from external sources, so Markdown or HTML must be rendered safely. The Gallery's Markdown renderer only handles trusted repository documents and is not suitable as-is for chat content.

## Implemented so far

- The Gallery's **AI Components** section has a composed [`/chat`](../../src/bin/gallery/app/_ai/chat.rs) page and individual [`/bubble`](../../src/bin/gallery/app/_ai/bubble.rs), [`/message-list`](../../src/bin/gallery/app/_ai/message_list.rs), and [`/sender`](../../src/bin/gallery/app/_ai/sender.rs) examples. Each example can reveal its source code.
- [`chat_bubble`, `chat_message_list`, and `chat_sender`](../../src/components/chat/mod.rs) are exported as public components, each in its own `chat/` submodule.
- The Chat page displays sample user and assistant messages. A browser-side signal holds the draft, and submitting displays the most recently sent message.
- The individual examples cover both message roles, the accessible message region, and local sender interaction.
- The preview supports English and Chinese. It does not call a model, persist conversations, or append a complete multi-turn history yet.

## Next requirements and acceptance

1. **Complete basic Chat interaction:** empty, sending, complete, error, and cancelled states; append multiple turns; keyboard behavior; scrolling and focus. Each state should have an operable Gallery example.
2. **Add conversation management:** list, active item, creation, and selection. Routes express the active conversation; the host supplies history data.
3. **Connect streaming:** use stable message IDs and incrementally update the active assistant message. Completion, failure, and cancellation remain consistent without re-rendering the whole page for every token.
4. **Expand content components:** Markdown, Think, ThoughtChain, Sources, Actions, attachments, files, and input suggestions while reusing the Chat message and composer structure.
5. **Maintain component quality:** accessible names and states, narrow-screen layout, English and Chinese copy, reduced-motion support, and an interactive Gallery example for each new component.

This document records the current direction and acceptance scope. The public API will be refined as the Chat interactions are implemented.
