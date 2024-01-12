// you only need one
export const check = (a: (number | string)[], x: number | string): boolean => {
  const array = a.some((v) => x === v);
  return array;
};

// CSV representation of array
export function toCsvText(array: number[][]): string {
  return array.map((row) => row.join(",")).join("\n");
}

// Double Char
export function doubleChar(str: string): string {
  const array = Array.from(str)
    .map((character) => character + character)
    .join("");

  return array;
}

// Count of positives / sum of negatives
export function countPositivesSumNegatives(input: number[] | null) {
  if (input === null || input.length === 0) {
    return [];
  }

  const positiveCount = input.filter((num) => num > 0).length;
  const negativeSum = input
    .filter((num) => num < 0)
    .reduce((acc, num) => acc + num, 0);

  return [positiveCount, negativeSum];
}

// Safen User Input Part I - htmlspecialchars
export function htmlspecialchars(formData: string): string {
  const replacements: { [key: string]: string } = {
    "<": "&lt;",
    ">": "&gt;",
    '"': "&quot;",
    "&": "&amp;",
  };

  return formData
    .split("")
    .map((char) => replacements[char] || char)
    .join("");
}

// How many stairs will Suzuki climb in 20 years?
export function stairsIn20(stairs: number[][]): number {
  return (
    stairs.reduce(
      (acc, day) =>
        acc +
        day.reduce((dayTotal, stairsClimbed) => dayTotal + stairsClimbed, 0),
      0
    ) * 20
  );
}

// What is between?
export function between(a: number, b: number): number[] {
  const length = Math.abs(b - a) + 1;
  const start = Math.min(a, b);

  const array = Array.from({ length }, (_, index) => start + index);

  return array;
}

// How good are you really?
export function betterThanAverage(
  classPoints: number[],
  yourPoints: number
): boolean {
  const allPoints = classPoints.concat([yourPoints]);

  const total = allPoints.reduce((acc, curr) => acc + curr, 0);

  const avarage = total / allPoints.length;

  return yourPoints > avarage;
}

// A wolf in sheep's clothing
export function warnTheSheep(queue: string[]): string {
  const reversedQueue = [...queue].reverse();
  const wolfIndex = reversedQueue.indexOf("wolf");

  if (wolfIndex === 0) {
    return "Pls go away and stop eating my sheep";
  } else {
    return `Oi! Sheep number ${wolfIndex}! You are about to be eaten by a wolf!`;
  }
}

// Convert number to reversed array of digits
export const digitize = (n: number): number[] => {
  return Array.from(n.toString()).reverse().map(Number);
};
