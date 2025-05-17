mod app;
mod routes;
mod middleware;

#[tokio::main]
async fn main() {
    app::run().await;
}