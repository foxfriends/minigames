# Guide

> This guide explains how to develop a new minigame.

Games are built independently from the server and bot. Games communicate with the server via HTTP
and WebSocket, so they can be developed using whatever technology you choose, so long as the game
can be opened by navigating to a URL in the browser (as this is how users will open games from
Discord).

To aid in the understanding of this guide, first some terminology:

<dl>
  <dt>Bot</dt>
  <dd>The Discord bot (e.g. from `/bot`)</dd>
  <dt>Main Server</dt>
  <dd>The coordinating server (e.g. from `/server`)</dd>
  <dt>REST API</dt>
  <dd>The REST API exposed by the server, for interacting with the minigames application</dd>
  <dt>WebSocket API</dt>
  <dd>The WebSocket API exposed by the server, for implementing interaction between players</dd>
  <dt>Minigame Server</dt>
  <dd>The independent server that hosts the minigame (what you are building)</dd>
  <dt>Minigame Client</dt>
  <dd>The user interface by which the player plays the game (also built by you)</dd>
</dl>

## Development Environment

It is recommended that you run your own instance of the bot and main server when
developing a new minigame. You can do so by following the instructions in the
[README](./README.md). Since you shouldn't need to modify the code of the bot or
server, the Docker approach is recommended.

> When run in development mode, the default port for the Main server's REST API is 8000.

It is recommended to set the `SUPERUSER_ID` environment variable of the server to your
Discord User ID, which you can find by following [this guide][discord-user-id].

[discord-user-id]: https://support.discord.com/hc/en-us/articles/206346498-Where-can-I-find-my-User-Server-Message-ID-

## Minigame Server

The first step of creating a minigame is setting up a minigame server. This server
serves two purposes:
1.  To tell the Main Server that the minigame exists and is ready to host games.
2.  To facilitate the playing of the game (e.g. serving the game client).

Though there is technically no requirement that the game *plays* in the browser, it
is required that the minigame is accessible via a URL which, when opened by a browser
starts the game. When a user is directed to play your minigame, they will be sent to
the URL `<public_url>/?game_id=<game_id>&token=<token>`.

The name of your game, its public URL, and other settings are configured from the web
dashboard served by the main server. Open up your main server in the browser (likely
`http://locahost:8000`) and navigate to `Dashboard`, then `Developers`, and then click
the `New Game` button and fill in the appropriate fields.

If not running with `SUPERUSER_ID` set to your Discord User ID, you will also need to
check off which Discord servers your game should be made available in. Only servers
that you manage and have the bot installed will be available in the list.

### Availability

The minigame server should tell the main server that it has started, adding the
game to the list of games that will be able to the bot. Otherwise, the main server will
eventually pick up the availability of the game by performing a health check (described
below), but that takes a while.

Similarly, when the minigame server shuts down it should notify the main server
that it is no longer available to host games. The two requests are as follows:

__Registering__:

```
POST /api/v1/servers/<name>/available
X-Api-Key: <secret_key>

where:
  <name> is the name of your game, which must already be registered on the Main server
  <secret_key> is your game server's secret key
```

__Unregistering__:

```
POST /api/v1/game/<name>/unavailable
X-Api-Key: <secret_key>

where:
  <name> is the name of your game, which must not already be registered on the Main server
  <secret_key> is your game server's secret key
```

### Health Check

The minigame server should have a route `GET /health` which responds with your game server's
secret key when called by the main server. This route will be reached occasionally (every
half hour or so) by the main server. If this address cannot be reached, or the response is not
as expected, the game will be removed from the registry on the main server.

⚠ __Be careful not to respond with your API key on every request__! Only requests containing
the proper validation should be responded to with the secret key, otherwise you risk exposing
your secret key to malicious callers. A request is validated as follows:

1.  Check that the request contains the header `X-Minigames-Server`.
2.  The the value of the `X-Minigames-Server` header should be a JWT. Validate it as follows:
    1.  Retrieve the public verification key from `<main_server_url>/.well-known/openid-configuration`
    2.  Verify the JWT with that key (SPKI PEM encoded) and algorithm `RS256`.
    3.  Ensure the `iss` matches the `<main_server_url>` you retrieved it from.
    4.  Ensure the `aud` matches the name of your game.

If all that turns out okay, include the `X-Api-Key` header in your response with the value
of your secret key. Otherwise, you may respond however you like as long as it __does not__
include your secret key.

>   Note that if you use one of the provided server packages, this endpoint will be created
>   automatically for you. This is recommended, to avoid errors in the validation process.

## Minigame Client

The game logic must be handled entirely by the minigame implementation. The communication
between minigame clients is performed through the main server, via the WebSocket endpoint,
which is to be served alongside the main server's HTTP server.

The *only* function the main server performs in this area is sending data back and forth.
All other things, like determining whose turn it is, or whether changes to the state are
valid, must be handled by the respective clients. If necessary, you can use your minigame
server to facilitate some communication between players, but this is not recommended.

At this time, we provide one library [`@minigames/react`][./packages/client-react] to
help when developing minigames in [React][]. If you intend to build your game with any
other technology, you will need to implement the following pieces yourself.

[React]: https://reactjs.org

### Validate the token

> 🚧 This section is a work in progress. The method by which tokens are validated is
> likely to change soon.

### Retrieve game information

The minigame client will likely need to know some information about the game, which
can be retrieved via the REST API with the following request:

```
GET /api/v1/games/<name>/<game_id>

where:
  <name> is the name of your game (same as used when registering)
  <game_id> is the game_id provided in the query parameters when opening your game

---

200 Ok
Content-Type: application/json
{
  // An array of information on each player
  "players": []{
    // The user's Discord ID.
    "id": string,
    // Whether this player was the one that initiated the challenge via the Discord bot
    "isChallenger": boolean
  },
  // Whether this game is completed
  "isComplete": boolean,
  // The Discord user ID of the winner, or null in the case of a draw or if the game is not complete
  "winnerId": string | null,
}
```

### WebSocket protocol

The game client should connect to the Main server's WebSocket endpoint.

> In development mode, is served from a different port than the REST API, 8001 by default.

```
GET /?token=<token>

where:
  <token> is the token provided in the query parameters when opening your game
```

### Request Messages

The WebSocket accepts JSON encoded messages, of the form `{ id: <id>, payload: <action> }`,
where the `<id>` is any string (which will be included in a response, if a response is
generated), and the `<payload>` is one of the objects described below.

#### Subscribe

```json
{ "Subscribe": <game_id> }

where:
  <game_id> is the game_id provided in the query parameters when opening your game
```

The `Subscribe` event subscribes the WebSocket to notifications about updates to the
game state.

Include the `<game_id>` which was provided in the query parameters when navigating to
the game.

Note that subscribing does not cause the current game state to be sent to the client,
only future events. You will likely need to send the `Get` event as well, when first
loading the game.

#### Unsubscribe

```json
{ "Unsubscribe": <game_id> }

where:
  <game_id> is the game_id provided in the query parameters when opening your game
```

The `Unsubscribe` event undoes the subscription, ending such notifications.

#### Get

```json
{ "Get": <game_id> }

where:
  <game_id> is the game_id provided in the query parameters when opening your game
```

The `Get` event retrieves the current state of the requested game immediately.
This is the only request which has an explicit response.

Include the `<game_id>` which was provided in the query parameters when navigating to
the game.

Note that the state will be `null` if the game has not yet been initialized. In this
situation, it is the responsibility of the game client to generate a default state,
and send that state back to the server if necessary.

#### Set

```json
{ "Set": [<game_id>, <state>] }

where:
  <game_id> is the game_id provided in the query parameters when opening your game
  <state> is the new game state
```

The `Set` event sets the state of the requested game. Whenever this event is received,
the server will broadcast the new state to all clients that subscribed to this game.

### Responses

The main server will send events back in the same form (`{ id: <id>, payload: <response> }`).
The client is expected to respond to these events appropriately.

The `<id>` will be the same ID value that was sent in the request if the message was
sent in response to a message. Otherwise, if the message is sent in response to a subscription,
the `<id>` will be the empty string (`""`).

The `<response>` payload will be one of the following:

#### Update

```json
{ "Update": <state> }

where:
  <state> is the current game state
```

An updated game state. The client should accept this state and show it to the user.

#### Error

```json
{ "Error": <message> }

where:
  <message> is a string describing the error
```

An error has occurred while handling a message sent by this client. This will only be
sent in response to a message.

### Game Completion

When the game is completed, the game client is expected to send a REST API call to
the main server, to indicate the winner (or the lack of winner, in case of a draw).
Once the main server has received the same result from all players, the game is
officially considered completed.

```
POST /api/v1/complete
Authorization: Bearer <token>
Content-Type: application/json

{ "gameId": <game_id>, "winnerId": <winner_id> }

where:
  <token> is the token provided in the query parameters when opening your game
  <game_id> is the game_id provided in the query parameters when opening your game
  <winner_id> is the user ID of the winner, or `null` in case of a draw
```
