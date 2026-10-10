use crate::{
    app::page_header,
    demo::component_example,
    locale::Locale,
    markdown::{markdown_document, rust_code_block},
};
use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};
use topcoat_ant_design::json_viewer;

const DOCUMENT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/components/json-viewer.md"
));
const DOCUMENT_EN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/docs/en/components/json-viewer.md"
));

#[page]
pub(in crate::app) async fn json_viewer_page(cx: &Cx) -> Result<impl View> {
    let locale = Locale::current(cx);
    let document = locale.select(DOCUMENT_EN, DOCUMENT);
    let data = serde_json::json!({
        "kind": "Pod", "apiVersion": "v1",
        "metadata": {"name": "api-7b8fc", "namespace": "production", "labels": {"app": "api"}},
        "spec": {"containers": [{"name": "api", "image": "example/api:1.0"}]},
        "status": {"phase": "Running", "ready": true, "restarts": 0, "message": null}
    });
    Ok(view! {
        page_header(eyebrow: "DATA DISPLAY", title: "JSONViewer",
            description: locale.select("Explore JSON trees and copy collapsed nodes.", "折叠浏览 JSON，悬停或聚焦未展开节点即可复制。"))
        <div class="grid gap-6">
            component_example(id: "json-viewer-preview", title: locale.select("JSON tree", "JSON 树"),
                description: locale.select("Expand an object or array. Copy a collapsed node without changing its state.", "展开对象或数组；复制未展开节点时保持折叠状态。"), source: rust_code_block(document, 0),
                json_viewer(value: &data, language: locale.ui())
            )
            markdown_document(source: document)
        </div>
    })
}
