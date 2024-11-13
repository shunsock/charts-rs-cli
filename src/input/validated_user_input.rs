use serde_json::Value;

#[derive(Debug)]
pub struct ValidateUserInput {
    pub chart_name: String,
    pub json: Value,
}
