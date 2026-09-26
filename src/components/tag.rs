use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

use crate::native_ui::badge::{BadgeVariant, badge};

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
    let tone_class = match tone {
        TagTone::Default => "gr-tag-default",
        TagTone::Success => "gr-tag-success",
        TagTone::Warning => "gr-tag-warning",
        TagTone::Error => "gr-tag-error",
        TagTone::Processing => "gr-tag-processing",
    };
    let caller_class = attrs.remove("class");
    let root_class = class!("native-ui gr-tag", tone_class, caller_class);
    Ok(
        view! { badge(variant: BadgeVariant::Outline, attrs: topcoat::view::attributes! { class=(root_class) (attrs) }, (child)) },
    )
}
