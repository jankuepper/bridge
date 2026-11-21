use anyhow::Result;
use bridge::run;

#[actix_web::main]
async fn main() -> Result<()> {
    run().await
}
