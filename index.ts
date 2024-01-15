export function circleOfNumbers(n: number, firstNumber: number) {
  return (firstNumber + n / 2) % n;
}

console.log("circleOfNumbers", circleOfNumbers(10, 2));
