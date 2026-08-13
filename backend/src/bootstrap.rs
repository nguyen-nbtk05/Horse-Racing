use crate::{
    config::Settings,
    infrastructure::database,
};


pub async fn run() -> anyhow::Result<()> {
    let settings = Settings::load()?;

    let _pool =
        database::connect(&settings.database).await?;

    println!("Database connected successfully");

    Ok(())
}