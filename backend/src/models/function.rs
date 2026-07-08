use std::collections::HashMap;

use sea_orm::entity::prelude::*;

use crate::schemas::functions::{Language as SchemaLanguage, Trigger};

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

impl From<SchemaLanguage> for Language {
    fn from(value: SchemaLanguage) -> Self {
        match value {
            SchemaLanguage::C => Self::C,
            SchemaLanguage::Go => Self::Go,
            SchemaLanguage::Python => Self::Python,
            SchemaLanguage::Rust => Self::Rust,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "trigger_type")]
pub enum TriggerType {
    #[sea_orm(string_value = "http")]
    HTTP,
    #[sea_orm(string_value = "cron")]
    Cron,
}

impl From<Trigger> for TriggerType {
    fn from(value: Trigger) -> Self {
        match value {
            Trigger::HTTP => Self::HTTP,
            Trigger::Cron => Self::Cron,
        }
    }
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "functions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(unique)]
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
