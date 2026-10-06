use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

#[derive(DeriveIden)]
enum Participants {
    Table,
    Id,
    DrawId,
    Name,
    AccessToken,
    FormCompletedAt,
    TeamId,
    IsCaptain,
}

#[derive(DeriveIden)]
enum Draws {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Teams {
    Table,
    Id,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261006_090004_create_participants"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Participants::Table)
                    .if_not_exists()
                    .col(pk_auto(Participants::Id))
                    .col(integer(Participants::DrawId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-participants-draw_id")
                            .from(Participants::Table, Participants::DrawId)
                            .to(Draws::Table, Draws::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(string(Participants::Name))
                    .col(string_null(Participants::AccessToken).unique_key().take())
                    .col(timestamp_with_time_zone_null(Participants::FormCompletedAt))
                    .col(integer_null(Participants::TeamId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-participants-team_id")
                            .from(Participants::Table, Participants::TeamId)
                            .to(Teams::Table, Teams::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(boolean(Participants::IsCaptain).default(false).take())
                    .to_owned(),
            )
            .await?;

        // Pas deux participants du même nom dans un tirage
        manager
            .create_index(
                Index::create()
                    .name("idx-participants-draw_id-name")
                    .table(Participants::Table)
                    .col(Participants::DrawId)
                    .col(Participants::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Au plus un capitaine par équipe (index unique partiel)
        manager
            .create_index(
                Index::create()
                    .name("idx-participants-team_id-captain")
                    .table(Participants::Table)
                    .col(Participants::TeamId)
                    .unique()
                    .and_where(Expr::col(Participants::IsCaptain).eq(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Participants::Table).to_owned())
            .await
    }
}
