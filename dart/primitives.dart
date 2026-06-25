import 'dart:math' as math;

class _IntResult {
  final Int value;
  final Object? error;

  _IntResult(this.value, this.error);
}

class Bool {
  final bool value;

  Bool(this.value);

  Bool.trueValue() : this(true);
  Bool.falseValue() : this(false);

  Bool not() => Bool(!value);
  Bool and(Bool other) => Bool(value && other.value);
  Bool or(Bool other) => Bool(value || other.value);
  Bool xor(Bool other) => Bool(value != other.value);
  bool equals(Bool other) => value == other.value;
  String toStringValue() => value.toString();
  bool toBoolean() => value;
}

class Int {
  final int value;

  Int(this.value);

  factory Int.zero() => Int(0);
  factory Int.one() => Int(1);

  Int add(Int other) => Int(value + other.value);
  Int subtract(Int other) => Int(value - other.value);
  Int multiply(Int other) => Int(value * other.value);

  _IntResult divide(Int other) {
    if (other.value == 0) {
      return _IntResult(Int(0), ArgumentError('division by zero'));
    }
    return _IntResult(Int(value ~/ other.value), null);
  }

  _IntResult modulo(Int other) {
    if (other.value == 0) {
      return _IntResult(Int(0), ArgumentError('modulo by zero'));
    }
    return _IntResult(Int(value % other.value), null);
  }

  Int power(Int exponent) => Int(math.pow(value, exponent.value).toInt());
  Int bitwiseAnd(Int other) => Int(value & other.value);
  Int bitwiseOr(Int other) => Int(value | other.value);
  Int bitwiseXor(Int other) => Int(value ^ other.value);
  Int bitwiseNot() => Int(~value);
  Int leftShift(Int shift) => Int(value << shift.value);
  Int rightShift(Int shift) => Int(value >> shift.value);
  Int negate() => Int(-value);
  Int abs() => Int(value.abs());
  bool equals(Int other) => value == other.value;
  bool lessThan(Int other) => value < other.value;
  bool lessThanOrEqual(Int other) => value <= other.value;
  bool greaterThan(Int other) => value > other.value;
  bool greaterThanOrEqual(Int other) => value >= other.value;
  bool isEven() => value % 2 == 0;
  bool isOdd() => value % 2 != 0;
  bool isPositive() => value > 0;
  bool isNegative() => value < 0;
  bool isZero() => value == 0;
  String toStringValue() => value.toString();
  int toNumber() => value;
  static Int min(Int a, Int b) => a.value < b.value ? a : b;
  static Int max(Int a, Int b) => a.value > b.value ? a : b;
}

class Float {
  final double value;

  Float(this.value);

  factory Float.zero() => Float(0);
  factory Float.one() => Float(1);
  factory Float.pi() => Float(math.pi);
  factory Float.e() => Float(math.e);
  factory Float.nan() => Float(double.nan);
  factory Float.positiveInfinity() => Float(double.infinity);
  factory Float.negativeInfinity() => Float(double.negativeInfinity);

  Float add(Float other) => Float(value + other.value);
  Float subtract(Float other) => Float(value - other.value);
  Float multiply(Float other) => Float(value * other.value);
  Float divide(Float other) {
    if (other.value == 0) {
      if (value > 0) return Float.positiveInfinity();
      return Float.negativeInfinity();
    }
    return Float(value / other.value);
  }

  Float power(Float exponent) =>
      Float(math.pow(value, exponent.value).toDouble());
  Float sqrt() => Float(math.sqrt(value));
  Float abs() => Float(value.abs());
  Float negate() => Float(-value);
  Float floor() => Float(value.floorToDouble());
  Float ceil() => Float(value.ceilToDouble());
  Float round() => Float(value.roundToDouble());
  Float trunc() => Float(value.truncateToDouble());
  Float sin() => Float(math.sin(value));
  Float cos() => Float(math.cos(value));
  Float tan() => Float(math.tan(value));
  Float log() => Float(math.log(value));
  Float log10() => Float(math.log(value) / math.ln10);
  Float exp() => Float(math.exp(value));
  bool equals(Float other, [double epsilon = 1e-10]) =>
      (value - other.value).abs() < epsilon;
  bool lessThan(Float other) => value < other.value;
  bool lessThanOrEqual(Float other) => value <= other.value;
  bool greaterThan(Float other) => value > other.value;
  bool greaterThanOrEqual(Float other) => value >= other.value;
  bool isNaN() => value.isNaN;
  bool isInfinity() => value.isInfinite;
  bool isFinite() => value.isFinite;
  bool isPositive() => value > 0;
  bool isNegative() => value < 0;
  bool isZero() => value == 0;
  bool isInteger() => value % 1 == 0;
  String toStringValue() => value.toString();
  String toFixed(int digits) => value.toStringAsFixed(digits);
  double toNumber() => value;
  static Float min(Float a, Float b) => a.value < b.value ? a : b;
  static Float max(Float a, Float b) => a.value > b.value ? a : b;
  static Float clamp(Float value, Float minVal, Float maxVal) {
    return Float(math.min(value.value, math.max(minVal.value, value.value)));
  }

  static Float lerp(Float a, Float b, Float t) {
    return Float(a.value + (b.value - a.value) * t.value);
  }
}

class Str {
  final String value;

  Str(this.value);

  factory Str.empty() => Str('');

  int get length => value.length;
  bool get isEmpty => value.isEmpty;
  String charAt(int index) {
    if (index < 0 || index >= value.length) return '';
    return value[index];
  }

  int charCodeAt(int index) {
    if (index < 0 || index >= value.length) return 0;
    return value.codeUnitAt(index);
  }

  Str concat(Str other) => Str(value + other.value);
  Str substring(int start, [int? end]) {
    int e = end ?? value.length;
    if (start < 0) start = 0;
    if (e > value.length) e = value.length;
    if (start >= e) return Str.empty();
    return Str(value.substring(start, e));
  }

  Str slice(int start, [int? end]) {
    int s = start < 0 ? value.length + start : start;
    int e = end == null ? value.length : (end < 0 ? value.length + end : end);
    if (s < 0) s = 0;
    if (e > value.length) e = value.length;
    if (s >= e) return Str.empty();
    return Str(value.substring(s, e));
  }

  int indexOf(Str searchValue, [int fromIndex = 0]) {
    if (fromIndex < 0) fromIndex = 0;
    int result = value.indexOf(searchValue.value, fromIndex);
    return result;
  }

  int lastIndexOf(Str searchValue, [int? fromIndex]) {
    int f = fromIndex ?? value.length;
    if (f > value.length) f = value.length;
    return value.lastIndexOf(searchValue.value, f);
  }

  bool includes(Str searchValue, [int fromIndex = 0]) =>
      indexOf(searchValue, fromIndex) != -1;
  bool startsWith(Str searchValue, [int position = 0]) {
    int pos = position < 0 ? 0 : position;
    if (pos + searchValue.value.length > value.length) return false;
    return value.startsWith(searchValue.value, pos);
  }

  bool endsWith(Str searchValue, [int? length]) {
    int l = length ?? value.length;
    if (l > value.length) l = value.length;
    if (searchValue.value.length > l) return false;
    return value.endsWith(searchValue.value);
  }

  Str toLowerCase() => Str(value.toLowerCase());
  Str toUpperCase() => Str(value.toUpperCase());
  Str trim() => Str(value.trim());
  Str trimStart() => Str(value.trimLeft());
  Str trimEnd() => Str(value.trimRight());
  Str padStart(int targetLength, [Str? padString]) {
    String pad = padString?.value ?? ' ';
    if (pad.isEmpty) pad = ' ';
    if (value.length >= targetLength) return this;
    String padding = '';
    while (padding.length < targetLength - value.length) {
      padding += pad;
    }
    padding = padding.substring(0, targetLength - value.length);
    return Str(padding + value);
  }

  Str padEnd(int targetLength, [Str? padString]) {
    String pad = padString?.value ?? ' ';
    if (pad.isEmpty) pad = ' ';
    if (value.length >= targetLength) return this;
    String padding = '';
    while (padding.length < targetLength - value.length) {
      padding += pad;
    }
    padding = padding.substring(0, targetLength - value.length);
    return Str(value + padding);
  }

  Str repeat(int count) {
    if (count <= 0) return Str.empty();
    return Str(value * count);
  }

  Str replace(dynamic searchValue, Str replaceValue) {
    if (searchValue is Str) {
      return Str(value.replaceFirst(searchValue.value, replaceValue.value));
    }
    return this;
  }

  Str replaceAll(dynamic searchValue, Str replaceValue) {
    if (searchValue is Str) {
      return Str(value.replaceAll(searchValue.value, replaceValue.value));
    }
    return this;
  }

  List<Str> split(dynamic separator, [int? limit]) {
    List<String> parts = [];
    if (separator is Str) {
      if (limit == null) {
        parts = value.split(separator.value);
      } else {
        parts = value.split(separator.value).take(limit).toList();
      }
    }
    return parts.map((p) => Str(p)).toList();
  }

  bool equals(Str other) => value == other.value;
  bool equalsIgnoreCase(Str other) =>
      value.toLowerCase() == other.value.toLowerCase();
  int compare(Str other) => value.compareTo(other.value);
  bool isWhitespace() => value.trim().isEmpty;
  bool isAlpha() => RegExp(r'^[a-zA-Z]+$').hasMatch(value);
  bool isNumeric() => RegExp(r'^[0-9]+$').hasMatch(value);
  bool isAlphanumeric() => RegExp(r'^[a-zA-Z0-9]+$').hasMatch(value);
  Str reverse() => Str(value.split('').reversed.join(''));
  int countOccurrences(Str substring) {
    int count = 0;
    int pos = 0;
    while (true) {
      int idx = value.indexOf(substring.value, pos);
      if (idx == -1) break;
      count++;
      pos = idx + substring.value.length;
    }
    return count;
  }

  List<Str> words() {
    return value
        .split(RegExp(r'\s+'))
        .where((w) => w.isNotEmpty)
        .map((w) => Str(w))
        .toList();
  }

  List<Str> lines() =>
      value.split(RegExp(r'\r?\n')).map((l) => Str(l)).toList();
  Str capitalize() {
    if (isEmpty) return this;
    return Str(value[0].toUpperCase() + value.substring(1).toLowerCase());
  }

  Str titleCase() {
    return Str(
      value.replaceAllMapped(RegExp(r'\w\S*'), (match) {
        String txt = match.group(0)!;
        if (txt.isEmpty) return txt;
        return txt[0].toUpperCase() + txt.substring(1).toLowerCase();
      }),
    );
  }

  Str camelCase() {
    List<Str> words = this.words();
    if (words.isEmpty) return Str.empty();
    Str first = words[0].toLowerCase();
    List<String> rest = words
        .sublist(1)
        .map((w) => w.capitalize().value)
        .toList();
    return Str(first.value + rest.join(''));
  }

  Str snakeCase() {
    String result = value.replaceAllMapped(
      RegExp(r'([A-Z])'),
      (match) => '_${match.group(1)!}',
    );
    result = result.toLowerCase();
    result = result.startsWith('_') ? result.substring(1) : result;
    result = result.replaceAll(RegExp(r'\s+'), '_');
    return Str(result);
  }

  Str kebabCase() {
    String result = value.replaceAllMapped(
      RegExp(r'([A-Z])'),
      (match) => '-${match.group(1)!}',
    );
    result = result.toLowerCase();
    result = result.startsWith('-') ? result.substring(1) : result;
    result = result.replaceAll(RegExp(r'\s+'), '-');
    return Str(result);
  }

  List<String> toCharArray() => value.split('');
  String toStringValue() => value;
  static Str fromCharArray(List<String> chars) => Str(chars.join(''));
  static Str fromCharCode(List<int> codes) => Str(String.fromCharCodes(codes));
  static Str join(List<Str> strList, [Str? separator]) {
    Str sep = separator ?? Str.empty();
    List<String> parts = strList.map((s) => s.value).toList();
    return Str(parts.join(sep.value));
  }
}

class Null {
  static final Null _instance = Null._internal();
  factory Null.value() => _instance;
  Null._internal();
  bool get isNull => true;
  bool equals(dynamic other) => other == null || other is Null;
  String toStringValue() => 'null';
  dynamic toJson() => null;
}
