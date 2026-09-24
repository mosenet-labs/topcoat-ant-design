Show an operation result in the upper-right corner of the viewport. The component enters the browser Top Layer. When a native modal Dialog is open, it temporarily mounts inside that dialog so it remains visible and interactive above the modal.

The caller owns a `Signal<String>` containing the message. A nonempty message shows the notification. It hides when the timer expires, the user closes it, or the caller clears the message. It stores no business state and sends no server request.

## Parameters

| Parameter | Type | Description |
| --- | --- | --- |
| `message` | `&Signal<String>` | Body and visibility state; an empty string hides it. |
| `title` | `&str` | Short heading, such as “Saved”. |
| `tone` | `NotificationTone` | `Success`, `Info`, `Warning`, or `Error`. |
| `language` | `UiLanguage` | Optional labels supplied by the component; defaults to English. Use `ChineseSimplified` for Chinese. |
| `attrs` | `Attributes` | Optional root attributes; caller and component classes are merged. |

## Example

```rust,ignore
use topcoat_ant_design::{NotificationTone, notification};
use topcoat::{
    Result,
    context::Cx,
    runtime::signal,
    view::{View, attributes, component, view},
};

#[component]
async fn save_example(cx: &Cx) -> Result<impl View> {
    let notice = signal(cx, String::new);

    Ok(view! {
        <button @click=$(|_e| notice.set("Saved successfully.".to_owned()))>
            "Save"
        </button>
        notification(
            message: &notice,
            title: "Saved",
            tone: NotificationTone::Success,
            attrs: attributes! { data-source="save-form" },
        )
    })
}
```

## Behavior and accessibility

- Auto-close is approximately 4.5 seconds by default.
- Hovering or moving focus inside pauses the timer.
- Both manual close and timer expiration clear the caller's message.
- One message signal controls one notification; the component has no queue.
- `Success`, `Info`, and `Warning` use `role="status"` and `aria-live="polite"`. `Error` uses `role="alert"` and `aria-live="assertive"` for urgent failures.
