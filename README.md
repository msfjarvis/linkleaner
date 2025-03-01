# linkleaner [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

Telegram bot to replace social media links with their improved preview variants. The mapping of supported link types to what services are used to "fix" these links is given below.

| Link type | Fix mechanism |
|-----------|---------------|
| [Accelerated Mobile Pages (AMP)](https://amp.dev) | [AmputatorBot](https://www.amputatorbot.com/) |
| [Instagram](https://instagram.com) | [InstaFix](https://github.com/Wikidepia/InstaFix) |
| [Medium](https://medium.com) | [LibMedium](https://git.batsense.net/realaravinth/libmedium) |
| [Reddit](https://reddit.com) | [FxReddit](https://github.com/MinnDevelopment/fxreddit) |
| [TikTok](https://tiktok.com) | [fxTikTok](https://github.com/okdargy/fxtiktok) |
| [Twitter](https://twitter.com) / [X](https://x.com) | [TweetFix](https://github.com/FixTweet/FixTweet) |
| [YouTube Shorts](https://www.youtube.com/shorts) | Rewrite URL to normal YouTube player |

### Running

- Copy `.env.sample` as `.env` and fill in the bot token
- Use `cargo run --release` to start the bot
