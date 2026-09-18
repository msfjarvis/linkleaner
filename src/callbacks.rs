use crate::AsyncError;
use teloxide::{
    Bot,
    payloads::AnswerCallbackQuerySetters,
    prelude::Requester,
    types::{CallbackQuery, UserId},
};

pub(crate) const DELETE_CALLBACK_PREFIX: &str = "delete:";

pub(crate) async fn handler(bot: Bot, query: CallbackQuery) -> Result<(), AsyncError> {
    let Some(owner_id) = query.data.as_deref().and_then(delete_owner) else {
        return Ok(());
    };

    if query.from.id != owner_id {
        bot.answer_callback_query(query.id)
            .text("Only the original sender can delete this message.")
            .show_alert(true)
            .await?;
        return Ok(());
    }

    bot.answer_callback_query(query.id).await?;
    if let Some(message) = query.message {
        bot.delete_message(message.chat().id, message.id()).await?;
    }
    Ok(())
}

fn delete_owner(data: &str) -> Option<UserId> {
    data.strip_prefix(DELETE_CALLBACK_PREFIX)?
        .parse()
        .ok()
        .map(UserId)
}

#[cfg(test)]
mod tests {
    use super::delete_owner;
    use teloxide::types::UserId;

    #[test]
    fn parses_delete_owner() {
        assert_eq!(delete_owner("delete:123"), Some(UserId(123)));
        assert_eq!(delete_owner("other:123"), None);
        assert_eq!(delete_owner("delete:not-a-user"), None);
    }
}
