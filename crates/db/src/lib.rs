use std::path::PathBuf;
use prisma_client_rust::migrations::MigrateDeployError;
use crate::prisma::PrismaClient;
use std::fs;
use std::sync::Arc;
use prisma::settings;

pub mod prisma;
pub mod context;


pub fn get_db_path() -> PathBuf {
    let config_dir = tauri::api::path::config_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Config directory not found")
    }).expect("unable to get path");
    config_dir.join("hero")

}

pub async fn load_and_migrate() -> Result<Arc<PrismaClient>, MigrateDeployError>{
    let db_path = get_db_path();
    let db_url = format!("file:{}/hero.db?mode=rwc", db_path.display());
    if !db_path.exists() {
        fs::create_dir_all(&db_path).expect("Folder creation failed");
    }
    let client = prisma::new_client_with_url(&db_url).await.expect("failed to create client");
    client._migrate_deploy().await?;
    let _ = client.settings().upsert(
        settings::id::equals(1),
        settings::create(vec![]), vec![]
    ).exec().await;
    Ok(Arc::new(client))
}