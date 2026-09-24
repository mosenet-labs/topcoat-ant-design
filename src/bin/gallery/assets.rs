use std::error::Error;

use topcoat::{
    Result,
    asset::{Asset, AssetConfig, Manifest, ManifestEntry},
    router::{Body, response::Response, route},
};
use topcoat_ant_design::{STYLESHEET, STYLESHEET_SHA256};

/// Gallery 布局的 Tailwind 样式，与组件库公开样式分开打包。
pub(crate) const GALLERY_STYLESHEET: Asset = topcoat::tailwind::stylesheet!();

/// 构造 Gallery 使用的内嵌静态资源配置。
pub(crate) fn config() -> std::result::Result<AssetConfig, Box<dyn Error>> {
    let mut manifest = Manifest::parse(include_str!(concat!(
        env!("OUT_DIR"),
        "/runtime-manifest.toml"
    )))?;
    manifest.assets.extend([
        ManifestEntry {
            id: STYLESHEET.id(),
            file: "topcoat-ant-design-css".to_owned(),
            hash: STYLESHEET_SHA256.trim().to_owned(),
            content_type: "text/css".to_owned(),
        },
        ManifestEntry {
            id: GALLERY_STYLESHEET.id(),
            file: "gallery-css".to_owned(),
            hash: include_str!(concat!(env!("OUT_DIR"), "/gallery-tailwind.sha256"))
                .trim()
                .to_owned(),
            content_type: "text/css".to_owned(),
        },
    ]);

    Ok(AssetConfig::hosted_at("/assets", manifest))
}

#[route(GET "/assets/topcoat-ant-design-css")]
pub(crate) async fn component_css() -> Result<Response> {
    embedded_asset(
        "text/css; charset=utf-8",
        include_str!(concat!(env!("OUT_DIR"), "/topcoat-ant-design.css")),
    )
}

#[route(GET "/assets/gallery-css")]
pub(crate) async fn gallery_css() -> Result<Response> {
    embedded_asset(
        "text/css; charset=utf-8",
        include_str!(concat!(env!("OUT_DIR"), "/tailwind.css")),
    )
}

#[route(GET "/assets/topcoat-runtime-js")]
pub(crate) async fn topcoat_runtime_js() -> Result<Response> {
    embedded_asset(
        "text/javascript; charset=utf-8",
        include_str!(concat!(env!("OUT_DIR"), "/topcoat-runtime.js")),
    )
}

fn embedded_asset(content_type: &'static str, body: &'static str) -> Result<Response> {
    Ok(Response::builder()
        .header("content-type", content_type)
        .header("cache-control", "no-cache")
        .body(Body::from(body))?)
}
