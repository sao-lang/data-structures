package main

import (
	"errors"
	"fmt"
)

type Tuple[T any] struct {
	items []T
}

func NewTuple[T any](items ...T) *Tuple[T] {
	return &Tuple[T]{
		items: append([]T{}, items...),
	}
}

func (t *Tuple[T]) Size() int {
	return len(t.items)
}

func (t *Tuple[T]) IsEmpty() bool {
	return len(t.items) == 0
}

func (t *Tuple[T]) At(index int) (T, error) {
	var zero T
	if index < 0 || index >= len(t.items) {
		return zero, errors.New("index out of bounds")
	}
	return t.items[index], nil
}

func (t *Tuple[T]) First() (T, error) {
	var zero T
	if t.IsEmpty() {
		return zero, errors.New("tuple is empty")
	}
	return t.items[0], nil
}

func (t *Tuple[T]) Last() (T, error) {
	var zero T
	if t.IsEmpty() {
		return zero, errors.New("tuple is empty")
	}
	return t.items[len(t.items)-1], nil
}

func (t *Tuple[T]) ToSlice() []T {
	return append([]T{}, t.items...)
}

func (t *Tuple[T]) Map(f func(T) T) *Tuple[T] {
	result := make([]T, len(t.items))
	for i, item := range t.items {
		result[i] = f(item)
	}
	return NewTuple(result...)
}

func (t *Tuple[T]) Filter(f func(T) bool) *Tuple[T] {
	var result []T
	for _, item := range t.items {
		if f(item) {
			result = append(result, item)
		}
	}
	return NewTuple(result...)
}

func (t *Tuple[T]) Reduce(f func(T, T) T, initial T) T {
	result := initial
	for _, item := range t.items {
		result = f(result, item)
	}
	return result
}

func (t *Tuple[T]) Concat(other *Tuple[T]) *Tuple[T] {
	result := append(t.ToSlice(), other.ToSlice()...)
	return NewTuple(result...)
}

func (t *Tuple[T]) Slice(start, end int) *Tuple[T] {
	if start < 0 {
		start = 0
	}
	if end > len(t.items) {
		end = len(t.items)
	}
	if start > end {
		return NewTuple[T]()
	}
	return NewTuple(t.items[start:end]...)
}

func (t *Tuple[T]) Take(n int) *Tuple[T] {
	if n <= 0 {
		return NewTuple[T]()
	}
	if n > len(t.items) {
		n = len(t.items)
	}
	return NewTuple(t.items[:n]...)
}

func (t *Tuple[T]) Drop(n int) *Tuple[T] {
	if n <= 0 {
		return NewTuple(t.items...)
	}
	if n >= len(t.items) {
		return NewTuple[T]()
	}
	return NewTuple(t.items[n:]...)
}

func (t *Tuple[T]) Contains(item T, equal func(a, b T) bool) bool {
	for _, i := range t.items {
		if equal(i, item) {
			return true
		}
	}
	return false
}

func (t *Tuple[T]) Index(item T, equal func(a, b T) bool) int {
	for i, it := range t.items {
		if equal(it, item) {
			return i
		}
	}
	return -1
}

func (t *Tuple[T]) Count(item T, equal func(a, b T) bool) int {
	count := 0
	for _, it := range t.items {
		if equal(it, item) {
			count++
		}
	}
	return count
}

func (t *Tuple[T]) Reverse() *Tuple[T] {
	n := len(t.items)
	result := make([]T, n)
	for i := 0; i < n; i++ {
		result[i] = t.items[n-1-i]
	}
	return NewTuple(result...)
}

func (t *Tuple[T]) String() string {
	return fmt.Sprintf("Tuple%v", t.items)
}

type Pair[T, U any] struct {
	first  T
	second U
}

func NewPair[T, U any](first T, second U) *Pair[T, U] {
	return &Pair[T, U]{
		first:  first,
		second: second,
	}
}

func (p *Pair[T, U]) First() T {
	return p.first
}

func (p *Pair[T, U]) Second() U {
	return p.second
}

func (p *Pair[T, U]) Swap() *Pair[U, T] {
	return NewPair(p.second, p.first)
}

func (p *Pair[T, U]) ToTuple() *Tuple[any] {
	return NewTuple[any](p.first, p.second)
}

func (p *Pair[T, U]) MapFirst(f func(T) T) *Pair[T, U] {
	return NewPair(f(p.first), p.second)
}

func (p *Pair[T, U]) MapSecond(f func(U) U) *Pair[T, U] {
	return NewPair(p.first, f(p.second))
}

func (p *Pair[T, U]) MapBoth(f1 func(T) T, f2 func(U) U) *Pair[T, U] {
	return NewPair(f1(p.first), f2(p.second))
}

func (p *Pair[T, U]) String() string {
	return fmt.Sprintf("Pair(%v, %v)", p.first, p.second)
}

type Triple[T, U, V any] struct {
	first  T
	second U
	third  V
}

func NewTriple[T, U, V any](first T, second U, third V) *Triple[T, U, V] {
	return &Triple[T, U, V]{
		first:  first,
		second: second,
		third:  third,
	}
}

func (t *Triple[T, U, V]) First() T {
	return t.first
}

func (t *Triple[T, U, V]) Second() U {
	return t.second
}

func (t *Triple[T, U, V]) Third() V {
	return t.third
}

func (t *Triple[T, U, V]) ToTuple() *Tuple[any] {
	return NewTuple[any](t.first, t.second, t.third)
}

func (t *Triple[T, U, V]) MapFirst(f func(T) T) *Triple[T, U, V] {
	return NewTriple(f(t.first), t.second, t.third)
}

func (t *Triple[T, U, V]) MapSecond(f func(U) U) *Triple[T, U, V] {
	return NewTriple(t.first, f(t.second), t.third)
}

func (t *Triple[T, U, V]) MapThird(f func(V) V) *Triple[T, U, V] {
	return NewTriple(t.first, t.second, f(t.third))
}

func (t *Triple[T, U, V]) String() string {
	return fmt.Sprintf("Triple(%v, %v, %v)", t.first, t.second, t.third)
}

func Zip[T, U any](t1 *Tuple[T], t2 *Tuple[U]) *Tuple[*Pair[T, U]] {
	minLen := len(t1.items)
	if len(t2.items) < minLen {
		minLen = len(t2.items)
	}
	result := make([]*Pair[T, U], minLen)
	for i := 0; i < minLen; i++ {
		result[i] = NewPair(t1.items[i], t2.items[i])
	}
	return NewTuple(result...)
}

// func main() {
// 	fmt.Println("=== Tuple Example ===")
// 	t1 := NewTuple(1, 2, 3, 4, 5)
// 	fmt.Printf("Tuple: %v\n", t1)
// 	fmt.Printf("Size: %d\n", t1.Size())
// 	first, _ := t1.First()
// 	fmt.Printf("First: %d\n", first)
// 	last, _ := t1.Last()
// 	fmt.Printf("Last: %d\n", last)
// 	at2, _ := t1.At(2)
// 	fmt.Printf("At index 2: %d\n", at2)
// 	fmt.Println()

// 	fmt.Println("=== Tuple Operations ===")
// 	t2 := NewTuple("a", "b", "c")
// 	fmt.Printf("Tuple t2: %v\n", t2)
// 	t2Int := NewTuple(6, 7, 8)
// 	fmt.Printf("Concat t1 + [6,7,8]: %v\n", t1.Concat(t2Int))
// 	fmt.Printf("Slice t1[1:4]: %v\n", t1.Slice(1, 4))
// 	fmt.Printf("Take 3 from t1: %v\n", t1.Take(3))
// 	fmt.Printf("Drop 2 from t1: %v\n", t1.Drop(2))
// 	fmt.Printf("Reverse t1: %v\n", t1.Reverse())
// 	fmt.Printf("Map t1 (x * 2): %v\n", t1.Map(func(x int) int { return x * 2 }))
// 	fmt.Printf("Filter t1 (even): %v\n", t1.Filter(func(x int) bool { return x%2 == 0 }))
// 	fmt.Printf("Reduce t1 (sum): %d\n", t1.Reduce(func(a, b int) int { return a + b }, 0))
// 	fmt.Println()

// 	fmt.Println("=== Pair Example ===")
// 	p := NewPair(10, "hello")
// 	fmt.Printf("Pair: %v\n", p)
// 	fmt.Printf("First: %d\n", p.First())
// 	fmt.Printf("Second: %s\n", p.Second())
// 	fmt.Printf("Swap: %v\n", p.Swap())
// 	fmt.Printf("Map first (+5): %v\n", p.MapFirst(func(x int) int { return x + 5 }))
// 	fmt.Printf("Map second (upper): %v\n", p.MapSecond(func(s string) string {
// 		result := ""
// 		for _, c := range s {
// 			if c >= 'a' && c <= 'z' {
// 				result += string(c - 32)
// 			} else {
// 				result += string(c)
// 			}
// 		}
// 		return result
// 	}))
// 	fmt.Println()

// 	fmt.Println("=== Triple Example ===")
// 	tri := NewTriple("a", 100, true)
// 	fmt.Printf("Triple: %v\n", tri)
// 	fmt.Printf("First: %s\n", tri.First())
// 	fmt.Printf("Second: %d\n", tri.Second())
// 	fmt.Printf("Third: %t\n", tri.Third())
// 	fmt.Println()

// 	fmt.Println("=== Zip Example ===")
// 	t3 := NewTuple(1, 2, 3)
// 	t4 := NewTuple("x", "y", "z")
// 	zipped := Zip(t3, t4)
// 	fmt.Printf("Zip %v and %v: ", t3, t4)
// 	fmt.Print("Tuple(")
// 	for i, pair := range zipped.ToSlice() {
// 		if i > 0 {
// 			fmt.Print(", ")
// 		}
// 		fmt.Printf("(%d, %s)", pair.First(), pair.Second())
// 	}
// 	fmt.Println(")")
// 	fmt.Println()

// 	fmt.Println("=== Comparison Example ===")
// 	t5 := NewTuple(1, 2, 3)
// 	eq := func(a, b int) bool { return a == b }
// 	fmt.Printf("Contains 2 in t5: %t\n", t5.Contains(2, eq))
// 	fmt.Printf("Index of 3 in t5: %d\n", t5.Index(3, eq))
// }
