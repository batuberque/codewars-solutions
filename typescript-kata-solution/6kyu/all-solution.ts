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
