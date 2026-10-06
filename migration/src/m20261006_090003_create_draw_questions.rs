use sea_orm_migration::{prelude::*, schema::*, sea_query::extension::postgres::Type};

pub struct Migration;

#[derive(DeriveIden)]
enum DrawQuestions {
    Table,
    DrawId,
    Question,
}

#[derive(DeriveIden)]
enum Draws {
    Table,
    Id,
}

#[derive(DeriveIden, Clone, Copy)]
enum QuestionKind {
    #[sea_orm(iden = "question_kind")]
    Enum,
    TshirtSize,
    ShoeSize,
    Allergies,
    Wishes,
    Dislikes,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261006_090003_create_draw_questions"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let question_kinds = [
            QuestionKind::TshirtSize,
            QuestionKind::ShoeSize,
            QuestionKind::Allergies,
            QuestionKind::Wishes,
            QuestionKind::Dislikes,
        ];

        manager
            .create_type(
                Type::create()
                    .as_enum(QuestionKind::Enum)
                    .values(question_kinds)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DrawQuestions::Table)
                    .if_not_exists()
                    .col(integer(DrawQuestions::DrawId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-draw_questions-draw_id")
                            .from(DrawQuestions::Table, DrawQuestions::DrawId)
                            .to(Draws::Table, Draws::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(enumeration(
                        DrawQuestions::Question,
                        QuestionKind::Enum,
                        question_kinds,
                    ))
                    .primary_key(
                        Index::create()
                            .col(DrawQuestions::DrawId)
                            .col(DrawQuestions::Question),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DrawQuestions::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(QuestionKind::Enum).to_owned())
            .await
    }
}
