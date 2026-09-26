use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

use crate::native_ui::tabs as native_tabs;

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/tabs.md"
))]
#[component]
pub async fn tabs(
    cx: &Cx,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let caller_class = attrs.remove("class");
    let root_class = class!("gr-tabs", caller_class);
    attrs.extend(attributes! { cx =>
        class=(root_class)
        aria-label=(label)
    });

    Ok(view! {
        native_tabs::tabs(attrs: attributes! { class="native-ui" },
            <nav (attrs)>
                native_tabs::tabs_list((child))
            </nav>
        )
    })
}

/// Route-based tab link; the host route determines `active`.
#[component]
pub async fn tab_link(
    cx: &Cx,
    href: &str,
    active: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let caller_class = attrs.remove("class");
    let link_class = class!("gr-tab-link", caller_class);
    attrs.extend(attributes! { cx =>
        class=(link_class)
        href=(href)
    });

    Ok(view! {
        native_tabs::tabs_trigger(active: active, attrs: attrs, (child))
    })
}
