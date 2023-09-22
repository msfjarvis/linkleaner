# linkleaner [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/) [![Built with Garnix](https://img.shields.io/endpoint?url=https%3A%2F%2Fgarnix.io%2Fapi%2Fbadges%2Fmsfjarvis%2Flinkleaner%3Fbranch%3Dmain)](https://garnix.io)

Telegram bot to replace social media links with their improved preview variants. The mapping of supported link types to what services are used to "fix" these links is given below.

| Link type | Fix mechanism |
|-----------|---------------|
| [Accelerated Mobile Pages (AMP)](https://amp.dev) | [AmputatorBot](https://www.amputatorbot.com/) |
| [Instagram](https://instagram.com) | [InstaFix](https://github.com/Wikidepia/InstaFix) |
| [Medium](https://medium.com) | [Scribe](https://sr.ht/~edwardloveall/Scribe/) |
| [Twitter](https://twitter.com) / [X](https://x.com) | [TweetFix](https://github.com/FixTweet/FixTweet) |
| [YouTube Shorts](https://www.youtube.com/shorts) | Rewrite URL to normal YouTube player

### Running

- Copy `.env.sample` as `.env` and fill in the bot token
- Use `cargo run --release` to start the bot
