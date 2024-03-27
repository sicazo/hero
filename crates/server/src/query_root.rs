use crate::{
    entities::application_data::Entity as ApplicationDataEntity,
    entities::application_data::Model as ApplicationData, state::ServerState,
};
use async_graphql::*;
use sea_orm::{DatabaseConnection, EntityTrait};
pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &str {
        "Hello world!"
    }

    async fn test(&self) -> &str {
        "test"
    }

    async fn settings(&self, ctx: &Context<'_>) -> Result<ApplicationData> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        match ApplicationDataEntity::find_by_id(1)
            .one(&db.to_owned())
            .await
            .unwrap()
        {
            Some(data) => Ok(ApplicationData { ..data }),
            None => Err(async_graphql::Error::new("Not found")),
        }
    }
}
