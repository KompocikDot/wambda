use axum::body::Bytes;
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
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

fn decode_b64(s: &str) -> Option<String> {
    String::from_utf8(BASE64.decode(s).ok()?).ok()
}

// stored as "b64(key):b64(value),b64(key):b64(value)"; malformed pairs are skipped
pub fn unstringify_envs(stringified_envs: &str) -> Vec<Env> {
    // dencrypt these values somehow - maybe with user's api key?
    stringified_envs
        .split(',')
        .filter_map(|key_val| {
            let (key, value) = key_val.split_once(':')?;
            Some(Env {
                key: decode_b64(key)?,
                value: decode_b64(value)?,
            })
        })
        .collect()
}

impl From<FunctionModel> for SingleFunctionResponse {
    fn from(value: FunctionModel) -> Self {
        Self {
            uuid: value.uuid,
            name: value.name,
            memory_limit: value.memory_limit,
            timeout: value.timeout,
            envs: unstringify_envs(&value.envs),
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
