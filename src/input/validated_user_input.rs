use serde_json::Value;

pub struct ValidateUserInput {
    pub chart_name: String,
    pub json: Value,
}
