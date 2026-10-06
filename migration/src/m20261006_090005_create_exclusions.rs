use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

#[derive(DeriveIden)]
enum Exclusions {
    Table,
    ParticipantA,
    ParticipantB,
}

#[derive(DeriveIden)]
enum Participants {
    Table,
    Id,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261006_090005_create_exclusions"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Exclusions::Table)
                    .if_not_exists()
                    .col(integer(Exclusions::ParticipantA))
                    .col(integer(Exclusions::ParticipantB))
                    .primary_key(
                        Index::create()
                            .col(Exclusions::ParticipantA)
                            .col(Exclusions::ParticipantB),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-exclusions-participant_a")
                            .from(Exclusions::Table, Exclusions::ParticipantA)
                            .to(Participants::Table, Participants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-exclusions-participant_b")
                            .from(Exclusions::Table, Exclusions::ParticipantB)
                            .to(Participants::Table, Participants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // Une paire stockée dans un seul sens, et jamais (x, x)
                    .check(
                        Expr::col(Exclusions::ParticipantA).lt(Expr::col(Exclusions::ParticipantB)),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Exclusions::Table).to_owned())
            .await
    }
}
