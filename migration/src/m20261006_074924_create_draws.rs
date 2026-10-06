use sea_orm_migration::{prelude::*, schema::*, sea_query::extension::postgres::Type};

pub struct Migration;

#[derive(DeriveIden)]
enum Draws {
    Table,
    Id,
    OwnerId,
    Kind,
    Name,
    ShareToken,
    DrawnAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum DrawKind {
    #[sea_orm(iden = "draw_kind")]
    Enum,
    SecretSanta,
    Teams,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261006_074924_create_draws"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(DrawKind::Enum)
                    .values([DrawKind::SecretSanta, DrawKind::Teams])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Draws::Table)
                    .if_not_exists()
                    .col(pk_auto(Draws::Id))
                    .col(integer(Draws::OwnerId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-draws-owner_id")
                            .from(Draws::Table, Draws::OwnerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(enumeration(
                        Draws::Kind,
                        DrawKind::Enum,
                        [DrawKind::SecretSanta, DrawKind::Teams],
                    ))
                    .col(string(Draws::Name))
                    .col(string_null(Draws::ShareToken).unique_key().take())
                    .col(timestamp_with_time_zone_null(Draws::DrawnAt))
                    .col(timestamp_with_time_zone_default_now(Draws::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Draws::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(DrawKind::Enum).to_owned())
            .await
    }
}
