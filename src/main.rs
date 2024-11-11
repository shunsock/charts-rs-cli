mod chart_type;
mod image_builder;
mod input;
mod receiver;
mod validator;

use image_builder::ImageBuilder;
use input::user_input::UserInput;
use input::validated_user_input::ValidateUserInput;
use receiver::Receiver;
use std::process::exit;
use validator::Validator;

fn main() {
    let receiver: Receiver = Receiver::new();
    let args: UserInput = receiver.receive();

    let validated_user_input: ValidateUserInput = match Validator::new(args).validate() {
        Ok(validated_user_input) => validated_user_input,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    match ImageBuilder::create_chart(
        validated_user_input.chart_name.as_str(),
        validated_user_input.json.to_string().as_str(),
    ) {
        Ok(_) => println!("[success] 図を作成しました"),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
