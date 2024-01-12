// Credit Card Mask
export function maskify(cc: string): string {
  const array = Array.from(cc)
    .fill("#", 0, cc.length - 4)
    .join("");

  if (cc.length > 4) {
    return array;
  }

  return cc;
}

// Learnin Typescript Mixins
export class Serializable {
  serialize(): string {
    return JSON.stringify(this);
  }

  deserialize(source: string): void {
    const properties = JSON.parse(source);
    Object.assign(this, properties);
  }
}

// How Green Is My Valley?
export function makeValley(arr: number[]): number[] {
  const sortedArray = arr.sort((a, b) => b - a);

  let leftWing: number[] = [];
  let rightWing: number[] = [];

  sortedArray.filter((v, i) =>
    i % 2 === 0 ? leftWing.push(sortedArray[i]) : rightWing.push(sortedArray[i])
  );

  rightWing.sort((a, b) => a - b);

  return leftWing.concat(rightWing);
}

// Simple Fun #11: Swap Adjacent Bits
export function swapAdjacentBits(n: number): number {
  const binaryStr = n.toString(2).padStart(32, "0").split("");

  const resultStr = binaryStr.reduce((acc, curr, index, arr) => {
    if (index % 2 === 0) {
      return acc;
    } else {
      return acc + curr + arr[index - 1];
    }
  }, "");

  return parseInt(resultStr, 2);
}
