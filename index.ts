export const simpson = (n: number): number => {
  const a = 0;
  const b = Math.PI;
  const h = (b - a) / n;

  const f = (x: number): number => (3 / 2) * Math.pow(Math.sin(x), 3);

  const range = (start: number, end: number, step: number): number[] =>
    Array.from(
      { length: (end - start) / step + 1 },
      (_, i) => start + i * step
    );

  const sum = (arr: number[]): number => arr.reduce((acc, val) => acc + val, 0);

  const oddTerms = sum(range(1, n - 1, 2).map((i) => 4 * f(a + i * h)));
  const evenTerms = sum(range(2, n - 2, 2).map((i) => 2 * f(a + i * h)));

  return (h / 3) * (f(a) + f(b) + oddTerms + evenTerms);
};

console.log("Simpson", simpson(290));
