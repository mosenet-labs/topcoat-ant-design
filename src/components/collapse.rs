use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, Signal},
    view::{Attributes, Child, View, attributes, class, component, view},
};

/// Create the Topcoat reactive attributes required by a Collapse trigger.
///
/// The caller still supplies `type="button"`, styling, and button content.
pub fn collapse_trigger_attributes(cx: &Cx, id: &str, open: &Signal<bool>) -> Attributes {
    let open = open.clone();
    attributes! { cx =>
        aria-controls=(id)
        :aria-expanded=$(if open.get() { "true" } else { "false" })
        @click=$(|_e: Event| open.toggle())
    }
}

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/collapse.md"
))]
#[component]
pub async fn collapse(
    cx: &Cx,
    id: &str,
    open: &Signal<bool>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let caller_class = attrs.remove("class");
    let root_class = class!("gr-collapse", caller_class);
    let open = open.clone();
    let state = attributes! { cx =>
        id=(id)
        class=(root_class)
        :data-state=$(if open.get() { "open" } else { "closed" })
        :aria-hidden=$(if open.get() { "false" } else { "true" })
    };
    attrs.extend(state);

    Ok(view! {
        <div (attrs)>
            <div class="gr-collapse-inner">(child)</div>
        </div>
    })
}
