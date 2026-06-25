package main

import (
	"errors"
)

type MinHeap[T ordered] struct {
	heap []T
}

func NewMinHeap[T ordered]() *MinHeap[T] {
	return &MinHeap[T]{heap: make([]T, 0)}
}

func (mh *MinHeap[T]) Size() int {
	return len(mh.heap)
}

func (mh *MinHeap[T]) IsEmpty() bool {
	return len(mh.heap) == 0
}

func (mh *MinHeap[T]) getParentIndex(index int) int {
	return (index - 1) / 2
}

func (mh *MinHeap[T]) getLeftChildIndex(index int) int {
	return 2*index + 1
}

func (mh *MinHeap[T]) getRightChildIndex(index int) int {
	return 2*index + 2
}

func (mh *MinHeap[T]) hasParent(index int) bool {
	return mh.getParentIndex(index) >= 0
}

func (mh *MinHeap[T]) hasLeftChild(index int) bool {
	return mh.getLeftChildIndex(index) < len(mh.heap)
}

func (mh *MinHeap[T]) hasRightChild(index int) bool {
	return mh.getRightChildIndex(index) < len(mh.heap)
}

func (mh *MinHeap[T]) parent(index int) T {
	return mh.heap[mh.getParentIndex(index)]
}

func (mh *MinHeap[T]) leftChild(index int) T {
	return mh.heap[mh.getLeftChildIndex(index)]
}

func (mh *MinHeap[T]) rightChild(index int) T {
	return mh.heap[mh.getRightChildIndex(index)]
}

func (mh *MinHeap[T]) swap(indexOne, indexTwo int) {
	mh.heap[indexOne], mh.heap[indexTwo] = mh.heap[indexTwo], mh.heap[indexOne]
}

func (mh *MinHeap[T]) heapifyUp() {
	index := len(mh.heap) - 1
	for mh.hasParent(index) && mh.parent(index) > mh.heap[index] {
		mh.swap(mh.getParentIndex(index), index)
		index = mh.getParentIndex(index)
	}
}

func (mh *MinHeap[T]) heapifyDown() {
	index := 0
	for mh.hasLeftChild(index) {
		smallerChildIndex := mh.getLeftChildIndex(index)
		if mh.hasRightChild(index) && mh.rightChild(index) < mh.leftChild(index) {
			smallerChildIndex = mh.getRightChildIndex(index)
		}

		if mh.heap[index] < mh.heap[smallerChildIndex] {
			break
		} else {
			mh.swap(index, smallerChildIndex)
		}
		index = smallerChildIndex
	}
}

func (mh *MinHeap[T]) Peek() (T, error) {
	var zero T
	if mh.IsEmpty() {
		return zero, errors.New("heap is empty")
	}
	return mh.heap[0], nil
}

func (mh *MinHeap[T]) Poll() (T, error) {
	var zero T
	if mh.IsEmpty() {
		return zero, errors.New("heap is empty")
	}
	item := mh.heap[0]
	lastIndex := len(mh.heap) - 1
	lastItem := mh.heap[lastIndex]
	mh.heap = mh.heap[:lastIndex]
	if len(mh.heap) > 0 {
		mh.heap[0] = lastItem
		mh.heapifyDown()
	}
	return item, nil
}

func (mh *MinHeap[T]) Add(item T) {
	mh.heap = append(mh.heap, item)
	mh.heapifyUp()
}

func (mh *MinHeap[T]) ToSlice() []T {
	result := make([]T, len(mh.heap))
	copy(result, mh.heap)
	return result
}

func (mh *MinHeap[T]) Clear() {
	mh.heap = make([]T, 0)
}

type MaxHeap[T ordered] struct {
	heap []T
}

func NewMaxHeap[T ordered]() *MaxHeap[T] {
	return &MaxHeap[T]{heap: make([]T, 0)}
}

func (mh *MaxHeap[T]) Size() int {
	return len(mh.heap)
}

func (mh *MaxHeap[T]) IsEmpty() bool {
	return len(mh.heap) == 0
}

func (mh *MaxHeap[T]) getParentIndex(index int) int {
	return (index - 1) / 2
}

func (mh *MaxHeap[T]) getLeftChildIndex(index int) int {
	return 2*index + 1
}

func (mh *MaxHeap[T]) getRightChildIndex(index int) int {
	return 2*index + 2
}

func (mh *MaxHeap[T]) hasParent(index int) bool {
	return mh.getParentIndex(index) >= 0
}

func (mh *MaxHeap[T]) hasLeftChild(index int) bool {
	return mh.getLeftChildIndex(index) < len(mh.heap)
}

func (mh *MaxHeap[T]) hasRightChild(index int) bool {
	return mh.getRightChildIndex(index) < len(mh.heap)
}

func (mh *MaxHeap[T]) parent(index int) T {
	return mh.heap[mh.getParentIndex(index)]
}

func (mh *MaxHeap[T]) leftChild(index int) T {
	return mh.heap[mh.getLeftChildIndex(index)]
}

func (mh *MaxHeap[T]) rightChild(index int) T {
	return mh.heap[mh.getRightChildIndex(index)]
}

func (mh *MaxHeap[T]) swap(indexOne, indexTwo int) {
	mh.heap[indexOne], mh.heap[indexTwo] = mh.heap[indexTwo], mh.heap[indexOne]
}

func (mh *MaxHeap[T]) heapifyUp() {
	index := len(mh.heap) - 1
	for mh.hasParent(index) && mh.parent(index) < mh.heap[index] {
		mh.swap(mh.getParentIndex(index), index)
		index = mh.getParentIndex(index)
	}
}

func (mh *MaxHeap[T]) heapifyDown() {
	index := 0
	for mh.hasLeftChild(index) {
		largerChildIndex := mh.getLeftChildIndex(index)
		if mh.hasRightChild(index) && mh.rightChild(index) > mh.leftChild(index) {
			largerChildIndex = mh.getRightChildIndex(index)
		}

		if mh.heap[index] > mh.heap[largerChildIndex] {
			break
		} else {
			mh.swap(index, largerChildIndex)
		}
		index = largerChildIndex
	}
}

func (mh *MaxHeap[T]) Peek() (T, error) {
	var zero T
	if mh.IsEmpty() {
		return zero, errors.New("heap is empty")
	}
	return mh.heap[0], nil
}

func (mh *MaxHeap[T]) Poll() (T, error) {
	var zero T
	if mh.IsEmpty() {
		return zero, errors.New("heap is empty")
	}
	item := mh.heap[0]
	lastIndex := len(mh.heap) - 1
	lastItem := mh.heap[lastIndex]
	mh.heap = mh.heap[:lastIndex]
	if len(mh.heap) > 0 {
		mh.heap[0] = lastItem
		mh.heapifyDown()
	}
	return item, nil
}

func (mh *MaxHeap[T]) Add(item T) {
	mh.heap = append(mh.heap, item)
	mh.heapifyUp()
}

func (mh *MaxHeap[T]) ToSlice() []T {
	result := make([]T, len(mh.heap))
	copy(result, mh.heap)
	return result
}

func (mh *MaxHeap[T]) Clear() {
	mh.heap = make([]T, 0)
}
