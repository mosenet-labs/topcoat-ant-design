# Native modal dialog

`native_dialog` wraps a browser modal `<dialog>` for forms that need focus trapping, Escape dismissal, and a busy state. It keeps the native dialog in sync with a Topcoat `Signal<bool>` and preserves the caller's `@close` handler.

Use `NativeDialogConfig::new(id, title)`, an `open` signal, and a `busy` signal. Call `native_dialog_close_attributes(cx, id)` on a cancel button. The `id` must come from trusted code.
