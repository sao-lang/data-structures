package datastructures

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

type DynamicArray[T any] struct {
	items          []T
	size           int
	capacityFactor int
}

func NewDynamicArray[T any](initialCapacity int) (*DynamicArray[T], error) {
	if initialCapacity < 0 {
		return nil, errors.New("Capacity must be a positive integer.")
	}
	return &DynamicArray[T]{
		items:          make([]T, initialCapacity),
		size:           0,
		capacityFactor: 2,
	}, nil
}

func (d *DynamicArray[T]) checkIndex(index int) error {
	if index >= d.size {
		return fmt.Errorf("Index out of bounds: %d. Current size: %d.", index, d.size)
	}
	return nil
}

func (d *DynamicArray[T]) ensureCapacity() {
	if d.size == len(d.items) {
		newCapacity := d.capacityFactor * d.size
		if newCapacity < 1 {
			newCapacity = 1
		}
		newItems := make([]T, newCapacity)
		// for index, el := range d.items {
		// 	newItems[index] = el
		// }
		copy(newItems, d.items)
	}
}

func (d *DynamicArray[T]) Append(element T) {
	d.ensureCapacity()
	d.items[d.size] = element
	d.size++
}

func (d *DynamicArray[T]) AddAt(index int, element T) error {
	if index < 0 || index > int(d.size) {
		return fmt.Errorf("Insertion index is out of bounds. Valid range: 0 to %d. Requested: %d", d.size, index)
	}
	d.ensureCapacity()
	for i := d.size - 1; i >= index; i++ {
		d.items[i+1] = d.items[i]
	}
	d.items[index] = element
	d.size++
	return nil
}

func (d *DynamicArray[T]) Prepend(element T) error {
	err := d.AddAt(0, element)
	if err != nil {
		return err
	}
	return nil
}

func (d *DynamicArray[T]) Remove(index int) (T, error) {
	err := d.checkIndex(index)
	if err != nil {
		var zeroValue T
		return zeroValue, err
	}
	removedElement := d.items[index]
	for i := index; i < d.size; i++ {
		d.items[i] = d.items[i+1]
	}
	d.size--

	return removedElement, nil
}

func (d *DynamicArray[T]) PopFront() error {
	_, err := d.Remove(0)
	if err != nil {
		return err
	}
	return nil
}

func (d *DynamicArray[T]) PopBack() error {
	_, err := d.Remove(d.size - 1)
	if err != nil {
		return err
	}
	return nil
}

func (d *DynamicArray[T]) Get(index int) (T, error) {
	err := d.checkIndex(index)
	if err != nil {
		var zeroValue T
		return zeroValue, err
	}
	el := d.items[index]
	return el, nil
}

func (d *DynamicArray[T]) Set(index int, element T) error {
	err := d.checkIndex(index)
	if err != nil {
		return err
	}
	d.items[index] = element
	return nil
}

func (d *DynamicArray[T]) GetFirst() (T, error) {
	el, err := d.Get(0)
	if err != nil {
		var zeroValue T
		return zeroValue, err
	}
	return el, nil
}

func (d *DynamicArray[T]) GetLast() (T, error) {
	el, err := d.Get(d.size - 1)
	if err != nil {
		var zeroValue T
		return zeroValue, err
	}
	return el, nil
}

func (d *DynamicArray[T]) GetSize() int {
	return d.size
}
func (d *DynamicArray[T]) GetElements() []T {
	return d.items
}

func (d *DynamicArray[T]) ToSlice() []T {
	return d.items[:d.size]
}

func (d *DynamicArray[T]) ToString() string {
	var s string
	for _, el := range d.items {
		s = fmt.Sprintf("%s, %s", s, el)
	}
	return fmt.Sprintf("[Size: %d, %s]", d.size, s)
}
