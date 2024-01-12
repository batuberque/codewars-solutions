// RGB To Hex Conversion
function convert(integer: number) {
  if (integer < 0) {
    integer = 0;
  }

  if (integer > 255) {
    integer = 255;
  }

  const str = Number(integer).toString(16);

  return str.length == 1 ? "0" + str : str;
}

export function rgb(r: number, g: number, b: number): string {
  return (convert(r) + convert(g) + convert(b)).toUpperCase();
}

// Convert A Hex String To RGB
export function hexStringToRGB(hexString: string): {
  r: number;
  g: number;
  b: number;
} {
  const num = parseInt(hexString.slice(1), 16);

  const r = (num >> 16) & 255;
  const g = (num >> 8) & 255;
  const b = num & 255;

  return { r, g, b };
}

// Not very secure
export function alphanumeric(string: string): boolean {
  const isAlphanumeric = (char: string): boolean => {
    const code = char.charCodeAt(0);
    return (
      (code >= 48 && code <= 57) ||
      (code >= 65 && code <= 90) ||
      (code >= 97 && code <= 122)
    );
  };

  return string.length > 0 && string.split("").every(isAlphanumeric);
}

// ISBN-10 Validation
export function validISBN10(isbn: string): boolean {
  if (isbn.length !== 10) return false;

  const digits = isbn.split("").map((char, index) => {
    if (index === 9 && (char === "X" || char === "x")) return 10;
    if (char < "0" || char > "9") return NaN;
    return parseInt(char);
  });

  if (digits.some(isNaN)) return false;

  const sum = digits.reduce(
    (acc, current, index) => acc + current * (index + 1),
    0
  );
  return sum % 11 === 0;
}
