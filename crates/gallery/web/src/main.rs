use anyhow::Result;
use gallery_web::http;

#[tokio::main]
async fn main() -> Result<()> {
    http::serve().await?;
    Ok(())
}
