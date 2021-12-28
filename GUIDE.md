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

## Minigame Server

The first step of creating a minigame is setting up a minigame server. This server
serves two purposes:
1.  To tell the Main Server that the minigame exists and is ready to host games.
2.  To facilitate the playing of the game (e.g. serving the game client).

Though there is technically no requirement that the game *plays* in the browser, it
is required that the minigame includes a server that provides a public URL which, when
visited in a browser starts the game. When a user is directed to play your minigame,
they will be sent to the URL `<public_url>/?game_id=<game_id>&token=<token>`.

### Registering and Unregistering

The minigame server needs to tell the main server that it has started, adding the
game to the list of games that will be able to the bot.

Similarly, when the minigame server shuts down it should notify the main server
that it is no longer available to host games. The two requests are as follows:

> 🚧 The `Authorization: Bearer <token>` part of these requests is not yet
> implemented. You can leave that out of your requests for now.

__Registering__:

```
POST /game/<name>
Authorization: Bearer <token>
Content-Type: application/json

{ "url": "<public_url>" }

where:
  <name> is the name of your game, which must not already be registered on the Main server
  <token> is your API key
  <public_url> is the URL at which your game can be reached, as described above
```

__Unregistering__:

```
DELETE /game/<name>
Authorization: Bearer <token>

where:
  <name> is the name of your game, which must not already be registered on the Main server
  <token> is your API key
```

### Health Check

The minigame server should have a route `GET /health` which responds with an empty response,
which will be reached occasionally by the main server. If this address cannot be reached,
the game will be removed from the registry on the main server.

> 🚧 This check has not yet been implemented on the main server, so you don't have to
> worry about this right yet.

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

### Retrieve game information

The minigame client will likely need to know some information about the game, which
can be retrieved via the REST API with the following request:

```
GET /games/<name>/<game_id>

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
    "is_challenger": boolean 
  },
  // Whether this game is completed
  "is_complete": boolean,
  // The Discord user ID of the winner, or null in the case of a draw or if the game is not complete
  "winner_id": string | null,
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
POST /complete
Authorization: Bearer <token>
Content-Type: application/json

{ "gameId": <game_id>, "winnerId": <winner_id> }

where:
  <token> is the token provided in the query parameters when opening your game
  <game_id> is the game_id provided in the query parameters when opening your game
  <winner_id> is the user ID of the winner, or `null` in case of a draw
```
