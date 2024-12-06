mod routes;
mod server;
mod types;

#[tokio::main]
async fn main() {
    server::run_server().await;
}
