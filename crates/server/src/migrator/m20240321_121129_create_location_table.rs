use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Location::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Location::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Location::Name).string().not_null())
                    .col(ColumnDef::new(Location::Path).string().not_null())
                    .col(
                        ColumnDef::new(Location::Tag)
                            .enumeration(Alias::new("Tag"), Tag::iter())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Location::IsFavorite)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Location::NumOfKeys)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Location::NumOfUntranslatedKeys)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Location::AddedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Location::ModifiedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Location::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Location {
    Table,
    Id,
    Name,
    Tag,
    Path,
    IsFavorite,
    NumOfKeys,
    NumOfUntranslatedKeys,
    AddedAt,
    ModifiedAt,
}
#[derive(DeriveIden, EnumIter)]
pub enum Tag {
    Frontend,
    Backend,
}
