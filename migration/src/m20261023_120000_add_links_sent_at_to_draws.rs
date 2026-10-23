use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

#[derive(DeriveIden)]
enum Draws {
    Table,
    LinksSentAt,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261023_120000_add_links_sent_at_to_draws"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Secret Santa : date d'envoi des liens perso, fige les emails des participants
        manager
            .alter_table(
                Table::alter()
                    .table(Draws::Table)
                    .add_column(timestamp_with_time_zone_null(Draws::LinksSentAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Draws::Table)
                    .drop_column(Draws::LinksSentAt)
                    .to_owned(),
            )
            .await
    }
}
