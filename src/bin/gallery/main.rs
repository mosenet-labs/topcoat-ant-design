mod app;
mod assets;
mod demo;
mod markdown;

use std::{env, error::Error};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(3100);
    let listener = TcpListener::bind((host.as_str(), port)).await?;
    // Gallery 把资源嵌入同一个二进制；变量在传入 Router 前完成实际构造。
    let app_assets = assets::config()?;
    let router = app::router(app_assets);

    println!("Topcoat Ant Design Gallery: http://{host}:{port}");
    topcoat::serve(listener, router).await?;
    Ok(())
}
