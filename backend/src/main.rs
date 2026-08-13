#[tokio::main]
async fn main() -> anyhow::Result<()> {
    horse_racing_backend::bootstrap::run().await
}