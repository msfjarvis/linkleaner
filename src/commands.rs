use teloxide::utils::command::BotCommand;

pub(crate) type SearchTerm = String;

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub(crate) enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "return a picture matching a given query")]
    Pic { search_term: SearchTerm },
    #[command(description = "return a random picture")]
    Random,
    #[command(description = "search picture based on given string")]
    Search { search_term: SearchTerm },
}
