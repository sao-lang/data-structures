from typing import Any, Optional, List, Union
import math
import re


class Int:
    def __init__(self, value: int):
        if not isinstance(value, int):
            raise ValueError('Int must be an integer')
        self._value = value

    @property
    def value(self) -> int:
        return self._value

    @staticmethod
    def zero() -> 'Int':
        return Int(0)

    @staticmethod
    def one() -> 'Int':
        return Int(1)

    def add(self, other: 'Int') -> 'Int':
        return Int(self._value + other._value)

    def subtract(self, other: 'Int') -> 'Int':
        return Int(self._value - other._value)

    def multiply(self, other: 'Int') -> 'Int':
        return Int(self._value * other._value)

    def divide(self, other: 'Int') -> 'Int':
        if other._value == 0:
            raise ValueError('Division by zero')
        return Int(math.floor(self._value / other._value))

    def modulo(self, other: 'Int') -> 'Int':
        if other._value == 0:
            raise ValueError('Modulo by zero')
        return Int(self._value % other._value)

    def power(self, exponent: 'Int') -> 'Int':
        return Int(int(math.pow(self._value, exponent._value)))

    def bitwise_and(self, other: 'Int') -> 'Int':
        return Int(self._value & other._value)

    def bitwise_or(self, other: 'Int') -> 'Int':
        return Int(self._value | other._value)

    def bitwise_xor(self, other: 'Int') -> 'Int':
        return Int(self._value ^ other._value)

    def bitwise_not(self) -> 'Int':
        return Int(~self._value)

    def left_shift(self, shift: 'Int') -> 'Int':
        return Int(self._value << shift._value)

    def right_shift(self, shift: 'Int') -> 'Int':
        return Int(self._value >> shift._value)

    def unsigned_right_shift(self, shift: 'Int') -> 'Int':
        return Int(self._value >> shift._value)

    def negate(self) -> 'Int':
        return Int(-self._value)

    def abs(self) -> 'Int':
        return Int(abs(self._value))

    def equals(self, other: 'Int') -> bool:
        return self._value == other._value

    def less_than(self, other: 'Int') -> bool:
        return self._value < other._value

    def less_than_or_equal(self, other: 'Int') -> bool:
        return self._value <= other._value

    def greater_than(self, other: 'Int') -> bool:
        return self._value > other._value

    def greater_than_or_equal(self, other: 'Int') -> bool:
        return self._value >= other._value

    def is_even(self) -> bool:
        return self._value % 2 == 0

    def is_odd(self) -> bool:
        return self._value % 2 != 0

    def is_positive(self) -> bool:
        return self._value > 0

    def is_negative(self) -> bool:
        return self._value < 0

    def is_zero(self) -> bool:
        return self._value == 0

    def to_string(self) -> str:
        return str(self._value)

    def to_number(self) -> int:
        return self._value

    @staticmethod
    def min(a: 'Int', b: 'Int') -> 'Int':
        return Int(min(a._value, b._value))

    @staticmethod
    def max(a: 'Int', b: 'Int') -> 'Int':
        return Int(max(a._value, b._value))

    def __str__(self) -> str:
        return self.to_string()

    def __repr__(self) -> str:
        return f"Int({self._value})"

    # 算术运算符
    def __add__(self, other: 'Int') -> 'Int':
        return self.add(other)

    def __sub__(self, other: 'Int') -> 'Int':
        return self.subtract(other)

    def __mul__(self, other: 'Int') -> 'Int':
        return self.multiply(other)

    def __truediv__(self, other: 'Int') -> 'Int':
        return self.divide(other)

    def __floordiv__(self, other: 'Int') -> 'Int':
        return self.divide(other)

    def __mod__(self, other: 'Int') -> 'Int':
        return self.modulo(other)

    def __pow__(self, other: 'Int') -> 'Int':
        return self.power(other)

    # 位运算符
    def __and__(self, other: 'Int') -> 'Int':
        return self.bitwise_and(other)

    def __or__(self, other: 'Int') -> 'Int':
        return self.bitwise_or(other)

    def __xor__(self, other: 'Int') -> 'Int':
        return self.bitwise_xor(other)

    def __invert__(self) -> 'Int':
        return self.bitwise_not()

    def __lshift__(self, other: 'Int') -> 'Int':
        return self.left_shift(other)

    def __rshift__(self, other: 'Int') -> 'Int':
        return self.right_shift(other)

    # 比较运算符
    def __eq__(self, other: 'Int') -> bool:
        return self.equals(other)

    def __ne__(self, other: 'Int') -> bool:
        return not self.equals(other)

    def __lt__(self, other: 'Int') -> bool:
        return self.less_than(other)

    def __le__(self, other: 'Int') -> bool:
        return self.less_than_or_equal(other)

    def __gt__(self, other: 'Int') -> bool:
        return self.greater_than(other)

    def __ge__(self, other: 'Int') -> bool:
        return self.greater_than_or_equal(other)

    # 一元运算符
    def __pos__(self) -> 'Int':
        return Int(self._value)

    def __neg__(self) -> 'Int':
        return self.negate()

    # 其他
    def __bool__(self) -> bool:
        return not self.is_zero()

    def __int__(self) -> int:
        return self._value

    def __float__(self) -> float:
        return float(self._value)


class Float:
    def __init__(self, value: float):
        self._value = value

    @property
    def value(self) -> float:
        return self._value

    @staticmethod
    def zero() -> 'Float':
        return Float(0.0)

    @staticmethod
    def one() -> 'Float':
        return Float(1.0)

    @staticmethod
    def pi() -> 'Float':
        return Float(math.pi)

    @staticmethod
    def e() -> 'Float':
        return Float(math.e)

    @staticmethod
    def nan() -> 'Float':
        return Float(float('nan'))

    @staticmethod
    def positive_infinity() -> 'Float':
        return Float(float('inf'))

    @staticmethod
    def negative_infinity() -> 'Float':
        return Float(float('-inf'))

    def add(self, other: 'Float') -> 'Float':
        return Float(self._value + other._value)

    def subtract(self, other: 'Float') -> 'Float':
        return Float(self._value - other._value)

    def multiply(self, other: 'Float') -> 'Float':
        return Float(self._value * other._value)

    def divide(self, other: 'Float') -> 'Float':
        if other._value == 0:
            return Float.positive_infinity() if self._value > 0 else Float.negative_infinity()
        return Float(self._value / other._value)

    def power(self, exponent: 'Float') -> 'Float':
        return Float(math.pow(self._value, exponent._value))

    def sqrt(self) -> 'Float':
        return Float(math.sqrt(self._value))

    def abs(self) -> 'Float':
        return Float(abs(self._value))

    def negate(self) -> 'Float':
        return Float(-self._value)

    def floor(self) -> 'Float':
        return Float(math.floor(self._value))

    def ceil(self) -> 'Float':
        return Float(math.ceil(self._value))

    def round(self) -> 'Float':
        return Float(round(self._value))

    def trunc(self) -> 'Float':
        return Float(math.trunc(self._value))

    def sin(self) -> 'Float':
        return Float(math.sin(self._value))

    def cos(self) -> 'Float':
        return Float(math.cos(self._value))

    def tan(self) -> 'Float':
        return Float(math.tan(self._value))

    def log(self) -> 'Float':
        return Float(math.log(self._value))

    def log10(self) -> 'Float':
        return Float(math.log10(self._value))

    def exp(self) -> 'Float':
        return Float(math.exp(self._value))

    def equals(self, other: 'Float', epsilon: float = 1e-10) -> bool:
        return abs(self._value - other._value) < epsilon

    def less_than(self, other: 'Float') -> bool:
        return self._value < other._value

    def less_than_or_equal(self, other: 'Float') -> bool:
        return self._value <= other._value

    def greater_than(self, other: 'Float') -> bool:
        return self._value > other._value

    def greater_than_or_equal(self, other: 'Float') -> bool:
        return self._value >= other._value

    def is_nan(self) -> bool:
        return math.isnan(self._value)

    def is_infinity(self) -> bool:
        return not math.isfinite(self._value) and not self.is_nan()

    def is_finite(self) -> bool:
        return math.isfinite(self._value)

    def is_positive(self) -> bool:
        return self._value > 0

    def is_negative(self) -> bool:
        return self._value < 0

    def is_zero(self) -> bool:
        return self._value == 0

    def is_integer(self) -> bool:
        return self._value.is_integer()

    def to_string(self) -> str:
        return str(self._value)

    def to_fixed(self, digits: int) -> str:
        return f"{self._value:.{digits}f}"

    def to_exponential(self, fraction_digits: Optional[int] = None) -> str:
        if fraction_digits is not None:
            return f"{self._value:.{fraction_digits}e}"
        return f"{self._value:e}"

    def to_precision(self, precision: Optional[int] = None) -> str:
        if precision is not None:
            return f"{self._value:.{precision}g}"
        return str(self._value)

    def to_number(self) -> float:
        return self._value

    @staticmethod
    def min(a: 'Float', b: 'Float') -> 'Float':
        return Float(min(a._value, b._value))

    @staticmethod
    def max(a: 'Float', b: 'Float') -> 'Float':
        return Float(max(a._value, b._value))

    @staticmethod
    def clamp(value: 'Float', min_val: 'Float', max_val: 'Float') -> 'Float':
        return Float(min(max(value._value, min_val._value), max_val._value))

    @staticmethod
    def lerp(a: 'Float', b: 'Float', t: 'Float') -> 'Float':
        return Float(a._value + (b._value - a._value) * t._value)

    def __str__(self) -> str:
        return self.to_string()

    def __repr__(self) -> str:
        return f"Float({self._value})"

    # 算术运算符
    def __add__(self, other: 'Float') -> 'Float':
        return self.add(other)

    def __sub__(self, other: 'Float') -> 'Float':
        return self.subtract(other)

    def __mul__(self, other: 'Float') -> 'Float':
        return self.multiply(other)

    def __truediv__(self, other: 'Float') -> 'Float':
        return self.divide(other)

    def __pow__(self, other: 'Float') -> 'Float':
        return self.power(other)

    # 比较运算符
    def __eq__(self, other: 'Float') -> bool:
        return self.equals(other)

    def __ne__(self, other: 'Float') -> bool:
        return not self.equals(other)

    def __lt__(self, other: 'Float') -> bool:
        return self.less_than(other)

    def __le__(self, other: 'Float') -> bool:
        return self.less_than_or_equal(other)

    def __gt__(self, other: 'Float') -> bool:
        return self.greater_than(other)

    def __ge__(self, other: 'Float') -> bool:
        return self.greater_than_or_equal(other)

    # 一元运算符
    def __pos__(self) -> 'Float':
        return Float(self._value)

    def __neg__(self) -> 'Float':
        return self.negate()

    # 其他
    def __bool__(self) -> bool:
        return not self.is_zero()

    def __int__(self) -> int:
        return int(self._value)

    def __float__(self) -> float:
        return self._value


class Bool:
    def __init__(self, value: bool):
        self._value = value

    @property
    def value(self) -> bool:
        return self._value

    @staticmethod
    def true() -> 'Bool':
        return Bool(True)

    @staticmethod
    def false() -> 'Bool':
        return Bool(False)

    def not_(self) -> 'Bool':
        return Bool(not self._value)

    def and_(self, other: 'Bool') -> 'Bool':
        return Bool(self._value and other._value)

    def or_(self, other: 'Bool') -> 'Bool':
        return Bool(self._value or other._value)

    def xor(self, other: 'Bool') -> 'Bool':
        return Bool(self._value != other._value)

    def equals(self, other: 'Bool') -> bool:
        return self._value == other._value

    def to_string(self) -> str:
        return str(self._value).lower()

    def to_boolean(self) -> bool:
        return self._value

    def __str__(self) -> str:
        return self.to_string()

    def __repr__(self) -> str:
        return f"Bool({self._value})"

    # 逻辑运算符
    def __and__(self, other: 'Bool') -> 'Bool':
        return self.and_(other)

    def __or__(self, other: 'Bool') -> 'Bool':
        return self.or_(other)

    def __xor__(self, other: 'Bool') -> 'Bool':
        return self.xor(other)

    def __not__(self) -> 'Bool':
        return self.not_()

    # 比较运算符
    def __eq__(self, other: 'Bool') -> bool:
        return self.equals(other)

    def __ne__(self, other: 'Bool') -> bool:
        return not self.equals(other)

    # 其他
    def __bool__(self) -> bool:
        return self._value

    def __int__(self) -> int:
        return 1 if self._value else 0

    def __float__(self) -> float:
        return 1.0 if self._value else 0.0


class Str:
    def __init__(self, value: str):
        self._value = value

    @property
    def value(self) -> str:
        return self._value

    @staticmethod
    def empty() -> 'Str':
        return Str('')

    @property
    def length(self) -> int:
        return len(self._value)

    def is_empty(self) -> bool:
        return len(self._value) == 0

    def char_at(self, index: int) -> str:
        return self._value[index] if 0 <= index < len(self._value) else ''

    def char_code_at(self, index: int) -> int:
        return ord(self._value[index]) if 0 <= index < len(self._value) else 0

    def concat(self, other: 'Str') -> 'Str':
        return Str(self._value + other._value)

    def substring(self, start: int, end: Optional[int] = None) -> 'Str':
        return Str(self._value[start:end])

    def slice(self, start: int, end: Optional[int] = None) -> 'Str':
        return Str(self._value[start:end])

    def index_of(self, search_value: 'Str', from_index: Optional[int] = None) -> int:
        return self._value.find(search_value._value, from_index)

    def last_index_of(self, search_value: 'Str', from_index: Optional[int] = None) -> int:
        return self._value.rfind(search_value._value, 0, from_index)

    def includes(self, search_value: 'Str', from_index: Optional[int] = None) -> bool:
        return search_value._value in self._value[from_index:]

    def starts_with(self, search_value: 'Str', position: Optional[int] = None) -> bool:
        return self._value.startswith(search_value._value, position)

    def ends_with(self, search_value: 'Str', length: Optional[int] = None) -> bool:
        return self._value.endswith(search_value._value, 0, length)

    def to_lower_case(self) -> 'Str':
        return Str(self._value.lower())

    def to_upper_case(self) -> 'Str':
        return Str(self._value.upper())

    def trim(self) -> 'Str':
        return Str(self._value.strip())

    def trim_start(self) -> 'Str':
        return Str(self._value.lstrip())

    def trim_end(self) -> 'Str':
        return Str(self._value.rstrip())

    def pad_start(self, target_length: int, pad_string: Optional['Str'] = None) -> 'Str':
        pad = pad_string._value if pad_string else ' '
        return Str(self._value.rjust(target_length, pad))

    def pad_end(self, target_length: int, pad_string: Optional['Str'] = None) -> 'Str':
        pad = pad_string._value if pad_string else ' '
        return Str(self._value.ljust(target_length, pad))

    def repeat(self, count: int) -> 'Str':
        return Str(self._value * count)

    def replace(self, search_value: Union['Str', Any], replace_value: 'Str') -> 'Str':
        if isinstance(search_value, Str):
            return Str(self._value.replace(search_value._value, replace_value._value, 1))
        return Str(re.sub(str(search_value), replace_value._value, self._value, 1))

    def replace_all(self, search_value: Union['Str', Any], replace_value: 'Str') -> 'Str':
        if isinstance(search_value, Str):
            return Str(self._value.replace(search_value._value, replace_value._value))
        return Str(re.sub(str(search_value), replace_value._value, self._value))

    def split(self, separator: Union['Str', Any], limit: Optional[int] = None) -> List['Str']:
        if isinstance(separator, Str):
            parts = self._value.split(separator._value, limit)
        else:
            parts = re.split(str(separator), self._value, limit)
        return [Str(part) for part in parts]

    def equals(self, other: 'Str') -> bool:
        return self._value == other._value

    def equals_ignore_case(self, other: 'Str') -> bool:
        return self._value.lower() == other._value.lower()

    def compare(self, other: 'Str') -> int:
        if self._value < other._value:
            return -1
        elif self._value > other._value:
            return 1
        return 0

    def is_whitespace(self) -> bool:
        return bool(re.match(r'^\s*$', self._value))

    def is_alpha(self) -> bool:
        return bool(re.match(r'^[a-zA-Z]+$', self._value))

    def is_numeric(self) -> bool:
        return bool(re.match(r'^[0-9]+$', self._value))

    def is_alphanumeric(self) -> bool:
        return bool(re.match(r'^[a-zA-Z0-9]+$', self._value))

    def reverse(self) -> 'Str':
        return Str(self._value[::-1])

    def count_occurrences(self, substring: 'Str') -> int:
        count = 0
        pos = 0
        while True:
            pos = self._value.find(substring._value, pos)
            if pos == -1:
                break
            count += 1
            pos += len(substring._value)
        return count

    def words(self) -> List['Str']:
        word_list = re.split(r'\s+', self._value)
        return [Str(word) for word in word_list if word]

    def lines(self) -> List['Str']:
        line_list = self._value.splitlines()
        return [Str(line) for line in line_list]

    def capitalize(self) -> 'Str':
        if not self._value:
            return self
        return Str(self._value[0].upper() + self._value[1:].lower())

    def title_case(self) -> 'Str':
        return Str(self._value.title())

    def camel_case(self) -> 'Str':
        word_list = self.words()
        if not word_list:
            return Str.empty()
        first = word_list[0].to_lower_case()
        rest = [word.capitalize() for word in word_list[1:]]
        return Str(first._value + ''.join(word._value for word in rest))

    def snake_case(self) -> 'Str':
        s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', self._value)
        s2 = re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()
        s3 = re.sub(r'\s+', '_', s2)
        return Str(s3.strip('_'))

    def kebab_case(self) -> 'Str':
        s1 = re.sub('(.)([A-Z][a-z]+)', r'\1-\2', self._value)
        s2 = re.sub('([a-z0-9])([A-Z])', r'\1-\2', s1).lower()
        s3 = re.sub(r'\s+', '-', s2)
        return Str(s3.strip('-'))

    def to_char_array(self) -> List[str]:
        return list(self._value)

    def to_string(self) -> str:
        return self._value

    @staticmethod
    def from_char_array(chars: List[str]) -> 'Str':
        return Str(''.join(chars))

    @staticmethod
    def from_char_code(*codes: int) -> 'Str':
        return Str(''.join(chr(code) for code in codes))

    @staticmethod
    def join(strings: List['Str'], separator: 'Str' = None) -> 'Str':
        sep = separator._value if separator else ''
        return Str(sep.join(s._value for s in strings))

    def __str__(self) -> str:
        return self.to_string()

    def __repr__(self) -> str:
        return f"Str({repr(self._value)})"

    # 字符串运算符
    def __add__(self, other: 'Str') -> 'Str':
        return self.concat(other)

    def __radd__(self, other: str) -> 'Str':
        return Str(other + self._value)

    def __mul__(self, times: int) -> 'Str':
        return self.repeat(times)

    def __rmul__(self, times: int) -> 'Str':
        return self.repeat(times)

    # 比较运算符
    def __eq__(self, other: 'Str') -> bool:
        return self.equals(other)

    def __ne__(self, other: 'Str') -> bool:
        return not self.equals(other)

    def __lt__(self, other: 'Str') -> bool:
        return self.compare(other) < 0

    def __le__(self, other: 'Str') -> bool:
        return self.compare(other) <= 0

    def __gt__(self, other: 'Str') -> bool:
        return self.compare(other) > 0

    def __ge__(self, other: 'Str') -> bool:
        return self.compare(other) >= 0

    # 索引访问
    def __getitem__(self, index: int) -> str:
        return self.char_at(index)

    # 包含检查
    def __contains__(self, substring: 'Str') -> bool:
        return self.includes(substring)

    # 长度
    def __len__(self) -> int:
        return self.length

    # 迭代
    def __iter__(self):
        for char in self._value:
            yield char

    # 其他
    def __bool__(self) -> bool:
        return not self.is_empty()


class Null:
    _instance = None

    def __new__(cls):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance

    def is_null(self) -> bool:
        return True

    def equals(self, other: Any) -> bool:
        return other is None or isinstance(other, Null)

    def to_string(self) -> str:
        return 'null'

    def to_json(self) -> None:
        return None

    def __str__(self) -> str:
        return self.to_string()

    def __repr__(self) -> str:
        return 'null'


null_value = Null()


class BigNumber:
    def __init__(self, value: Union[str, int, 'BigNumber']):
        if isinstance(value, BigNumber):
            self._value = value._value
        elif isinstance(value, str):
            self._value = int(value)
        elif isinstance(value, int):
            self._value = value
        else:
            raise ValueError('BigNumber must be initialized with string, int, or BigNumber')

    @property
    def value(self) -> int:
        return self._value

    @staticmethod
    def zero() -> 'BigNumber':
        return BigNumber(0)

    @staticmethod
    def one() -> 'BigNumber':
        return BigNumber(1)

    @staticmethod
    def from_string(value: str) -> 'BigNumber':
        return BigNumber(value)

    def add(self, other: 'BigNumber') -> 'BigNumber':
        return BigNumber(self._value + other._value)

    def subtract(self, other: 'BigNumber') -> 'BigNumber':
        return BigNumber(self._value - other._value)

    def multiply(self, other: 'BigNumber') -> 'BigNumber':
        return BigNumber(self._value * other._value)

    def divide(self, other: 'BigNumber') -> 'BigNumber':
        if other._value == 0:
            raise ValueError('Division by zero')
        return BigNumber(self._value // other._value)

    def modulo(self, other: 'BigNumber') -> 'BigNumber':
        if other._value == 0:
            raise ValueError('Modulo by zero')
        return BigNumber(self._value % other._value)

    def power(self, exponent: 'BigNumber') -> 'BigNumber':
        if exponent._value < 0:
            raise ValueError('Exponent must be non-negative')
        return BigNumber(self._value ** exponent._value)

    def bitwise_and(self, other: 'BigNumber') -> 'BigNumber':
        return BigNumber(self._value & other._value)

    def bitwise_or(self, other: 'BigNumber') -> 'BigNumber':
        return BigNumber(self._value | other._value)

    def bitwise_xor(self, other: 'BigNumber') -> 'BigNumber':
        return BigNumber(self._value ^ other._value)

    def bitwise_not(self) -> 'BigNumber':
        return BigNumber(~self._value)

    def left_shift(self, shift: 'BigNumber') -> 'BigNumber':
        return BigNumber(self._value << shift._value)

    def right_shift(self, shift: 'BigNumber') -> 'BigNumber':
        return BigNumber(self._value >> shift._value)

    def negate(self) -> 'BigNumber':
        return BigNumber(-self._value)

    def abs(self) -> 'BigNumber':
        return BigNumber(abs(self._value))

    def equals(self, other: 'BigNumber') -> bool:
        return self._value == other._value

    def less_than(self, other: 'BigNumber') -> bool:
        return self._value < other._value

    def less_than_or_equal(self, other: 'BigNumber') -> bool:
        return self._value <= other._value

    def greater_than(self, other: 'BigNumber') -> bool:
        return self._value > other._value

    def greater_than_or_equal(self, other: 'BigNumber') -> bool:
        return self._value >= other._value

    def is_even(self) -> bool:
        return self._value % 2 == 0

    def is_odd(self) -> bool:
        return self._value % 2 != 0

    def is_positive(self) -> bool:
        return self._value > 0

    def is_negative(self) -> bool:
        return self._value < 0

    def is_zero(self) -> bool:
        return self._value == 0

    def is_one(self) -> bool:
        return self._value == 1

    def sign(self) -> int:
        if self._value > 0:
            return 1
        elif self._value < 0:
            return -1
        return 0

    def to_string(self) -> str:
        return str(self._value)

    def to_hex_string(self) -> str:
        return '0x' + hex(self._value)[2:]

    def to_binary_string(self) -> str:
        return '0b' + bin(self._value)[2:]

    def to_octal_string(self) -> str:
        return '0o' + oct(self._value)[2:]

    def to_big_int(self) -> int:
        return self._value

    def to_number(self) -> float:
        return float(self._value)

    def is_safe_number(self) -> bool:
        return abs(self._value) <= (2 ** 53 - 1)

    @staticmethod
    def min(a: 'BigNumber', b: 'BigNumber') -> 'BigNumber':
        return a if a._value < b._value else b

    @staticmethod
    def max(a: 'BigNumber', b: 'BigNumber') -> 'BigNumber':
        return a if a._value > b._value else b

    @staticmethod
    def gcd(a: 'BigNumber', b: 'BigNumber') -> 'BigNumber':
        x = a.abs()._value
        y = b.abs()._value
        while y != 0:
            x, y = y, x % y
        return BigNumber(x)

    @staticmethod
    def lcm(a: 'BigNumber', b: 'BigNumber') -> 'BigNumber':
        if a.is_zero() or b.is_zero():
            return BigNumber.zero()
        return a.multiply(b).abs().divide(BigNumber.gcd(a, b))

    @staticmethod
    def factorial(n: 'BigNumber') -> 'BigNumber':
        if n.is_negative():
            raise ValueError('Factorial is not defined for negative numbers')
        result = 1
        for i in range(2, n._value + 1):
            result *= i
        return BigNumber(result)

    @staticmethod
    def fibonacci(n: 'BigNumber') -> 'BigNumber':
        if n.is_negative():
            raise ValueError('Fibonacci is not defined for negative numbers')
        if n.is_zero() or n.equals(BigNumber(1)):
            return n
        a, b = 0, 1
        for _ in range(2, n._value + 1):
            a, b = b, a + b
        return BigNumber(b)

    def __str__(self) -> str:
        return self.to_string()

    def __repr__(self) -> str:
        return f"BigNumber({self._value})"

    # 算术运算符
    def __add__(self, other: 'BigNumber') -> 'BigNumber':
        return self.add(other)

    def __sub__(self, other: 'BigNumber') -> 'BigNumber':
        return self.subtract(other)

    def __mul__(self, other: 'BigNumber') -> 'BigNumber':
        return self.multiply(other)

    def __truediv__(self, other: 'BigNumber') -> 'BigNumber':
        return self.divide(other)

    def __floordiv__(self, other: 'BigNumber') -> 'BigNumber':
        return self.divide(other)

    def __mod__(self, other: 'BigNumber') -> 'BigNumber':
        return self.modulo(other)

    def __pow__(self, other: 'BigNumber') -> 'BigNumber':
        return self.power(other)

    # 位运算符
    def __and__(self, other: 'BigNumber') -> 'BigNumber':
        return self.bitwise_and(other)

    def __or__(self, other: 'BigNumber') -> 'BigNumber':
        return self.bitwise_or(other)

    def __xor__(self, other: 'BigNumber') -> 'BigNumber':
        return self.bitwise_xor(other)

    def __invert__(self) -> 'BigNumber':
        return self.bitwise_not()

    def __lshift__(self, other: 'BigNumber') -> 'BigNumber':
        return self.left_shift(other)

    def __rshift__(self, other: 'BigNumber') -> 'BigNumber':
        return self.right_shift(other)

    # 比较运算符
    def __eq__(self, other: 'BigNumber') -> bool:
        return self.equals(other)

    def __ne__(self, other: 'BigNumber') -> bool:
        return not self.equals(other)

    def __lt__(self, other: 'BigNumber') -> bool:
        return self.less_than(other)

    def __le__(self, other: 'BigNumber') -> bool:
        return self.less_than_or_equal(other)

    def __gt__(self, other: 'BigNumber') -> bool:
        return self.greater_than(other)

    def __ge__(self, other: 'BigNumber') -> bool:
        return self.greater_than_or_equal(other)

    # 一元运算符
    def __pos__(self) -> 'BigNumber':
        return BigNumber(self._value)

    def __neg__(self) -> 'BigNumber':
        return self.negate()

    # 其他
    def __bool__(self) -> bool:
        return not self.is_zero()

    def __int__(self) -> int:
        return int(self._value)

    def __float__(self) -> float:
        return float(self._value)


class BigDecimal:
    def __init__(self, value: Union[str, float, int], scale: int = 0):
        if isinstance(value, (float, int)):
            value = str(value)
        
        parts = value.split('.')
        integer_part = parts[0]
        fractional_part = parts[1] if len(parts) > 1 else ''
        
        if scale >= 0:
            while len(fractional_part) < scale:
                fractional_part += '0'
            fractional_part = fractional_part[:scale]
        
        combined = integer_part + fractional_part
        self._integer_part = BigNumber(combined)
        self._scale = scale

    @property
    def integer_part(self) -> BigNumber:
        return self._integer_part

    @property
    def scale(self) -> int:
        return self._scale

    @staticmethod
    def zero(scale: int = 0) -> 'BigDecimal':
        return BigDecimal('0', scale)

    @staticmethod
    def one(scale: int = 0) -> 'BigDecimal':
        return BigDecimal('1', scale)

    def add(self, other: 'BigDecimal') -> 'BigDecimal':
        max_scale = max(self._scale, other._scale)
        this_scaled = self._scale_to(max_scale)
        other_scaled = other._scale_to(max_scale)
        sum_ = this_scaled.add(other_scaled)
        return BigDecimal._from_scaled_integer(sum_, max_scale)

    def subtract(self, other: 'BigDecimal') -> 'BigDecimal':
        max_scale = max(self._scale, other._scale)
        this_scaled = self._scale_to(max_scale)
        other_scaled = other._scale_to(max_scale)
        diff = this_scaled.subtract(other_scaled)
        return BigDecimal._from_scaled_integer(diff, max_scale)

    def multiply(self, other: 'BigDecimal') -> 'BigDecimal':
        product = self._integer_part.multiply(other._integer_part)
        new_scale = self._scale + other._scale
        return BigDecimal._from_scaled_integer(product, new_scale)

    @staticmethod
    def _from_scaled_integer(scaled_integer: BigNumber, scale: int) -> 'BigDecimal':
        result = BigDecimal('0', scale)
        result._integer_part = scaled_integer
        result._scale = scale
        return result

    def divide(self, other: 'BigDecimal', precision: int = 10) -> 'BigDecimal':
        this_scaled = self._scale_to(self._scale + precision + other._scale)
        other_scaled = other._scale_to(other._scale)
        quotient = this_scaled.divide(other_scaled)
        return BigDecimal._from_scaled_integer(quotient, self._scale + precision)

    def _scale_to(self, new_scale: int) -> BigNumber:
        if new_scale == self._scale:
            return self._integer_part
        scale_diff = new_scale - self._scale
        if scale_diff > 0:
            return self._integer_part.multiply(BigNumber(10 ** scale_diff))
        else:
            return self._integer_part.divide(BigNumber(10 ** (-scale_diff)))

    def equals(self, other: 'BigDecimal') -> bool:
        max_scale = max(self._scale, other._scale)
        this_scaled = self._scale_to(max_scale)
        other_scaled = other._scale_to(max_scale)
        return this_scaled.equals(other_scaled)

    def compare(self, other: 'BigDecimal') -> int:
        max_scale = max(self._scale, other._scale)
        this_scaled = self._scale_to(max_scale)
        other_scaled = other._scale_to(max_scale)
        if this_scaled.less_than(other_scaled):
            return -1
        elif this_scaled.greater_than(other_scaled):
            return 1
        else:
            return 0

    def is_zero(self) -> bool:
        return self._integer_part.is_zero()

    def to_string(self) -> str:
        str_val = self._integer_part.to_string()
        sign = '-' if str_val.startswith('-') else ''
        abs_str = str_val.replace('-', '')
        
        if self._scale == 0:
            return str_val
        
        padded = abs_str.zfill(self._scale + 1)
        integer_part = padded[:-self._scale]
        fractional_part = padded[-self._scale:]
        
        return sign + integer_part + '.' + fractional_part

    def to_fixed(self, decimal_places: int) -> str:
        temp = BigDecimal(self.to_string(), decimal_places)
        return temp.to_string()

    def __str__(self) -> str:
        return self.to_string()

    def __repr__(self) -> str:
        return f"BigDecimal({self.to_string()})"

    # 算术运算符
    def __add__(self, other: 'BigDecimal') -> 'BigDecimal':
        return self.add(other)

    def __sub__(self, other: 'BigDecimal') -> 'BigDecimal':
        return self.subtract(other)

    def __mul__(self, other: 'BigDecimal') -> 'BigDecimal':
        return self.multiply(other)

    def __truediv__(self, other: 'BigDecimal') -> 'BigDecimal':
        return self.divide(other)

    # 比较运算符
    def __eq__(self, other: 'BigDecimal') -> bool:
        return self.equals(other)

    def __ne__(self, other: 'BigDecimal') -> bool:
        return not self.equals(other)

    def __lt__(self, other: 'BigDecimal') -> bool:
        return self.compare(other) < 0

    def __le__(self, other: 'BigDecimal') -> bool:
        return self.compare(other) <= 0

    def __gt__(self, other: 'BigDecimal') -> bool:
        return self.compare(other) > 0

    def __ge__(self, other: 'BigDecimal') -> bool:
        return self.compare(other) >= 0

    # 其他
    def __bool__(self) -> bool:
        return not self.is_zero()

    def __float__(self) -> float:
        return float(self.to_string())


if __name__ == "__main__":
    print("=== Null Example ===")
    n = null_value
    print(f"null: {n}")
    print(f"null.is_null(): {n.is_null()}")
    print(f"null.equals(null): {n.equals(null_value)}")
    print()

    print("=== Int Example ===")
    a = Int(10)
    b = Int(20)
    print(f"a = {a}, b = {b}")
    print(f"a.add(b) = {a.add(b)}")
    print(f"a.subtract(b) = {a.subtract(b)}")
    print(f"a.multiply(b) = {a.multiply(b)}")
    print(f"a.divide(b) = {a.divide(b)}")
    print(f"a.modulo(b) = {a.modulo(b)}")
    print(f"a.power(Int(3)) = {a.power(Int(3))}")
    print(f"a.negate() = {a.negate()}")
    print(f"a.abs() = {a.abs()}")
    print(f"a.less_than(b): {a.less_than(b)}")
    print(f"a.equals(Int(10)): {a.equals(Int(10))}")
    print(f"a.is_even(): {a.is_even()}")
    print(f"a.is_odd(): {a.is_odd()}")
    print(f"a.is_positive(): {a.is_positive()}")
    print(f"Int.min(a, b) = {Int.min(a, b)}")
    print(f"Int.max(a, b) = {Int.max(a, b)}")
    # 运算符重载测试
    print(f"a + b = {a + b}")
    print(f"a - b = {a - b}")
    print(f"a * b = {a * b}")
    print(f"a / b = {a / b}")
    print(f"a % b = {a % b}")
    print(f"a ** Int(3) = {a ** Int(3)}")
    print(f"-a = {-a}")
    print(f"a < b: {a < b}")
    print(f"a == b: {a == b}")
    print(f"a != b: {a != b}")
    print()

    print("=== Float Example ===")
    x = Float(3.14)
    y = Float(2.5)
    print(f"x = {x}, y = {y}")
    print(f"x.add(y) = {x.add(y)}")
    print(f"x.subtract(y) = {x.subtract(y)}")
    print(f"x.multiply(y) = {x.multiply(y)}")
    print(f"x.divide(y) = {x.divide(y)}")
    print(f"x.power(y) = {x.power(y)}")
    print(f"x.sqrt() = {x.sqrt()}")
    print(f"x.sin() = {x.sin()}")
    print(f"x.cos() = {x.cos()}")
    print(f"x.floor() = {x.floor()}")
    print(f"x.ceil() = {x.ceil()}")
    print(f"x.round() = {x.round()}")
    print(f"x.is_positive(): {x.is_positive()}")
    print(f"Float.pi() = {Float.pi()}")
    print(f"Float.e() = {Float.e()}")
    print(f"Float.clamp(x, Float(0), Float(3)) = {Float.clamp(x, Float(0), Float(3))}")
    # 运算符重载测试
    print(f"x + y = {x + y}")
    print(f"x - y = {x - y}")
    print(f"x * y = {x * y}")
    print(f"x / y = {x / y}")
    print(f"x ** y = {x ** y}")
    print(f"-x = {-x}")
    print(f"x < y: {x < y}")
    print(f"x == y: {x == y}")
    print(f"x != y: {x != y}")
    print()

    print("=== Bool Example ===")
    t = Bool.true()
    f = Bool.false()
    print(f"t = {t}, f = {f}")
    print(f"t.and_(f) = {t.and_(f)}")
    print(f"t.or_(f) = {t.or_(f)}")
    print(f"t.xor(t) = {t.xor(t)}")
    print(f"t.not_() = {t.not_()}")
    print(f"t.equals(f): {t.equals(f)}")
    # 运算符重载测试
    print(f"t and f = {t and f}")
    print(f"t or f = {t or f}")
    print(f"t ^ f = {t ^ f}")
    print(f"not t = {not t}")
    print(f"t == f: {t == f}")
    print(f"t != f: {t != f}")
    print()

    print("=== Str Example ===")
    s1 = Str("Hello")
    s2 = Str("World")
    print(f"s1 = {s1}, s2 = {s2}")
    print(f"s1.concat(s2) = {s1.concat(s2)}")
    print(f"s1.length = {s1.length}")
    print(f"s1.char_at(0) = {s1.char_at(0)}")
    print(f"s1.substring(1, 4) = {s1.substring(1, 4)}")
    print(f"s1.includes(Str('ell')) = {s1.includes(Str('ell'))}")
    print(f"s1.to_upper_case() = {s1.to_upper_case()}")
    print(f"s1.to_lower_case() = {s1.to_lower_case()}")
    print(f"Str('  test  ').trim() = {Str('  test  ').trim()}")
    print(f"s1.index_of(Str('ll')) = {s1.index_of(Str('ll'))}")
    print(f"s1.replace(Str('l'), Str('x')) = {s1.replace(Str('l'), Str('x'))}")
    print(f"s1.starts_with(Str('He')) = {s1.starts_with(Str('He'))}")
    print(f"s1.ends_with(Str('o')) = {s1.ends_with(Str('o'))}")
    print(f"s1.reverse() = {s1.reverse()}")
    print(f"s1.capitalize() = {s1.capitalize()}")
    print(f"s1.camel_case() = {s1.camel_case()}")
    # 运算符重载测试
    print(f"s1 + s2 = {s1 + s2}")
    print(f"s1 * 3 = {s1 * 3}")
    print(f"len(s1) = {len(s1)}")
    print(f"s1[0] = {s1[0]}")
    print(f"Str('ell') in s1: {Str('ell') in s1}")
    print(f"s1 < s2: {s1 < s2}")
    print(f"s1 == s2: {s1 == s2}")
    print()

    print("=== BigNumber Example ===")
    bn1 = BigNumber("123456789012345678901234567890")
    bn2 = BigNumber("987654321098765432109876543210")
    print(f"bn1 = {bn1}")
    print(f"bn2 = {bn2}")
    print(f"bn1.add(bn2) = {bn1.add(bn2)}")
    print(f"bn2.subtract(bn1) = {bn2.subtract(bn1)}")
    print(f"bn1.multiply(BigNumber(2)) = {bn1.multiply(BigNumber(2))}")
    print(f"bn1.power(BigNumber(2)) = {bn1.power(BigNumber(2))}")
    print(f"bn1.less_than(bn2): {bn1.less_than(bn2)}")
    print(f"bn1.is_even(): {bn1.is_even()}")
    print(f"bn1.is_positive(): {bn1.is_positive()}")
    print(f"bn1.to_hex_string() = {bn1.to_hex_string()}")
    print(f"BigNumber.gcd(bn1, bn2) = {BigNumber.gcd(bn1, bn2)}")
    print(f"BigNumber.factorial(BigNumber(10)) = {BigNumber.factorial(BigNumber(10))}")
    print(f"BigNumber.fibonacci(BigNumber(10)) = {BigNumber.fibonacci(BigNumber(10))}")
    # 运算符重载测试
    print(f"bn1 + bn2 = {bn1 + bn2}")
    print(f"bn2 - bn1 = {bn2 - bn1}")
    print(f"bn1 * BigNumber(2) = {bn1 * BigNumber(2)}")
    print(f"bn1 ** BigNumber(2) = {bn1 ** BigNumber(2)}")
    print(f"-bn1 = {-bn1}")
    print(f"bn1 < bn2: {bn1 < bn2}")
    print(f"bn1 == bn2: {bn1 == bn2}")
    print()

    print("=== BigDecimal Example ===")
    bd1 = BigDecimal("123.456", 3)
    bd2 = BigDecimal("78.9", 1)
    print(f"bd1 = {bd1}")
    print(f"bd2 = {bd2}")
    print(f"bd1.add(bd2) = {bd1.add(bd2)}")
    print(f"bd1.subtract(bd2) = {bd1.subtract(bd2)}")
    print(f"bd1.multiply(bd2) = {bd1.multiply(bd2)}")
    print(f"bd1.divide(bd2, 5) = {bd1.divide(bd2, 5)}")
    print(f"bd1.to_fixed(2) = {bd1.to_fixed(2)}")
    # 运算符重载测试
    print(f"bd1 + bd2 = {bd1 + bd2}")
    print(f"bd1 - bd2 = {bd1 - bd2}")
    print(f"bd1 * bd2 = {bd1 * bd2}")
    print(f"bd1 / bd2 = {bd1 / bd2}")
    print(f"bd1 < bd2: {bd1 < bd2}")
    print(f"bd1 == bd2: {bd1 == bd2}")
