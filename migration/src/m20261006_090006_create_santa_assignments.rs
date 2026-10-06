use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

#[derive(DeriveIden)]
enum SantaAssignments {
    Table,
    GiverId,
    ReceiverId,
}

#[derive(DeriveIden)]
enum Participants {
    Table,
    Id,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261006_090006_create_santa_assignments"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SantaAssignments::Table)
                    .if_not_exists()
                    .col(integer(SantaAssignments::GiverId))
                    .primary_key(Index::create().col(SantaAssignments::GiverId))
                    .col(integer_uniq(SantaAssignments::ReceiverId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-santa_assignments-giver_id")
                            .from(SantaAssignments::Table, SantaAssignments::GiverId)
                            .to(Participants::Table, Participants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-santa_assignments-receiver_id")
                            .from(SantaAssignments::Table, SantaAssignments::ReceiverId)
                            .to(Participants::Table, Participants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::col(SantaAssignments::GiverId)
                            .ne(Expr::col(SantaAssignments::ReceiverId)),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SantaAssignments::Table).to_owned())
            .await
    }
}
