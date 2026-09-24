use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use crate::app::overview_content;

#[page]
pub(in crate::app) async fn overview_page(cx: &Cx) -> Result<impl View> {
    let _ = cx;
    Ok(view! { overview_content() })
}
