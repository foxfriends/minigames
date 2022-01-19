export function mk(start, end) {
  if (start >= end) {
    throw new TypeError("Line start must be smaller then line end");
  }
  return [start, end];
}

export function eq([as, ae], [bs, be]) {
  return as === bs && ae === be;
}
