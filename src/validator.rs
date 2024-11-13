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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_chart_name_is_none() {
        let user_input = UserInput {
            chart_name: None,
            inline_json: Some("{}".to_string()),
            file_json: None,
        };
        let validator = Validator::new(user_input);
        let result = validator.validate();
        assert_eq!(
            result.unwrap_err().to_string(),
            "チャート名が指定されていません"
        );
    }

    #[test]
    fn test_both_json_sources_are_none() {
        let user_input = UserInput {
            chart_name: Some("Test Chart".to_string()),
            inline_json: None,
            file_json: None,
        };
        let validator = Validator::new(user_input);
        let result = validator.validate();
        assert_eq!(result.unwrap_err().to_string(), "JSONが指定されていません");
    }

    #[test]
    fn test_both_json_sources_are_some() {
        let user_input = UserInput {
            chart_name: Some("Test Chart".to_string()),
            inline_json: Some("{}".to_string()),
            file_json: Some("test.json".to_string()),
        };
        let validator = Validator::new(user_input);
        let result = validator.validate();
        assert_eq!(
            result.unwrap_err().to_string(),
            "inline-jsonとfile-jsonは同時に指定できません"
        );
    }

    #[test]
    fn test_file_json_not_found() {
        let user_input = UserInput {
            chart_name: Some("Test Chart".to_string()),
            inline_json: None,
            file_json: Some("non_existent.json".to_string()),
        };
        let validator = Validator::new(user_input);
        let result = validator.validate();
        assert_eq!(
            result.unwrap_err().to_string(),
            "ファイルの読み込みに失敗しました"
        );
    }

    #[test]
    fn test_invalid_json() {
        let user_input = UserInput {
            chart_name: Some("Test Chart".to_string()),
            inline_json: Some("{invalid_json}".to_string()),
            file_json: None,
        };
        let validator = Validator::new(user_input);
        let result = validator.validate();
        assert_eq!(
            result.unwrap_err().to_string(),
            "JSONのパースに失敗しました。正しいJSON形式か確認してください"
        );
    }

    #[test]
    fn test_valid_input_with_inline_json() {
        let user_input = UserInput {
            chart_name: Some("Test Chart".to_string()),
            inline_json: Some(r#"{"key": "value"}"#.to_string()),
            file_json: None,
        };
        let validator = Validator::new(user_input);
        let result = validator.validate();
        assert!(result.is_ok());

        let validated_input = result.unwrap();
        assert_eq!(validated_input.chart_name, "Test Chart");
        assert_eq!(validated_input.json, json!({"key": "value"}));
    }

    #[test]
    fn test_valid_input_with_file_json() {
        // テスト用ファイルの作成
        let file_path = "test_valid.json";
        let mut file = fs::File::create(file_path).unwrap();
        writeln!(file, r#"{{"key": "value"}}"#).unwrap();

        let user_input = UserInput {
            chart_name: Some("Test Chart".to_string()),
            inline_json: None,
            file_json: Some(file_path.to_string()),
        };
        let validator = Validator::new(user_input);
        let result = validator.validate();
        assert!(result.is_ok());

        let validated_input = result.unwrap();
        assert_eq!(validated_input.chart_name, "Test Chart");
        assert_eq!(validated_input.json, json!({"key": "value"}));

        // テスト用ファイルの削除
        fs::remove_file(file_path).unwrap();
    }
}
