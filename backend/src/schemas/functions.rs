use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromField, TryFromMultipart};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::function::{
    Language as ModelLanguage, MainPageFunctionQuery, Model as FunctionModel,
    TriggerType as ModelTrigger,
};

#[derive(Deserialize, Serialize, TryFromField, Clone)]
pub enum Language {
    #[serde(rename = "c")]
    #[field(rename = "c")]
    C,
    #[serde(rename = "go")]
    #[field(rename = "go")]
    Go,
    #[serde(rename = "python")]
    #[field(rename = "python")]
    Python,
    #[serde(rename = "rust")]
    #[field(rename = "rust")]
    Rust,
}

impl From<ModelLanguage> for Language {
    fn from(value: ModelLanguage) -> Self {
        match value {
            ModelLanguage::C => Language::C,
            ModelLanguage::Go => Language::Go,
            ModelLanguage::Python => Language::Python,
            ModelLanguage::Rust => Language::Rust,
        }
    }
}

#[derive(Deserialize, Serialize, TryFromField, Clone)]
pub enum Trigger {
    #[serde(rename = "http")]
    #[field(rename = "http")]
    HTTP,
    #[field(rename = "cron")]
    #[serde(rename = "cron")]
    Cron,
}

impl From<ModelTrigger> for Trigger {
    fn from(value: ModelTrigger) -> Self {
        match value {
            ModelTrigger::HTTP => Trigger::HTTP,
            ModelTrigger::Cron => Trigger::Cron,
        }
    }
}

#[derive(Validate, TryFromMultipart)]
#[try_from_multipart(rename_all = "camelCase")]
pub struct CreateFunctionPayload {
    pub name: String,
    pub language: Language,
    pub trigger: Trigger,
    pub memory_limit: i32,
    pub timeout: i32,
    pub envs_stringified: String,

    #[form_data(limit = "4Gb")]
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
pub struct FunctionResponse {
    pub uuid: Uuid,
    pub name: String,
    pub language: Language,
    pub trigger: Trigger,
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

impl From<FunctionModel> for FunctionResponse {
    fn from(value: FunctionModel) -> Self {
        Self {
            uuid: value.uuid,
            name: value.name,
            language: value.language.clone().into(),
            trigger: value.trigger.clone().into(),
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
    pub language: Language,
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
                    language: m.language.clone().into(),
                    status: m.status,
                    invocations: m.invocations,
                    avg_latency: m.avg_latency,
                })
                .collect(),
        }
    }
}
