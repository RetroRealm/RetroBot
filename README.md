# RetroBot

Official Discord bot for [RetroRealm](https://github.com/RetroRealm). Exposes [Playmatch](https://github.com/RetroRealm/playmatch) as Discord slash commands so users can identify ROMs, browse matched companies and platforms, and submit metadata suggestions without leaving Discord.

Every response is rendered with Discord's Components V2 (top-level `Container`, `TextDisplay`, `Separator`, `ActionRow`). No classic embeds.

## Features

### Supported

- [x] `/list companies` and `/list platforms` with paginated tables
- [x] `/get game <hashes|name+size>` for a full Playmatch metadata card (matched platform, company, ROM files, DAT file, provider mappings)
- [x] `/suggest game|company|platform` with owner approval buttons and an outcome DM back to the submitter
- [x] `/match game|company|platform` for manual metadata matching, restricted to bot owners and trusted roles
- [x] `/toggle_update` role toggle for the RetroRealm guild
- [x] `/ping` and a custom `/help`
- [x] Components V2 flag (`IS_COMPONENTS_V2`) on every outgoing message

### Planned

- [ ] Pin serenity and poise back to released crates.io versions once a Components V2 release ships
- [ ] Expose further Playmatch endpoints as commands as they stabilise

## Getting Started

### Prerequisites

1. Rust 1.95+ from [here](https://www.rust-lang.org/tools/install)
2. A Discord bot token. Create an application at the [Discord Developer Portal](https://discord.com/developers/applications), enable a Bot user, and use the `applications.commands` scope when inviting it
3. Access to a Playmatch instance. The public one at [playmatch.retrorealm.dev](https://playmatch.retrorealm.dev/swagger-ui/) works, or self-host from [RetroRealm/playmatch](https://github.com/RetroRealm/playmatch)

### Development

1. Clone the repository
2. Create a `.env` in the repository root with the variables below
3. Run `cargo run` (debug) or `cargo run --release`

#### Environment variables

| Variable | Required | Purpose |
|---|---|---|
| `DISCORD_TOKEN` | yes | Bot token from the Discord Developer Portal |
| `PLAYMATCH_API_AUTH` | yes | Bearer token for the Playmatch API |
| `PLAYMATCH_API_URL` | no | Playmatch base URL, defaults to `https://playmatch.retrorealm.dev` |
| `REDIS_URL` | yes | Redis connection string (e.g. `redis://localhost:6379`), used to persist posted suggestion cards across restarts |
| `DISCORD_STATUS` | no | Activity kind (`playing`, `listening`, `watching`, `competing`) |
| `DISCORD_STATUS_NAME` | no | Activity text shown next to the kind |
| `DISCORD_RETROREALM_SERVER_ID` | for guild commands | Guild id used when registering guild-scoped commands |
| `DISCORD_RETROREALM_STAFF_ROLE_ID` | no | Role id that bypasses trusted-role checks |
| `DISCORD_TRUSTED_ROLE_IDS` | no | Comma-separated role ids allowed to run `/match` |
| `DISCORD_RETROREALM_UPDATE_ROLE_ID` | no | Role id toggled by `/toggle_update` |
| `DISCORD_RETROREALM_SUGGESTION_CHANNEL_ID` | no | Channel id where suggestion review cards are posted |

## Deployment

Docker images are available [here](https://github.com/RetroRealm/RetroBot/pkgs/container/retrobot) at `ghcr.io/retrorealm/retrobot`.

## Built With

* [Rust](https://www.rust-lang.org/) - the programming language used
* [tokio](https://tokio.rs/) - async runtime
* [serenity](https://github.com/serenity-rs/serenity) - Discord API client (`next` branch, for Components V2)
* [poise](https://github.com/serenity-rs/poise) - command framework on top of serenity (`serenity-next` branch)
* [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
* [progenitor](https://github.com/oxidecomputer/progenitor) - generator for the in-tree `playmatch_client` crate

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of
conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see
the [tags on this repository](https://github.com/RetroRealm/RetroBot/tags).

## Authors

* **DevYukine** - *Initial work* - [DevYukine](https://github.com/DevYukine)

See also the list of [contributors](https://github.com/RetroRealm/RetroBot/contributors) who participated in this
project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* [Playmatch](https://github.com/RetroRealm/playmatch) - the backing API this bot is built around
* [serenity](https://github.com/serenity-rs/serenity) / [poise](https://github.com/serenity-rs/poise) - the Discord framework
