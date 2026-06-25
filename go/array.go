package main

import (
	"errors"
)

type FixedArray[T any] struct {
	capacity int
	length   int
	data     []T
}

func NewFixedArray[T any](capacity int) (*FixedArray[T], error) {
	if capacity <= 0 {
		return nil, errors.New("capacity must be positive")
	}
	return &FixedArray[T]{
		capacity: capacity,
		length:   0,
		data:     make([]T, capacity),
	}, nil
}

func (fa *FixedArray[T]) Capacity() int {
	return fa.capacity
}

func (fa *FixedArray[T]) Length() int {
	return fa.length
}

func (fa *FixedArray[T]) IsEmpty() bool {
	return fa.length == 0
}

func (fa *FixedArray[T]) IsFull() bool {
	return fa.length == fa.capacity
}

func (fa *FixedArray[T]) At(index int) (T, error) {
	var zero T
	if index < 0 || index >= fa.length {
		return zero, errors.New("index out of bounds")
	}
	return fa.data[index], nil
}

func (fa *FixedArray[T]) Set(index int, value T) error {
	if index < 0 || index >= fa.length {
		return errors.New("index out of bounds")
	}
	fa.data[index] = value
	return nil
}

func (fa *FixedArray[T]) Push(value T) error {
	if fa.IsFull() {
		return errors.New("array is full")
	}
	fa.data[fa.length] = value
	fa.length++
	return nil
}

func (fa *FixedArray[T]) Pop() (T, error) {
	var zero T
	if fa.IsEmpty() {
		return zero, errors.New("array is empty")
	}
	fa.length--
	value := fa.data[fa.length]
	var zeroVal T
	fa.data[fa.length] = zeroVal
	return value, nil
}

func (fa *FixedArray[T]) Insert(index int, value T) error {
	if fa.IsFull() {
		return errors.New("array is full")
	}
	if index < 0 || index > fa.length {
		return errors.New("index out of bounds")
	}
	for i := fa.length; i > index; i-- {
		fa.data[i] = fa.data[i-1]
	}
	fa.data[index] = value
	fa.length++
	return nil
}

func (fa *FixedArray[T]) Remove(index int) (T, error) {
	var zero T
	if fa.IsEmpty() {
		return zero, errors.New("array is empty")
	}
	if index < 0 || index >= fa.length {
		return zero, errors.New("index out of bounds")
	}
	value := fa.data[index]
	for i := index; i < fa.length-1; i++ {
		fa.data[i] = fa.data[i+1]
	}
	fa.length--
	var zeroVal T
	fa.data[fa.length] = zeroVal
	return value, nil
}

func (fa *FixedArray[T]) Find(value T, equal func(a, b T) bool) int {
	for i := 0; i < fa.length; i++ {
		if equal(fa.data[i], value) {
			return i
		}
	}
	return -1
}

func (fa *FixedArray[T]) ToArray() []T {
	result := make([]T, fa.length)
	copy(result, fa.data[:fa.length])
	return result
}

func (fa *FixedArray[T]) Clear() {
	fa.data = make([]T, fa.capacity)
	fa.length = 0
}

func (fa *FixedArray[T]) Iter() func() (T, bool) {
	index := 0
	length := fa.length
	return func() (T, bool) {
		if index >= length {
			var zero T
			return zero, false
		}
		value := fa.data[index]
		index++
		return value, true
	}
}

type DynamicArray[T any] struct {
	capacity     int
	length       int
	data         []T
	growthFactor int
}

func NewDynamicArray[T any](initialCapacity int) (*DynamicArray[T], error) {
	if initialCapacity <= 0 {
		return nil, errors.New("initial capacity must be positive")
	}
	return &DynamicArray[T]{
		capacity:     initialCapacity,
		length:       0,
		data:         make([]T, initialCapacity),
		growthFactor: 2,
	}, nil
}

func (da *DynamicArray[T]) Capacity() int {
	return da.capacity
}

func (da *DynamicArray[T]) Length() int {
	return da.length
}

func (da *DynamicArray[T]) IsEmpty() bool {
	return da.length == 0
}

func (da *DynamicArray[T]) resize() {
	newCapacity := da.capacity * da.growthFactor
	newData := make([]T, newCapacity)
	copy(newData, da.data)
	da.data = newData
	da.capacity = newCapacity
}

func (da *DynamicArray[T]) At(index int) (T, error) {
	var zero T
	if index < 0 || index >= da.length {
		return zero, errors.New("index out of bounds")
	}
	return da.data[index], nil
}

func (da *DynamicArray[T]) Set(index int, value T) error {
	if index < 0 || index >= da.length {
		return errors.New("index out of bounds")
	}
	da.data[index] = value
	return nil
}

func (da *DynamicArray[T]) Push(value T) {
	if da.length >= da.capacity {
		da.resize()
	}
	da.data[da.length] = value
	da.length++
}

func (da *DynamicArray[T]) Pop() (T, error) {
	var zero T
	if da.IsEmpty() {
		return zero, errors.New("array is empty")
	}
	da.length--
	value := da.data[da.length]
	var zeroVal T
	da.data[da.length] = zeroVal
	return value, nil
}

func (da *DynamicArray[T]) Insert(index int, value T) error {
	if index < 0 || index > da.length {
		return errors.New("index out of bounds")
	}
	if da.length >= da.capacity {
		da.resize()
	}
	for i := da.length; i > index; i-- {
		da.data[i] = da.data[i-1]
	}
	da.data[index] = value
	da.length++
	return nil
}

func (da *DynamicArray[T]) Remove(index int) (T, error) {
	var zero T
	if da.IsEmpty() {
		return zero, errors.New("array is empty")
	}
	if index < 0 || index >= da.length {
		return zero, errors.New("index out of bounds")
	}
	value := da.data[index]
	for i := index; i < da.length-1; i++ {
		da.data[i] = da.data[i+1]
	}
	da.length--
	var zeroVal T
	da.data[da.length] = zeroVal
	return value, nil
}

func (da *DynamicArray[T]) Find(value T, equal func(a, b T) bool) int {
	for i := 0; i < da.length; i++ {
		if equal(da.data[i], value) {
			return i
		}
	}
	return -1
}

func (da *DynamicArray[T]) ToArray() []T {
	result := make([]T, da.length)
	copy(result, da.data[:da.length])
	return result
}

func (da *DynamicArray[T]) Clear() {
	da.data = make([]T, 10)
	da.capacity = 10
	da.length = 0
}

func (da *DynamicArray[T]) Sort(less func(a, b T) bool) {
	for i := 0; i < da.length; i++ {
		for j := i + 1; j < da.length; j++ {
			if less(da.data[j], da.data[i]) {
				da.data[i], da.data[j] = da.data[j], da.data[i]
			}
		}
	}
}

func (da *DynamicArray[T]) Iter() func() (T, bool) {
	index := 0
	length := da.length
	return func() (T, bool) {
		if index >= length {
			var zero T
			return zero, false
		}
		value := da.data[index]
		index++
		return value, true
	}
}
