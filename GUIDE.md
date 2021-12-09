# Guide

Games are built independently from the server and bot. Games communicate with the server via HTTP,
so they can be developed using whatever technology you choose, so long as the game can be opened
by navigating to a URL in the browser (as this is how users will open games from Discord).

To aid in the understanding of this guide, first some terminology:

<dl>
  <dt>Bot</dt>
  <dd>The Discord bot (e.g. from `/bot`)</dd>
  <dt>Main Server</dt>
  <dd>The coordinating server (e.g. from `/server`)</dd>
  <dt>Minigame Server</dt>
  <dd>The independent server that hosts the minigame (what you are building)</dd>
  <dt>Minigame Client</dt>
  <dd>The user interface by which the player plays the game</dd>
</dl>

## Minigame Server

### Public URL

Though there is technically no requirement that the game *plays* in the browser, it is required
that the minigame includes a server that provides a public URL which, when visited in a browser
starts the game.

### Registering and Unregistering

The minigame server needs to tell the main server that it has started, adding the
game to the list of games that will be able to the bot.

Similarly, when the minigame server shuts down it should notify the main server.
The two requests are as follows:

Registering:

```
POST /game/<name>
Authorization: Bearer <token>
Content-Type: application/json

{ "url": "PUBLIC_URL" }
```

Unregistering:

```
DELETE /game/<name>
Authorization: Bearer <token>
```

### Health Check

The minigame server should have a route `GET /health` which responds with an empty response,
which will be reached occasionally by the main server. If this address cannot be reached,
the game will be removed from the registry on the main server.

## Minigame Client

The game logic must be handled entirely by the minigame implementation. The communication
between minigame clients is performed through the main server, via the WebSocket endpoint,
which is to be served alongside the main server's HTTP server.

The *only* function the main server performs in this area is sending data back and forth.
All other things, like determining whose turn it is, or whether changes to the state are
valid, must be handled by the respective clients. Helpers will eventually be provided to
cover some common situations.

### Requests

The WebSocket accepts JSON encoded messages, of the form `{ id: <id>, payload: <action> }`,
where the `<id>` is any string (which will be included in a response, if a response is
generated), and the `<payload>` is one of the objects described below.

#### Subscribe

```javascript
{ "Subscribe": <game_id> }
```

The `Subscribe` event subscribes the WebSocket to notifications about updates to the
game state.

Include the `<game_id>` which was provided in the query parameters when navigating to
the game.

Note that subscribing does not cause the current game state to be sent to the client,
only future events. You will likely need to send the `Get` event as well, when first
loading the game.

#### Unsubscribe

```javascript
{ "Unsubscribe": <game_id> }
```

The `Unsubscribe` event undoes the subscription, ending such notifications.

#### Get

```javascript
{ "Get": <game_id> }
```

The `Get` event retrieves the current state of the requested game immediately.
This is the only request which has an explicit response.

Include the `<game_id>` which was provided in the query parameters when navigating to
the game.

#### Set

```javascript
{ "Set": [<game_id>, <state>] }
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

```javascript
{ "Update": <state> }
```

An updated game state. The client should accept this state and show it to the user.

#### Error

```javascript
{ "Error": <message> }
```

An error has occurred while handling a message sent by this client. This will only be
sent in response to a message.

## Implementations

Implementations of the common parts of the minigame servers will be provided. At this
time, a single minigame client implementation is being actively developed for React,
and a single minigame server implementation is being developed as a wrapper for the 
Vite dev server.
