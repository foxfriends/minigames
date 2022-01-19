const collect = (gen) => [...gen()];

export default function intersection(lhs, rhs) {
  return collect(function* () {
    for (const item of lhs) {
      if (rhs.includes(item)) {
        yield item;
      }
    }
  });
}
