# Minigames

Minigames for Discord servers. So far does nothing. Will have more info later.

## Environment Setup

This project uses [Deno][] (1.16), [Rust][] (1.56), [PostgreSQL][] (14) and [Redis][] (5). Install all
of those however you like.

Once installed, use those to further install [trex][] and [sqlx-cli][] (with at least the `postgres` feature)

In future, hopefully we'll have a Docker or Codespace or something for all of this.

1.  On the [Discord Developer Portal][], Create an application, and then a bot within that application.
2.  Copy `bot/.env.example` to `bot/.env` and put the appropriate values in.
3.  Copy `server/.env.example` to `server/.env` and put the appropriate values in.
4.  Create the database with `sqlx database create`.
5.  Migrate the database with `sqlx migrate run`.
6.  Add the bot to the Discord server you wish to add it to by visiting the link output by `trex run add`.

## Running

Once all setup steps have been completed, you can run the app. To have it fully working requires that
Redis and PostgreSQL are running already, then:
1.  *If migrations have changed*, before starting the server run `sqlx migrate run`.
2.  Run the server with `cargo run` in the `server` directory.
3.  Run the bot with `trex run bot` in the `bot` directory.

[Deno]: https://deno.land/
[Rust]: http://rust-lang.org/
[PostgreSQL]: https://www.postgresql.org/
[Redis]: https://redis.io/
[trex]: https://deno.land/x/trex
[sqlx-cli]: https://crates.io/crates/sqlx-cli
[Discord Developer Portal]: https://discord.com/developers/

## Contributing

Pull Requests and Issues are always welcome. Do note that this project is in very early stages
so things will be moving quickly. You may want to wait until it settles down a bit. If you want
to get involved though, do reach out and a roadmap can be set out.

Do keep to a consistent style, as enforced by `trex run fmt` and `cargo fmt`. Pull requests will
not be accepted until CI passes, which includes those mentioned formats, as well as `trex run lint`,
`cargo clippy`, and `cargo sqlx prepare --check` to all succeed.
