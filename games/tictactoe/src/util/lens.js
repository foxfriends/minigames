function lens(get, set) {
  return { get, set };
}

export const set = (lens) => (val) => (obj) => {
  return lens.set(val, obj);
};

export const get = (lens) => (obj) => {
  return lens.get(obj);
};

export function mod(lens, fn, obj) {
  return lens.set(fn(lens.get(obj)), obj);
}

export function prop(key) {
  return lens(
    (obj) => obj[key],
    (val, obj) => ({ ...obj, [key]: val }),
  );
}

export function nth(i) {
  return lens(
    (arr) => arr[i],
    (val, arr) => {
      const dup = [...arr];
      dup[i] = val;
      return dup;
    },
  );
}

export const identity = lens(
  (x) => x,
  (x) => x,
);

export function compose(...lenses) {
  return lenses.reduce((already, current) => {
    return lens(
      (obj) => current.get(already.get(obj)),
      (val, obj) => already.set(current.set(val, already.get(obj)), obj),
    );
  }, identity);
}
