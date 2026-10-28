use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::function::{
    MainPageFunctionQuery, Model as FunctionModel,
};


#[derive(Validate, TryFromMultipart)]
#[try_from_multipart(rename_all = "camelCase")]
pub struct CreateFunctionPayload {
    pub name: String,
    pub memory_limit: i32,
    pub timeout: i32,
    pub envs_stringified: String,

    #[form_data(limit = "250MB")]
    pub wasm: FieldData<Bytes>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFunctionResponse {
    pub uuid: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    pub key: String,
    pub value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleFunctionResponse {
    pub uuid: Uuid,
    pub name: String,
    pub memory_limit: i32,
    pub timeout: i32,
    pub envs: Vec<Env>,
}

fn unstringify_envs(stringified_envs: String) -> Vec<Env> {
    // dencrypt these values somehow - maybe with user's api key?
    //
    let mut envs: Vec<Env> = Vec::new();
    if stringified_envs.len() == 0 {
        return envs;
    }

    for key_val in stringified_envs.split(",") {
        let items: Vec<&str> = key_val.split(":").collect();
        envs.push(Env {
            key: items[0].to_owned(),
            value: items[1].to_owned(),
        });
    }

    return envs;
}

impl From<FunctionModel> for SingleFunctionResponse {
    fn from(value: FunctionModel) -> Self {
        Self {
            uuid: value.uuid,
            name: value.name,
            memory_limit: value.memory_limit,
            timeout: value.timeout,
            envs: unstringify_envs(value.envs),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MainPageFunction {
    pub uuid: Uuid,
    pub name: String,
    pub status: String,
    pub invocations: i64,
    pub avg_latency: Option<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", transparent)]
pub struct MainPageFunctionsResponse {
    #[serde(flatten)]
    pub functions: Vec<MainPageFunction>,
}

impl From<Vec<MainPageFunctionQuery>> for MainPageFunctionsResponse {
    fn from(models: Vec<MainPageFunctionQuery>) -> Self {
        Self {
            functions: models
                .into_iter()
                .map(|m| MainPageFunction {
                    uuid: m.uuid,
                    name: m.name,
                    status: m.status,
                    invocations: m.invocations,
                    avg_latency: m.avg_latency,
                })
                .collect(),
        }
    }
}

#[derive(Validate, Deserialize)]
pub struct UpdateFunctionPayload {
    pub name: String,
    pub memory_limit: i32,
    pub timeout: i32,
    pub envs_stringified: String,
}
