use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub full_name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,

    #[sea_orm(has_many)]
    pub functions: HasMany<super::function::Entity>,

    #[sea_orm(has_many)]
    pub api_keys: HasMany<super::api_key::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
