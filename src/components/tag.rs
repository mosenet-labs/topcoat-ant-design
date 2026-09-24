use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// Semantic tag color. The label must also state the status without relying on color.
#[derive(Clone, Copy, Default)]
pub enum TagTone {
    #[default]
    Default,
    Success,
    Warning,
    Error,
    Processing,
}

/// Tag for status or categories with caller attributes and text children.
#[component]
pub async fn tag(
    #[default] tone: TagTone,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let color = match tone {
        TagTone::Default => "border-[#d9d9d9] bg-[#fafafa] text-[#595959]",
        TagTone::Success => "border-[#b7eb8f] bg-[#f6ffed] text-[#389e0d]",
        TagTone::Warning => "border-[#ffe58f] bg-[#fffbe6] text-[#ad6800]",
        TagTone::Error => "border-[#ffccc7] bg-[#fff2f0] text-[#cf1322]",
        TagTone::Processing => "border-[#91caff] bg-[#e6f4ff] text-[#0958d9]",
    };
    let caller_class = attrs.remove("class");
    let root_class = class!(
        "gr-tag inline-flex items-center whitespace-nowrap rounded border border-solid px-[7px] text-xs leading-5",
        color,
        caller_class
    );
    Ok(view! { <span class=(root_class) (attrs)>(child)</span> })
}
