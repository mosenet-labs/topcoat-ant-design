# FormField 表单字段

`form_field` 统一字段标签、必填标记、辅助说明和错误信息。组件只负责字段外壳，不读取输入值，也不替业务层执行校验。

调用方需要把 `FormFieldConfig::new` 中的 `id` 同时设置到内部输入控件；存在说明或错误时，可把生成的 `{id}-help` 设置为控件的 `aria-describedby`。

```rust,ignore
form_field(
    config: FormFieldConfig::new("automation-name", "自动化名称")
        .required()
        .with_hint("用于项目内识别这项自动化。"),
    <input
        id="automation-name"
        name="name"
        required
        aria-describedby="automation-name-help"
    >
)
```

服务端返回字段错误后，可以改用 `with_error`。错误信息会替代普通说明，并使用 `role="alert"`：

```rust,ignore
form_field(
    config: FormFieldConfig::new("command-name", "指令")
        .with_error("指令只能包含小写字母、数字、连字符和下划线。"),
    <input
        id="command-name"
        name="command"
        aria-invalid="true"
        aria-describedby="command-name-help"
    >
)
```
