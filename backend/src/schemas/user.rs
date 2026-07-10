use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAccountDetailsPayload {
    #[validate(length(min = 1, max = 255))]
    pub full_name: String,
    #[validate(email)]
    pub email: String,
}
