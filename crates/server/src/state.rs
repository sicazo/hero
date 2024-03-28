use crate::query_root::Query;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct ServerState {
    pub db: DatabaseConnection,
    pub schema: Schema<Query, EmptyMutation, EmptySubscription>,
}
