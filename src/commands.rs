use teloxide::utils::command::BotCommand;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub(crate) enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "return a picture matching a given query")]
    Pic,
    #[command(description = "return a random picture")]
    Random,
    #[command(description = "search picture based on given string")]
    Search,
}
