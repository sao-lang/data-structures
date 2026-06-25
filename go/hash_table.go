package main

import (
	"errors"
	"fmt"
)

type HashNode[K comparable, V any] struct {
	key   K
	value V
	next  *HashNode[K, V]
}

type HashTable[K comparable, V any] struct {
	capacity   int
	size       int
	buckets    []*HashNode[K, V]
	loadFactor float64
}

func NewHashTable[K comparable, V any](initialCapacity int) (*HashTable[K, V], error) {
	if initialCapacity <= 0 {
		return nil, errors.New("initial capacity must be positive")
	}
	return &HashTable[K, V]{
		capacity:   initialCapacity,
		size:       0,
		buckets:    make([]*HashNode[K, V], initialCapacity),
		loadFactor: 0.7,
	}, nil
}

func (ht *HashTable[K, V]) Size() int {
	return ht.size
}

func (ht *HashTable[K, V]) IsEmpty() bool {
	return ht.size == 0
}

func (ht *HashTable[K, V]) Capacity() int {
	return ht.capacity
}

func (ht *HashTable[K, V]) hash(key K) int {
	keyStr := fmt.Sprintf("%v", key)
	hash := 0
	for _, char := range keyStr {
		hash = (hash << 5) - hash + int(char)
	}
	if hash < 0 {
		hash = -hash
	}
	return hash % ht.capacity
}

func (ht *HashTable[K, V]) resize() {
	oldBuckets := ht.buckets
	ht.capacity *= 2
	ht.size = 0
	ht.buckets = make([]*HashNode[K, V], ht.capacity)

	for _, bucket := range oldBuckets {
		current := bucket
		for current != nil {
			ht.Set(current.key, current.value)
			current = current.next
		}
	}
}

func (ht *HashTable[K, V]) Set(key K, value V) {
	if float64(ht.size)/float64(ht.capacity) >= ht.loadFactor {
		ht.resize()
	}

	index := ht.hash(key)
	current := ht.buckets[index]

	for current != nil {
		if current.key == key {
			current.value = value
			return
		}
		current = current.next
	}

	newNode := &HashNode[K, V]{key: key, value: value, next: ht.buckets[index]}
	ht.buckets[index] = newNode
	ht.size++
}

func (ht *HashTable[K, V]) Get(key K) (V, error) {
	var zero V
	index := ht.hash(key)
	current := ht.buckets[index]

	for current != nil {
		if current.key == key {
			return current.value, nil
		}
		current = current.next
	}

	return zero, errors.New("key not found")
}

func (ht *HashTable[K, V]) Has(key K) bool {
	index := ht.hash(key)
	current := ht.buckets[index]

	for current != nil {
		if current.key == key {
			return true
		}
		current = current.next
	}

	return false
}

func (ht *HashTable[K, V]) Delete(key K) bool {
	index := ht.hash(key)
	current := ht.buckets[index]
	var prev *HashNode[K, V] = nil

	for current != nil {
		if current.key == key {
			if prev != nil {
				prev.next = current.next
			} else {
				ht.buckets[index] = current.next
			}
			ht.size--
			return true
		}
		prev = current
		current = current.next
	}

	return false
}

func (ht *HashTable[K, V]) Keys() []K {
	keys := make([]K, 0, ht.size)
	for _, bucket := range ht.buckets {
		current := bucket
		for current != nil {
			keys = append(keys, current.key)
			current = current.next
		}
	}
	return keys
}

func (ht *HashTable[K, V]) Values() []V {
	values := make([]V, 0, ht.size)
	for _, bucket := range ht.buckets {
		current := bucket
		for current != nil {
			values = append(values, current.value)
			current = current.next
		}
	}
	return values
}

func (ht *HashTable[K, V]) Entries() [][2]interface{} {
	entries := make([][2]interface{}, 0, ht.size)
	for _, bucket := range ht.buckets {
		current := bucket
		for current != nil {
			entries = append(entries, [2]interface{}{current.key, current.value})
			current = current.next
		}
	}
	return entries
}

func (ht *HashTable[K, V]) Clear() {
	ht.buckets = make([]*HashNode[K, V], ht.capacity)
	ht.size = 0
}

func (ht *HashTable[K, V]) ForEach(callback func(value V, key K, ht *HashTable[K, V])) {
	for _, bucket := range ht.buckets {
		current := bucket
		for current != nil {
			callback(current.value, current.key, ht)
			current = current.next
		}
	}
}
