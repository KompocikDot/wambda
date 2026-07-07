use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "language")]
pub enum Language {
    #[sea_orm(string_value = "c")]
    C,
    #[sea_orm(string_value = "go")]
    Go,
    #[sea_orm(string_value = "python")]
    Python,
    #[sea_orm(string_value = "rust")]
    Rust,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "trigger_type")]
pub enum TriggerType {
    #[sea_orm(string_value = "http")]
    HTTP,
    #[sea_orm(string_value = "cron")]
    Cron,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "functions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub uuid: Uuid,
    pub language: Language,
    pub trigger: TriggerType,
    pub memory_limit: i32,
    pub timeout: i32,
    pub envs: String,

    pub owner_id: i32,
    #[sea_orm(belongs_to, from = "owner_id", to = "id")]
    pub owner: HasOne<super::user::Entity>,

    #[sea_orm(has_many)]
    pub runs: HasMany<super::run::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
