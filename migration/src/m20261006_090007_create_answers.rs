use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

#[derive(DeriveIden)]
enum Answers {
    Table,
    ParticipantId,
    Question,
    Value,
}

#[derive(DeriveIden)]
enum Participants {
    Table,
    Id,
}

// Type créé par m20261006_090003_create_draw_questions
#[derive(DeriveIden)]
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
        "m20261006_090007_create_answers"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Answers::Table)
                    .if_not_exists()
                    .col(integer(Answers::ParticipantId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-answers-participant_id")
                            .from(Answers::Table, Answers::ParticipantId)
                            .to(Participants::Table, Participants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(enumeration(
                        Answers::Question,
                        QuestionKind::Enum,
                        [
                            QuestionKind::TshirtSize,
                            QuestionKind::ShoeSize,
                            QuestionKind::Allergies,
                            QuestionKind::Wishes,
                            QuestionKind::Dislikes,
                        ],
                    ))
                    .primary_key(
                        Index::create()
                            .col(Answers::ParticipantId)
                            .col(Answers::Question),
                    )
                    .col(string(Answers::Value))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Answers::Table).to_owned())
            .await
    }
}
