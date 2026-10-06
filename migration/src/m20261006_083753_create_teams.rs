use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

#[derive(DeriveIden)]
enum Teams {
    Table,
    Id,
    DrawId,
    Name,
    Position,
}

#[derive(DeriveIden)]
enum Draws {
    Table,
    Id,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261006_083753_create_teams"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Teams::Table)
                    .if_not_exists()
                    .col(pk_auto(Teams::Id))
                    .col(integer(Teams::DrawId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-teams-draw_id")
                            .from(Teams::Table, Teams::DrawId)
                            .to(Draws::Table, Draws::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(string(Teams::Name))
                    .col(small_integer(Teams::Position))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Teams::Table).to_owned())
            .await
    }
}
