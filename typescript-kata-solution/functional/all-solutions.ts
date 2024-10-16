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
