# Dialog

The official Dialog renders a panel controlled by an `open` expression. Compose it from content, header, title, description, and footer. The host controls the open state and any modal focus behavior it needs.

```rust,ignore
use topcoat_ant_design::{dialog, dialog_content, dialog_description, dialog_footer, dialog_header, dialog_title, button};

let open = signal(cx, || false);
view! {
    button(attrs: attributes! { @click=$(|_event: Event| open.set(true)) }, "Edit")
    dialog(open: $(open.get()), attrs: attributes! { aria-labelledby="edit-title" },
        dialog_content(
            dialog_header(
                dialog_title(attrs: attributes! { id="edit-title" }, "Edit connection")
                dialog_description("Update the connection settings.")
            )
            dialog_footer(
                button(attrs: attributes! { @click=$(|_event: Event| open.set(false)) }, "Close")
            )
        )
    )
}
```
