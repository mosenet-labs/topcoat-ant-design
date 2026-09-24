use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// 标签的语义颜色；标签文字应同时表达状态，避免仅依赖颜色。
#[derive(Clone, Copy, Default)]
pub enum TagTone {
    #[default]
    Default,
    Success,
    Warning,
    Error,
    Processing,
}

/// 用于状态与分类的公共标签，支持调用方属性及文本子节点。
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
