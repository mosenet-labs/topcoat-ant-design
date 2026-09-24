use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

/// 表单字段的稳定标识、标签和辅助信息。
#[derive(Clone, Copy)]
pub struct FormFieldConfig<'a> {
    id: &'a str,
    label: &'a str,
    required: bool,
    hint: Option<&'a str>,
    error: Option<&'a str>,
}

impl<'a> FormFieldConfig<'a> {
    /// 创建一个基础字段。调用方应把相同的 `id` 设置到内部控件上。
    pub const fn new(id: &'a str, label: &'a str) -> Self {
        Self {
            id,
            label,
            required: false,
            hint: None,
            error: None,
        }
    }

    /// 显示必填标记；实际必填约束仍由内部控件和服务端共同负责。
    pub const fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// 添加正常状态下的简短说明。
    pub const fn with_hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    /// 添加字段错误；传入后会覆盖辅助说明并使用 `role=alert`。
    pub const fn with_error(mut self, error: &'a str) -> Self {
        self.error = Some(error);
        self
    }
}

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/form-field.md"
))]
#[component]
pub async fn form_field(
    cx: &Cx,
    config: FormFieldConfig<'_>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let FormFieldConfig {
        id,
        label,
        required,
        hint,
        error,
    } = config;
    let caller_class = attrs.remove("class");
    let root_class = class!("grid gap-2", caller_class);
    let help_id = format!("{id}-help");
    attrs.extend(attributes! { cx => class=(root_class) });

    Ok(view! {
        <div (attrs)>
            <label class="text-sm font-medium leading-5 text-[#262626]" for=(id)>
                (label)
                if required { <span class="ml-1 text-[#ff4d4f]" aria-hidden="true">"*"</span> }
            </label>
            (child)
            if let Some(message) = error {
                <p class="m-0 text-xs leading-5 text-[#ff4d4f]" id=(help_id.as_str()) role="alert">(message)</p>
            } else if let Some(message) = hint {
                <p class="m-0 text-xs leading-5 text-[#8c8c8c]" id=(help_id.as_str())>(message)</p>
            }
        </div>
    })
}
