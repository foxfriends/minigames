export function identity(x) {
  return x;
}

export function compose(...fns) {
  return fns.reduce(
    (first, fn) =>
      (...args) =>
        fn(first(...args)),
  );
}
