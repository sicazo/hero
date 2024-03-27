pub use sea_orm_migration::prelude::*;

mod m20240321_121129_create_location_table;
mod m20240327_150619_application_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240321_121129_create_location_table::Migration),
            Box::new(m20240327_150619_application_data::Migration),
        ]
    }
}
