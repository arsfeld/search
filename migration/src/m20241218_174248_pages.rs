use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Pages::Table)
                    .col(pk_auto(Pages::Id))
                    .col(string(Pages::Url))
                    .col(integer(Pages::WebsiteId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pages-website_ids")
                            .from(Pages::Table, Pages::WebsiteId)
                            .to(Websites::Table, Websites::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Pages::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Pages {
    Table,
    Id,
    Url,
    WebsiteId,
}

#[derive(DeriveIden)]
enum Websites {
    Table,
    Id,
}
