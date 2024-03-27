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
                    .table(ApplicationData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ApplicationData::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ApplicationData::InitialLaunch)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(ApplicationData::FileChangeNotification)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ApplicationData::FinishedScanNotification)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ApplicationData::FinishedTranslationNotification)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ApplicationData::NavOpen)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ApplicationData::NotificationsEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                      ColumnDef::new(ApplicationData::ResizablePanelState).json().not_null().default("{home_collapsed_size: 4, home_default_sizes: [96], home_nav_collapsed: true}")
                    )
                    .col(
                      ColumnDef::new(ApplicationData::Theme).enumeration(Alias::new("Theme"), Theme::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(ApplicationData::ToastRichColors).boolean().not_null().default(true))
                    .col(ColumnDef::new(ApplicationData::TranslationDefaultLanguage).string().default("en-GB"))
                    .col(ColumnDef::new(ApplicationData::TranslateNewStrings).boolean().default(false))
                    .col(ColumnDef::new(ApplicationData::TranslateUpdatedStrings).boolean().default(false))
                    .col(ColumnDef::new(ApplicationData::TranslationCommand).string())
                    .to_owned(),

            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ApplicationData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ApplicationData {
    Table,
    Id,
    InitialLaunch,
    FileChangeNotification,
    FinishedScanNotification,
    FinishedTranslationNotification,
    NavOpen,
    NotificationsEnabled,
    ResizablePanelState,
    Theme,
    ToastRichColors,
    TranslationDefaultLanguage,
    TranslateNewStrings,
    TranslateUpdatedStrings,
    TranslationCommand,
}
#[derive(DeriveIden, EnumIter)]
pub enum Theme {
    Light,
    Dark,
}
