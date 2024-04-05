#[tokio::main]
async fn main() {
    paastel_rest::serve().await.unwrap();
}
