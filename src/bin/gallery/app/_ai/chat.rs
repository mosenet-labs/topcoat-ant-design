pub(in crate::app) mod flow;
pub(in crate::app) mod live;
mod new;
mod notes;

pub(in crate::app) use live::chat_live_page;
pub(in crate::app) use new::chat_new_page;
pub(in crate::app) use notes::chat_notes_page;

use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

#[page]
pub(in crate::app) async fn chat_page(cx: &Cx) -> Result<impl View> {
    let _ = cx;
    Ok(view! { flow::chat_flow(session: "design") })
}
