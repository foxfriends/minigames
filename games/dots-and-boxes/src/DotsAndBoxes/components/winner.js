import { useWinner } from "@foxfriends/minigames-client-react";

function counts(obj, key) {
  obj[key] = (obj[key] ?? 0) + 1;
  return obj;
}

export default function useDotsAndBoxesWinner() {
  return useWinner((gameState) => {
    const done = gameState.boxes.every((box) => box !== null);
    if (!done) {
      return;
    }
    const scores = gameState.boxes.reduce(counts, {});
    const [a, b] = Object.keys(scores);
    if (scores[a] === scores[b]) {
      return null;
    }
    return scores[a] > scores[b] ? a : b;
  });
}
