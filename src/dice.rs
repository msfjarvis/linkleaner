use once_cell::sync::Lazy;
use regex::Regex;
use teloxide::{types::Message, Bot};

use crate::{
    message::BotExt,
    utils::{extract_dice_count, AsyncError},
};

static MATCH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^/(\d*)d(\d*)").unwrap());

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    if let Some(text) = message.text()
        && let Some(caps) = MATCH_REGEX.captures(text)
        && let Some(count) = caps.get(1)
        && let Some(sides) = caps.get(2)
        && let Ok(count) = extract_dice_count(count.as_str(), 6)
        && let Ok(sides) = sides.as_str().parse::<u8>()
    {
        let mut total = 0;
        for _ in 1..=count {
            total += roll_die(sides);
        }
        let text = format!("Rolled {count} die(s) with {sides} sides. Total: {total}");
        bot.try_reply(&message, text).await?;
    }
    Ok(())
}

// The dptree handler requires this to be a move
#[allow(clippy::needless_pass_by_value)]
pub fn is_die_roll(message: Message) -> bool {
    return if let Some(text) = message.text() {
        MATCH_REGEX.is_match(text)
    } else {
        false
    };
}

pub fn roll_die(sides: u8) -> u8 {
    rand::random::<u8>() % sides + 1
}

#[cfg(test)]
mod test {
    use super::MATCH_REGEX;

    #[test]
    fn test_regex_matches() {
        assert!(MATCH_REGEX.is_match("/1d6"));
        assert!(MATCH_REGEX.is_match("/2d6"));
        assert!(MATCH_REGEX.is_match("/10d6"));
        assert!(MATCH_REGEX.is_match("/1d20"));
        assert!(MATCH_REGEX.is_match("/2d20"));
        assert!(MATCH_REGEX.is_match("/10d20"));
        assert_eq!(
            "10",
            MATCH_REGEX
                .captures("/10d6")
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
        );
        assert_eq!(
            "6",
            MATCH_REGEX
                .captures("/10d6")
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
        );
    }
}
