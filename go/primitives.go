package main

import (
	"errors"
	"math"
	"math/big"
	"regexp"
	"strconv"
	"strings"
	"unicode"
)

type Bool struct {
	value bool
}

func NewBool(value bool) *Bool {
	return &Bool{value: value}
}

func (b *Bool) Value() bool {
	return b.value
}

func BoolTrue() *Bool {
	return NewBool(true)
}

func BoolFalse() *Bool {
	return NewBool(false)
}

func (b *Bool) Not() *Bool {
	return NewBool(!b.value)
}

func (b *Bool) And(other *Bool) *Bool {
	return NewBool(b.value && other.value)
}

func (b *Bool) Or(other *Bool) *Bool {
	return NewBool(b.value || other.value)
}

func (b *Bool) Xor(other *Bool) *Bool {
	return NewBool(b.value != other.value)
}

func (b *Bool) Equals(other *Bool) bool {
	return b.value == other.value
}

func (b *Bool) ToString() string {
	return strconv.FormatBool(b.value)
}

func (b *Bool) ToBoolean() bool {
	return b.value
}

type Int struct {
	value int64
}

func NewInt(value int64) *Int {
	return &Int{value: value}
}

func (i *Int) Value() int64 {
	return i.value
}

func IntZero() *Int {
	return NewInt(0)
}

func IntOne() *Int {
	return NewInt(1)
}

func (i *Int) Add(other *Int) *Int {
	return NewInt(i.value + other.value)
}

func (i *Int) Subtract(other *Int) *Int {
	return NewInt(i.value - other.value)
}

func (i *Int) Multiply(other *Int) *Int {
	return NewInt(i.value * other.value)
}

func (i *Int) Divide(other *Int) (*Int, error) {
	if other.value == 0 {
		return nil, errors.New("division by zero")
	}
	return NewInt(i.value / other.value), nil
}

func (i *Int) Modulo(other *Int) (*Int, error) {
	if other.value == 0 {
		return nil, errors.New("modulo by zero")
	}
	return NewInt(i.value % other.value), nil
}

func (i *Int) Power(exponent *Int) *Int {
	return NewInt(int64(math.Pow(float64(i.value), float64(exponent.value))))
}

func (i *Int) BitwiseAnd(other *Int) *Int {
	return NewInt(i.value & other.value)
}

func (i *Int) BitwiseOr(other *Int) *Int {
	return NewInt(i.value | other.value)
}

func (i *Int) BitwiseXor(other *Int) *Int {
	return NewInt(i.value ^ other.value)
}

func (i *Int) BitwiseNot() *Int {
	return NewInt(^i.value)
}

func (i *Int) LeftShift(shift *Int) *Int {
	return NewInt(i.value << shift.value)
}

func (i *Int) RightShift(shift *Int) *Int {
	return NewInt(i.value >> shift.value)
}

func (i *Int) Negate() *Int {
	return NewInt(-i.value)
}

func (i *Int) Abs() *Int {
	return NewInt(int64(math.Abs(float64(i.value))))
}

func (i *Int) Equals(other *Int) bool {
	return i.value == other.value
}

func (i *Int) LessThan(other *Int) bool {
	return i.value < other.value
}

func (i *Int) LessThanOrEqual(other *Int) bool {
	return i.value <= other.value
}

func (i *Int) GreaterThan(other *Int) bool {
	return i.value > other.value
}

func (i *Int) GreaterThanOrEqual(other *Int) bool {
	return i.value >= other.value
}

func (i *Int) IsEven() bool {
	return i.value%2 == 0
}

func (i *Int) IsOdd() bool {
	return i.value%2 != 0
}

func (i *Int) IsPositive() bool {
	return i.value > 0
}

func (i *Int) IsNegative() bool {
	return i.value < 0
}

func (i *Int) IsZero() bool {
	return i.value == 0
}

func (i *Int) ToString() string {
	return strconv.FormatInt(i.value, 10)
}

func (i *Int) ToNumber() int64 {
	return i.value
}

func IntMin(a, b *Int) *Int {
	if a.value < b.value {
		return a
	}
	return b
}

func IntMax(a, b *Int) *Int {
	if a.value > b.value {
		return a
	}
	return b
}

type Float struct {
	value float64
}

func NewFloat(value float64) *Float {
	return &Float{value: value}
}

func (f *Float) Value() float64 {
	return f.value
}

func FloatZero() *Float {
	return NewFloat(0)
}

func FloatOne() *Float {
	return NewFloat(1)
}

func FloatPi() *Float {
	return NewFloat(math.Pi)
}

func FloatE() *Float {
	return NewFloat(math.E)
}

func FloatNaN() *Float {
	return NewFloat(math.NaN())
}

func FloatPositiveInfinity() *Float {
	return NewFloat(math.Inf(1))
}

func FloatNegativeInfinity() *Float {
	return NewFloat(math.Inf(-1))
}

func (f *Float) Add(other *Float) *Float {
	return NewFloat(f.value + other.value)
}

func (f *Float) Subtract(other *Float) *Float {
	return NewFloat(f.value - other.value)
}

func (f *Float) Multiply(other *Float) *Float {
	return NewFloat(f.value * other.value)
}

func (f *Float) Divide(other *Float) *Float {
	if other.value == 0 {
		if f.value > 0 {
			return FloatPositiveInfinity()
		}
		return FloatNegativeInfinity()
	}
	return NewFloat(f.value / other.value)
}

func (f *Float) Power(exponent *Float) *Float {
	return NewFloat(math.Pow(f.value, exponent.value))
}

func (f *Float) Sqrt() *Float {
	return NewFloat(math.Sqrt(f.value))
}

func (f *Float) Abs() *Float {
	return NewFloat(math.Abs(f.value))
}

func (f *Float) Negate() *Float {
	return NewFloat(-f.value)
}

func (f *Float) Floor() *Float {
	return NewFloat(math.Floor(f.value))
}

func (f *Float) Ceil() *Float {
	return NewFloat(math.Ceil(f.value))
}

func (f *Float) Round() *Float {
	return NewFloat(math.Round(f.value))
}

func (f *Float) Trunc() *Float {
	return NewFloat(math.Trunc(f.value))
}

func (f *Float) Sin() *Float {
	return NewFloat(math.Sin(f.value))
}

func (f *Float) Cos() *Float {
	return NewFloat(math.Cos(f.value))
}

func (f *Float) Tan() *Float {
	return NewFloat(math.Tan(f.value))
}

func (f *Float) Log() *Float {
	return NewFloat(math.Log(f.value))
}

func (f *Float) Log10() *Float {
	return NewFloat(math.Log10(f.value))
}

func (f *Float) Exp() *Float {
	return NewFloat(math.Exp(f.value))
}

func (f *Float) Equals(other *Float, epsilon ...float64) bool {
	eps := 1e-10
	if len(epsilon) > 0 {
		eps = epsilon[0]
	}
	return math.Abs(f.value-other.value) < eps
}

func (f *Float) LessThan(other *Float) bool {
	return f.value < other.value
}

func (f *Float) LessThanOrEqual(other *Float) bool {
	return f.value <= other.value
}

func (f *Float) GreaterThan(other *Float) bool {
	return f.value > other.value
}

func (f *Float) GreaterThanOrEqual(other *Float) bool {
	return f.value >= other.value
}

func (f *Float) IsNaN() bool {
	return math.IsNaN(f.value)
}

func (f *Float) IsInfinity() bool {
	return math.IsInf(f.value, 0) && !f.IsNaN()
}

func (f *Float) IsFinite() bool {
	return !math.IsInf(f.value, 0)
}

func (f *Float) IsPositive() bool {
	return f.value > 0
}

func (f *Float) IsNegative() bool {
	return f.value < 0
}

func (f *Float) IsZero() bool {
	return f.value == 0
}

func (f *Float) IsInteger() bool {
	return math.Mod(f.value, 1) == 0
}

func (f *Float) ToString() string {
	return strconv.FormatFloat(f.value, 'f', -1, 64)
}

func (f *Float) ToFixed(digits int) string {
	return strconv.FormatFloat(f.value, 'f', digits, 64)
}

func (f *Float) ToNumber() float64 {
	return f.value
}

func FloatMin(a, b *Float) *Float {
	return NewFloat(math.Min(a.value, b.value))
}

func FloatMax(a, b *Float) *Float {
	return NewFloat(math.Max(a.value, b.value))
}

func FloatClamp(value, min, max *Float) *Float {
	return NewFloat(math.Min(math.Max(value.value, min.value), max.value))
}

func FloatLerp(a, b, t *Float) *Float {
	return NewFloat(a.value + (b.value-a.value)*t.value)
}

type Str struct {
	value string
}

func NewStr(value string) *Str {
	return &Str{value: value}
}

func (s *Str) Value() string {
	return s.value
}

func StrEmpty() *Str {
	return NewStr("")
}

func (s *Str) Length() int {
	return len(s.value)
}

func (s *Str) IsEmpty() bool {
	return len(s.value) == 0
}

func (s *Str) CharAt(index int) string {
	if index < 0 || index >= len(s.value) {
		return ""
	}
	return string(s.value[index])
}

func (s *Str) CharCodeAt(index int) int {
	if index < 0 || index >= len(s.value) {
		return 0
	}
	return int(s.value[index])
}

func (s *Str) Concat(other *Str) *Str {
	return NewStr(s.value + other.value)
}

func (s *Str) Substring(start int, end ...int) *Str {
	e := len(s.value)
	if len(end) > 0 {
		e = end[0]
	}
	if start < 0 {
		start = 0
	}
	if e > len(s.value) {
		e = len(s.value)
	}
	if start >= e {
		return StrEmpty()
	}
	return NewStr(s.value[start:e])
}

func (s *Str) Slice(start int, end ...int) *Str {
	e := len(s.value)
	if len(end) > 0 {
		e = end[0]
	}
	if start < 0 {
		start = len(s.value) + start
	}
	if e < 0 {
		e = len(s.value) + e
	}
	if start < 0 {
		start = 0
	}
	if e > len(s.value) {
		e = len(s.value)
	}
	if start >= e {
		return StrEmpty()
	}
	return NewStr(s.value[start:e])
}

func (s *Str) IndexOf(searchValue *Str, fromIndex ...int) int {
	from := 0
	if len(fromIndex) > 0 {
		from = fromIndex[0]
	}
	if from < 0 {
		from = 0
	}
	return strings.Index(s.value[from:], searchValue.value) + from
}

func (s *Str) LastIndexOf(searchValue *Str, fromIndex ...int) int {
	from := len(s.value)
	if len(fromIndex) > 0 {
		from = fromIndex[0]
	}
	if from > len(s.value) {
		from = len(s.value)
	}
	return strings.LastIndex(s.value[:from], searchValue.value)
}

func (s *Str) Includes(searchValue *Str, fromIndex ...int) bool {
	return s.IndexOf(searchValue, fromIndex...) != -1
}

func (s *Str) StartsWith(searchValue *Str, position ...int) bool {
	pos := 0
	if len(position) > 0 {
		pos = position[0]
	}
	if pos < 0 {
		pos = 0
	}
	if pos+len(searchValue.value) > len(s.value) {
		return false
	}
	return strings.HasPrefix(s.value[pos:], searchValue.value)
}

func (s *Str) EndsWith(searchValue *Str, length ...int) bool {
	l := len(s.value)
	if len(length) > 0 {
		l = length[0]
	}
	if l > len(s.value) {
		l = len(s.value)
	}
	if len(searchValue.value) > l {
		return false
	}
	return strings.HasSuffix(s.value[:l], searchValue.value)
}

func (s *Str) ToLowerCase() *Str {
	return NewStr(strings.ToLower(s.value))
}

func (s *Str) ToUpperCase() *Str {
	return NewStr(strings.ToUpper(s.value))
}

func (s *Str) Trim() *Str {
	return NewStr(strings.TrimSpace(s.value))
}

func (s *Str) TrimStart() *Str {
	return NewStr(strings.TrimLeftFunc(s.value, unicode.IsSpace))
}

func (s *Str) TrimEnd() *Str {
	return NewStr(strings.TrimRightFunc(s.value, unicode.IsSpace))
}

func (s *Str) PadStart(targetLength int, padString ...*Str) *Str {
	pad := " "
	if len(padString) > 0 {
		pad = padString[0].value
	}
	if pad == "" {
		pad = " "
	}
	if len(s.value) >= targetLength {
		return s
	}
	padding := ""
	for len(padding) < targetLength-len(s.value) {
		padding += pad
	}
	padding = padding[:targetLength-len(s.value)]
	return NewStr(padding + s.value)
}

func (s *Str) PadEnd(targetLength int, padString ...*Str) *Str {
	pad := " "
	if len(padString) > 0 {
		pad = padString[0].value
	}
	if pad == "" {
		pad = " "
	}
	if len(s.value) >= targetLength {
		return s
	}
	padding := ""
	for len(padding) < targetLength-len(s.value) {
		padding += pad
	}
	padding = padding[:targetLength-len(s.value)]
	return NewStr(s.value + padding)
}

func (s *Str) Repeat(count int) *Str {
	if count <= 0 {
		return StrEmpty()
	}
	return NewStr(strings.Repeat(s.value, count))
}

func (s *Str) Replace(searchValue interface{}, replaceValue *Str) *Str {
	switch sv := searchValue.(type) {
	case *Str:
		return NewStr(strings.Replace(s.value, sv.value, replaceValue.value, 1))
	case *regexp.Regexp:
		return NewStr(sv.ReplaceAllString(s.value, replaceValue.value))
	default:
		return s
	}
}

func (s *Str) ReplaceAll(searchValue interface{}, replaceValue *Str) *Str {
	switch sv := searchValue.(type) {
	case *Str:
		return NewStr(strings.ReplaceAll(s.value, sv.value, replaceValue.value))
	case *regexp.Regexp:
		return NewStr(sv.ReplaceAllString(s.value, replaceValue.value))
	default:
		return s
	}
}

func (s *Str) Split(separator interface{}, limit ...int) []*Str {
	var parts []string
	l := -1
	if len(limit) > 0 {
		l = limit[0]
	}
	switch sep := separator.(type) {
	case *Str:
		if l == -1 {
			parts = strings.Split(s.value, sep.value)
		} else {
			parts = strings.SplitN(s.value, sep.value, l)
		}
	case *regexp.Regexp:
		if l == -1 {
			parts = sep.Split(s.value, -1)
		} else {
			parts = sep.Split(s.value, l)
		}
	}
	result := make([]*Str, len(parts))
	for i, part := range parts {
		result[i] = NewStr(part)
	}
	return result
}

func (s *Str) Equals(other *Str) bool {
	return s.value == other.value
}

func (s *Str) EqualsIgnoreCase(other *Str) bool {
	return strings.EqualFold(s.value, other.value)
}

func (s *Str) Compare(other *Str) int {
	return strings.Compare(s.value, other.value)
}

func (s *Str) IsWhitespace() bool {
	whitespaceRegex := regexp.MustCompile(`^\s*$`)
	return whitespaceRegex.MatchString(s.value)
}

func (s *Str) IsAlpha() bool {
	alphaRegex := regexp.MustCompile(`^[a-zA-Z]+$`)
	return alphaRegex.MatchString(s.value)
}

func (s *Str) IsNumeric() bool {
	numericRegex := regexp.MustCompile(`^[0-9]+$`)
	return numericRegex.MatchString(s.value)
}

func (s *Str) IsAlphanumeric() bool {
	alphanumericRegex := regexp.MustCompile(`^[a-zA-Z0-9]+$`)
	return alphanumericRegex.MatchString(s.value)
}

func (s *Str) Reverse() *Str {
	runes := []rune(s.value)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return NewStr(string(runes))
}

func (s *Str) CountOccurrences(substring *Str) int {
	count := 0
	pos := 0
	for {
		idx := strings.Index(s.value[pos:], substring.value)
		if idx == -1 {
			break
		}
		count++
		pos += idx + len(substring.value)
	}
	return count
}

func (s *Str) Words() []*Str {
	wordsRegex := regexp.MustCompile(`\s+`)
	parts := wordsRegex.Split(s.value, -1)
	result := make([]*Str, 0)
	for _, part := range parts {
		if part != "" {
			result = append(result, NewStr(part))
		}
	}
	return result
}

func (s *Str) Lines() []*Str {
	linesRegex := regexp.MustCompile(`\r?\n`)
	parts := linesRegex.Split(s.value, -1)
	result := make([]*Str, len(parts))
	for i, part := range parts {
		result[i] = NewStr(part)
	}
	return result
}

func (s *Str) Capitalize() *Str {
	if s.IsEmpty() {
		return s
	}
	runes := []rune(s.value)
	first := unicode.ToUpper(runes[0])
	rest := strings.ToLower(string(runes[1:]))
	return NewStr(string(first) + rest)
}

func (s *Str) TitleCase() *Str {
	titleCaseRegex := regexp.MustCompile(`\w\S*`)
	result := titleCaseRegex.ReplaceAllStringFunc(s.value, func(txt string) string {
		if len(txt) == 0 {
			return txt
		}
		runes := []rune(txt)
		first := unicode.ToUpper(runes[0])
		rest := strings.ToLower(string(runes[1:]))
		return string(first) + rest
	})
	return NewStr(result)
}

func (s *Str) CamelCase() *Str {
	words := s.Words()
	if len(words) == 0 {
		return StrEmpty()
	}
	first := words[0].ToLowerCase()
	rest := make([]string, len(words)-1)
	for i, word := range words[1:] {
		rest[i] = word.Capitalize().value
	}
	return NewStr(first.value + strings.Join(rest, ""))
}

func (s *Str) SnakeCase() *Str {
	snakeCaseRegex1 := regexp.MustCompile(`([A-Z])`)
	result := snakeCaseRegex1.ReplaceAllString(s.value, `_$1`)
	result = strings.ToLower(result)
	result = strings.TrimPrefix(result, "_")
	spaceRegex := regexp.MustCompile(`\s+`)
	result = spaceRegex.ReplaceAllString(result, "_")
	return NewStr(result)
}

func (s *Str) KebabCase() *Str {
	kebabCaseRegex1 := regexp.MustCompile(`([A-Z])`)
	result := kebabCaseRegex1.ReplaceAllString(s.value, `-$1`)
	result = strings.ToLower(result)
	result = strings.TrimPrefix(result, "-")
	spaceRegex := regexp.MustCompile(`\s+`)
	result = spaceRegex.ReplaceAllString(result, "-")
	return NewStr(result)
}

func (s *Str) ToCharArray() []string {
	result := make([]string, len(s.value))
	for i, c := range s.value {
		result[i] = string(c)
	}
	return result
}

func (s *Str) ToString() string {
	return s.value
}

func StrFromCharArray(chars []string) *Str {
	return NewStr(strings.Join(chars, ""))
}

func StrFromCharCode(codes ...int) *Str {
	runes := make([]rune, len(codes))
	for i, code := range codes {
		runes[i] = rune(code)
	}
	return NewStr(string(runes))
}

func StrJoin(strList []*Str, separator ...*Str) *Str {
	sep := StrEmpty()
	if len(separator) > 0 {
		sep = separator[0]
	}
	parts := make([]string, len(strList))
	for i, s := range strList {
		parts[i] = s.value
	}
	return NewStr(strings.Join(parts, sep.value))
}

type Null struct{}

var nullInstance *Null = &Null{}

func NullValue() *Null {
	return nullInstance
}

func (n *Null) IsNull() bool {
	return true
}

func (n *Null) Equals(other interface{}) bool {
	_, ok := other.(*Null)
	return other == nil || ok
}

func (n *Null) ToString() string {
	return "null"
}

func (n *Null) ToJSON() interface{} {
	return nil
}

type BigNumber struct {
	value *big.Int
}

func NewBigNumber(value interface{}) *BigNumber {
	bn := &BigNumber{value: big.NewInt(0)}
	switch v := value.(type) {
	case string:
		bn.value, _ = new(big.Int).SetString(v, 10)
	case int:
		bn.value = big.NewInt(int64(v))
	case int64:
		bn.value = big.NewInt(v)
	case *big.Int:
		bn.value = new(big.Int).Set(v)
	}
	return bn
}

func (bn *BigNumber) Value() *big.Int {
	return bn.value
}

func BigNumberZero() *BigNumber {
	return NewBigNumber(0)
}

func BigNumberOne() *BigNumber {
	return NewBigNumber(1)
}

func BigNumberFromString(value string) *BigNumber {
	return NewBigNumber(value)
}

func (bn *BigNumber) Add(other *BigNumber) *BigNumber {
	result := new(big.Int).Add(bn.value, other.value)
	return NewBigNumber(result)
}

func (bn *BigNumber) Subtract(other *BigNumber) *BigNumber {
	result := new(big.Int).Sub(bn.value, other.value)
	return NewBigNumber(result)
}

func (bn *BigNumber) Multiply(other *BigNumber) *BigNumber {
	result := new(big.Int).Mul(bn.value, other.value)
	return NewBigNumber(result)
}

func (bn *BigNumber) Divide(other *BigNumber) (*BigNumber, error) {
	if other.value.Cmp(big.NewInt(0)) == 0 {
		return nil, errors.New("division by zero")
	}
	result := new(big.Int).Div(bn.value, other.value)
	return NewBigNumber(result), nil
}

func (bn *BigNumber) Modulo(other *BigNumber) (*BigNumber, error) {
	if other.value.Cmp(big.NewInt(0)) == 0 {
		return nil, errors.New("modulo by zero")
	}
	result := new(big.Int).Mod(bn.value, other.value)
	return NewBigNumber(result), nil
}

func (bn *BigNumber) Power(exponent *BigNumber) *BigNumber {
	if exponent.value.Cmp(big.NewInt(0)) < 0 {
		panic("exponent must be non-negative")
	}
	result := new(big.Int).Exp(bn.value, exponent.value, nil)
	return NewBigNumber(result)
}

func (bn *BigNumber) BitwiseAnd(other *BigNumber) *BigNumber {
	result := new(big.Int).And(bn.value, other.value)
	return NewBigNumber(result)
}

func (bn *BigNumber) BitwiseOr(other *BigNumber) *BigNumber {
	result := new(big.Int).Or(bn.value, other.value)
	return NewBigNumber(result)
}

func (bn *BigNumber) BitwiseXor(other *BigNumber) *BigNumber {
	result := new(big.Int).Xor(bn.value, other.value)
	return NewBigNumber(result)
}

func (bn *BigNumber) BitwiseNot() *BigNumber {
	result := new(big.Int).Not(bn.value)
	return NewBigNumber(result)
}

func (bn *BigNumber) LeftShift(shift *BigNumber) *BigNumber {
	result := new(big.Int).Lsh(bn.value, uint(shift.value.Int64()))
	return NewBigNumber(result)
}

func (bn *BigNumber) RightShift(shift *BigNumber) *BigNumber {
	result := new(big.Int).Rsh(bn.value, uint(shift.value.Int64()))
	return NewBigNumber(result)
}

func (bn *BigNumber) Negate() *BigNumber {
	result := new(big.Int).Neg(bn.value)
	return NewBigNumber(result)
}

func (bn *BigNumber) Abs() *BigNumber {
	result := new(big.Int).Abs(bn.value)
	return NewBigNumber(result)
}

func (bn *BigNumber) Equals(other *BigNumber) bool {
	return bn.value.Cmp(other.value) == 0
}

func (bn *BigNumber) LessThan(other *BigNumber) bool {
	return bn.value.Cmp(other.value) < 0
}

func (bn *BigNumber) LessThanOrEqual(other *BigNumber) bool {
	return bn.value.Cmp(other.value) <= 0
}

func (bn *BigNumber) GreaterThan(other *BigNumber) bool {
	return bn.value.Cmp(other.value) > 0
}

func (bn *BigNumber) GreaterThanOrEqual(other *BigNumber) bool {
	return bn.value.Cmp(other.value) >= 0
}

func (bn *BigNumber) IsEven() bool {
	return bn.value.Bit(0) == 0
}

func (bn *BigNumber) IsOdd() bool {
	return bn.value.Bit(0) == 1
}

func (bn *BigNumber) IsPositive() bool {
	return bn.value.Cmp(big.NewInt(0)) > 0
}

func (bn *BigNumber) IsNegative() bool {
	return bn.value.Cmp(big.NewInt(0)) < 0
}

func (bn *BigNumber) IsZero() bool {
	return bn.value.Cmp(big.NewInt(0)) == 0
}

func (bn *BigNumber) IsOne() bool {
	return bn.value.Cmp(big.NewInt(1)) == 0
}

func (bn *BigNumber) Sign() int {
	return bn.value.Sign()
}

func (bn *BigNumber) ToString() string {
	return bn.value.String()
}

func (bn *BigNumber) ToHexString() string {
	return "0x" + bn.value.Text(16)
}

func (bn *BigNumber) ToBinaryString() string {
	return "0b" + bn.value.Text(2)
}

func (bn *BigNumber) ToOctalString() string {
	return "0o" + bn.value.Text(8)
}

func (bn *BigNumber) ToBigInt() *big.Int {
	return bn.value
}

func (bn *BigNumber) ToNumber() float64 {
	return float64(bn.value.Int64())
}

func (bn *BigNumber) IsSafeNumber() bool {
	maxSafe := big.NewInt(1<<53 - 1)
	minSafe := new(big.Int).Neg(maxSafe)
	return bn.value.Cmp(minSafe) >= 0 && bn.value.Cmp(maxSafe) <= 0
}

func BigNumberMin(a, b *BigNumber) *BigNumber {
	if a.value.Cmp(b.value) < 0 {
		return a
	}
	return b
}

func BigNumberMax(a, b *BigNumber) *BigNumber {
	if a.value.Cmp(b.value) > 0 {
		return a
	}
	return b
}

func BigNumberGcd(a, b *BigNumber) *BigNumber {
	x := a.Abs().value
	y := b.Abs().value
	zero := big.NewInt(0)
	for y.Cmp(zero) != 0 {
		x, y = y, new(big.Int).Mod(x, y)
	}
	return NewBigNumber(x)
}

func BigNumberLcm(a, b *BigNumber) *BigNumber {
	if a.IsZero() || b.IsZero() {
		return BigNumberZero()
	}
	gcd := BigNumberGcd(a, b)
	product, _ := a.Multiply(b).Divide(gcd)
	return product.Abs()
}

func BigNumberFactorial(n *BigNumber) *BigNumber {
	if n.IsNegative() {
		panic("factorial is not defined for negative numbers")
	}
	result := big.NewInt(1)
	for i := big.NewInt(2); i.Cmp(n.value) <= 0; i.Add(i, big.NewInt(1)) {
		result.Mul(result, i)
	}
	return NewBigNumber(result)
}

func BigNumberFibonacci(n *BigNumber) *BigNumber {
	if n.IsNegative() {
		panic("fibonacci is not defined for negative numbers")
	}
	if n.IsZero() || n.IsOne() {
		return n
	}
	a := big.NewInt(0)
	b := big.NewInt(1)
	for i := big.NewInt(2); i.Cmp(n.value) <= 0; i.Add(i, big.NewInt(1)) {
		a, b = b, new(big.Int).Add(a, b)
	}
	return NewBigNumber(b)
}

type BigDecimal struct {
	integerPart *BigNumber
	scale       int
}

func NewBigDecimal(value interface{}, scale ...int) *BigDecimal {
	s := 0
	if len(scale) > 0 {
		s = scale[0]
	}
	bd := &BigDecimal{}
	var strVal string
	switch v := value.(type) {
	case string:
		strVal = v
	case float64:
		strVal = strconv.FormatFloat(v, 'f', -1, 64)
	case int:
		strVal = strconv.Itoa(v)
	}
	parts := strings.SplitN(strVal, ".", 2)
	integerPart := parts[0]
	fractionalPart := ""
	if len(parts) > 1 {
		fractionalPart = parts[1]
	}
	if s >= 0 {
		for len(fractionalPart) < s {
			fractionalPart += "0"
		}
		if len(fractionalPart) > s {
			fractionalPart = fractionalPart[:s]
		}
	}
	combined := integerPart + fractionalPart
	bd.integerPart = NewBigNumber(combined)
	bd.scale = s
	return bd
}

func (bd *BigDecimal) IntegerPart() *BigNumber {
	return bd.integerPart
}

func (bd *BigDecimal) Scale() int {
	return bd.scale
}

func BigDecimalZero(scale ...int) *BigDecimal {
	s := 0
	if len(scale) > 0 {
		s = scale[0]
	}
	return NewBigDecimal("0", s)
}

func BigDecimalOne(scale ...int) *BigDecimal {
	s := 0
	if len(scale) > 0 {
		s = scale[0]
	}
	return NewBigDecimal("1", s)
}

func (bd *BigDecimal) scaleTo(newScale int) *BigNumber {
	if newScale == bd.scale {
		return bd.integerPart
	}
	scaleDiff := newScale - bd.scale
	if scaleDiff > 0 {
		multiplier := NewBigNumber(1)
		for i := 0; i < scaleDiff; i++ {
			multiplier = multiplier.Multiply(NewBigNumber(10))
		}
		return bd.integerPart.Multiply(multiplier)
	} else {
		divisor := NewBigNumber(1)
		for i := 0; i < -scaleDiff; i++ {
			divisor = divisor.Multiply(NewBigNumber(10))
		}
		result, _ := bd.integerPart.Divide(divisor)
		return result
	}
}

func (bd *BigDecimal) Add(other *BigDecimal) *BigDecimal {
	maxScale := bd.scale
	if other.scale > maxScale {
		maxScale = other.scale
	}
	thisScaled := bd.scaleTo(maxScale)
	otherScaled := other.scaleTo(maxScale)
	sum := thisScaled.Add(otherScaled)
	return BigDecimalFromScaledInteger(sum, maxScale)
}

func (bd *BigDecimal) Subtract(other *BigDecimal) *BigDecimal {
	maxScale := bd.scale
	if other.scale > maxScale {
		maxScale = other.scale
	}
	thisScaled := bd.scaleTo(maxScale)
	otherScaled := other.scaleTo(maxScale)
	diff := thisScaled.Subtract(otherScaled)
	return BigDecimalFromScaledInteger(diff, maxScale)
}

func (bd *BigDecimal) Multiply(other *BigDecimal) *BigDecimal {
	product := bd.integerPart.Multiply(other.integerPart)
	newScale := bd.scale + other.scale
	return BigDecimalFromScaledInteger(product, newScale)
}

func BigDecimalFromScaledInteger(scaledInteger *BigNumber, scale int) *BigDecimal {
	result := NewBigDecimal("0", scale)
	result.integerPart = scaledInteger
	return result
}

func (bd *BigDecimal) Divide(other *BigDecimal, precision ...int) *BigDecimal {
	p := 10
	if len(precision) > 0 {
		p = precision[0]
	}
	thisScaled := bd.scaleTo(bd.scale + p + other.scale)
	quotient, _ := thisScaled.Divide(other.integerPart)
	return NewBigDecimal(quotient.ToString(), bd.scale+p)
}

func (bd *BigDecimal) ToString() string {
	str := bd.integerPart.ToString()
	sign := ""
	if strings.HasPrefix(str, "-") {
		sign = "-"
		str = str[1:]
	}
	if bd.scale == 0 {
		return sign + str
	}
	padded := str
	for len(padded) < bd.scale+1 {
		padded = "0" + padded
	}
	integerPart := padded[:len(padded)-bd.scale]
	fractionalPart := padded[len(padded)-bd.scale:]
	return sign + integerPart + "." + fractionalPart
}

func (bd *BigDecimal) ToFixed(decimalPlaces int) string {
	scaled := bd.scaleTo(decimalPlaces)
	temp := NewBigDecimal(scaled.ToString(), decimalPlaces)
	return temp.ToString()
}
