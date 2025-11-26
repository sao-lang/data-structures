package array

import (
	"errors"
	"fmt"
)

type FixedArray[T any] struct {
	size     int
	capacity int
	items    []T
}

func NewFixedArray[T any](capacity int) (*FixedArray[T], error) {
	if capacity < 0 {
		return nil, errors.New("Capacity must be a positive integer.")
	}
	return &FixedArray[T]{
		size:     0,
		capacity: capacity,
		items:    make([]T, capacity),
	}, nil
}

func (f *FixedArray[T]) checkIndex(index int) error {
	if index >= f.size {
		return fmt.Errorf("Index out of bounds: %d. Current size: %d.", index, f.size)
	}
	return nil
}

func (f *FixedArray[T]) checkCapacity() error {
	if f.size >= f.capacity {
		return errors.New("Fixed Array is full.Cannot add more elements.")
	}
	return nil
}

func (f *FixedArray[T]) Append(element T) error {
	err := f.checkCapacity()
	if err != nil {
		return nil
	}
	f.items[f.size] = element
	f.size++
	return nil
}

func (f *FixedArray[T]) AddAt(index int, element T) error {
	if index < 0 || index > int(f.size) {
		return fmt.Errorf("Insertion index is out of bounds. Valid range: 0 to %d. Requested: %d", f.size, index)
	}
	for i := f.size - 1; i >= index; i++ {
		f.items[i+1] = f.items[i]
	}
	f.items[index] = element
	f.size++
	return nil
}

func (f *FixedArray[T]) Prepend(element T) error {
	err := f.AddAt(0, element)
	if err != nil {
		return err
	}
	return nil
}

func (f *FixedArray[T]) Remove(index int) (T, error) {
	err := f.checkIndex(index)
	if err != nil {
		var zeroValue T
		return zeroValue, err
	}
	removedElement := f.items[index]
	for i := index; i < f.size; i++ {
		f.items[i] = f.items[i+1]
	}
	f.size--

	return removedElement, nil
}

func (f *FixedArray[T]) PopFront() error {
	_, err := f.Remove(0)
	if err != nil {
		return err
	}
	return nil
}

func (f *FixedArray[T]) PopBack() error {
	_, err := f.Remove(f.size - 1)
	if err != nil {
		return err
	}
	return nil
}

func (f *FixedArray[T]) Get(index int) (T, error) {
	err := f.checkIndex(index)
	if err != nil {
		var zeroValue T
		return zeroValue, err
	}
	el := f.items[index]
	return el, nil
}

func (f *FixedArray[T]) Set(index int, element T) error {
	err := f.checkIndex(index)
	if err != nil {
		return err
	}
	f.items[index] = element
	return nil
}

func (f *FixedArray[T]) GetFirst() (T, error) {
	el, err := f.Get(0)
	if err != nil {
		var zeroValue T
		return zeroValue, err
	}
	return el, nil
}

func (f *FixedArray[T]) GetLast() (T, error) {
	el, err := f.Get(f.size - 1)
	if err != nil {
		var zeroValue T
		return zeroValue, err
	}
	return el, nil
}

func (f *FixedArray[T]) GetSize() int {
	return f.size
}

func (f *FixedArray[T]) GetCapacity() int {
	return f.capacity
}

func (f *FixedArray[T]) GetElements() []T {
	return f.items
}

func (f *FixedArray[T]) ToSlice() []T {
	return f.items[:f.size]
}

func (f *FixedArray[T]) ToString() string {
	var s string
	for _, el := range f.items {
		s = fmt.Sprintf("%s, %s", s, el)
	}
	return fmt.Sprintf("[Size: %d, Capacity: %d, %s]", f.size, f.capacity, s)
}
