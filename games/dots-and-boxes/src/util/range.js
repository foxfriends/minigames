export default function range(low, high) {
  const arr = [];
  for (let i = low; i < high; ++i) {
    arr.push(i);
  }
  return arr;
}
