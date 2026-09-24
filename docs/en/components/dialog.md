Use a native modal dialog for tasks that need focused editing or confirmation.

Dialog supplies a consistent heading, close button, backdrop, entry animation, and accessibility semantics. The caller supplies content, footer actions, and submission behavior. While an asynchronous operation is running, `busy` disables Escape and the close button so that in-progress state is not lost.

## Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `config` | `DialogConfig<'_>` | Trusted DOM ID, title, and optional eyebrow. |
| `busy` | `&Signal<bool>` | When `true`, disables the close button and Escape. |
| `language` | `UiLanguage` | Optional labels supplied by the component; defaults to English. Use `ChineseSimplified` for Chinese. |
| `attrs` | `Attributes` | Optional root attributes; caller and component classes are merged. |
| `child` | `Child` | Caller-provided form, content, and footer actions. |

## Example

```rust,ignore
use topcoat_ant_design::{
    DialogConfig, dialog, dialog_close_attributes, dialog_trigger_attributes,
};
use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, signal},
    view::{View, attributes, component, view},
};

#[component]
async fn connection_dialog(cx: &Cx) -> Result<impl View> {
    let busy = signal(cx, || false);
    let trigger = dialog_trigger_attributes(cx, "connection-dialog");
    let cancel = dialog_close_attributes(cx, "connection-dialog");

    Ok(view! {
        <button type="button" (trigger)>"Edit connection"</button>
        dialog(
            config: DialogConfig::new("connection-dialog", "Edit connection")
                .with_eyebrow("CONNECTION"),
            busy: &busy,
            attrs: attributes! { @close=$(|_event: Event| clear_secret()) },
            <form @submit=$(|event: Event| event.prevent_default())>
                <div>"Form contents"</div>
                <footer>
                    <button type="button" (cancel)>"Cancel"</button>
                    <button type="submit">"Save"</button>
                </footer>
            </form>
        )
    })
}
```

## Behavior and accessibility

- `dialog_trigger_attributes` opens the modal with native `commandfor="…" command="show-modal"`.
- `dialog_close_attributes` closes it with native `command="close"`.
- The component supplies the header close button. Handle `@close` on the root to clear application state.
- `busy = true` keeps the dialog open while work is in progress.
- Entry animation is disabled for users who request reduced motion.
- Native `<dialog>` carries `role="dialog"`, `aria-modal="true"`, and `aria-labelledby`. The trigger has `aria-haspopup="dialog"` and `aria-controls`.

The ID also associates the dialog with its heading. Generate it from trusted code, not unchecked user input.
