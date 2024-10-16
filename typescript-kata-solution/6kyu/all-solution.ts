// Getting MAD
export function gettingMad(array: number[]): number {
  const sortedArray = [...array].sort((a, b) => a - b);

  return sortedArray.reduce((minDiff, current, index, arr) => {
    if (index === 0) return minDiff;

    const currentDiff = Math.abs(current - arr[index - 1]);

    return Math.min(minDiff, currentDiff);
  }, Infinity);
}

console.log("gettingMad", gettingMad([-11, 2, -4]));

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

/**
 * Returns the second largest number from the given array of numbers.
 *
 * @param nums - An array of numbers.
 * @returns The second largest number.
 */
export const returnSecondMax = (nums: number[]): number => {
  const uniqueNums: Set<number> = new Set(nums);
  console.log("uniqueNums", uniqueNums);

  const sortedNums: number[] = Array.from(uniqueNums).sort((a, b) => b - a);

  return sortedNums[1];
};

console.log("returnSecondMax", returnSecondMax([1, 3, 5, 12, 4, 21, 3, 2]));

export function uniqueInOrder(
  iterable: string | (string | number)[]
): (string | number)[] {
  // Dizi haline getirmek için spread operatorü kullanıyoruz
  const arr = [...iterable];
  const result: (string | number)[] = [];

  // Map kullanarak diziyi döndürüyoruz
  arr.map((v) => {
    console.log(
      `result indis value'si için: ${result}`,
      result[result.length - 1]
    );
    // Eğer sonuç dizisi boşsa veya son elemanı v'ye eşit değilse ekliyoruz
    if (result.length === 0 || result[result.length - 1] !== v) {
      result.push(v);
    }
  });

  return result;
}

uniqueInOrder("AAAABBBCCDAABBB");
