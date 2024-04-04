use sea_orm::{ConnectOptions, Database, DatabaseConnection, EntityTrait, Set};
use server::entities::application_data;
use server::entities::prelude::ApplicationData;
use server::migrator::Migrator;
use server::migrator::MigratorTrait;

pub async fn setup_db() -> DatabaseConnection{
    let config_dir = tauri::api::path::config_dir()
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Config directory not found")
        })
        .expect("failed to get config dir");
    let folder_path = config_dir.join("translationHero");
    let url = folder_path.join("hero.sqlite?mode=rwc");
    let db_url = format!("sqlite:{}", url.display());
    let mut db_options = ConnectOptions::new(&db_url);
    db_options.sqlx_logging(false);
    let db = Database::connect(db_options)
        .await
        .expect("failed to connect to db");
    Migrator::up(&db, None).await.expect("failed to migrate db");

    match ApplicationData::find_by_id(1).one(&db).await.unwrap() {
        Some(_) => println!("Not first start"),
        None => {
            let data = application_data::ActiveModel {
                theme: Set("Light".to_owned()),
                ..Default::default()
            };
            let _ = ApplicationData::insert(data).exec(&db).await;
        }
    }
    db

}