use teloxide::utils::command::BotCommands;

pub(crate) type SearchTerm = String;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub(crate) enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "return a picture matching a given query")]
    Pic { search_term: SearchTerm },
    #[command(description = "return a random picture")]
    Random,
    #[command(description = "search picture based on given string")]
    Search { search_term: SearchTerm },
    #[command(description = "enable or disable Instagram link replacement", parse_with = parse_bool)]
    Ddinstagram { filter_state: Option<bool> },
    #[command(description = "enable or disable Twitter link replacement", parse_with = parse_bool)]
    Vxtwitter { filter_state: Option<bool> },
}

fn parse_bool(input: String) -> Result<(Option<bool>,), teloxide::utils::command::ParseError> {
    match input.to_lowercase().as_str() {
        "true" | "on" | "yes" | "enable" => Ok((Some(true),)),
        "false" | "off" | "no" | "disable" => Ok((Some(false),)),
        _ => Ok((None,)),
    }
}
