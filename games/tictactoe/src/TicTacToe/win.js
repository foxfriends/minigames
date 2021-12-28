const WIN = [
  [0, 1, 2],
  [3, 4, 5],
  [6, 7, 8],
  [0, 3, 6],
  [1, 4, 7],
  [2, 5, 8],
  [0, 4, 8],
  [2, 4, 6],
];

export default function win({ cells }) {
  return WIN.find((pattern) => {
    const values = new Set(pattern.map((i) => cells[i].value));
    return values.size === 1 && !values.has(null);
  });
}
