use topcoat::{
    Result,
    context::Cx,
    view::{Attributes, Child, View, attributes, class, component, view},
};

#[doc = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/tabs.md"
))]
#[component]
pub async fn tabs(
    cx: &Cx,
    label: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let caller_class = attrs.remove("class");
    let root_class = class!(
        "gr-tabs flex min-h-12 items-end gap-7 overflow-x-auto border-b border-[#f0f0f0] font-mono",
        caller_class,
    );
    attrs.extend(attributes! { cx =>
        class=(root_class)
        aria-label=(label)
    });

    Ok(view! {
        <nav (attrs)>(child)</nav>
    })
}

/// 路由型页签链接；`active` 由宿主路由判定。
#[component]
pub async fn tab_link(
    cx: &Cx,
    href: &str,
    active: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let caller_class = attrs.remove("class");
    let link_class = class!(
        "gr-tab-link relative inline-flex min-h-12 shrink-0 items-center border-b-2 px-0.5 pt-0.5 text-sm font-medium no-underline transition-colors duration-200 focus-visible:rounded-sm focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[#91caff]",
        "border-[#1677ff] text-[#1677ff]" if active,
        "border-transparent text-[#595959] hover:text-[#1677ff]" if !active,
        caller_class,
    );
    attrs.extend(attributes! { cx =>
        class=(link_class)
        href=(href)
        if active {
            aria-current="page"
        }
    });

    Ok(view! {
        <a (attrs)>(child)</a>
    })
}
