#![allow(dead_code)]

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg, BitAnd, BitOr, BitXor, Not, Shl, Shr};
use num_bigint::{BigInt, Sign};
use num_traits::{cast::ToPrimitive, sign::Signed};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bool {
    value: bool,
}

impl Bool {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn value(&self) -> bool {
        self.value
    }

    pub fn true_value() -> Self {
        Self::new(true)
    }

    pub fn false_value() -> Self {
        Self::new(false)
    }

    pub fn not(&self) -> Self {
        Self::new(!self.value)
    }

    pub fn and(&self, other: &Self) -> Self {
        Self::new(self.value && other.value)
    }

    pub fn or(&self, other: &Self) -> Self {
        Self::new(self.value || other.value)
    }

    pub fn xor(&self, other: &Self) -> Self {
        Self::new(self.value != other.value)
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.value == other.value
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }

    pub fn to_boolean(&self) -> bool {
        self.value
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Not for Bool {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bool::not(&self)
    }
}

impl BitAnd for Bool {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bool::and(&self, &rhs)
    }
}

impl BitOr for Bool {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bool::or(&self, &rhs)
    }
}

impl BitXor for Bool {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bool::xor(&self, &rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Int {
    value: i64,
}

impl Int {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn zero() -> Self {
        Self::new(0)
    }

    pub fn one() -> Self {
        Self::new(1)
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.value + other.value)
    }

    pub fn subtract(&self, other: &Self) -> Self {
        Self::new(self.value - other.value)
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self::new(self.value * other.value)
    }

    pub fn divide(&self, other: &Self) -> Result<Self, String> {
        if other.value == 0 {
            return Err("division by zero".to_string());
        }
        Ok(Self::new(self.value / other.value))
    }

    pub fn modulo(&self, other: &Self) -> Result<Self, String> {
        if other.value == 0 {
            return Err("modulo by zero".to_string());
        }
        Ok(Self::new(self.value % other.value))
    }

    pub fn power(&self, exponent: &Self) -> Self {
        Self::new(self.value.pow(exponent.value as u32))
    }

    pub fn bitwise_and(&self, other: &Self) -> Self {
        Self::new(self.value & other.value)
    }

    pub fn bitwise_or(&self, other: &Self) -> Self {
        Self::new(self.value | other.value)
    }

    pub fn bitwise_xor(&self, other: &Self) -> Self {
        Self::new(self.value ^ other.value)
    }

    pub fn bitwise_not(&self) -> Self {
        Self::new(!self.value)
    }

    pub fn left_shift(&self, shift: &Self) -> Self {
        Self::new(self.value << shift.value)
    }

    pub fn right_shift(&self, shift: &Self) -> Self {
        Self::new(self.value >> shift.value)
    }

    pub fn negate(&self) -> Self {
        Self::new(-self.value)
    }

    pub fn abs(&self) -> Self {
        Self::new(self.value.abs())
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.value == other.value
    }

    pub fn less_than(&self, other: &Self) -> bool {
        self.value < other.value
    }

    pub fn less_than_or_equal(&self, other: &Self) -> bool {
        self.value <= other.value
    }

    pub fn greater_than(&self, other: &Self) -> bool {
        self.value > other.value
    }

    pub fn greater_than_or_equal(&self, other: &Self) -> bool {
        self.value >= other.value
    }

    pub fn is_even(&self) -> bool {
        self.value % 2 == 0
    }

    pub fn is_odd(&self) -> bool {
        self.value % 2 != 0
    }

    pub fn is_positive(&self) -> bool {
        self.value > 0
    }

    pub fn is_negative(&self) -> bool {
        self.value < 0
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }

    pub fn to_number(&self) -> i64 {
        self.value
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for Int {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Int::add(&self, &rhs)
    }
}

impl Sub for Int {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Int::subtract(&self, &rhs)
    }
}

impl Mul for Int {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Int::multiply(&self, &rhs)
    }
}

impl Neg for Int {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Int::negate(&self)
    }
}

impl BitAnd for Int {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Int::bitwise_and(&self, &rhs)
    }
}

impl BitOr for Int {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Int::bitwise_or(&self, &rhs)
    }
}

impl BitXor for Int {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Int::bitwise_xor(&self, &rhs)
    }
}

impl Not for Int {
    type Output = Self;
    fn not(self) -> Self::Output {
        Int::bitwise_not(&self)
    }
}

impl Shl<Int> for Int {
    type Output = Self;
    fn shl(self, rhs: Int) -> Self::Output {
        Int::left_shift(&self, &rhs)
    }
}

impl Shr<Int> for Int {
    type Output = Self;
    fn shr(self, rhs: Int) -> Self::Output {
        Int::right_shift(&self, &rhs)
    }
}

pub fn int_min(a: &Int, b: &Int) -> Int {
    if a.value < b.value {
        *a
    } else {
        *b
    }
}

pub fn int_max(a: &Int, b: &Int) -> Int {
    if a.value > b.value {
        *a
    } else {
        *b
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Float {
    value: f64,
}

impl Float {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn zero() -> Self {
        Self::new(0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0)
    }

    pub fn pi() -> Self {
        Self::new(std::f64::consts::PI)
    }

    pub fn e() -> Self {
        Self::new(std::f64::consts::E)
    }

    pub fn nan() -> Self {
        Self::new(std::f64::NAN)
    }

    pub fn positive_infinity() -> Self {
        Self::new(std::f64::INFINITY)
    }

    pub fn negative_infinity() -> Self {
        Self::new(std::f64::NEG_INFINITY)
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.value + other.value)
    }

    pub fn subtract(&self, other: &Self) -> Self {
        Self::new(self.value - other.value)
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self::new(self.value * other.value)
    }

    pub fn divide(&self, other: &Self) -> Self {
        if other.value == 0.0 {
            if self.value > 0.0 {
                return Self::positive_infinity();
            }
            return Self::negative_infinity();
        }
        Self::new(self.value / other.value)
    }

    pub fn power(&self, exponent: &Self) -> Self {
        Self::new(self.value.powf(exponent.value))
    }

    pub fn sqrt(&self) -> Self {
        Self::new(self.value.sqrt())
    }

    pub fn abs(&self) -> Self {
        Self::new(self.value.abs())
    }

    pub fn negate(&self) -> Self {
        Self::new(-self.value)
    }

    pub fn floor(&self) -> Self {
        Self::new(self.value.floor())
    }

    pub fn ceil(&self) -> Self {
        Self::new(self.value.ceil())
    }

    pub fn round(&self) -> Self {
        Self::new(self.value.round())
    }

    pub fn trunc(&self) -> Self {
        Self::new(self.value.trunc())
    }

    pub fn sin(&self) -> Self {
        Self::new(self.value.sin())
    }

    pub fn cos(&self) -> Self {
        Self::new(self.value.cos())
    }

    pub fn tan(&self) -> Self {
        Self::new(self.value.tan())
    }

    pub fn log(&self) -> Self {
        Self::new(self.value.ln())
    }

    pub fn log10(&self) -> Self {
        Self::new(self.value.log10())
    }

    pub fn exp(&self) -> Self {
        Self::new(self.value.exp())
    }

    pub fn equals(&self, other: &Self, epsilon: Option<f64>) -> bool {
        let eps = epsilon.unwrap_or(1e-10);
        (self.value - other.value).abs() < eps
    }

    pub fn less_than(&self, other: &Self) -> bool {
        self.value < other.value
    }

    pub fn less_than_or_equal(&self, other: &Self) -> bool {
        self.value <= other.value
    }

    pub fn greater_than(&self, other: &Self) -> bool {
        self.value > other.value
    }

    pub fn greater_than_or_equal(&self, other: &Self) -> bool {
        self.value >= other.value
    }

    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }

    pub fn is_infinity(&self) -> bool {
        self.value.is_infinite() && !self.is_nan()
    }

    pub fn is_finite(&self) -> bool {
        self.value.is_finite()
    }

    pub fn is_positive(&self) -> bool {
        self.value > 0.0
    }

    pub fn is_negative(&self) -> bool {
        self.value < 0.0
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    pub fn is_integer(&self) -> bool {
        self.value.fract() == 0.0
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }

    pub fn to_fixed(&self, digits: usize) -> String {
        format!("{0:.1$}", self.value, digits)
    }

    pub fn to_number(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for Float {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Float::add(&self, &rhs)
    }
}

impl Sub for Float {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Float::subtract(&self, &rhs)
    }
}

impl Mul for Float {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Float::multiply(&self, &rhs)
    }
}

impl Div for Float {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Float::divide(&self, &rhs)
    }
}

impl Neg for Float {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Float::negate(&self)
    }
}

pub fn float_min(a: &Float, b: &Float) -> Float {
    Float::new(a.value.min(b.value))
}

pub fn float_max(a: &Float, b: &Float) -> Float {
    Float::new(a.value.max(b.value))
}

pub fn float_clamp(value: &Float, min: &Float, max: &Float) -> Float {
    Float::new(value.value.clamp(min.value, max.value))
}

pub fn float_lerp(a: &Float, b: &Float, t: &Float) -> Float {
    Float::new(a.value + (b.value - a.value) * t.value)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Str {
    value: String,
}

impl Str {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn empty() -> Self {
        Self::new(String::new())
    }

    pub fn length(&self) -> usize {
        self.value.len()
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn char_at(&self, index: usize) -> Option<char> {
        self.value.chars().nth(index)
    }

    pub fn char_code_at(&self, index: usize) -> Option<u32> {
        self.value.chars().nth(index).map(|c| c as u32)
    }

    pub fn concat(&self, other: &Self) -> Self {
        Self::new(format!("{}{}", self.value, other.value))
    }

    pub fn substring(&self, start: usize, end: Option<usize>) -> Self {
        let e = end.unwrap_or(self.value.len());
        let s = start.min(self.value.len());
        let e = e.min(self.value.len());
        if s >= e {
            return Self::empty();
        }
        Self::new(self.value[s..e].to_string())
    }

    pub fn slice(&self, start: isize, end: Option<isize>) -> Self {
        let len = self.value.len() as isize;
        let mut s = if start < 0 { start + len } else { start };
        let mut e = end.unwrap_or(len);
        if e < 0 {
            e += len;
        }
        s = s.max(0);
        e = e.max(0).min(len);
        if s >= e {
            return Self::empty();
        }
        Self::new(self.value[s as usize..e as usize].to_string())
    }

    pub fn index_of(&self, search_value: &Self, from_index: Option<usize>) -> Option<usize> {
        let from = from_index.unwrap_or(0);
        if from >= self.value.len() {
            return None;
        }
        self.value[from..].find(&search_value.value).map(|i| i + from)
    }

    pub fn last_index_of(&self, search_value: &Self, from_index: Option<usize>) -> Option<usize> {
        let from = from_index.unwrap_or(self.value.len());
        if from == 0 {
            return None;
        }
        self.value[..from].rfind(&search_value.value)
    }

    pub fn includes(&self, search_value: &Self, from_index: Option<usize>) -> bool {
        self.index_of(search_value, from_index).is_some()
    }

    pub fn starts_with(&self, search_value: &Self, position: Option<usize>) -> bool {
        let pos = position.unwrap_or(0);
        if pos + search_value.value.len() > self.value.len() {
            return false;
        }
        self.value[pos..].starts_with(&search_value.value)
    }

    pub fn ends_with(&self, search_value: &Self, length: Option<usize>) -> bool {
        let len = length.unwrap_or(self.value.len());
        if search_value.value.len() > len {
            return false;
        }
        self.value[..len].ends_with(&search_value.value)
    }

    pub fn to_lower_case(&self) -> Self {
        Self::new(self.value.to_lowercase())
    }

    pub fn to_upper_case(&self) -> Self {
        Self::new(self.value.to_uppercase())
    }

    pub fn trim(&self) -> Self {
        Self::new(self.value.trim().to_string())
    }

    pub fn trim_start(&self) -> Self {
        Self::new(self.value.trim_start().to_string())
    }

    pub fn trim_end(&self) -> Self {
        Self::new(self.value.trim_end().to_string())
    }

    pub fn pad_start(&self, target_length: usize, pad_string: Option<&Self>) -> Self {
        let pad = pad_string.map(|s| s.value.as_str()).unwrap_or(" ");
        if pad.is_empty() {
            return self.clone();
        }
        if self.value.len() >= target_length {
            return self.clone();
        }
        let padding_needed = target_length - self.value.len();
        let mut padding = String::new();
        while padding.len() < padding_needed {
            padding.push_str(pad);
        }
        padding.truncate(padding_needed);
        Self::new(format!("{}{}", padding, self.value))
    }

    pub fn pad_end(&self, target_length: usize, pad_string: Option<&Self>) -> Self {
        let pad = pad_string.map(|s| s.value.as_str()).unwrap_or(" ");
        if pad.is_empty() {
            return self.clone();
        }
        if self.value.len() >= target_length {
            return self.clone();
        }
        let padding_needed = target_length - self.value.len();
        let mut padding = String::new();
        while padding.len() < padding_needed {
            padding.push_str(pad);
        }
        padding.truncate(padding_needed);
        Self::new(format!("{}{}", self.value, padding))
    }

    pub fn repeat(&self, count: usize) -> Self {
        if count == 0 {
            return Self::empty();
        }
        Self::new(self.value.repeat(count))
    }

    pub fn replace(&self, search_value: &Self, replace_value: &Self) -> Self {
        Self::new(self.value.replacen(&search_value.value, &replace_value.value, 1))
    }

    pub fn replace_all(&self, search_value: &Self, replace_value: &Self) -> Self {
        Self::new(self.value.replace(&search_value.value, &replace_value.value))
    }

    pub fn split(&self, separator: &Self, limit: Option<usize>) -> Vec<Self> {
        let parts: Vec<&str> = if let Some(l) = limit {
            self.value.splitn(l, &separator.value).collect()
        } else {
            self.value.split(&separator.value).collect()
        };
        parts.into_iter().map(|s| Self::new(s.to_string())).collect()
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.value == other.value
    }

    pub fn equals_ignore_case(&self, other: &Self) -> bool {
        self.value.eq_ignore_ascii_case(&other.value)
    }

    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }

    pub fn reverse(&self) -> Self {
        Self::new(self.value.chars().rev().collect())
    }

    pub fn count_occurrences(&self, substring: &Self) -> usize {
        let mut count = 0;
        let mut pos = 0;
        while let Some(idx) = self.value[pos..].find(&substring.value) {
            count += 1;
            pos += idx + substring.value.len();
        }
        count
    }

    pub fn words(&self) -> Vec<Self> {
        self.value
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| Self::new(s.to_string()))
            .collect()
    }

    pub fn lines(&self) -> Vec<Self> {
        self.value.lines().map(|s| Self::new(s.to_string())).collect()
    }

    pub fn capitalize(&self) -> Self {
        let mut chars = self.value.chars();
        match chars.next() {
            None => Self::empty(),
            Some(first) => {
                let rest: String = chars.as_str().to_lowercase();
                Self::new(format!("{}{}", first.to_uppercase(), rest))
            }
        }
    }

    pub fn title_case(&self) -> Self {
        let mut result = String::new();
        let mut capitalize_next = true;
        for c in self.value.chars() {
            if c.is_whitespace() {
                result.push(c);
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else {
                result.push(c.to_lowercase().next().unwrap());
            }
        }
        Self::new(result)
    }

    pub fn camel_case(&self) -> Self {
        let words = self.words();
        if words.is_empty() {
            return Self::empty();
        }
        let mut result = words[0].to_lower_case().value;
        for word in &words[1..] {
            result.push_str(&word.capitalize().value);
        }
        Self::new(result)
    }

    pub fn snake_case(&self) -> Self {
        let mut result = String::new();
        for (i, c) in self.value.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
            } else if c.is_whitespace() {
                result.push('_');
            } else {
                result.push(c);
            }
        }
        Self::new(result)
    }

    pub fn kebab_case(&self) -> Self {
        let mut result = String::new();
        for (i, c) in self.value.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 {
                    result.push('-');
                }
                result.push(c.to_lowercase().next().unwrap());
            } else if c.is_whitespace() {
                result.push('-');
            } else {
                result.push(c);
            }
        }
        Self::new(result)
    }

    pub fn to_char_array(&self) -> Vec<char> {
        self.value.chars().collect()
    }

    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for Str {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Str::concat(&self, &rhs)
    }
}

pub fn str_from_char_array(chars: &[char]) -> Str {
    Str::new(chars.iter().collect())
}

pub fn str_from_char_code(codes: &[u32]) -> Str {
    let s: String = codes.iter().filter_map(|&c| char::from_u32(c)).collect();
    Str::new(s)
}

pub fn str_join(strings: &[Str], separator: Option<&Str>) -> Str {
    let sep = separator.map(|s| s.value.as_str()).unwrap_or("");
    let parts: Vec<&str> = strings.iter().map(|s| s.value.as_str()).collect();
    Str::new(parts.join(sep))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Null;

impl Null {
    pub fn value() -> Self {
        Null
    }

    pub fn is_null(&self) -> bool {
        true
    }

    pub fn to_string(&self) -> String {
        "null".to_string()
    }
}

impl fmt::Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "null")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigNumber {
    value: BigInt,
}

impl BigNumber {
    pub fn new(value: BigInt) -> Self {
        Self { value }
    }

    pub fn from_string(value: &str) -> Result<Self, String> {
        value.parse().map(Self::new).map_err(|_| "invalid number".to_string())
    }

    pub fn value(&self) -> &BigInt {
        &self.value
    }

    pub fn zero() -> Self {
        Self::new(BigInt::from(0))
    }

    pub fn one() -> Self {
        Self::new(BigInt::from(1))
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(&self.value + &other.value)
    }

    pub fn subtract(&self, other: &Self) -> Self {
        Self::new(&self.value - &other.value)
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self::new(&self.value * &other.value)
    }

    pub fn divide(&self, other: &Self) -> Result<Self, String> {
        if other.value == BigInt::from(0) {
            return Err("division by zero".to_string());
        }
        Ok(Self::new(&self.value / &other.value))
    }

    pub fn modulo(&self, other: &Self) -> Result<Self, String> {
        if other.value == BigInt::from(0) {
            return Err("modulo by zero".to_string());
        }
        Ok(Self::new(&self.value % &other.value))
    }

    pub fn power(&self, exponent: &Self) -> Self {
        let exp = exponent.value.to_i64().unwrap_or(0);
        if exp < 0 {
            panic!("exponent must be non-negative");
        }
        Self::new(self.value.pow(exp as u32))
    }

    pub fn bitwise_and(&self, other: &Self) -> Self {
        Self::new(&self.value & &other.value)
    }

    pub fn bitwise_or(&self, other: &Self) -> Self {
        Self::new(&self.value | &other.value)
    }

    pub fn bitwise_xor(&self, other: &Self) -> Self {
        Self::new(&self.value ^ &other.value)
    }

    pub fn bitwise_not(&self) -> Self {
        Self::new(!&self.value)
    }

    pub fn left_shift(&self, shift: &Self) -> Self {
        let s = shift.value.to_usize().unwrap_or(0);
        Self::new(&self.value << s)
    }

    pub fn right_shift(&self, shift: &Self) -> Self {
        let s = shift.value.to_usize().unwrap_or(0);
        Self::new(&self.value >> s)
    }

    pub fn negate(&self) -> Self {
        Self::new(-&self.value)
    }

    pub fn abs(&self) -> Self {
        Self::new(self.value.clone().abs())
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.value == other.value
    }

    pub fn less_than(&self, other: &Self) -> bool {
        self.value < other.value
    }

    pub fn less_than_or_equal(&self, other: &Self) -> bool {
        self.value <= other.value
    }

    pub fn greater_than(&self, other: &Self) -> bool {
        self.value > other.value
    }

    pub fn greater_than_or_equal(&self, other: &Self) -> bool {
        self.value >= other.value
    }

    pub fn is_even(&self) -> bool {
        !self.value.bit(0)
    }

    pub fn is_odd(&self) -> bool {
        self.value.bit(0)
    }

    pub fn is_positive(&self) -> bool {
        self.value.sign() == Sign::Plus
    }

    pub fn is_negative(&self) -> bool {
        self.value.sign() == Sign::Minus
    }

    pub fn is_zero(&self) -> bool {
        self.value == BigInt::from(0)
    }

    pub fn is_one(&self) -> bool {
        self.value == BigInt::from(1)
    }

    pub fn sign(&self) -> i32 {
        match self.value.sign() {
            Sign::Minus => -1,
            Sign::NoSign => 0,
            Sign::Plus => 1,
        }
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }

    pub fn to_hex_string(&self) -> String {
        format!("0x{:x}", self.value)
    }

    pub fn to_binary_string(&self) -> String {
        format!("0b{:b}", self.value)
    }

    pub fn to_octal_string(&self) -> String {
        format!("0o{:o}", self.value)
    }
}

impl fmt::Display for BigNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for BigNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        BigNumber::add(&self, &rhs)
    }
}

impl Sub for BigNumber {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        BigNumber::subtract(&self, &rhs)
    }
}

impl Mul for BigNumber {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        BigNumber::multiply(&self, &rhs)
    }
}

impl Div for BigNumber {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        BigNumber::divide(&self, &rhs).expect("division by zero")
    }
}

impl Neg for BigNumber {
    type Output = Self;
    fn neg(self) -> Self::Output {
        BigNumber::negate(&self)
    }
}

impl BitAnd for BigNumber {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        BigNumber::bitwise_and(&self, &rhs)
    }
}

impl BitOr for BigNumber {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        BigNumber::bitwise_or(&self, &rhs)
    }
}

impl BitXor for BigNumber {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        BigNumber::bitwise_xor(&self, &rhs)
    }
}

impl Not for BigNumber {
    type Output = Self;
    fn not(self) -> Self::Output {
        BigNumber::bitwise_not(&self)
    }
}

pub fn big_number_min(a: &BigNumber, b: &BigNumber) -> BigNumber {
    if a.value < b.value {
        a.clone()
    } else {
        b.clone()
    }
}

pub fn big_number_max(a: &BigNumber, b: &BigNumber) -> BigNumber {
    if a.value > b.value {
        a.clone()
    } else {
        b.clone()
    }
}

pub fn big_number_gcd(a: &BigNumber, b: &BigNumber) -> BigNumber {
    let mut x = a.abs().value.clone();
    let mut y = b.abs().value.clone();
    let zero = BigInt::from(0);
    while y != zero {
        let temp = y.clone();
        y = x % temp.clone();
        x = temp;
    }
    BigNumber::new(x)
}

pub fn big_number_lcm(a: &BigNumber, b: &BigNumber) -> BigNumber {
    if a.is_zero() || b.is_zero() {
        return BigNumber::zero();
    }
    let gcd = big_number_gcd(a, b);
    let product = a.multiply(b);
    product.divide(&gcd).unwrap().abs()
}

pub fn big_number_factorial(n: &BigNumber) -> BigNumber {
    if n.is_negative() {
        panic!("factorial is not defined for negative numbers");
    }
    let mut result = BigInt::from(1);
    let mut i = BigInt::from(2);
    while i <= n.value {
        result *= i.clone();
        i += BigInt::from(1);
    }
    BigNumber::new(result)
}

pub fn big_number_fibonacci(n: &BigNumber) -> BigNumber {
    if n.is_negative() {
        panic!("fibonacci is not defined for negative numbers");
    }
    if n.is_zero() || n.is_one() {
        return n.clone();
    }
    let mut a = BigInt::from(0);
    let mut b = BigInt::from(1);
    let mut i = BigInt::from(2);
    while i <= n.value {
        let temp = b.clone();
        b = a + temp.clone();
        a = temp;
        i += BigInt::from(1);
    }
    BigNumber::new(b)
}
