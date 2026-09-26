#[cfg(feature = "gallery")]
use std::path::Path;
use std::{env, fs, path::PathBuf};

use sha2::{Digest, Sha256};
#[cfg(feature = "gallery")]
use topcoat_asset::{MANIFEST_VERSION, Manifest, ManifestEntry, RawAsset};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo OUT_DIR"));

    // 将 Ant Design 图标集固定在仓库缓存中，CI 构建无需临时访问 Iconify。
    topcoat::icon::iconify::BuildConfig::new()
        .cache_dir("icons")
        .icon_set("ant-design")
        .stage()
        .expect("stage Ant Design Iconify set");
    topcoat::icon::iconify::BuildConfig::new()
        .cache_dir("icons")
        .icon_set("lucide")
        .stage()
        .expect("stage Lucide Iconify set for official UI components");

    // UI crate 自己生成组件样式，宿主只需把公开的 Asset 加入最终资源包。
    let component_stylesheet_path = if env::var_os("DOCS_RS").is_some() {
        // docs.rs 禁止网络访问，无法由 Topcoat 下载 Tailwind CLI。
        let path = out.join("topcoat-ant-design.css");
        fs::copy("assets/topcoat-ant-design.css", &path)
            .expect("copy bundled component stylesheet for docs.rs");
        path
    } else {
        topcoat::tailwind::BuildConfig::new()
            .input("styles.css")
            .output(out.join("topcoat-ant-design.css"))
            .render()
            .expect("render component Tailwind stylesheet")
    };
    let component_stylesheet =
        fs::read(component_stylesheet_path).expect("read component Tailwind stylesheet");
    fs::write(
        out.join("component-tailwind.sha256"),
        format!("{:x}", Sha256::digest(&component_stylesheet)),
    )
    .expect("write component Tailwind stylesheet hash");

    #[cfg(feature = "gallery")]
    build_gallery(&out);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/topcoat-ant-design.css");
    println!("cargo:rerun-if-changed=assets/topcoat-ant-design-tokens.css");
    println!("cargo:rerun-if-changed=icons/ant-design.json");
    println!("cargo:rerun-if-changed=styles.css");
    println!("cargo:rerun-if-changed=src/components");
    println!("cargo:rerun-if-changed=assets/topcoat-ant-design-theme.css");
    println!("cargo:rerun-if-changed=src/ui");
    #[cfg(feature = "gallery")]
    println!("cargo:rerun-if-changed=src/bin/gallery");
}

#[cfg(feature = "gallery")]
fn build_gallery(out: &Path) {
    // Gallery 有独立的 Tailwind 入口，不把文档站布局混入公共组件样式。
    let gallery_stylesheet_path = topcoat::tailwind::BuildConfig::new()
        .input("src/bin/gallery/styles.css")
        .render()
        .expect("render Gallery Tailwind stylesheet");
    let gallery_stylesheet =
        fs::read(gallery_stylesheet_path).expect("read Gallery Tailwind stylesheet");

    // Gallery 是可独立启动的单一二进制，因此把 Runtime 和样式清单一起嵌入。
    let id = topcoat_runtime::SCRIPT.id();
    let binary =
        fs::read(env::current_exe().expect("build script executable")).expect("read build script");
    let asset = RawAsset::find_in_binary(&binary)
        .into_iter()
        .find(|asset| asset.id() == id)
        .expect("Topcoat runtime asset declaration");
    let source = asset.resolved_path();
    println!("cargo:rerun-if-changed={}", source.display());
    let script = fs::read(source).expect("read Topcoat runtime script");
    fs::write(out.join("topcoat-runtime.js"), &script).expect("embed Topcoat runtime script");

    fs::write(
        out.join("gallery-tailwind.sha256"),
        format!("{:x}", Sha256::digest(&gallery_stylesheet)),
    )
    .expect("write Gallery Tailwind stylesheet hash");
    Manifest {
        version: MANIFEST_VERSION,
        assets: vec![ManifestEntry {
            id,
            file: "topcoat-runtime-js".to_owned(),
            hash: format!("{:x}", Sha256::digest(&script)),
            content_type: "text/javascript".to_owned(),
        }],
    }
    .save(out.join("runtime-manifest.toml"))
    .expect("write runtime asset manifest");
}
