const map = (array: number[], callback: (el: number) => number): number[] => {
  const mappedArray: number[] = [];

  array.forEach((el) => {
    mappedArray.push(callback(el));
  });

  return mappedArray;
};

const loggerMap = map([1, 2, 3], (el) => el * 2);

// console.log("loggerMap", loggerMap);

const forEach = <T>(array: T[], callback: (el: T) => void): void => {
  return array.forEach(callback);
};

// const loggerFor = forEach([1, "20", 3], (el) => console.log(el));

const mapWith = <T>(array: T[], callback: (el: T) => T): T[] => {
  const mappedArray: T[] = [];

  forEach(array, (el) => mappedArray.push(callback(el)));

  return mappedArray;
};

const loggerMapWith = mapWith([1, 2, 3], (el) => el * 2);

// console.log("loggerMapWith", loggerMapWith);

const reduce = <T, U>(
  array: T[],
  callback: (acc: U, el: T) => U,
  acc: U
): U => {
  return array.reduce(callback, acc);
};

const loggerReduce = reduce([1, 2, 3], (acc, el) => acc + el, 0);

// console.log("loggerReduce", loggerReduce);

const intersection = (arrays: number[][]): number[] => {
  return arrays.reduce((acc, curr) => {
    return curr.filter((el) => acc.includes(el));
  });
};

const loggerInsection = intersection([
  [5, 10, 15, 20],
  [15, 88, 1, 5, 7],
  [5, 10, 15, 20],
]);

// console.log("loggerInsection", loggerInsection);

const union = (array: number[][]): number[] => {
  return array.reduce((acc, curr) => {
    const newElement = curr.filter((el) => !acc.includes(el));
    return acc.concat(newElement);
  });
};

const loggerUnion = union([
  [5, 10, 15, 20],
  [15, 88, 1, 5, 7],
  [5, 10, 15, 20],
]);

// console.log("loggerUnion", loggerUnion);

const objFilter = <T extends Record<string, any>, K extends keyof T>(
  obj: T,
  keys: K[]
): Pick<T, K> => {
  return Object.fromEntries(
    Object.entries(obj).filter(([key]) => keys.includes(key as K))
  ) as Pick<T, K>;
};

const startingObj = {
  a: 1,
  b: 2,
  c: 3,
  d: 4,
};

console.log(objFilter(startingObj, ["a", "b", "c", "d"]));

const rating = (
  arrOfFuncs: ((val: number) => boolean)[],
  value: number
): number => {
  const trueCount = arrOfFuncs.reduce((acc, func) => {
    return acc + (func(value) ? 1 : 0);
  }, 0);
  return (trueCount / arrOfFuncs.length) * 100;
};

const isEven = (n: number) => n % 2 === 0;
const greaterThanFour = (n: number) => n > 4;
const isSquare = (n: number) => Math.sqrt(n) % 1 === 0;
const hasSix = (n: number) => n.toString().includes("6");
const checks = [isEven, greaterThanFour, isSquare, hasSix];
console.log(rating(checks, 64)); // should log: 100
console.log(rating(checks, 66)); // should log: 75

const pipe = (arrOfFuncs: ((val: any) => any)[], value: any): any => {
  return arrOfFuncs.reduce((acc, func) => func(acc), value);
};

// Test
const capitalize = (str: string) => str.toUpperCase();
const addLowerCase = (str: string) => str + str.toLowerCase();
const repeat = (str: string) => str + str;
const capAddLowRepeat = [capitalize, addLowerCase, repeat];
console.log(pipe(capAddLowRepeat, "cat")); // should log: 'CATcatCATcat'
