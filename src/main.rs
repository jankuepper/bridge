use anyhow::Result;
use bridge::run;

#[actix_web::main]
async fn main() {
    run().await
}