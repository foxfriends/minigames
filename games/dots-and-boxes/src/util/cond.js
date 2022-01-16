export default function cond(cases) {
  return (value) => {
    for (const [predicate, action] of cases) {
      if (predicate(value)) return action(value);
    }
    return undefined;
  };
}

export function otherwise() {
  return true;
}
