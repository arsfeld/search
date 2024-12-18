use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Pages {
    Table,
    Body,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        //
        // add column
        //
        manager
            .alter_table(
                Table::alter()
                    .table(Pages::Table)
                    .add_column_if_not_exists(string(Pages::Body))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Pages::Table)
                    .drop_column(Pages::Body)
                    .to_owned(),
            )
            .await
    }
}
