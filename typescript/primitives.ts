export class Bool {
  private readonly _value: boolean;

  constructor(value: boolean) {
    this._value = value;
  }

  get value(): boolean {
    return this._value;
  }

  static true(): Bool {
    return new Bool(true);
  }

  static false(): Bool {
    return new Bool(false);
  }

  not(): Bool {
    return new Bool(!this._value);
  }

  and(other: Bool): Bool {
    return new Bool(this._value && other._value);
  }

  or(other: Bool): Bool {
    return new Bool(this._value || other._value);
  }

  xor(other: Bool): Bool {
    return new Bool(this._value !== other._value);
  }

  equals(other: Bool): boolean {
    return this._value === other._value;
  }

  toString(): string {
    return this._value.toString();
  }

  toBoolean(): boolean {
    return this._value;
  }

  valueOf(): boolean {
    return this._value;
  }

  [Symbol.toPrimitive](hint: string): boolean | string {
    if (hint === 'string') {
      return this._value.toString();
    }
    return this._value;
  }
}

export class Int {
  private readonly _value: number;

  constructor(value: number) {
    if (!Number.isInteger(value)) {
      throw new Error('Int must be an integer');
    }
    this._value = value;
  }

  get value(): number {
    return this._value;
  }

  static zero(): Int {
    return new Int(0);
  }

  static one(): Int {
    return new Int(1);
  }

  add(other: Int): Int {
    return new Int(this._value + other._value);
  }

  subtract(other: Int): Int {
    return new Int(this._value - other._value);
  }

  multiply(other: Int): Int {
    return new Int(this._value * other._value);
  }

  divide(other: Int): Int {
    if (other._value === 0) {
      throw new Error('Division by zero');
    }
    return new Int(Math.floor(this._value / other._value));
  }

  modulo(other: Int): Int {
    if (other._value === 0) {
      throw new Error('Modulo by zero');
    }
    return new Int(this._value % other._value);
  }

  power(exponent: Int): Int {
    return new Int(Math.pow(this._value, exponent._value));
  }

  bitwiseAnd(other: Int): Int {
    return new Int(this._value & other._value);
  }

  bitwiseOr(other: Int): Int {
    return new Int(this._value | other._value);
  }

  bitwiseXor(other: Int): Int {
    return new Int(this._value ^ other._value);
  }

  bitwiseNot(): Int {
    return new Int(~this._value);
  }

  leftShift(shift: Int): Int {
    return new Int(this._value << shift._value);
  }

  rightShift(shift: Int): Int {
    return new Int(this._value >> shift._value);
  }

  unsignedRightShift(shift: Int): Int {
    return new Int(this._value >>> shift._value);
  }

  negate(): Int {
    return new Int(-this._value);
  }

  abs(): Int {
    return new Int(Math.abs(this._value));
  }

  equals(other: Int): boolean {
    return this._value === other._value;
  }

  lessThan(other: Int): boolean {
    return this._value < other._value;
  }

  lessThanOrEqual(other: Int): boolean {
    return this._value <= other._value;
  }

  greaterThan(other: Int): boolean {
    return this._value > other._value;
  }

  greaterThanOrEqual(other: Int): boolean {
    return this._value >= other._value;
  }

  isEven(): boolean {
    return this._value % 2 === 0;
  }

  isOdd(): boolean {
    return this._value % 2 !== 0;
  }

  isPositive(): boolean {
    return this._value > 0;
  }

  isNegative(): boolean {
    return this._value < 0;
  }

  isZero(): boolean {
    return this._value === 0;
  }

  toString(): string {
    return this._value.toString();
  }

  toNumber(): number {
    return this._value;
  }

  static min(a: Int, b: Int): Int {
    return new Int(Math.min(a._value, b._value));
  }

  static max(a: Int, b: Int): Int {
    return new Int(Math.max(a._value, b._value));
  }

  valueOf(): number {
    return this._value;
  }

  [Symbol.toPrimitive](hint: string): number | string {
    if (hint === 'string') {
      return this._value.toString();
    }
    return this._value;
  }
}

export class Float {
  private readonly _value: number;

  constructor(value: number) {
    this._value = value;
  }

  get value(): number {
    return this._value;
  }

  static zero(): Float {
    return new Float(0);
  }

  static one(): Float {
    return new Float(1);
  }

  static pi(): Float {
    return new Float(Math.PI);
  }

  static e(): Float {
    return new Float(Math.E);
  }

  static nan(): Float {
    return new Float(NaN);
  }

  static positiveInfinity(): Float {
    return new Float(Infinity);
  }

  static negativeInfinity(): Float {
    return new Float(-Infinity);
  }

  add(other: Float): Float {
    return new Float(this._value + other._value);
  }

  subtract(other: Float): Float {
    return new Float(this._value - other._value);
  }

  multiply(other: Float): Float {
    return new Float(this._value * other._value);
  }

  divide(other: Float): Float {
    if (other._value === 0) {
      return this._value > 0 ? Float.positiveInfinity() : Float.negativeInfinity();
    }
    return new Float(this._value / other._value);
  }

  power(exponent: Float): Float {
    return new Float(Math.pow(this._value, exponent._value));
  }

  sqrt(): Float {
    return new Float(Math.sqrt(this._value));
  }

  abs(): Float {
    return new Float(Math.abs(this._value));
  }

  negate(): Float {
    return new Float(-this._value);
  }

  floor(): Float {
    return new Float(Math.floor(this._value));
  }

  ceil(): Float {
    return new Float(Math.ceil(this._value));
  }

  round(): Float {
    return new Float(Math.round(this._value));
  }

  trunc(): Float {
    return new Float(Math.trunc(this._value));
  }

  sin(): Float {
    return new Float(Math.sin(this._value));
  }

  cos(): Float {
    return new Float(Math.cos(this._value));
  }

  tan(): Float {
    return new Float(Math.tan(this._value));
  }

  log(): Float {
    return new Float(Math.log(this._value));
  }

  log10(): Float {
    return new Float(Math.log10(this._value));
  }

  exp(): Float {
    return new Float(Math.exp(this._value));
  }

  equals(other: Float, epsilon: number = 1e-10): boolean {
    return Math.abs(this._value - other._value) < epsilon;
  }

  lessThan(other: Float): boolean {
    return this._value < other._value;
  }

  lessThanOrEqual(other: Float): boolean {
    return this._value <= other._value;
  }

  greaterThan(other: Float): boolean {
    return this._value > other._value;
  }

  greaterThanOrEqual(other: Float): boolean {
    return this._value >= other._value;
  }

  isNaN(): boolean {
    return Number.isNaN(this._value);
  }

  isInfinity(): boolean {
    return !Number.isFinite(this._value) && !this.isNaN();
  }

  isFinite(): boolean {
    return Number.isFinite(this._value);
  }

  isPositive(): boolean {
    return this._value > 0;
  }

  isNegative(): boolean {
    return this._value < 0;
  }

  isZero(): boolean {
    return this._value === 0;
  }

  isInteger(): boolean {
    return Number.isInteger(this._value);
  }

  toString(): string {
    return this._value.toString();
  }

  toFixed(digits: number): string {
    return this._value.toFixed(digits);
  }

  toExponential(fractionDigits?: number): string {
    return this._value.toExponential(fractionDigits);
  }

  toPrecision(precision?: number): string {
    return this._value.toPrecision(precision);
  }

  toNumber(): number {
    return this._value;
  }

  static min(a: Float, b: Float): Float {
    return new Float(Math.min(a._value, b._value));
  }

  static max(a: Float, b: Float): Float {
    return new Float(Math.max(a._value, b._value));
  }

  static clamp(value: Float, min: Float, max: Float): Float {
    return new Float(Math.min(Math.max(value._value, min._value), max._value));
  }

  static lerp(a: Float, b: Float, t: Float): Float {
    return new Float(a._value + (b._value - a._value) * t._value);
  }

  valueOf(): number {
    return this._value;
  }

  [Symbol.toPrimitive](hint: string): number | string {
    if (hint === 'string') {
      return this._value.toString();
    }
    return this._value;
  }
}

export class Str {
  private readonly _value: string;

  constructor(value: string) {
    this._value = value;
  }

  get value(): string {
    return this._value;
  }

  static empty(): Str {
    return new Str('');
  }

  get length(): number {
    return this._value.length;
  }

  isEmpty(): boolean {
    return this._value.length === 0;
  }

  charAt(index: number): string {
    return this._value.charAt(index);
  }

  charCodeAt(index: number): number {
    return this._value.charCodeAt(index);
  }

  concat(other: Str): Str {
    return new Str(this._value + other._value);
  }

  substring(start: number, end?: number): Str {
    return new Str(this._value.substring(start, end));
  }

  slice(start: number, end?: number): Str {
    return new Str(this._value.slice(start, end));
  }

  indexOf(searchValue: Str, fromIndex?: number): number {
    return this._value.indexOf(searchValue._value, fromIndex);
  }

  lastIndexOf(searchValue: Str, fromIndex?: number): number {
    return this._value.lastIndexOf(searchValue._value, fromIndex);
  }

  includes(searchValue: Str, fromIndex?: number): boolean {
    return this._value.includes(searchValue._value, fromIndex);
  }

  startsWith(searchValue: Str, position?: number): boolean {
    return this._value.startsWith(searchValue._value, position);
  }

  endsWith(searchValue: Str, length?: number): boolean {
    return this._value.endsWith(searchValue._value, length);
  }

  toLowerCase(): Str {
    return new Str(this._value.toLowerCase());
  }

  toUpperCase(): Str {
    return new Str(this._value.toUpperCase());
  }

  trim(): Str {
    return new Str(this._value.trim());
  }

  trimStart(): Str {
    return new Str(this._value.trimStart());
  }

  trimEnd(): Str {
    return new Str(this._value.trimEnd());
  }

  padStart(targetLength: number, padString?: Str): Str {
    return new Str(this._value.padStart(targetLength, padString?._value));
  }

  padEnd(targetLength: number, padString?: Str): Str {
    return new Str(this._value.padEnd(targetLength, padString?._value));
  }

  repeat(count: number): Str {
    return new Str(this._value.repeat(count));
  }

  replace(searchValue: Str | RegExp, replaceValue: Str): Str {
    if (searchValue instanceof Str) {
      return new Str(this._value.replace(searchValue._value, replaceValue._value));
    }
    return new Str(this._value.replace(searchValue, replaceValue._value));
  }

  replaceAll(searchValue: Str | RegExp, replaceValue: Str): Str {
    if (searchValue instanceof Str) {
      return new Str(this._value.replaceAll(searchValue._value, replaceValue._value));
    }
    return new Str(this._value.replaceAll(searchValue, replaceValue._value));
  }

  split(separator: Str | RegExp, limit?: number): Str[] {
    if (separator instanceof Str) {
      return this._value.split(separator._value, limit).map(s => new Str(s));
    }
    return this._value.split(separator, limit).map(s => new Str(s));
  }

  equals(other: Str): boolean {
    return this._value === other._value;
  }

  equalsIgnoreCase(other: Str): boolean {
    return this._value.toLowerCase() === other._value.toLowerCase();
  }

  compare(other: Str): number {
    return this._value.localeCompare(other._value);
  }

  isWhitespace(): boolean {
    return /^\s*$/.test(this._value);
  }

  isAlpha(): boolean {
    return /^[a-zA-Z]+$/.test(this._value);
  }

  isNumeric(): boolean {
    return /^[0-9]+$/.test(this._value);
  }

  isAlphanumeric(): boolean {
    return /^[a-zA-Z0-9]+$/.test(this._value);
  }

  reverse(): Str {
    return new Str(this._value.split('').reverse().join(''));
  }

  countOccurrences(substring: Str): number {
    let count = 0;
    let pos = 0;
    while ((pos = this._value.indexOf(substring._value, pos)) !== -1) {
      count++;
      pos += substring._value.length;
    }
    return count;
  }

  words(): Str[] {
    return this._value.split(/\s+/).filter(w => w.length > 0).map(w => new Str(w));
  }

  lines(): Str[] {
    return this._value.split(/\r?\n/).map(l => new Str(l));
  }

  capitalize(): Str {
    if (this._value.length === 0) return this;
    return new Str(this._value.charAt(0).toUpperCase() + this._value.slice(1).toLowerCase());
  }

  titleCase(): Str {
    return new Str(
      this._value.replace(/\w\S*/g, txt => txt.charAt(0).toUpperCase() + txt.substr(1).toLowerCase())
    );
  }

  camelCase(): Str {
    const words = this.words();
    if (words.length === 0) return Str.empty();
    const first = words[0].toLowerCase();
    const rest = words.slice(1).map(w => w.capitalize());
    return new Str(first.value + rest.map(w => w.value).join(''));
  }

  snakeCase(): Str {
    return new Str(
      this._value.replace(/([A-Z])/g, '_$1').toLowerCase().replace(/^_/, '').replace(/\s+/g, '_')
    );
  }

  kebabCase(): Str {
    return new Str(
      this._value.replace(/([A-Z])/g, '-$1').toLowerCase().replace(/^-/, '').replace(/\s+/g, '-')
    );
  }

  toCharArray(): string[] {
    return this._value.split('');
  }

  toString(): string {
    return this._value;
  }

  static fromCharArray(chars: string[]): Str {
    return new Str(chars.join(''));
  }

  static fromCharCode(...codes: number[]): Str {
    return new Str(String.fromCharCode(...codes));
  }

  static join(strings: Str[], separator: Str = Str.empty()): Str {
    return new Str(strings.map(s => s._value).join(separator._value));
  }

  valueOf(): string {
    return this._value;
  }

  [Symbol.toPrimitive](hint: string): string {
    return this._value;
  }
}

export class Null {
  private static readonly _instance: Null = new Null();

  private constructor() {}

  static get instance(): Null {
    return Null._instance;
  }

  isNull(): boolean {
    return true;
  }

  equals(other: any): boolean {
    return other === null || other instanceof Null;
  }

  toString(): string {
    return 'null';
  }

  toJSON(): null {
    return null;
  }
}

export const nullValue = Null.instance;

export class BigNumber {
  private readonly _value: bigint;

  constructor(value: string | number | bigint) {
    if (typeof value === 'string') {
      this._value = BigInt(value);
    } else if (typeof value === 'number') {
      if (!Number.isInteger(value)) {
        throw new Error('BigNumber must be initialized with an integer');
      }
      this._value = BigInt(value);
    } else {
      this._value = value;
    }
  }

  get value(): bigint {
    return this._value;
  }

  static zero(): BigNumber {
    return new BigNumber(0n);
  }

  static one(): BigNumber {
    return new BigNumber(1n);
  }

  static fromString(value: string): BigNumber {
    return new BigNumber(value);
  }

  add(other: BigNumber): BigNumber {
    return new BigNumber(this._value + other._value);
  }

  subtract(other: BigNumber): BigNumber {
    return new BigNumber(this._value - other._value);
  }

  multiply(other: BigNumber): BigNumber {
    return new BigNumber(this._value * other._value);
  }

  divide(other: BigNumber): BigNumber {
    if (other._value === 0n) {
      throw new Error('Division by zero');
    }
    return new BigNumber(this._value / other._value);
  }

  modulo(other: BigNumber): BigNumber {
    if (other._value === 0n) {
      throw new Error('Modulo by zero');
    }
    return new BigNumber(this._value % other._value);
  }

  power(exponent: BigNumber): BigNumber {
    if (exponent._value < 0n) {
      throw new Error('Exponent must be non-negative');
    }
    let result = 1n;
    let base = this._value;
    let exp = exponent._value;
    while (exp > 0n) {
      if (exp % 2n === 1n) {
        result *= base;
      }
      base *= base;
      exp = exp / 2n;
    }
    return new BigNumber(result);
  }

  bitwiseAnd(other: BigNumber): BigNumber {
    return new BigNumber(this._value & other._value);
  }

  bitwiseOr(other: BigNumber): BigNumber {
    return new BigNumber(this._value | other._value);
  }

  bitwiseXor(other: BigNumber): BigNumber {
    return new BigNumber(this._value ^ other._value);
  }

  bitwiseNot(): BigNumber {
    return new BigNumber(~this._value);
  }

  leftShift(shift: BigNumber): BigNumber {
    return new BigNumber(this._value << shift._value);
  }

  rightShift(shift: BigNumber): BigNumber {
    return new BigNumber(this._value >> shift._value);
  }

  negate(): BigNumber {
    return new BigNumber(-this._value);
  }

  abs(): BigNumber {
    return new BigNumber(this._value < 0n ? -this._value : this._value);
  }

  equals(other: BigNumber): boolean {
    return this._value === other._value;
  }

  lessThan(other: BigNumber): boolean {
    return this._value < other._value;
  }

  lessThanOrEqual(other: BigNumber): boolean {
    return this._value <= other._value;
  }

  greaterThan(other: BigNumber): boolean {
    return this._value > other._value;
  }

  greaterThanOrEqual(other: BigNumber): boolean {
    return this._value >= other._value;
  }

  isEven(): boolean {
    return this._value % 2n === 0n;
  }

  isOdd(): boolean {
    return this._value % 2n !== 0n;
  }

  isPositive(): boolean {
    return this._value > 0n;
  }

  isNegative(): boolean {
    return this._value < 0n;
  }

  isZero(): boolean {
    return this._value === 0n;
  }

  isOne(): boolean {
    return this._value === 1n;
  }

  sign(): number {
    if (this._value > 0n) return 1;
    if (this._value < 0n) return -1;
    return 0;
  }

  toString(): string {
    return this._value.toString();
  }

  toHexString(): string {
    return '0x' + this._value.toString(16);
  }

  toBinaryString(): string {
    return '0b' + this._value.toString(2);
  }

  toOctalString(): string {
    return '0o' + this._value.toString(8);
  }

  toBigInt(): bigint {
    return this._value;
  }

  toNumber(): number {
    return Number(this._value);
  }

  isSafeNumber(): boolean {
    const num = Number(this._value);
    return Number.isSafeInteger(num);
  }

  static min(a: BigNumber, b: BigNumber): BigNumber {
    return a._value < b._value ? a : b;
  }

  static max(a: BigNumber, b: BigNumber): BigNumber {
    return a._value > b._value ? a : b;
  }

  static gcd(a: BigNumber, b: BigNumber): BigNumber {
    let x = a.abs()._value;
    let y = b.abs()._value;
    while (y !== 0n) {
      [x, y] = [y, x % y];
    }
    return new BigNumber(x);
  }

  static lcm(a: BigNumber, b: BigNumber): BigNumber {
    if (a.isZero() || b.isZero()) {
      return BigNumber.zero();
    }
    return a.multiply(b).abs().divide(BigNumber.gcd(a, b));
  }

  static factorial(n: BigNumber): BigNumber {
    if (n.isNegative()) {
      throw new Error('Factorial is not defined for negative numbers');
    }
    let result = 1n;
    for (let i = 2n; i <= n._value; i++) {
      result *= i;
    }
    return new BigNumber(result);
  }

  static fibonacci(n: BigNumber): BigNumber {
    if (n.isNegative()) {
      throw new Error('Fibonacci is not defined for negative numbers');
    }
    if (n.isZero() || n.equals(new BigNumber(1n))) {
      return n;
    }
    let a = 0n, b = 1n;
    for (let i = 2n; i <= n._value; i++) {
      [a, b] = [b, a + b];
    }
    return new BigNumber(b);
  }

  valueOf(): bigint {
    return this._value;
  }

  [Symbol.toPrimitive](hint: string): bigint | string {
    if (hint === 'string') {
      return this._value.toString();
    }
    return this._value;
  }
}

export class BigDecimal {
  private readonly _integerPart: BigNumber;
  private readonly _scale: number;

  constructor(value: string | number, scale: number = 0) {
    if (typeof value === 'number') {
      value = value.toString();
    }
    
    const parts = value.split('.');
    let integerPart = parts[0];
    let fractionalPart = parts[1] || '';
    
    if (scale >= 0) {
      while (fractionalPart.length < scale) {
        fractionalPart += '0';
      }
      fractionalPart = fractionalPart.substring(0, scale);
    }
    
    const combined = integerPart + fractionalPart;
    this._integerPart = new BigNumber(combined);
    this._scale = scale;
  }

  get integerPart(): BigNumber {
    return this._integerPart;
  }

  get scale(): number {
    return this._scale;
  }

  static zero(scale: number = 0): BigDecimal {
    return new BigDecimal('0', scale);
  }

  static one(scale: number = 0): BigDecimal {
    return new BigDecimal('1', scale);
  }

  add(other: BigDecimal): BigDecimal {
    const maxScale = Math.max(this._scale, other._scale);
    const thisScaled = this._scaleTo(maxScale);
    const otherScaled = other._scaleTo(maxScale);
    const sum = thisScaled.add(otherScaled);
    return BigDecimal._fromScaledInteger(sum, maxScale);
  }

  subtract(other: BigDecimal): BigDecimal {
    const maxScale = Math.max(this._scale, other._scale);
    const thisScaled = this._scaleTo(maxScale);
    const otherScaled = other._scaleTo(maxScale);
    const diff = thisScaled.subtract(otherScaled);
    return BigDecimal._fromScaledInteger(diff, maxScale);
  }

  multiply(other: BigDecimal): BigDecimal {
    const product = this._integerPart.multiply(other._integerPart);
    const newScale = this._scale + other._scale;
    return BigDecimal._fromScaledInteger(product, newScale);
  }

  private static _fromScaledInteger(scaledInteger: BigNumber, scale: number): BigDecimal {
    const result = new BigDecimal('0', scale);
    (result as any)._integerPart = scaledInteger;
    (result as any)._scale = scale;
    return result;
  }

  divide(other: BigDecimal, precision: number = 10): BigDecimal {
    const thisScaled = this._scaleTo(this._scale + precision);
    const quotient = thisScaled.divide(other._integerPart);
    return new BigDecimal(quotient.toString(), this._scale + precision);
  }

  private _scaleTo(newScale: number): BigNumber {
    if (newScale === this._scale) {
      return this._integerPart;
    }
    const scaleDiff = newScale - this._scale;
    if (scaleDiff > 0) {
      return this._integerPart.multiply(new BigNumber(10n ** BigInt(scaleDiff)));
    } else {
      return this._integerPart.divide(new BigNumber(10n ** BigInt(-scaleDiff)));
    }
  }

  toString(): string {
    const str = this._integerPart.toString();
    const sign = str.startsWith('-') ? '-' : '';
    const absStr = str.replace('-', '');
    
    if (this._scale === 0) {
      return str;
    }
    
    let padded = absStr.padStart(this._scale + 1, '0');
    const integerPart = padded.slice(0, -this._scale);
    const fractionalPart = padded.slice(-this._scale);
    
    return sign + integerPart + '.' + fractionalPart;
  }

  toFixed(decimalPlaces: number): string {
    const scaled = this._scaleTo(decimalPlaces);
    const temp = new BigDecimal(scaled.toString(), decimalPlaces);
    return temp.toString();
  }
}
