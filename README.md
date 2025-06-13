# linkleaner [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

Telegram bot to replace social media links with their improved preview variants. The mapping of supported link types to what services are used to "fix" these links is given below.

| Link type | Fix mechanism |
|-----------|---------------|
| [Accelerated Mobile Pages (AMP)](https://amp.dev) | [AmputatorBot](https://www.amputatorbot.com/) |
| [Instagram](https://instagram.com) | [kkinstagram](https://kkinstagram.com/) |
| [Medium](https://medium.com) | [LibMedium](https://git.batsense.net/realaravinth/libmedium) |
| [Reddit](https://reddit.com) | [FxReddit](https://github.com/MinnDevelopment/fxreddit) |
| [TikTok](https://tiktok.com) | [fxTikTok](https://github.com/okdargy/fxtiktok) |
| [Twitter](https://twitter.com) / [X](https://x.com) | [TweetFix](https://github.com/FixTweet/FixTweet) |
| [YouTube Shorts](https://www.youtube.com/shorts) | Rewrite URL to normal YouTube player |


## Development

There are no external dependencies other than a nightly Rust compiler.

I personally use [Nix](https://nixos.org/) to manage my development environment, but the repository
also contains a `rust-toolchain.toml` for use with [Rustup](https://rustup.rs) containing the exact
version of Rust nightly the project is tested with.

## Deployment

### fly.io (recommended)

My personal copy of linkleaner is hosted on [fly.io](https://fly.io) and the release workflow is designed to facilitate that.

To get set up, refer to the environment variables defined in the `.env.sample` file and set them on fly.io itself with the real values.

The bundled `fly.toml` is not a template and refers to my own deployment, make sure to swap
out `app` for your own unique name. You can keep the image the same if you wish to keep in
sync with upstream releases, or push your own Docker image.

### Docker

The release CI pushes the `ghcr.io/msfjarvis/linkleaner:latest` image to GitHub Container Registry
which can be pulled periodically for the latest code. You can run this image directly with
the environment variables from `.env.sample` since it has a configured entrypoint.
