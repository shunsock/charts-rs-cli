use crate::input::user_input::UserInput;
use crate::input::validated_user_input::ValidateUserInput;
use serde_json::Value;
use std::error::Error;

pub struct Validator {
    user_input: UserInput,
}

impl Validator {
    pub fn new(user_input: UserInput) -> Self {
        Validator { user_input }
    }

    pub fn validate(self) -> Result<ValidateUserInput, Box<dyn Error>> {
        if self.user_input.chart_name.is_none() {
            return Err("チャート名が指定されていません".into());
        }

        if self.user_input.inline_json.is_none() && self.user_input.file_json.is_none() {
            return Err("JSONが指定されていません".into());
        }

        if self.user_input.inline_json.is_some() && self.user_input.file_json.is_some() {
            return Err("inline-jsonとfile-jsonは同時に指定できません".into());
        }

        // chart_nameはClapで必須項目にしているのでunwrapできる
        let chart_name = self.user_input.chart_name.unwrap();

        let json_strings = if self.user_input.inline_json.is_some() {
            self.user_input.inline_json.unwrap()
        } else {
            let json_path = self.user_input.file_json.unwrap();
            match std::fs::read_to_string(json_path) {
                Ok(json) => json,
                Err(_) => return Err("ファイルの読み込みに失敗しました".into()),
            }
        };

        let json: Value = serde_json::from_str(&json_strings)
            .map_err(|_| "JSONのパースに失敗しました。正しいJSON形式か確認してください")?;

        Ok(ValidateUserInput { chart_name, json })
    }
}
