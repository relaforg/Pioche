use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

#[derive(DeriveIden)]
enum SantaSettings {
    Table,
    DrawId,
    BudgetCents,
    ExchangeDate,
}

#[derive(DeriveIden)]
enum Draws {
    Table,
    Id,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261006_090001_create_santa_settings"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SantaSettings::Table)
                    .if_not_exists()
                    .col(integer(SantaSettings::DrawId))
                    .primary_key(Index::create().col(SantaSettings::DrawId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-santa_settings-draw_id")
                            .from(SantaSettings::Table, SantaSettings::DrawId)
                            .to(Draws::Table, Draws::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer_null(SantaSettings::BudgetCents))
                    .col(date_null(SantaSettings::ExchangeDate))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SantaSettings::Table).to_owned())
            .await
    }
}
