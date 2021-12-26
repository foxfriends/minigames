export default function useWinner(computeWinner) {
  useEffect(async () => {
    const winnerId = await computeWinner(gameState);
    if (winnerId) {
      const response = await fetch(`${apiUrl}/complete`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          gameId,
          winnerId,
        }),
      });
      if (response.status !== 200) {
        console.error(await response.json());
        return;
      }
    }
  }, [gameState]);
}
