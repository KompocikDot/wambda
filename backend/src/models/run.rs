use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
pub enum Status {
    #[sea_orm(string_value = "success")]
    Success,
    #[sea_orm(string_value = "error")]
    Error,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "runs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub triggered_at: ChronoUnixTimestampMillis,
    pub finish_time: ChronoUnixTimestampMillis,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub status: Status,

    pub function_id: i32,
    #[sea_orm(belongs_to, from = "function_id", to = "id", on_delete="Cascade")]
    pub function: HasOne<super::function::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
