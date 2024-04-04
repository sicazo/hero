use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerState {
    pub db: Arc<DatabaseConnection>,
}
