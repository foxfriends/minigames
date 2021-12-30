# Minigames Client: React

Client library for developing games for the Discord bot.

## Installation

This package is currently published on the Github NPM registry. To use it, follow
the [instructions](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-npm-registry#installing-packages-from-other-organizations)
on how to configure your NPM for the Github NPM registry, then install `@foxfriends/minigames-client-react`


```sh
npm install @foxfriends/minigames-client-react
```

## Usage

Interactions with the minigames servers have been abstracted away by the `<Minigame>`
component and some hooks exposed by this package.

### The `<Minigame>` component

The Minigame component should be included near the top of your application, as a parent
component of all your components that expect to interact with minigames. An example
usage is as follows:

```javascript
import * as React from "react";
import { render } from "react-dom";
import Minigame from "@foxfriends/minigames-client-react";
import App from "./App.jsx";

render(
  <Minigame name="<name>" apiUrl="<api_url>" socketUrl="<socket_url>">
    <App />
  </Minigame>,
  document.querySelector("#app"),
);
```

The `<Minigame>` component accepts a few props:

- `name`: the name as your game, same as when registering to the main server (see
  the [Guide](../../GUIDE.md) for more info).
- `api_url`: The URL at which the main server's REST API can be reached.
- `socket_url`: The URL at which the main server's WebSocket API can be reached.

By using the `<Minigame>` component this way, it will retrieve the `token` and `game_id`
query parameters from the current URL, validate the token, and make the appropriate API
calls to get your game set up and connected to the WebSocket. Once that is complete, you
will be able to interact with the minigame using the following hooks.

### `useGameInfo()`

Returns some general information about the game.

```typescript
import { useGameInfo } from "@foxfriends/minigames-client-react";

type PlayerInfo = {
  // The Discord user ID of this player.
  id: string;
  // Whether this player is the one that initiated the challenge.
  isChallenger: boolean;
};

type GameInfo = {
  // The current user's ID.
  me: string;
  // Whether the rest of the information has been loaded yet or not.
  loading: boolean;
  // An array of information about the players.
  players?: PlayerInfo[];
  // Whether the game has already been completed or not.
  isComplete?: boolean;
  // The ID of the winner, or null if no winner
  winnerId?: string | null;
};

const gameInfo: GameInfo = useGameInfo();
```

Some things to note about this information:

1.  While `loading` is `true`, the other values will be missing.
2.  The current user may be a spectator. Always check if there is a player with the current user's ID.
3.  This information does not update once it has been loaded, even when the game is competed and a winner is determined.

### `useGameState()`

Similar to the standard `useState()` hook, the `useGameState()` hook provides a
way to both get and set the game's current state.

```typescript
import { useGameState } from "@foxfriends/minigames-client-react";

const [gameState, setGameState]: [T, (T) => void] = useGameState();
```

Before the game loads, the value of `gameState` will be `undefined`. Once
loaded, `gameState` will be `null` if it has not yet been set, otherwise, it
will be the most recently set value.

One thing to note is that when `setGameState` is called, it does not immediately
set the state, but sends the new state to the server. Only once acknowledged by
the server will the `gameState` value be updated. `gameState` may also change
in response to a different client of the same game calling `setGameState`.

### `useInitialState()`

A helper for setting the initial game state. This hook takes a callback that
should simply return the initial state for the game. Internally there is logic
to ensure that this only gets called once among all clients, and only if the
game is in fact not initialized.

```typescript
import { useInitialState } from "@foxfriends/minigames-client-react";

useInitialState((): T => {
  /* ... */
});
```

The callback may be `async` if necessary, but it usually should not be
necessary. You may also want to use the list of `players` from `useGameInfo()`
to generate an initial state that references those players.

### `useWinner()`

A helper for setting the winner of a game. This hook takes a callback which
receives one parameter, the game state, and is expected to determine which
player is the winner. The return value of this callback should be:

- `undefined` if the game is not over;
- a player's ID if that player is the winner; or
- `null` if the game has ended in a draw.

This computation should rely only on the `gameState` parameter and other
non-changing information (e.g. it should be pure), as it is called each
time the game state changes, but that's it, so if the win condition relies
on external data, it might be missed.

```typescript
import { useWinner } from "@foxfriends/minigames-client-react";

useWinner((gameState: T): string | null | undefined => {
  /* ... */
});
```

Similar to `useInitialState()`, this callback may be `async` if necessary,
but this is probably a bad idea.
