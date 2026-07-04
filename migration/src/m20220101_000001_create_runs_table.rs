use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("runs")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(timestamp_with_time_zone("start_time"))
                    .col(timestamp_with_time_zone("finish_time"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("runs").to_owned())
            .await
    }
}
