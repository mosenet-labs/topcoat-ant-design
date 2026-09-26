use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

#[page]
pub(in crate::app) async fn chat_new_page(cx: &Cx) -> Result<impl View> {
    let _ = cx;
    Ok(view! { super::flow::chat_flow(session: "new") })
}
