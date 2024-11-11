use crate::input::user_input::UserInput;
use clap::{ArgMatches, Command};

pub struct Receiver {
    command: Command,
}

impl Receiver {
    pub fn new() -> Self {
        Receiver {
            command: Receiver::load(),
        }
    }

    pub fn receive(self) -> UserInput {
        let matches: ArgMatches = self.command.get_matches();
        let chart_name: Option<String> = matches.get_one::<String>("name").cloned();
        let inline_json: Option<String> = matches.get_one::<String>("inline").cloned();
        let file_json: Option<String> = matches.get_one::<String>("path").cloned();

        UserInput {
            chart_name,
            inline_json,
            file_json,
        }
    }

    fn load() -> Command {
        Command::new("charts")
            .about("Charts Command Line Interface")
            .version("0.1.0")
            .arg(
                clap::Arg::new("name")
                    .help("chart name")
                    .short('n')
                    .long("name")
                    .required(true),
            )
            .arg(
                clap::Arg::new("inline")
                    .help("inline JSON string")
                    .short('i')
                    .long("inline"),
            )
            .arg(
                clap::Arg::new("path")
                    .help("json file path")
                    .short('p')
                    .long("path"),
            )
    }
}
