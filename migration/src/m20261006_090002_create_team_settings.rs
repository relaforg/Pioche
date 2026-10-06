use sea_orm_migration::{prelude::*, schema::*, sea_query::extension::postgres::Type};

pub struct Migration;

#[derive(DeriveIden)]
enum TeamSettings {
    Table,
    DrawId,
    Mode,
    Value,
    HasCaptain,
    AllowUnequal,
    AllowRedraw,
}

#[derive(DeriveIden)]
enum Draws {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum TeamMode {
    #[sea_orm(iden = "team_mode")]
    Enum,
    Count,
    Size,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261006_090002_create_team_settings"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TeamMode::Enum)
                    .values([TeamMode::Size, TeamMode::Count])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TeamSettings::Table)
                    .if_not_exists()
                    .col(integer(TeamSettings::DrawId))
                    .primary_key(Index::create().col(TeamSettings::DrawId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-team_settings-draw_id")
                            .from(TeamSettings::Table, TeamSettings::DrawId)
                            .to(Draws::Table, Draws::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(enumeration(
                        TeamSettings::Mode,
                        TeamMode::Enum,
                        [TeamMode::Size, TeamMode::Count],
                    ))
                    .col(small_integer(TeamSettings::Value))
                    .col(boolean(TeamSettings::HasCaptain))
                    .col(boolean(TeamSettings::AllowUnequal))
                    .col(boolean(TeamSettings::AllowRedraw))
                    .check(Expr::col(TeamSettings::Value).gt(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TeamSettings::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(TeamMode::Enum).to_owned())
            .await
    }
}
