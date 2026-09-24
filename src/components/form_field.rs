use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

/// Stable field ID, label, and supporting information.
#[derive(Clone, Copy)]
pub struct FormFieldConfig<'a> {
    id: &'a str,
    label: &'a str,
    required: bool,
    hint: Option<&'a str>,
    error: Option<&'a str>,
}

impl<'a> FormFieldConfig<'a> {
    /// Create a basic field. Set the same `id` on the inner control.
    pub const fn new(id: &'a str, label: &'a str) -> Self {
        Self {
            id,
            label,
            required: false,
            hint: None,
            error: None,
        }
    }

    /// Show a required marker. The inner control and server still enforce the requirement.
    pub const fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Add a short hint for the normal state.
    pub const fn with_hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Add a field error, replacing the hint and using `role=alert`.
    pub const fn with_error(mut self, error: &'a str) -> Self {
        self.error = Some(error);
        self
    }
}

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/form-field.md"
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
