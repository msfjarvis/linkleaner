use teloxide::utils::command::BotCommands;

pub(crate) type SearchTerm = String;
pub(crate) type FilterState = String;

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
    #[command(description = "enable or disable Instagram link replacement")]
    Ddinstagram { filter_state: FilterState },
    #[command(description = "enable or disable Twitter link replacement")]
    Vxtwitter { filter_state: FilterState },
}
