package main

import (
	"errors"
)

type Stack[T any] struct {
	items []T
}

func NewStack[T any]() *Stack[T] {
	return &Stack[T]{items: make([]T, 0)}
}

func (s *Stack[T]) Size() int {
	return len(s.items)
}

func (s *Stack[T]) IsEmpty() bool {
	return len(s.items) == 0
}

func (s *Stack[T]) Push(item T) {
	s.items = append(s.items, item)
}

func (s *Stack[T]) Pop() (T, error) {
	var zero T
	if s.IsEmpty() {
		return zero, errors.New("stack is empty")
	}
	item := s.items[len(s.items)-1]
	s.items = s.items[:len(s.items)-1]
	return item, nil
}

func (s *Stack[T]) Peek() (T, error) {
	var zero T
	if s.IsEmpty() {
		return zero, errors.New("stack is empty")
	}
	return s.items[len(s.items)-1], nil
}

func (s *Stack[T]) Clear() {
	s.items = make([]T, 0)
}

func (s *Stack[T]) ToSlice() []T {
	result := make([]T, len(s.items))
	copy(result, s.items)
	return result
}

func (s *Stack[T]) Iter() func() (T, bool) {
	index := 0
	return func() (T, bool) {
		if index >= len(s.items) {
			var zero T
			return zero, false
		}
		value := s.items[index]
		index++
		return value, true
	}
}

type Queue[T any] struct {
	items []T
}

func NewQueue[T any]() *Queue[T] {
	return &Queue[T]{items: make([]T, 0)}
}

func (q *Queue[T]) Size() int {
	return len(q.items)
}

func (q *Queue[T]) IsEmpty() bool {
	return len(q.items) == 0
}

func (q *Queue[T]) Enqueue(item T) {
	q.items = append(q.items, item)
}

func (q *Queue[T]) Dequeue() (T, error) {
	var zero T
	if q.IsEmpty() {
		return zero, errors.New("queue is empty")
	}
	item := q.items[0]
	q.items = q.items[1:]
	return item, nil
}

func (q *Queue[T]) Peek() (T, error) {
	var zero T
	if q.IsEmpty() {
		return zero, errors.New("queue is empty")
	}
	return q.items[0], nil
}

func (q *Queue[T]) Clear() {
	q.items = make([]T, 0)
}

func (q *Queue[T]) ToSlice() []T {
	result := make([]T, len(q.items))
	copy(result, q.items)
	return result
}

func (q *Queue[T]) Iter() func() (T, bool) {
	index := 0
	return func() (T, bool) {
		if index >= len(q.items) {
			var zero T
			return zero, false
		}
		value := q.items[index]
		index++
		return value, true
	}
}

type CircularQueue[T any] struct {
	capacity int
	items    []T
	front    int
	rear     int
	size     int
}

func NewCircularQueue[T any](capacity int) (*CircularQueue[T], error) {
	if capacity <= 0 {
		return nil, errors.New("capacity must be positive")
	}
	return &CircularQueue[T]{
		capacity: capacity,
		items:    make([]T, capacity),
		front:    0,
		rear:     -1,
		size:     0,
	}, nil
}

func (cq *CircularQueue[T]) Capacity() int {
	return cq.capacity
}

func (cq *CircularQueue[T]) Size() int {
	return cq.size
}

func (cq *CircularQueue[T]) IsEmpty() bool {
	return cq.size == 0
}

func (cq *CircularQueue[T]) IsFull() bool {
	return cq.size == cq.capacity
}

func (cq *CircularQueue[T]) Enqueue(item T) bool {
	if cq.IsFull() {
		return false
	}
	cq.rear = (cq.rear + 1) % cq.capacity
	cq.items[cq.rear] = item
	cq.size++
	return true
}

func (cq *CircularQueue[T]) Dequeue() (T, error) {
	var zero T
	if cq.IsEmpty() {
		return zero, errors.New("queue is empty")
	}
	item := cq.items[cq.front]
	var zeroVal T
	cq.items[cq.front] = zeroVal
	cq.front = (cq.front + 1) % cq.capacity
	cq.size--
	return item, nil
}

func (cq *CircularQueue[T]) Peek() (T, error) {
	var zero T
	if cq.IsEmpty() {
		return zero, errors.New("queue is empty")
	}
	return cq.items[cq.front], nil
}

func (cq *CircularQueue[T]) Clear() {
	cq.items = make([]T, cq.capacity)
	cq.front = 0
	cq.rear = -1
	cq.size = 0
}

func (cq *CircularQueue[T]) ToSlice() []T {
	result := make([]T, 0, cq.size)
	for i := 0; i < cq.size; i++ {
		index := (cq.front + i) % cq.capacity
		result = append(result, cq.items[index])
	}
	return result
}

func (cq *CircularQueue[T]) Iter() func() (T, bool) {
	i := 0
	size := cq.size
	front := cq.front
	capacity := cq.capacity
	return func() (T, bool) {
		if i >= size {
			var zero T
			return zero, false
		}
		index := (front + i) % capacity
		value := cq.items[index]
		i++
		return value, true
	}
}

type Deque[T any] struct {
	items []T
}

func NewDeque[T any]() *Deque[T] {
	return &Deque[T]{items: make([]T, 0)}
}

func (d *Deque[T]) Size() int {
	return len(d.items)
}

func (d *Deque[T]) IsEmpty() bool {
	return len(d.items) == 0
}

func (d *Deque[T]) AddFront(item T) {
	d.items = append([]T{item}, d.items...)
}

func (d *Deque[T]) AddRear(item T) {
	d.items = append(d.items, item)
}

func (d *Deque[T]) RemoveFront() (T, error) {
	var zero T
	if d.IsEmpty() {
		return zero, errors.New("deque is empty")
	}
	item := d.items[0]
	d.items = d.items[1:]
	return item, nil
}

func (d *Deque[T]) RemoveRear() (T, error) {
	var zero T
	if d.IsEmpty() {
		return zero, errors.New("deque is empty")
	}
	item := d.items[len(d.items)-1]
	d.items = d.items[:len(d.items)-1]
	return item, nil
}

func (d *Deque[T]) PeekFront() (T, error) {
	var zero T
	if d.IsEmpty() {
		return zero, errors.New("deque is empty")
	}
	return d.items[0], nil
}

func (d *Deque[T]) PeekRear() (T, error) {
	var zero T
	if d.IsEmpty() {
		return zero, errors.New("deque is empty")
	}
	return d.items[len(d.items)-1], nil
}

func (d *Deque[T]) Clear() {
	d.items = make([]T, 0)
}

func (d *Deque[T]) ToSlice() []T {
	result := make([]T, len(d.items))
	copy(result, d.items)
	return result
}

func (d *Deque[T]) Iter() func() (T, bool) {
	index := 0
	return func() (T, bool) {
		if index >= len(d.items) {
			var zero T
			return zero, false
		}
		value := d.items[index]
		index++
		return value, true
	}
}
