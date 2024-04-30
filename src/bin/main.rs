use loco_rs::cli;
use migration::Migrator;
use normal_oj::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App, Migrator>().await
}
