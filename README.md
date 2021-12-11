# Minigames

Minigames for Discord servers. So far does nothing. Will have more info later.

## Development Environment Setup

### Server and Bot

The server uses [Rust][] (1.56) and [PostgreSQL][] (14). The bot uses [Deno][] (1.16).
Install all of those however you like.

Once installed, use `cargo` to further install [sqlx-cli][] (with at least the `postgres` feature).

You'll also be needing an ES256 key, which is easiest generated with [OpenSSL][], so maybe install that too.

1.  On the [Discord Developer Portal][], Create an application, and then a bot within that application.
2.  Copy `bot/.env.example` to `bot/.env` and put the appropriate values in.
3.  Copy `server/.env.example` to `server/.env` and put the appropriate values in.
4.  Use OpenSSL to generate the RS256 key for signing JWTs: 
    ```sh
    openssl genrsa -out jwt.pem
    ```
5.  Create the database with `sqlx database create`.
6.  Migrate the database with `sqlx migrate run`.
7.  Add the bot to the Discord server you wish to add it to by visiting the link output by `bot/scripts/add`.

[Node.js]: https://nodejs.org/en/
[Deno]: https://deno.land/
[Rust]: http://rust-lang.org/
[PostgreSQL]: https://www.postgresql.org/
[sqlx-cli]: https://crates.io/crates/sqlx-cli
[OpenSSL]: https://www.openssl.org/
[Discord Developer Portal]: https://discord.com/developers/

Once all setup steps have been completed, you can run the app. To have it fully working requires that
PostgreSQL is running already, then:
1.  *If migrations have changed*, before starting the server run `sqlx migrate run`.
2.  Run the server with `cargo run` in the `server` directory.
3.  Run the bot with `bot/scripts/bot`.

### Games

Games are developed independently of the server, and can really be built in any way you choose, so long
as it can interact with the server via HTTP.

For now, games have been developed using [Node.js][] (17), so ensure that is installed before attempting
to develop the games.

From there, follow the directions in each game's individual README to get them set up and running:
*   [Tic-tac-toe](./games/tictactoe/README.md)

For more information on how to build a game and connect it with a running server, see the [Guide][].

[Guide]: GUIDE.md

## Contributing

Pull Requests and Issues are always welcome. Do note that this project is in very early stages
so things will be moving quickly. You may want to wait until it settles down a bit. If you want
to get involved though, do reach out and a roadmap can be set out.

Do keep to a consistent style, as enforced by CI. The projects within this repository are set up
with standard linting and formatting practices for their respective languages.

Pull requests will not be accepted until CI passes. For each type of project, these are the commands
you need to ensure succeed:

```sh
# Rust
cargo clippy
cargo fmt
cargo check
cargo sqlx prepare

# Deno
deno lint
deno fmt

# Node
npm run lint
npm run stylelint
npm run fmt
npm run build
```
