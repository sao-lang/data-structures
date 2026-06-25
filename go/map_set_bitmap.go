package main

import (
	"errors"
	"fmt"
	"math/bits"
)

type MapHashNode[K comparable, V any] struct {
	key   K
	value V
	next  *MapHashNode[K, V]
}

type Map[K comparable, V any] struct {
	capacity   int
	size       int
	buckets    []*MapHashNode[K, V]
	loadFactor float64
}

func NewMap[K comparable, V any](initialCapacity int) (*Map[K, V], error) {
	if initialCapacity <= 0 {
		return nil, errors.New("initial capacity must be positive")
	}
	return &Map[K, V]{
		capacity:   initialCapacity,
		size:       0,
		buckets:    make([]*MapHashNode[K, V], initialCapacity),
		loadFactor: 0.7,
	}, nil
}

func (m *Map[K, V]) Size() int {
	return m.size
}

func (m *Map[K, V]) IsEmpty() bool {
	return m.size == 0
}

func (m *Map[K, V]) Capacity() int {
	return m.capacity
}

func (m *Map[K, V]) hash(key K) int {
	keyStr := fmt.Sprintf("%v", key)
	hash := 0
	for _, char := range keyStr {
		hash = (hash << 5) - hash + int(char)
	}
	if hash < 0 {
		hash = -hash
	}
	return hash % m.capacity
}

func (m *Map[K, V]) resize() {
	oldBuckets := m.buckets
	m.capacity *= 2
	m.size = 0
	m.buckets = make([]*MapHashNode[K, V], m.capacity)

	for _, bucket := range oldBuckets {
		current := bucket
		for current != nil {
			m.Set(current.key, current.value)
			current = current.next
		}
	}
}

func (m *Map[K, V]) Set(key K, value V) {
	if float64(m.size)/float64(m.capacity) >= m.loadFactor {
		m.resize()
	}

	index := m.hash(key)
	current := m.buckets[index]

	for current != nil {
		if current.key == key {
			current.value = value
			return
		}
		current = current.next
	}

	newNode := &MapHashNode[K, V]{key: key, value: value, next: m.buckets[index]}
	m.buckets[index] = newNode
	m.size++
}

func (m *Map[K, V]) Get(key K) (V, error) {
	var zero V
	index := m.hash(key)
	current := m.buckets[index]

	for current != nil {
		if current.key == key {
			return current.value, nil
		}
		current = current.next
	}

	return zero, errors.New("key not found")
}

func (m *Map[K, V]) Has(key K) bool {
	index := m.hash(key)
	current := m.buckets[index]

	for current != nil {
		if current.key == key {
			return true
		}
		current = current.next
	}

	return false
}

func (m *Map[K, V]) Delete(key K) bool {
	index := m.hash(key)
	current := m.buckets[index]
	var prev *MapHashNode[K, V] = nil

	for current != nil {
		if current.key == key {
			if prev != nil {
				prev.next = current.next
			} else {
				m.buckets[index] = current.next
			}
			m.size--
			return true
		}
		prev = current
		current = current.next
	}

	return false
}

func (m *Map[K, V]) Keys() []K {
	keys := make([]K, 0, m.size)
	for _, bucket := range m.buckets {
		current := bucket
		for current != nil {
			keys = append(keys, current.key)
			current = current.next
		}
	}
	return keys
}

func (m *Map[K, V]) Values() []V {
	values := make([]V, 0, m.size)
	for _, bucket := range m.buckets {
		current := bucket
		for current != nil {
			values = append(values, current.value)
			current = current.next
		}
	}
	return values
}

func (m *Map[K, V]) Entries() [][2]interface{} {
	entries := make([][2]interface{}, 0, m.size)
	for _, bucket := range m.buckets {
		current := bucket
		for current != nil {
			entries = append(entries, [2]interface{}{current.key, current.value})
			current = current.next
		}
	}
	return entries
}

func (m *Map[K, V]) Clear() {
	m.buckets = make([]*MapHashNode[K, V], m.capacity)
	m.size = 0
}

func (m *Map[K, V]) Update(other *Map[K, V]) {
	for _, entry := range other.Entries() {
		key, _ := entry[0].(K)
		value, _ := entry[1].(V)
		m.Set(key, value)
	}
}

// MapIterator 是Map的迭代器
type MapIterator[K comparable, V any] struct {
	mapObj      *Map[K, V]
	bucketIndex int
	current     *MapHashNode[K, V]
}

// Iterator 返回Map的迭代器
func (m *Map[K, V]) Iterator() *MapIterator[K, V] {
	return &MapIterator[K, V]{
		mapObj:      m,
		bucketIndex: -1,
		current:     nil,
	}
}

// Next 移动到下一个元素
func (it *MapIterator[K, V]) Next() bool {
	if it.current != nil {
		it.current = it.current.next
		if it.current != nil {
			return true
		}
	}

	it.bucketIndex++
	for it.bucketIndex < it.mapObj.capacity {
		if it.mapObj.buckets[it.bucketIndex] != nil {
			it.current = it.mapObj.buckets[it.bucketIndex]
			return true
		}
		it.bucketIndex++
	}

	return false
}

// Key 返回当前元素的键
func (it *MapIterator[K, V]) Key() K {
	return it.current.key
}

// Value 返回当前元素的值
func (it *MapIterator[K, V]) Value() V {
	return it.current.value
}

// Entry 返回当前元素的键值对
func (it *MapIterator[K, V]) Entry() (K, V) {
	return it.current.key, it.current.value
}

// ForEach 遍历Map中的所有元素
func (m *Map[K, V]) ForEach(f func(K, V)) {
	for _, bucket := range m.buckets {
		current := bucket
		for current != nil {
			f(current.key, current.value)
			current = current.next
		}
	}
}

type BitMap struct {
	size int
	bits []uint64
}

func NewBitMap(size int) (*BitMap, error) {
	if size <= 0 {
		return nil, errors.New("size must be positive")
	}
	numWords := (size + 63) / 64
	return &BitMap{
		size: size,
		bits: make([]uint64, numWords),
	}, nil
}

func (bm *BitMap) Size() int {
	return bm.size
}

func (bm *BitMap) getIndexAndMask(bit int) (int, uint64, error) {
	if bit < 0 || bit >= bm.size {
		return 0, 0, errors.New("bit index out of bounds")
	}
	index := bit / 64
	mask := uint64(1) << (bit % 64)
	return index, mask, nil
}

func (bm *BitMap) Set(bit int) error {
	index, mask, err := bm.getIndexAndMask(bit)
	if err != nil {
		return err
	}
	bm.bits[index] |= mask
	return nil
}

func (bm *BitMap) Clear(bit int) error {
	index, mask, err := bm.getIndexAndMask(bit)
	if err != nil {
		return err
	}
	bm.bits[index] &^= mask
	return nil
}

func (bm *BitMap) Toggle(bit int) error {
	index, mask, err := bm.getIndexAndMask(bit)
	if err != nil {
		return err
	}
	bm.bits[index] ^= mask
	return nil
}

func (bm *BitMap) Get(bit int) (bool, error) {
	index, mask, err := bm.getIndexAndMask(bit)
	if err != nil {
		return false, err
	}
	return (bm.bits[index] & mask) != 0, nil
}

func (bm *BitMap) SetAll() {
	for i := range bm.bits {
		bm.bits[i] = ^uint64(0)
	}
}

func (bm *BitMap) ClearAll() {
	for i := range bm.bits {
		bm.bits[i] = 0
	}
}

func (bm *BitMap) CountSetBits() int {
	count := 0
	for _, word := range bm.bits {
		count += bits.OnesCount64(word)
	}
	return count
}

func (bm *BitMap) FindFirstSet() (int, error) {
	for i := 0; i < bm.size; i++ {
		set, err := bm.Get(i)
		if err != nil {
			return -1, err
		}
		if set {
			return i, nil
		}
	}
	return -1, nil
}

func (bm *BitMap) FindFirstClear() (int, error) {
	for i := 0; i < bm.size; i++ {
		set, err := bm.Get(i)
		if err != nil {
			return -1, err
		}
		if !set {
			return i, nil
		}
	}
	return -1, nil
}

func (bm *BitMap) And(other *BitMap) (*BitMap, error) {
	if bm.size != other.size {
		return nil, errors.New("BitMaps must have the same size")
	}
	result, _ := NewBitMap(bm.size)
	for i := range bm.bits {
		result.bits[i] = bm.bits[i] & other.bits[i]
	}
	return result, nil
}

func (bm *BitMap) Or(other *BitMap) (*BitMap, error) {
	if bm.size != other.size {
		return nil, errors.New("BitMaps must have the same size")
	}
	result, _ := NewBitMap(bm.size)
	for i := range bm.bits {
		result.bits[i] = bm.bits[i] | other.bits[i]
	}
	return result, nil
}

func (bm *BitMap) Xor(other *BitMap) (*BitMap, error) {
	if bm.size != other.size {
		return nil, errors.New("BitMaps must have the same size")
	}
	result, _ := NewBitMap(bm.size)
	for i := range bm.bits {
		result.bits[i] = bm.bits[i] ^ other.bits[i]
	}
	return result, nil
}

func (bm *BitMap) Not() (*BitMap, error) {
	result, _ := NewBitMap(bm.size)
	for i := range bm.bits {
		result.bits[i] = ^bm.bits[i]
	}
	return result, nil
}

func (bm *BitMap) String() string {
	bitsStr := make([]byte, bm.size)
	for i := 0; i < bm.size; i++ {
		set, _ := bm.Get(i)
		if set {
			bitsStr[i] = '1'
		} else {
			bitsStr[i] = '0'
		}
	}
	return string(bitsStr)
}

// BitMapIterator 是BitMap的迭代器
type BitMapIterator struct {
	bitmap  *BitMap
	current int
}

// Iterator 返回BitMap的迭代器
func (bm *BitMap) Iterator() *BitMapIterator {
	return &BitMapIterator{
		bitmap:  bm,
		current: -1,
	}
}

// Next 移动到下一个元素
func (it *BitMapIterator) Next() bool {
	it.current++
	return it.current < it.bitmap.size
}

// Index 返回当前位的索引
func (it *BitMapIterator) Index() int {
	return it.current
}

// Value 返回当前位的值
func (it *BitMapIterator) Value() (bool, error) {
	return it.bitmap.Get(it.current)
}

// ForEach 遍历BitMap中的所有位
func (bm *BitMap) ForEach(f func(int, bool)) {
	for i := 0; i < bm.size; i++ {
		set, _ := bm.Get(i)
		f(i, set)
	}
}

type Set[T comparable] struct {
	m *Map[T, bool]
}

func NewSet[T comparable](initialCapacity int) (*Set[T], error) {
	m, err := NewMap[T, bool](initialCapacity)
	if err != nil {
		return nil, err
	}
	return &Set[T]{m: m}, nil
}

func (s *Set[T]) Size() int {
	return s.m.Size()
}

func (s *Set[T]) IsEmpty() bool {
	return s.m.IsEmpty()
}

func (s *Set[T]) Add(item T) {
	s.m.Set(item, true)
}

func (s *Set[T]) Remove(item T) bool {
	return s.m.Delete(item)
}

func (s *Set[T]) Has(item T) bool {
	return s.m.Has(item)
}

func (s *Set[T]) Clear() {
	s.m.Clear()
}

func (s *Set[T]) Items() []T {
	return s.m.Keys()
}

func (s *Set[T]) Union(other *Set[T]) (*Set[T], error) {
	result, _ := NewSet[T](mapSetMax(s.Size(), other.Size()) + 1)
	for _, item := range s.Items() {
		result.Add(item)
	}
	for _, item := range other.Items() {
		result.Add(item)
	}
	return result, nil
}

func (s *Set[T]) Intersection(other *Set[T]) (*Set[T], error) {
	result, _ := NewSet[T](16)
	smaller, larger := s, other
	if s.Size() > other.Size() {
		smaller, larger = other, s
	}
	for _, item := range smaller.Items() {
		if larger.Has(item) {
			result.Add(item)
		}
	}
	return result, nil
}

func (s *Set[T]) Difference(other *Set[T]) (*Set[T], error) {
	result, _ := NewSet[T](16)
	for _, item := range s.Items() {
		if !other.Has(item) {
			result.Add(item)
		}
	}
	return result, nil
}

func (s *Set[T]) SymmetricDifference(other *Set[T]) (*Set[T], error) {
	result, _ := NewSet[T](16)
	for _, item := range s.Items() {
		if !other.Has(item) {
			result.Add(item)
		}
	}
	for _, item := range other.Items() {
		if !s.Has(item) {
			result.Add(item)
		}
	}
	return result, nil
}

func (s *Set[T]) IsSubset(other *Set[T]) bool {
	if s.Size() > other.Size() {
		return false
	}
	for _, item := range s.Items() {
		if !other.Has(item) {
			return false
		}
	}
	return true
}

func (s *Set[T]) IsSuperset(other *Set[T]) bool {
	return other.IsSubset(s)
}

func (s *Set[T]) String() string {
	items := make([]string, 0, s.Size())
	for _, item := range s.Items() {
		items = append(items, fmt.Sprintf("%v", item))
	}
	return fmt.Sprintf("{%s}", mapSetJoin(items, ", "))
}

// SetIterator 是Set的迭代器
type SetIterator[T comparable] struct {
	set      *Set[T]
	iterator *MapIterator[T, bool]
}

// Iterator 返回Set的迭代器
func (s *Set[T]) Iterator() *SetIterator[T] {
	return &SetIterator[T]{
		set:      s,
		iterator: s.m.Iterator(),
	}
}

// Next 移动到下一个元素
func (it *SetIterator[T]) Next() bool {
	return it.iterator.Next()
}

// Value 返回当前元素的值
func (it *SetIterator[T]) Value() T {
	return it.iterator.Key()
}

// ForEach 遍历Set中的所有元素
func (s *Set[T]) ForEach(f func(T)) {
	s.m.ForEach(func(key T, _ bool) {
		f(key)
	})
}

func mapSetMax(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func mapSetJoin(items []string, sep string) string {
	if len(items) == 0 {
		return ""
	}
	result := items[0]
	for i := 1; i < len(items); i++ {
		result += sep + items[i]
	}
	return result
}

// func main() {
// 	// 测试Map的可遍历结构
// 	fmt.Println("=== 测试Map的可遍历结构 ===")
// 	m, _ := NewMap[string, int](16)
// 	m.Set("one", 1)
// 	m.Set("two", 2)
// 	m.Set("three", 3)

// 	// 使用ForEach方法遍历
// 	fmt.Println("使用ForEach方法遍历Map:")
// 	m.ForEach(func(key string, value int) {
// 		fmt.Printf("%s: %d\n", key, value)
// 	})

// 	// 使用迭代器遍历
// 	fmt.Println("使用迭代器遍历Map:")
// 	it := m.Iterator()
// 	for it.Next() {
// 		fmt.Printf("%s: %d\n", it.Key(), it.Value())
// 	}

// 	// 测试Set的可遍历结构
// 	fmt.Println("\n=== 测试Set的可遍历结构 ===")
// 	s, _ := NewSet[int](16)
// 	s.Add(1)
// 	s.Add(2)
// 	s.Add(3)

// 	// 使用ForEach方法遍历
// 	fmt.Println("使用ForEach方法遍历Set:")
// 	s.ForEach(func(item int) {
// 		fmt.Printf("%d\n", item)
// 	})

// 	// 使用迭代器遍历
// 	fmt.Println("使用迭代器遍历Set:")
// 	sit := s.Iterator()
// 	for sit.Next() {
// 		fmt.Printf("%d\n", sit.Value())
// 	}

// 	// 测试BitMap的可遍历结构
// 	fmt.Println("\n=== 测试BitMap的可遍历结构 ===")
// 	bm, _ := NewBitMap(10)
// 	bm.Set(0)
// 	bm.Set(2)
// 	bm.Set(5)

// 	// 使用ForEach方法遍历
// 	fmt.Println("使用ForEach方法遍历BitMap:")
// 	bm.ForEach(func(index int, value bool) {
// 		fmt.Printf("%d: %t\n", index, value)
// 	})

// 	// 使用迭代器遍历
// 	fmt.Println("使用迭代器遍历BitMap:")
// 	bmit := bm.Iterator()
// 	for bmit.Next() {
// 		value, _ := bmit.Value()
// 		fmt.Printf("%d: %t\n", bmit.Index(), value)
// 	}
// }
