pub use sea_orm_migration::prelude::*;

mod m20260928_000001_create_users;
mod m20261001_120653_create_sessions;
mod m20261006_074924_create_draws;
mod m20261006_083753_create_teams;
mod m20261006_090001_create_santa_settings;
mod m20261006_090002_create_team_settings;
mod m20261006_090003_create_draw_questions;
mod m20261006_090004_create_participants;
mod m20261006_090005_create_exclusions;
mod m20261006_090006_create_santa_assignments;
mod m20261006_090007_create_answers;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260928_000001_create_users::Migration),
            Box::new(m20261001_120653_create_sessions::Migration),
            Box::new(m20261006_074924_create_draws::Migration),
            Box::new(m20261006_083753_create_teams::Migration),
            Box::new(m20261006_090001_create_santa_settings::Migration),
            Box::new(m20261006_090002_create_team_settings::Migration),
            Box::new(m20261006_090003_create_draw_questions::Migration),
            Box::new(m20261006_090004_create_participants::Migration),
            Box::new(m20261006_090005_create_exclusions::Migration),
            Box::new(m20261006_090006_create_santa_assignments::Migration),
            Box::new(m20261006_090007_create_answers::Migration),
        ]
    }
}
