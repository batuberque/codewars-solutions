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

export function smaller(nums: number[]): number[] {
  return nums.map((current, index) => {
    const smallerCount = nums
      .slice(index + 1)
      .filter((rightNum) => rightNum < current).length;

    return smallerCount;
  });
}

console.log("smallar", smaller([5, 4, 3, 2, 1]));

export function circleOfNumbers(n: number, firstNumber: number) {
  return (firstNumber + n / 2) % n;
}

console.log("circleOfNumbers", circleOfNumbers(10, 2));

export function findNextSquare(sq: number): number {
  const square = Math.sqrt(sq);
  const plusOne = square + 1;
  const powPlusOne = Math.pow(plusOne, 2);

  if (sq % square === 0) {
    return powPlusOne;
  } else return -1;
}
findNextSquare(121);

console.log("log", findNextSquare(121));

export function openOrSenior(data: number[][]): string[] {
  return data.map((v) => (v[0] >= 55 && v[1] > 7 ? "Senior" : "Open"));
}

console.log(
  "log",
  openOrSenior([
    [45, 12],
    [55, 21],
    [19, -2],
    [104, 20],
  ])
);
