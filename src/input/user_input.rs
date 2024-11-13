#[derive(Debug)]
pub struct UserInput {
    pub chart_name: Option<String>,
    pub inline_json: Option<String>,
    pub file_json: Option<String>,
}
