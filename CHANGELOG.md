# Changelog

All notable changes to this project will be documented in this file.

## [2.8.0] - 2025-06-13

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to v1.44.2
- *(deps)* Update rust crate rand to v0.9.1
- *(deps)* Update rust crate tokio to v1.45.0
- *(deps)* Update rust crate tokio to v1.45.1
- *(deps)* Update rust crate reqwest to v0.12.16
- *(deps)* Update rust crate reqwest to v0.12.18
- *(deps)* Update rust crate reqwest to v0.12.19
- Drop removed input override
- Remove nil from packages

### 🚜 Refactor

- Use `autotrait`
- *(instagram)* Use kkinstagram.com

### ⚙️ Miscellaneous Tasks

- *(nix)* Use lix-project fork of flake-compat

## [2.7.4] - 2025-03-22

### 🐛 Bug Fixes

- *(ci)* Allow release workflow to create releases

## [2.7.3] - 2025-03-22

### 🐛 Bug Fixes

- *(deps)* Update rust crate reqwest to v0.12.13 (#63)
- *(deps)* Update rust crate reqwest to v0.12.14
- *(deps)* Update rust crate tokio to v1.44.1
- *(deps)* Update rust crate reqwest to v0.12.15

### 🚜 Refactor

- *(build)* Only deploy images to ghcr.io

### ⚙️ Miscellaneous Tasks

- *(ci)* Add missing `GH_TOKEN`

## [2.7.2] - 2025-03-10

### 🐛 Bug Fixes

- *(deps)* Update rust crate serde_json to v1.0.140
- Upgrade to Rust nightly-2025-03-04
- *(deps)* Update rust crate tokio to v1.44.0
- *(deps)* Update serde monorepo to v1.0.219
- *(rust)* Upgrade to 2024 edition

### 🚜 Refactor

- Commonize URL replacement logic
- Inline `BotExt#send_preview`
- Reduce calls to `get_urls_from_message`

### ⚙️ Miscellaneous Tasks

- *(nix)* Add `git-cliff` to devshell
- Beef up README
- *(ci)* Automatically create GitHub releases

## [2.7.1] - 2025-03-01

### 🐛 Bug Fixes

- *(tiktok)* Improve matcher compatibility

## [2.7.0] - 2025-03-01

### 🚀 Features

- Introduce TikTok support

### 🐛 Bug Fixes

- *(deps)* Regenerate lockfile
- *(cargo)* Downgrade to 2021 edition
- *(deps)* Update rust crate serde_json to v1.0.137
- *(deps)* Update to rand 0.9.0
- *(deps)* Update rust crate serde_json to v1.0.138
- *(deps)* Update rust crate serde_json to v1.0.139
- *(deps)* Update serde monorepo to v1.0.218

### ⚙️ Miscellaneous Tasks

- Drop useless variable shadowing
- Add missing Reddit section to README
- Fix README fmt

## [2.6.4] - 2025-01-14

### 🐛 Bug Fixes

- *(twitter)* Add support for video links

## [2.6.3] - 2025-01-14

### 🐛 Bug Fixes

- *(twitter)* Add support for photo URLs
- *(build)* Use nix2container to generate images
- *(ci)* Use nix2container to publish images

## [2.6.2] - 2024-12-30

### 🚀 Features

- Wire up inputs for a persistent volume
- Ensure all routers also match on URLs with trailing slashes
- *(tests)* Validate trailing slash handling

### 🐛 Bug Fixes

- Upgrade to Rust 2024 edition
- *(ci)* Cache Nix Store
- *(ci)* Pull flyctl from Nix
- *(ci)* Setup concurrency
- Panic if `add_route` gets a path with a trailing slash

### 🚜 Refactor

- *(reddit)* Cleanup conditional checks a bit
- Pull out URL matcher test util
- *(instagram)* Migrate from regex to matchit
- *(twitter)* Migrate from regex to matchit
- *(youtube)* Migrate from regex to matchit
- Break down utils module
- Inline all test URLs

### ⚙️ Miscellaneous Tasks

- Rename message.rs -> bot_ext.rs

## [2.6.1] - 2024-12-20

### 🐛 Bug Fixes

- *(reddit)* Also fix www.reddit.com

## [2.6.0] - 2024-12-19

### 🚀 Features

- *(reddit)* Use a more robust matcher

### 🐛 Bug Fixes

- *(deps)* Update serde monorepo to v1.0.216
- Upgrade to 2024-12-15 Rust nightly
- *(reddit)* Also capture `redd.it` domain
- *(reddit)* Make preview creation domain independent

### 🚜 Refactor

- Use a direct dependency on rust-url

### ⚙️ Miscellaneous Tasks

- Apply Renovate config migration
- Remove VSCode configuration
- *(ci)* Remove Garnix cache
- Reformat all files with nixfmt-rfc-style
- Add reformat to git-blame-ignore-revs
- Align test module names

## [2.5.0] - 2024-12-06

### 🚀 Features

- Add a button to view Twitter links on Nitter

### 🐛 Bug Fixes

- *(deps)* Pin `futures` and `reqwest`
- *(deps)* Update teloxide
- *(deps)* Update serde monorepo to v1.0.215
- *(deps)* Update rust crate serde_json to v1.0.133
- *(deps)* Update rust crate tracing to v0.1.41
- *(deps)* Update tokio-tracing monorepo
- *(deps)* Update rust crate tokio to v1.42.0

### ⚙️ Miscellaneous Tasks

- Adjust `flake.lock` maintenance schedule

## [2.4.2] - 2024-11-07

### 🐛 Bug Fixes

- *(flakes)* Expose `skopeo` from Nixpkgs
- *(ci)* Run Skopeo from linkleaner flake
- Allow checking fixer status without admin rights
- *(deps)* Update rust crate tokio to v1.41.1
- Prevent bot from fixing its own messages

### 🚜 Refactor

- *(commands)* Pull out filter flipping code
- Remove `where` clauses for Fn types

## [2.4.1] - 2024-10-29

### 🐛 Bug Fixes

- *(ci)* Rework publishing to deploy image manually

## [2.4.0] - 2024-10-29

### 🚀 Features

- Add `BotExt#send_preview`
- Port eligible fixers to `LinkPreviewOptions`

### 🐛 Bug Fixes

- *(flakes)* Add `skopeo`
- *(deps)* Update rust crate futures to v0.3.31
- *(deps)* Update rust crate serde_json to v1.0.129
- *(deps)* Update rust crate serde_json to v1.0.131
- *(deps)* Update rust crate serde_json to v1.0.132
- *(deps)* Update serde monorepo to v1.0.211
- *(deps)* Update rust crate tokio to v1.41.0
- *(deps)* Update serde monorepo to v1.0.213
- *(deps)* Update rust crate regex to v1.11.1
- *(deps)* Update rust crate console-subscriber to v0.4.1
- *(deps)* Update serde monorepo to v1.0.214
- *(deps)* Update rust crate reqwest to v0.12.9
- *(twitter)* Include mobile sites as well

### ⚙️ Miscellaneous Tasks

- *(nix)* Add `bacon` to devshell

## [2.3.13] - 2024-10-03

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to v1.39.2
- *(deps)* Update rust crate serde_json to v1.0.121
- *(deps)* Update rust crate console-subscriber to 0.4.0
- *(deps)* Update rust crate serde_json to v1.0.122
- *(deps)* Update rust crate regex to v1.10.6
- *(deps)* Update serde monorepo to v1.0.205
- *(deps)* Update serde monorepo to v1.0.206
- *(deps)* Update rust crate serde_json to v1.0.124
- *(deps)* Update serde monorepo to v1.0.207
- *(deps)* Update rust crate serde_json to v1.0.125
- *(deps)* Update serde monorepo to v1.0.208
- *(deps)* Update teloxide to 0.13.0
- Resolve `clippy::large_futures` lint
- *(deps)* Update rust crate tokio to v1.39.3
- *(deps)* Update rust crate reqwest to v0.12.6
- *(utils)* Ensure substring matches are ignored for domain test
- *(deps)* Update rust crate serde_json to v1.0.127
- *(deps)* Update serde monorepo to v1.0.209
- *(flakes)* Remove obsoleted input override
- *(deps)* Update rust crate regex to v1.11.0
- *(deps)* Update rust crate reqwest to v0.12.8
- *(dice)* Prevent false-positive matches
- *(reddit)* Skip query parameter scrubbing

### ⚙️ Miscellaneous Tasks

- *(nix)* Remove non-existent flake input
- Migrate CI to GHA
- Add `.envrc`

## [2.3.12] - 2024-07-25

### 🐛 Bug Fixes

- *(ci)* Excise DeterminateSystems

## [2.3.11] - 2024-07-25

### 🐛 Bug Fixes

- *(dice)* Inform user when invalid roll format is used
- *(deps)* Update rust crate tokio to v1.39.1
- *(rust)* Upgrade to current nightly

### 🚜 Refactor

- Replace `once_cell` with `std::cell:LazyCell`

## [2.3.10] - 2024-07-19

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to v1.38.1
- Reduce log levels for utils
- Register dice module as a branch instead of endpoint
- *(deamp)* Add some tests for `deamp::is_amp`
- *(deamp)* Detect amp in query params

## [2.3.9] - 2024-07-14

### 🐛 Bug Fixes

- *(deps)* Update serde monorepo to v1.0.204
- Improve reply logic

### ⚙️ Miscellaneous Tasks

- Fmt

## [2.3.8] - 2024-07-05

### 🚀 Features

- Initial Reddit support

### 🐛 Bug Fixes

- *(deps)* Update rust crate serde_json to v1.0.120 (#44)
- *(instagram)* Expand supported URL formats

### 🚜 Refactor

- Dedupe "not authored" message

### ⚙️ Miscellaneous Tasks

- Fmt

## [2.3.7] - 2024-06-29

### 🐛 Bug Fixes

- Reply silently when replacing messages

## [2.3.6] - 2024-06-26

### 🐛 Bug Fixes

- *(flakes)* Remove useless `pname` override
- *(deps)* Update rust crate serde_json to v1.0.118
- *(message)* Always attempt replying

### ⚙️ Miscellaneous Tasks

- Add `garnix.yaml`
- Temporarily remove rust-analyzer

## [2.3.5] - 2024-06-22

### 🐛 Bug Fixes

- *(message)* Add missing `parse_mode`

## [2.3.4] - 2024-06-22

### 🚜 Refactor

- Rework BotExt API and callers

## [2.3.3] - 2024-06-20

### 🐛 Bug Fixes

- *(deps)* Update rust crate reqwest to v0.12.5

## [2.3.2] - 2024-06-15

### 🚀 Features

- *(rust)* Upgrade to 2024-05-25 nightly

### 🐛 Bug Fixes

- Fmt
- *(deps)* Update serde monorepo to v1.0.203 (#42)
- Remove useless feature flag
- *(deps)* Update rust crate tokio to v1.38.0
- *(deps)* Update rust crate regex to v1.10.5
- *(deps)* Update rust crate console-subscriber to 0.3.0
- Disable Instagram fixer by default

### 💼 Other

- Add rust-analyzer to packages

### 🚜 Refactor

- Dedupe match check

## [2.3.1] - 2024-05-20

### 🐛 Bug Fixes

- *(dice)* Combine all rolls into a single total

## [2.3.0] - 2024-05-20

### 🚀 Features

- Add a dynamic die command

### 🐛 Bug Fixes

- Short-circuit skip token check

### 🚜 Refactor

- Remove handler nesting

## [2.2.0] - 2024-05-20

### 🚀 Features

- *(commands)* Add a command to roll dice

### 🐛 Bug Fixes

- *(deps)* Update rust crate serde_json to v1.0.117
- *(deps)* Update serde monorepo to v1.0.201
- *(commands)* Use `try_reply` extension
- *(message)* Send typing action in replies
- *(deps)* Update serde monorepo to v1.0.202

### 💼 Other

- Use my nixpkgs fork
- Rollback Crane

### 🚜 Refactor

- Use typed URLs
- Match on domains rather than regex

### ⚙️ Miscellaneous Tasks

- Fmt

## [2.1.0] - 2024-04-30

### 🚀 Features

- *(ci)* Publish Docker images to ghcr.io
- *(medium)* Switch to LibMedium

### 🐛 Bug Fixes

- Remove unneeded step from deployment

### 💼 Other

- Add a separate docker build for GHCR

## [2.0.0] - 2024-04-29

### 🐛 Bug Fixes

- Insert default values
- Use entry API for `update_fixer_state`
- Special case PMs
- Restore confirmation for fixer state toggle
- Prevent moving `Message` in `BotExt`
- Restore argument-less variant of commands
- Simplify private chat short-circuit
- *(flakes)* Improve parallelism

### 💼 Other

- Re-enable cargo-audit

### 🚜 Refactor

- Track fixer state per-chat rather than globally
- Simplify fixer state handling
- Add a typealias for async errors
- Commonize authorization check
- Commonize fixer state update

### ⚙️ Miscellaneous Tasks

- Address Clippy lints

## [1.9.2] - 2024-03-30

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.37.0

## [1.9.1] - 2024-03-26

### 🐛 Bug Fixes

- *(deps)* Update rust crate regex to 1.10.4
- *(ci)* Add dotfiles trigger to release workflow

## [1.9.0] - 2024-03-18

### 🚀 Features

- Set up Sentry integration

### 🐛 Bug Fixes

- *(deps)* Update rust crate regex to 1.10.3
- *(deps)* Update rust crate tokio to 1.36.0
- *(deps)* Upgrade to teloxide unstable

### 💼 Other

- Disable cargo-audit

### 🚜 Refactor

- De-dupe message sending

## [1.8.3] - 2024-01-19

### 🚀 Features

- *(rust)* Upgrade to latest nightly

### 💼 Other

- Add cc for linker

### ⚙️ Miscellaneous Tasks

- Fix flake inputs

## [1.8.2] - 2023-12-23

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.34.0
- *(deps)* Update rust crate tracing-subscriber to 0.3.18
- *(deps)* Update rust crate once_cell to 1.19.0
- *(deps)* Update rust crate tokio to 1.35.0
- *(deps)* Update rust crate tokio to 1.35.1

### ⚙️ Miscellaneous Tasks

- Fix Garnix badge
- Run `flake.lock` maintenance earlier

## [1.8.1] - 2023-10-23

### 🚀 Features

- *(rust)* Upgrade to latest nightly

### 🐛 Bug Fixes

- *(deps)* Update rust crate regex to 1.9.4
- *(deps)* Update rust crate regex to 1.9.5
- *(deps)* Update rust crate console-subscriber to 0.2.0
- *(deps)* Update rust crate regex to 1.9.6
- *(deps)* Update rust crate tokio to 1.33.0
- *(deps)* Update rust crate regex to 1.10.0
- *(deps)* Update rust crate tracing to 0.1.39
- *(deps)* Update rust crate regex to 1.10.1
- *(deps)* Update rust crate regex to 1.10.2
- *(deps)* Update rust crate tracing to 0.1.40
- *(flakes)* Drop now-removed crane inputs

### ⚙️ Miscellaneous Tasks

- *(twitter)* Add tests for root match group
- Update Garnix badges
- Refresh README
- Fix README URLs

## [1.8.0] - 2023-08-17

### 🚀 Features

- *(twitter)* Fuck Musk

### 🐛 Bug Fixes

- *(deps)* Update rust crate regex to 1.9.2
- *(deps)* Update rust crate regex to 1.9.3
- *(deps)* Update rust crate tokio to 1.30.0
- *(deps)* Update rust crate tokio to 1.31.0
- *(deps)* Update rust crate tokio to 1.32.0

## [1.7.2] - 2023-08-01

### 🚀 Features

- Remove ThreadsFix

### 🐛 Bug Fixes

- *(deps)* Update rust crate regex to 1.9.1
- *(rust)* Bump to nightly-2023-07-08
- *(threads)* Disable by default

## [1.7.1] - 2023-07-07

### 🐛 Bug Fixes

- Ensure Threads handler respects its filter logic

## [1.7.0] - 2023-07-06

### 🚀 Features

- Add fixer for threads.net

### 🐛 Bug Fixes

- *(deps)* Update rust crate console-subscriber to 0.1.10
- *(deps)* Update rust crate regex to 1.9.0
- *(utils)* Remove unsound URL cache
- *(utils)* Adjust logging levels

## [1.6.7] - 2023-06-30

### 🚀 Features

- *(flakes)* Adopt nix-systems for flake systems
- *(flakes)* Use numtide/devshell for Flake devShell

### 🐛 Bug Fixes

- *(flakes)* Switch flake-compat to nix-community fork
- *(flakes)* Adjust flake-compat URL
- *(deps)* Update rust crate tokio to 1.29.0
- *(deps)* Update rust crate tokio to 1.29.1
- *(twitter)* Temporarily use BetterTwitFix

### ⚙️ Miscellaneous Tasks

- Reformat TOML files with Taplo

## [1.6.6] - 2023-06-16

### 🚀 Features

- *(rust)* Bump Rust nightly

### 🐛 Bug Fixes

- *(deps)* Update rust crate regex to 1.8.4

## [1.6.5] - 2023-06-05

### 🐛 Bug Fixes

- Fmt

## [1.6.4] - 2023-06-05

### 🐛 Bug Fixes

- *(deps)* Update rust crate once_cell to 1.17.2
- *(deps)* Update rust crate once_cell to 1.18.0
- *(medium)* Enable by default
- *(medium)* Add another URL pattern to tests
- Add custom assertion messages to regex tests

## [1.6.3] - 2023-05-28

### 🚀 Features

- *(medium)* Switch over to `medium.rip`

### 🐛 Bug Fixes

- *(deps)* Update rust crate regex to 1.8.3
- Add flyctl and sync config
- *(deps)* Update rust crate tokio to 1.28.2

## [1.6.2] - 2023-05-25

### 🐛 Bug Fixes

- *(medium)* Fix copy-pasta fails

## [1.6.1] - 2023-05-25

### 🐛 Bug Fixes

- Fmt

## [1.6.0] - 2023-05-25

### 🚀 Features

- Add Medium link replacement via Scribe

### 🐛 Bug Fixes

- *(ci)* Adjust option name
- Add `/start` command

## [1.5.3] - 2023-05-24

### 🚀 Features

- *(nix)* Switch to provisioning toolchains using fenix

### 🐛 Bug Fixes

- *(deps)* Update rust crate console-subscriber to 0.1.9
- *(deps)* Update rust crate tokio to 1.28.1
- *(deps)* Update rust crate regex to 1.8.2

### ⚙️ Miscellaneous Tasks

- *(cargo)* Update dependencies
- *(ci)* Switch to `DeterminateSystems/nix-installer-action`

## [1.5.2] - 2023-04-26

### 🐛 Bug Fixes

- *(flakes)* Set `CARGO_REGISTRIES_CRATES_IO_PROTOCOL` in devShell
- *(deps)* Update rust crate h2 to 0.3.18
- *(deps)* Update rust crate tracing to 0.1.38
- *(deps)* Update rust crate tokio to 1.28.0

## [1.5.1] - 2023-04-22

### 🚀 Features

- *(ci)* Add GHA workflow for updating flake.lock every weekend

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.26.0
- *(deps)* Update rust crate regex to 1.7.2
- *(deps)* Update rust crate dotenvy to 0.15.7
- *(deps)* Update rust crate regex to 1.7.3
- *(deps)* Update rust crate tokio to 1.27.0
- *(deps)* Update rust crate regex to 1.8.0
- *(deps)* Update rust crate regex to 1.8.1
- *(deps)* Update rust crate tracing-subscriber to 0.3.17

### ⚙️ Miscellaneous Tasks

- *(nix)* Bump flake inputs
- *(nix)* Bump to crane 0.12.0
- Remove unused `justfile`

## [1.5.0] - 2023-02-23

### 🚀 Features

- Relax matcher regex to allow leading text

## [1.4.11] - 2023-02-22

### 🚀 Features

- *(ci)* Simplify GHA CI to only handle releases

### 🚜 Refactor

- Introduce rust-toolchain.toml

### ⚙️ Miscellaneous Tasks

- *(nix)* Bump flake inputs
- *(nix)* Bump flake inputs
- *(rust)* Bump Rust nightly
- *(nix)* Bump flake inputs
- Add Built With Garnix badge

## [1.4.10] - 2023-02-16

### 🐛 Bug Fixes

- *(deps)* Update rust crate once_cell to 1.17.1
- *(deps)* Update rust crate teloxide to 0.12.2

## [1.4.9] - 2023-02-11

### 🚜 Refactor

- `SendLinkleanerMessage` -> `TryReplyMessage`

### ⚙️ Miscellaneous Tasks

- Rename container tag
- Update deps

## [1.4.8] - 2023-02-10

### 🚀 Features

- Set up cargo-dist

### 🐛 Bug Fixes

- *(youtube)* Account for underscores in video IDs
- Short-circuit commands in `get_urls_from_messages`

## [1.4.7] - 2023-01-31

### 🚀 Features

- Memoize calls to `get_urls_from_message`

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.25.0

## [1.4.6] - 2023-01-21

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.24.2
- *(deps)* Update rust crate teloxide to 0.12.0

## [1.4.5] - 2023-01-09

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.24.1
- *(deps)* Update rust crate regex to 1.7.1

## [1.4.4] - 2023-01-05

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.23.1
- *(deps)* Update rust crate tokio to 1.24.0
- *(deps)* Update transitive deps

## [1.4.3] - 2023-01-02

### ⚙️ Miscellaneous Tasks

- `cargo fmt`

## [1.4.2] - 2023-01-01

### 🚜 Refactor

- *(amp)* Use the link helper
- Abstract away bot reply logic

## [1.4.1] - 2023-01-01

### 🚜 Refactor

- Optimize `utils::get_urls_from_message`

## [1.4.0] - 2022-12-31

### 🚀 Features

- *(commands)* Add a shorthand for twitchtheater

### 🐛 Bug Fixes

- *(ci)* Remove manual docker push

## [1.3.2] - 2022-12-30

### 🐛 Bug Fixes

- Enable `ddinstagram` feature by default
- *(deps)* Update rust crate once_cell to 1.17.0
- *(youtube)* Support video IDs with hyphens

### 🚜 Refactor

- Rework flake for sequential execution

### ⚙️ Miscellaneous Tasks

- Reformat Nix files with alejandra
- *(flakes)* Add cargo-nextest to devShell

## [1.3.1] - 2022-12-29

### 🐛 Bug Fixes

- *(youtube)* Account for query parameters

## [1.3.0] - 2022-12-28

### 🚀 Features

- Add YouTube support

### ⚙️ Miscellaneous Tasks

- Brush up README

## [1.2.4] - 2022-12-24

### 🚀 Features

- Enable ddinstagram feature

## [1.2.3] - 2022-12-23

### 🚜 Refactor

- *(ci)* Merge deploy and test workflows

### ⚙️ Miscellaneous Tasks

- *(ci)* Rename workflow

## [1.2.2] - 2022-12-23

### 🐛 Bug Fixes

- *(nix)* Add `nil` and fix mixed tabs
- *(ci)* Flyctl?
- *(ci)* Correct flyctl command
- *(logging)* Switch to compact output
- *(utils)* Consolidate logging in get_urls_from_message
- *(utils)* Add logging to `scrub_urls`

### ⚙️ Miscellaneous Tasks

- Remove unused `deny.toml`
- Remove unused Debian packaging

## [1.2.1] - 2022-12-23

### 🚀 Features

- *(nix)* Add `packages.container`
- *(fly)* Init
- *(ci)* Set up automated deploys to fly.io

### ⚙️ Miscellaneous Tasks

- Remove unused testdata
- *(ci)* Build `.#container` target
- Massage IDE configs
- Lower logging level

## [1.2.0] - 2022-12-19

### 🚀 Features

- Scrub query params for Twitter as well
- Put ddinstagram behind a feature flag

### 🚜 Refactor

- *(commands)* Drop provider-specific naming

### ⚙️ Miscellaneous Tasks

- *(nix)* Finish Flakes migration
- Update README

## [1.1.0] - 2022-12-16

### 🚀 Features

- *(ci)* Switch to shared Nix Flakes workflow
- *(twitter)* Use fxtwitter as upstream

### 🚜 Refactor

- *(nix)* Expand default nix flake checks
- Make module name service-independent

### ⚙️ Miscellaneous Tasks

- Upgrade toolchain to 2022-12-15 nightly

## [1.0.0] - 2022-12-15

### 🚀 Features

- Rename to linkleaner
- *(ci)* Enable writing back to Cachix

### 🐛 Bug Fixes

- *(deps)* Update rust crate teloxide to 0.11.3
- *(deps)* Update rust crate tokio to 1.23.0

### ⚙️ Miscellaneous Tasks

- *(nix)* Remove `linkleaner-doc` target
- *(nix)* Bump flake inputs

## [0.20.2] - 2022-11-24

### ⚙️ Miscellaneous Tasks

- Use teloxide helpers for user links

## [0.20.1] - 2022-11-23

### 🐛 Bug Fixes

- *(logging)* Mark `handle_error` with `#[track_caller]`

### ⚙️ Miscellaneous Tasks

- Add `cargo-release` to devshell inputs

## [0.20.0] - 2022-11-22

### 🚀 Features

- *(nix)* Setup `crane`-based compilation
- Directly install the specific nightly

### 🐛 Bug Fixes

- *(deps)* Update rust crate teloxide to 0.11.2
- *(deps)* Update rust crate tokio to 1.22.0
- *(flakes)* Add `rust-src` extension
- Encode text to UTF-16 before extracting entities
- *(release)* Upgrade to latest `cargo-release`

### 🚜 Refactor

- *(ci)* Run checks with `Nix`

### ⚙️ Miscellaneous Tasks

- *(nix)* Update `flake.lock`
- *(rust)* Bump nightly version
- Add Nix result patterns to gitignore
- Add workflow to run `update-flake-lock`
- Update dependencies

## [0.19.3] - 2022-11-07

### 🚀 Features

- Add `ping` command

### 🐛 Bug Fixes

- *(deps)* Update rust crate regex to 1.7.0

## [0.19.2] - 2022-10-31

### 🐛 Bug Fixes

- Clippy
- *(deny)* Adjust for current state
- *(deps)* Update rust crate once_cell to 1.16.0
- *(deps)* Update rust crate teloxide to 0.11.1

### ⚙️ Miscellaneous Tasks

- *(ci)* Split `cargo-deny` to its own job
- Update CI badge

## [0.19.1] - 2022-10-27

### 🚀 Features

- *(ddinstagram)* Scrub URL query params

### 🐛 Bug Fixes

- *(clippy)* Use string interpolation
- *(deps)* Update rust crate dotenvy to 0.15.6

## [0.19.0] - 2022-10-17

### 🚀 Features

- Allow toggling Instagram and Twitter link replacement (#22)
- Remove walls functionality

### 🐛 Bug Fixes

- *(commands)* Fix check for owner commands

### 🚜 Refactor

- Move out command handler

### ⚙️ Miscellaneous Tasks

- Upgrade to latest Rust nightly

## [0.18.0] - 2022-10-07

### 🚀 Features

- *(just)* Use argument capture and set an explicit default

### 🐛 Bug Fixes

- Remove unnecessary `' static` lifetime
- *(deps)* Update rust crate tokio to 1.21.1
- *(deps)* Update rust crate dotenvy to 0.15.5
- *(deps)* Update rust crate once_cell to 1.15.0
- *(deps)* Update rust crate imagesize to 0.10.1
- *(deps)* Update rust crate tokio to 1.21.2
- *(deps)* Update rust crate tracing to 0.1.37
- *(deps)* Update rust crate tracing-subscriber to 0.3.16

## [0.17.3] - 2022-09-09

### 🚀 Features

- Allow skipping link replacement with `#skip`

### 🐛 Bug Fixes

- *(deps)* Update rust crate dotenvy to 0.15.3
- *(deps)* Bump tokio, once_cell and others
- *(deps)* Update rust crate once_cell to 1.14.0
- *(deps)* Update rust crate tokio to 1.21.0
- *(deps)* Update rust crate console-subscriber to 0.1.8

### ⚙️ Miscellaneous Tasks

- Fix formatting

## [0.17.2] - 2022-09-01

### 🚀 Features

- *(logging)* Implement a custom logger for Teloxide
- *(teloxide)* Drop pending updates on bot startup

### 🐛 Bug Fixes

- *(deps)* Switch from `dotenv` to `dotenvy`
- *(deny)* Remove unmatched vulnerability

## [0.17.1] - 2022-09-01

### 🚀 Features

- Add `reqwest` and `serde` as a direct dependency
- *(amp)* Add module to de-amp links

### 🐛 Bug Fixes

- Resolve clippy warnings
- *(amputator)* Fix dptree crash from unhelpful clippy lint

### ⚙️ Miscellaneous Tasks

- Get rid of grouped imports
- *(deny)* Allow wildcard versions
- *(just)* Add `systemd` tasks

## [0.16.5] - 2022-08-23

### 🐛 Bug Fixes

- *(logging)* Directly print logging init failure

## [0.16.4] - 2022-08-23

### 🚀 Features

- *(vxtwitter)* Use `c.vxtwitter.com` instead

## [0.16.3] - 2022-08-19

### 🐛 Bug Fixes

- *(deps)* Update rust crate console-subscriber to 0.1.7
- *(deps)* Update rust crate once_cell to 1.13.1

### ⚙️ Miscellaneous Tasks

- Bump required Rust version
- *(ci)* Revert to using the shared workflow
- Set channel to `stable`
- Bump dependencies
- *(rust)* Bump nightly

## [0.16.2] - 2022-08-06

### 🚀 Features

- *(ddinstagram)* Support `/tv` links

### 🐛 Bug Fixes

- *(deps)* Update rust crate teloxide to 0.10.1
- *(deps)* Remove explicit `lazy_static` dependency
- *(deps)* Update rust crate fastrand to 1.8.0
- *(deps)* Configure Renovate to pin GHA digests
- *(ci)* Sync GHA workflow with `shared-workflows`
- *(deps)* Update rust crate tokio to 1.20.1
- *(deps)* Update rust crate tracing to 0.1.36
- *(deny)* Add exclusion for chrono

## [0.16.1] - 2022-07-22

### 🐛 Bug Fixes

- *(logging)* Use journald layer init shorthand

## [0.16.0] - 2022-07-21

### 🚀 Features

- *(package)* Setup systemd support for Debian packaging

### 🐛 Bug Fixes

- *(deps)* Update rust crate teloxide to 0.10.0
- *(deny)* Remove unnecessary skip

### 🚜 Refactor

- Fix clippy warnings

## [0.15.0] - 2022-07-21

### 🐛 Bug Fixes

- *(release)* Build Debian archive with `--cfg tokio_unstable`
- *(deps)* Update rust crate tracing-subscriber to 0.3.15
- *(logging)* Correctly support `tokio-console`

## [0.14.0] - 2022-07-20

### 🚀 Features

- *(deps)* Remove teloxide-core dependency
- *(deb)* Enable console feature for Debian packaging

### 🐛 Bug Fixes

- *(release)* Don't start a dev version
- *(ddinstagram)* Fix rustfmt crash
- *(logging)* Prevent crash when used with tokio-console

### ⚙️ Miscellaneous Tasks

- *(rust)* Bump nightly

## [0.13.1] - 2022-07-18

### 🐛 Bug Fixes

- *(ddinstagram)* Support reels

## [0.13.0] - 2022-07-18

### 🚀 Features

- *(ddinstagram)* Add handler for instagram messages

### 🐛 Bug Fixes

- *(deps)* Update rust crate imagesize to 0.10.0

### 🚜 Refactor

- *(dispatcher)* Make message filter extensible

### ⚙️ Miscellaneous Tasks

- *(just)* Add `just check`

## [0.12.2] - 2022-07-14

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.20.0
- *(vxtwitter)* Swallow message deletion error
- *(release)* Do not attempt to publish to crates.io

### ⚙️ Miscellaneous Tasks

- *(just)* Add fmt
- *(vscode)* Add suggested extensions
- *(just)* Allow re-installing current version
- *(just)* Add clippy task

## [0.12.1] - 2022-07-11

### 🚀 Features

- *(just)* Add a console job
- Take into account mobile.twitter links
- *(vxtwitter)* Add test for regex

### 🐛 Bug Fixes

- Report error and exit when tracing fails to init
- Format justfile with `justfile --fmt`

### 🚜 Refactor

- Extract logging to its own module
- Extract vxtwitter to its own module
- Extract walls to its own module
- *(walls)* Tweak logging format
- *(vxtwitter)* Use `#![feature(let_chains)]` to simplify handling
- *(vxtwitter)* Use capture groups for replacements

### ⚙️ Miscellaneous Tasks

- Convert tracing macros to imports
- Rename `to_relative_path` to `basename`
- *(ci)* Adjust CI to only test nightly
- *(just)* Add test job

## [0.12.0] - 2022-07-07

### 🚀 Features

- Add justfile to simplify workflows
- Configure tracing to only log our own traces
- Bump version

### 🐛 Bug Fixes

- Change feature name to match code guards
- Remove console feature from default deb packaging
- Only index files
- Add error on empty index

### ⚙️ Miscellaneous Tasks

- Increase logging level to DEBUG

## [0.11.1] - 2022-07-07

### 🚀 Features

- Bump version

### 🐛 Bug Fixes

- Set features for cargo-deb
- Actually call `configure_tracing` 3Head

## [0.11.0] - 2022-07-07

### 🚀 Features

- Reply to original message when replacing a reply
- Route logging through tracing
- Release 0.11.0

### 🐛 Bug Fixes

- Remove unused dependency

### 💼 Other

- Bump dependencies

## [0.10.0] - 2022-07-06

### 🚀 Features

- Sync Nix config with dotfiles
- Implement twitter -> vxtwitter auto-replace
- *(release)* Bump version

### 🐛 Bug Fixes

- *(deps)* Update rust crate teloxide to 0.8.1
- *(deps)* Update rust crate teloxide to 0.8.2
- *(deps)* Update rust crate console-subscriber to 0.1.5
- *(deps)* Update rust crate tokio to 1.18.1
- *(deps)* Update rust crate log to 0.4.17
- *(deps)* Update rust crate tokio to 1.18.2 (#16)
- *(deps)* Update rust crate console-subscriber to 0.1.6
- *(deps)* Update rust crate teloxide to 0.9.1
- *(deps)* Update transitive dependencies
- *(deps)* Update rust crate tokio to 1.19.0
- *(deps)* Update rust crate tokio to 1.19.1
- *(deps)* Update rust crate tokio to 1.19.2
- *(deps)* Update rust crate teloxide to 0.9.2
- Switch from lazy_static to once_cell
- *(cargo)* Add explicit teloxide-core dependency to fix build
- *(deps)* Update rust crate teloxide-core to 0.6.3
- Fix `clippy::explicit-auto-deref` warning
- *(deps)* Update rust crate once_cell to 1.12.1
- *(deps)* Update rust crate once_cell to 1.13.0
- *(vxtwitter)* Match regex only at start and enforce valid URLs

### 💼 Other

- Upgrade dependencies
- Fix `clippy::format_push_string` lint
- Use GNU target triple on Linux
- Set msrv to latest stable
- Update dependencies
- Switch to my presets
- Add rust-toolchain file

### ⚙️ Miscellaneous Tasks

- Reformat imports

## [0.9.0] - 2022-04-18

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.17.0
- *(deps)* Update rust crate teloxide to 0.7.1 (#12)
- *(deps)* Update rust crate log to 0.4.15
- *(deps)* Update rust crate log to 0.4.16
- *(deps)* Update rust crate teloxide to 0.7.2 (#13)
- *(deps)* Update rust crate teloxide to 0.7.3 (#14)
- *(deps)* Update rust crate console-subscriber to 0.1.4
- *(deps)* Update rust crate teloxide to 0.8.0 (#15)

### 💼 Other

- Bump console-subscriber
- Update shell config
- Remove config
- Fix `clippy::needless-pass-by-value` lint
- Silence `clippy::cast_possible_truncation` lint
- Update dependencies
- Bump version

### ⚙️ Miscellaneous Tasks

- Fix clippy 1.61 lints

## [0.8.0] - 2022-02-15

### 🐛 Bug Fixes

- *(deps)* Update rust crate console-subscriber to 0.1.2

### 💼 Other

- Update dependencies for pure shell
- Add dependabot configuration
- Update workflows
- *(deps)* Bump tokio from 1.11.0 to 1.12.0
- Update deps
- Fix clippy lints
- Switch to shared checks workflow
- Bump to 2021 edition
- Update dependencies
- *(deps)* Bump tokio from 1.12.0 to 1.13.0 (#4)
- Update dependencies
- *(deps)* Bump tokio from 1.13.0 to 1.14.0 (#5)
- Rename branch to main
- *(deps)* Bump tokio from 1.14.0 to 1.15.0 (#6)
- *(deps)* Bump fastrand from 1.5.0 to 1.6.0 (#7)
- Fix `clippy::pedantic` lints
- Add console-subscriber dependency
- Enable tokio_unstable feature by default
- Move crt-static flag to config.toml
- Make console-subscriber an optional dependency
- Sync shell config with dotfiles
- Run cargo diet
- Bump console-subscriber
- Upgrade dependencies
- Fix `clippy::pedantic` lints
- Bump thread_local and which
- *(deps)* Bump tokio from 1.15.0 to 1.16.1 (#8)
- Switch from Dependabot to Renovate
- Update dependencies
- Init
- Bump dependencies
- Bump version

## [0.7.0] - 2021-09-15

### 💼 Other

- Use rustls in place of openssl
- Bump sled, teloxide and tokio
- Bump version

## [0.6.0] - 2021-07-31

### 💼 Other

- Cleanup config
- Bump tokio
- Bump teloxide-core
- Update dependencies
- Update all dependencies
- Rename to shell.nix and remove cargo
- Upgrade dependencies
- Capitalize messages
- Use relative paths
- Bump version

## [0.5.2] - 2021-05-28

### 💼 Other

- Support pure nix shells
- Bump deps
- Upgrade dependencies
- Bump version

## [0.5.1] - 2021-04-28

### 💼 Other

- Bump version

## [0.5.0] - 2021-04-28

### 💼 Other

- Reduce memory use in tokenized search
- Add failing case test for tokenized search
- Bump libc
- Handle multiple keywords in search term
- Replace one character str with a char
- Bump version

## [0.4.0] - 2021-04-26

### 💼 Other

- Bump version

## [0.3.0] - 2021-04-25

### 💼 Other

- Bump version

## [0.2.2] - 2021-04-25

### 💼 Other

- Bump version

## [0.2.1] - 2021-04-25

### 💼 Other

- Patch teloxide-core to version with ChatAction fix
- Bump version

## [0.2.0] - 2021-04-25

### 💼 Other

- Restrict visibility
- Add some basic tests
- Add unit testing workflow
- Add status badge
- Sort
- Setup caching
- Update setup-rust-action
- Add security audit workflow
- Sync lockfile
- Bump deps
- Bump deps
- Cheap tricks for faster binaries
- Bump deps
- Add nix-shell config
- Add support for pure nix-shells
- Bump deps
- Regenerate lockfile
- Bump deps
- Update deps
- Add cargo-deb metadata
- Drop LLVM/Rust dependencies
- Rewrite CI pipeline
- Update rand
- Start breaking apart things
- Remove unused FILE_COUNT field
- Add bot name
- Disable clippy check in PRs
- Fix security audit workflow location
- Update dependencies and refactor command handling
- Remove unused import
- Bump version to v0.2.0

<!-- generated by git-cliff -->
