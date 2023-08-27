# linkleaner [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/) [![Built with Garnix](https://img.shields.io/endpoint?url=https%3A%2F%2Fgarnix.io%2Fapi%2Fbadges%2Fmsfjarvis%2Flinkleaner%3Fbranch%3Dmain)](https://garnix.io)

Telegram bot to replace social media links with their improved preview variants. Supported platforms:

- Accelerated Mobile Pages (AMP): [AmputatorBot](https://www.amputatorbot.com/)
- Instagram: [InstaFix](https://github.com/Wikidepia/InstaFix) (behind the `ddinstagram` feature)
- Medium: [Scribe](https://sr.ht/~edwardloveall/Scribe/)
- Twitter: [BetterTwitFix](https://github.com/dylanpdx/BetterTwitFix)
- YouTube Shorts: In-process transform (changes `/shorts/` URLs to regular video player)

### Running

- Copy `.env.sample` as `.env` and edit with the necessary details
- Use `cargo run` to start the bot
