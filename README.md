# Minigames

[Roadmap][] | [Guide][] | [Manual][] | Install (Unavailable)

[Roadmap]: https://github.com/users/foxfriends/projects/2
[Guide]: ./GUIDE.md
[Manual]: ./USAGE.md

Minigames and leaderboard for Discord servers. Challenge your friends to games, and determine who is
the best (mini)gamer once and for all.

## Development Environment Setup

### Preparation

Before running the project by any method, the following setup steps must be completed:

1.  On the [Discord Developer Portal][], Create an application, and then a bot within that application.
2.  Set up an OAuth2 redirect for the `/play` route of where you will be running the server (likely `http://localhost:8000/play`)
3.  Copy `bot/.env.example` to `bot/.env` and put the appropriate values in.
4.  Copy `server/.env.example` to `server/.env` and put the appropriate values in.
5.  Use [OpenSSL][] to generate the RS256 key for signing JWTs:
    ```sh
    openssl genrsa -out server/jwt.pem
    ```
6.  Add the bot to the Discord server you wish to add it to by visiting the link that is output by `bot/scripts/add`.

[OpenSSL]: https://www.openssl.org/
[Discord Developer Portal]: https://discord.com/developers/

### Docker

For those who are *only planning on building games* (e.g. not modifying the bot or server),
the `docker-compose.yml` in the root of this repository describes everything required to run the
bot and server. Install [Docker][] and [docker-compose][] and you will be able to run the server
with just `docker-compose up`.

[Docker]: https://www.docker.com/
[docker-compose]: https://docs.docker.com/compose/

This `docker-compose` uses the images published on the GitHub container registry. When run
this way, they have nothing to do with the source code on your system. __Using Docker for
developing the server and bot is not recommended for that reason__. Instead, follow the
instructions below to run the bot and server directly on your machine.

### Server and Bot

The server uses [Rust][] (1.56) and [PostgreSQL][] (14). The bot uses [Deno][] (1.16).
Install all of those however you like.

Once installed, use `cargo` to further install [sqlx-cli][] (with at least the `postgres` feature),
then you can set up the database:
1.  Create the database with `sqlx database create`.
2.  Migrate the database with `sqlx migrate run`.

[Node.js]: https://nodejs.org/en/
[Deno]: https://deno.land/
[Rust]: http://rust-lang.org/
[PostgreSQL]: https://www.postgresql.org/
[sqlx-cli]: https://crates.io/crates/sqlx-cli

Once all preparation and database setup steps have been completed, you can run the app. To have it
fully working requires that PostgreSQL is running already, then:
1.  *If migrations have changed*, before starting the server run `sqlx migrate run`.
2.  Run the server with `cargo run` in the `server` directory.
3.  Run the bot with `bot/scripts/bot`.

### Games

Games are developed independently of the server, and can really be built in any way you choose, so long
as it can interact with the server via HTTP and WebSocket.

For now, games have only been developed using [Node.js][] (17), so ensure that is installed before
attempting to work on them.

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
