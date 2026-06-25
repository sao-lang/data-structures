package main

import (
	"errors"
)

type ListNode[T any] struct {
	value T
	next  *ListNode[T]
}

type SinglyLinkedList[T any] struct {
	head   *ListNode[T]
	tail   *ListNode[T]
	length int
}

func NewSinglyLinkedList[T any]() *SinglyLinkedList[T] {
	return &SinglyLinkedList[T]{
		head:   nil,
		tail:   nil,
		length: 0,
	}
}

func (sll *SinglyLinkedList[T]) Length() int {
	return sll.length
}

func (sll *SinglyLinkedList[T]) IsEmpty() bool {
	return sll.length == 0
}

func (sll *SinglyLinkedList[T]) Head() (T, error) {
	var zero T
	if sll.head == nil {
		return zero, errors.New("list is empty")
	}
	return sll.head.value, nil
}

func (sll *SinglyLinkedList[T]) Tail() (T, error) {
	var zero T
	if sll.tail == nil {
		return zero, errors.New("list is empty")
	}
	return sll.tail.value, nil
}

func (sll *SinglyLinkedList[T]) Prepend(value T) {
	newNode := &ListNode[T]{value: value, next: nil}
	if sll.head == nil {
		sll.head = newNode
		sll.tail = newNode
	} else {
		newNode.next = sll.head
		sll.head = newNode
	}
	sll.length++
}

func (sll *SinglyLinkedList[T]) Append(value T) {
	newNode := &ListNode[T]{value: value, next: nil}
	if sll.tail == nil {
		sll.head = newNode
		sll.tail = newNode
	} else {
		sll.tail.next = newNode
		sll.tail = newNode
	}
	sll.length++
}

func (sll *SinglyLinkedList[T]) RemoveFirst() (T, error) {
	var zero T
	if sll.head == nil {
		return zero, errors.New("list is empty")
	}
	removedNode := sll.head
	sll.head = sll.head.next
	if sll.head == nil {
		sll.tail = nil
	}
	sll.length--
	return removedNode.value, nil
}

func (sll *SinglyLinkedList[T]) Clear() {
	sll.head = nil
	sll.tail = nil
	sll.length = 0
}

func (sll *SinglyLinkedList[T]) ToSlice() []T {
	result := make([]T, 0, sll.length)
	current := sll.head
	for current != nil {
		result = append(result, current.value)
		current = current.next
	}
	return result
}

func (sll *SinglyLinkedList[T]) Iter() func() (T, bool) {
	current := sll.head
	return func() (T, bool) {
		if current == nil {
			var zero T
			return zero, false
		}
		value := current.value
		current = current.next
		return value, true
	}
}

// 单向循环链表
type CircularLinkedList[T any] struct {
	tail   *ListNode[T]
	length int
}

func NewCircularLinkedList[T any]() *CircularLinkedList[T] {
	return &CircularLinkedList[T]{
		tail:   nil,
		length: 0,
	}
}

func (cll *CircularLinkedList[T]) Length() int {
	return cll.length
}

func (cll *CircularLinkedList[T]) IsEmpty() bool {
	return cll.length == 0
}

func (cll *CircularLinkedList[T]) Head() (T, error) {
	var zero T
	if cll.tail == nil {
		return zero, errors.New("list is empty")
	}
	return cll.tail.next.value, nil
}

func (cll *CircularLinkedList[T]) Tail() (T, error) {
	var zero T
	if cll.tail == nil {
		return zero, errors.New("list is empty")
	}
	return cll.tail.value, nil
}

func (cll *CircularLinkedList[T]) Prepend(value T) {
	newNode := &ListNode[T]{value: value, next: nil}
	if cll.tail == nil {
		newNode.next = newNode
		cll.tail = newNode
	} else {
		newNode.next = cll.tail.next
		cll.tail.next = newNode
	}
	cll.length++
}

func (cll *CircularLinkedList[T]) Append(value T) {
	newNode := &ListNode[T]{value: value, next: nil}
	if cll.tail == nil {
		newNode.next = newNode
		cll.tail = newNode
	} else {
		newNode.next = cll.tail.next
		cll.tail.next = newNode
		cll.tail = newNode
	}
	cll.length++
}

func (cll *CircularLinkedList[T]) RemoveFirst() (T, error) {
	var zero T
	if cll.tail == nil {
		return zero, errors.New("list is empty")
	}
	removedNode := cll.tail.next
	if cll.tail == removedNode {
		cll.tail = nil
	} else {
		cll.tail.next = removedNode.next
	}
	cll.length--
	return removedNode.value, nil
}

func (cll *CircularLinkedList[T]) Clear() {
	cll.tail = nil
	cll.length = 0
}

func (cll *CircularLinkedList[T]) ToSlice() []T {
	result := make([]T, 0, cll.length)
	if cll.tail == nil {
		return result
	}
	current := cll.tail.next
	for i := 0; i < cll.length; i++ {
		result = append(result, current.value)
		current = current.next
	}
	return result
}

func (cll *CircularLinkedList[T]) Iter() func() (T, bool) {
	var current *ListNode[T]
	count := 0
	length := cll.length
	if cll.tail != nil {
		current = cll.tail.next
	}
	return func() (T, bool) {
		if count >= length || current == nil {
			var zero T
			return zero, false
		}
		value := current.value
		current = current.next
		count++
		return value, true
	}
}

// 双向链表
type DoublyListNode[T any] struct {
	value T
	next  *DoublyListNode[T]
	prev  *DoublyListNode[T]
}

type DoublyLinkedList[T any] struct {
	head   *DoublyListNode[T]
	tail   *DoublyListNode[T]
	length int
}

func NewDoublyLinkedList[T any]() *DoublyLinkedList[T] {
	return &DoublyLinkedList[T]{
		head:   nil,
		tail:   nil,
		length: 0,
	}
}

func (dll *DoublyLinkedList[T]) Length() int {
	return dll.length
}

func (dll *DoublyLinkedList[T]) IsEmpty() bool {
	return dll.length == 0
}

func (dll *DoublyLinkedList[T]) Head() (T, error) {
	var zero T
	if dll.head == nil {
		return zero, errors.New("list is empty")
	}
	return dll.head.value, nil
}

func (dll *DoublyLinkedList[T]) Tail() (T, error) {
	var zero T
	if dll.tail == nil {
		return zero, errors.New("list is empty")
	}
	return dll.tail.value, nil
}

func (dll *DoublyLinkedList[T]) Prepend(value T) {
	newNode := &DoublyListNode[T]{value: value, next: nil, prev: nil}
	if dll.head == nil {
		dll.head = newNode
		dll.tail = newNode
	} else {
		newNode.next = dll.head
		dll.head.prev = newNode
		dll.head = newNode
	}
	dll.length++
}

func (dll *DoublyLinkedList[T]) Append(value T) {
	newNode := &DoublyListNode[T]{value: value, next: nil, prev: nil}
	if dll.tail == nil {
		dll.head = newNode
		dll.tail = newNode
	} else {
		newNode.prev = dll.tail
		dll.tail.next = newNode
		dll.tail = newNode
	}
	dll.length++
}

func (dll *DoublyLinkedList[T]) RemoveFirst() (T, error) {
	var zero T
	if dll.head == nil {
		return zero, errors.New("list is empty")
	}
	removedNode := dll.head
	dll.head = dll.head.next
	if dll.head == nil {
		dll.tail = nil
	} else {
		dll.head.prev = nil
	}
	dll.length--
	return removedNode.value, nil
}

func (dll *DoublyLinkedList[T]) RemoveLast() (T, error) {
	var zero T
	if dll.tail == nil {
		return zero, errors.New("list is empty")
	}
	removedNode := dll.tail
	dll.tail = dll.tail.prev
	if dll.tail == nil {
		dll.head = nil
	} else {
		dll.tail.next = nil
	}
	dll.length--
	return removedNode.value, nil
}

func (dll *DoublyLinkedList[T]) Clear() {
	dll.head = nil
	dll.tail = nil
	dll.length = 0
}

func (dll *DoublyLinkedList[T]) ToSlice() []T {
	result := make([]T, 0, dll.length)
	current := dll.head
	for current != nil {
		result = append(result, current.value)
		current = current.next
	}
	return result
}

func (dll *DoublyLinkedList[T]) ToSliceReverse() []T {
	result := make([]T, 0, dll.length)
	current := dll.tail
	for current != nil {
		result = append(result, current.value)
		current = current.prev
	}
	return result
}

func (dll *DoublyLinkedList[T]) Iter() func() (T, bool) {
	current := dll.head
	return func() (T, bool) {
		if current == nil {
			var zero T
			return zero, false
		}
		value := current.value
		current = current.next
		return value, true
	}
}

type DoublyCircularLinkedList[T any] struct {
	tail   *DoublyListNode[T]
	length int
}

func NewDoublyCircularLinkedList[T any]() *DoublyCircularLinkedList[T] {
	return &DoublyCircularLinkedList[T]{
		tail:   nil,
		length: 0,
	}
}

func (dcll *DoublyCircularLinkedList[T]) Length() int {
	return dcll.length
}

func (dcll *DoublyCircularLinkedList[T]) IsEmpty() bool {
	return dcll.length == 0
}

func (dcll *DoublyCircularLinkedList[T]) Head() (T, error) {
	var zero T
	if dcll.tail == nil {
		return zero, errors.New("list is empty")
	}
	return dcll.tail.next.value, nil
}

func (dcll *DoublyCircularLinkedList[T]) Tail() (T, error) {
	var zero T
	if dcll.tail == nil {
		return zero, errors.New("list is empty")
	}
	return dcll.tail.value, nil
}

func (dcll *DoublyCircularLinkedList[T]) Prepend(value T) {
	newNode := &DoublyListNode[T]{value: value, next: nil, prev: nil}
	if dcll.tail == nil {
		newNode.next = newNode
		newNode.prev = newNode
		dcll.tail = newNode
	} else {
		newNode.next = dcll.tail.next
		newNode.prev = dcll.tail
		dcll.tail.next.prev = newNode
		dcll.tail.next = newNode
	}
	dcll.length++
}

func (dcll *DoublyCircularLinkedList[T]) Append(value T) {
	newNode := &DoublyListNode[T]{value: value, next: nil, prev: nil}
	if dcll.tail == nil {
		newNode.next = newNode
		newNode.prev = newNode
		dcll.tail = newNode
	} else {
		newNode.next = dcll.tail.next
		newNode.prev = dcll.tail
		dcll.tail.next.prev = newNode
		dcll.tail.next = newNode
		dcll.tail = newNode
	}
	dcll.length++
}

func (dcll *DoublyCircularLinkedList[T]) RemoveFirst() (T, error) {
	var zero T
	if dcll.tail == nil {
		return zero, errors.New("list is empty")
	}
	removedNode := dcll.tail.next
	if dcll.tail == removedNode {
		dcll.tail = nil
	} else {
		dcll.tail.next = removedNode.next
		removedNode.next.prev = dcll.tail
	}
	dcll.length--
	return removedNode.value, nil
}

func (dcll *DoublyCircularLinkedList[T]) RemoveLast() (T, error) {
	var zero T
	if dcll.tail == nil {
		return zero, errors.New("list is empty")
	}
	removedNode := dcll.tail
	if dcll.tail == removedNode.next {
		dcll.tail = nil
	} else {
		dcll.tail = removedNode.prev
		dcll.tail.next = removedNode.next
		removedNode.next.prev = dcll.tail
	}
	dcll.length--
	return removedNode.value, nil
}

func (dcll *DoublyCircularLinkedList[T]) Clear() {
	dcll.tail = nil
	dcll.length = 0
}

func (dcll *DoublyCircularLinkedList[T]) ToSlice() []T {
	result := make([]T, 0, dcll.length)
	if dcll.tail == nil {
		return result
	}
	current := dcll.tail.next
	for i := 0; i < dcll.length; i++ {
		result = append(result, current.value)
		current = current.next
	}
	return result
}

func (dcll *DoublyCircularLinkedList[T]) ToSliceReverse() []T {
	result := make([]T, 0, dcll.length)
	if dcll.tail == nil {
		return result
	}
	current := dcll.tail
	for i := 0; i < dcll.length; i++ {
		result = append(result, current.value)
		current = current.prev
	}
	return result
}

func (dcll *DoublyCircularLinkedList[T]) Iter() func() (T, bool) {
	var current *DoublyListNode[T]
	count := 0
	length := dcll.length
	if dcll.tail != nil {
		current = dcll.tail.next
	}
	return func() (T, bool) {
		if count >= length || current == nil {
			var zero T
			return zero, false
		}
		value := current.value
		current = current.next
		count++
		return value, true
	}
}
