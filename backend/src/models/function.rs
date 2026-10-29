use sea_orm::{ExprTrait, entity::prelude::*};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "functions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub memory_limit: i32,
    pub timeout: i32,
    pub envs: String,

    pub wasm_hash: String,

    pub owner_id: i32,
    #[sea_orm(belongs_to, from = "owner_id", to = "id", on_delete="Cascade")]
    pub owner: HasOne<super::user::Entity>,

    #[sea_orm(has_many)]
    pub runs: HasMany<super::run::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(DerivePartialModel)]
#[sea_orm(entity = "Entity")]
pub struct MainPageFunctionQuery {
    pub uuid: Uuid,
    pub name: String,
    #[sea_orm(from_expr = "Expr::val(\"active\")")]
    pub status: String,
    #[sea_orm(from_expr = "Expr::col((super::run::Entity, super::run::Column::Id)).count()")]
    pub invocations: i64,
    #[sea_orm(
        from_expr = "Expr::col((super::run::Entity, super::run::Column::FinishTime))
            .sub(Expr::col((super::run::Entity, super::run::Column::TriggeredAt)))
            .avg()
            .cast_as(\"float8\")"
    )]
    pub avg_latency: Option<f64>,
}
