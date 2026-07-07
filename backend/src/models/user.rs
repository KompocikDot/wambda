use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub password: String,

    #[sea_orm(has_many)]
    pub functions: HasMany<super::function::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
