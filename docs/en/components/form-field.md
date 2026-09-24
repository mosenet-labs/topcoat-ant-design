# FormField

`form_field` provides a consistent label, required marker, hint, and error. It only renders the field shell; the caller owns the input value and validation.

Set the input's `id` to the value passed to `FormFieldConfig::new`. When a hint or error is present, associate the input with `{id}-help` through `aria-describedby`.

```rust,ignore
form_field(
    config: FormFieldConfig::new("automation-name", "Automation name")
        .required()
        .with_hint("Used to identify this automation in the project."),
    <input
        id="automation-name"
        name="name"
        required
        aria-describedby="automation-name-help"
    >
)
```

Use `with_error` for a server validation error. The error replaces the hint and uses `role="alert"`:

```rust,ignore
form_field(
    config: FormFieldConfig::new("command-name", "Command")
        .with_error("Use lowercase letters, digits, hyphens, or underscores."),
    <input
        id="command-name"
        name="command"
        aria-invalid="true"
        aria-describedby="command-name-help"
    >
)
```
