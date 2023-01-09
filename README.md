# linkleaner [![Check Rust code](https://github.com/msfjarvis/linkleaner/actions/workflows/test.yml/badge.svg)](https://github.com/msfjarvis/linkleaner/actions/workflows/test.yml) [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

Telegram bot to replace social media links with their improved preview variants. Supported platforms:

- Twitter: [FixTweet](https://github.com/FixTweet/FixTweet)
- Instagram: [InstaFix](https://github.com/Wikidepia/InstaFix) (behind the `ddinstagram` feature)
- Accelerated Mobile Pages (AMP): [AmputatorBot](https://www.amputatorbot.com/)
- YouTube Shorts: In-process transform (changes `/shorts/` URLs to regular video player)

### Running

- Copy `.env.sample` as `.env` and edit with the necessary details
- Use `cargo run` to start the bot
