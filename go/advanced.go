package main

import (
	"errors"
	"math"
	"math/rand"
	"sort"
)

type CircularSinglyListNode[T any] struct {
	value T
	next  *CircularSinglyListNode[T]
}

type CircularSinglyLinkedList[T any] struct {
	head   *CircularSinglyListNode[T]
	length int
}

func NewCircularSinglyLinkedList[T any]() *CircularSinglyLinkedList[T] {
	return &CircularSinglyLinkedList[T]{}
}

func (c *CircularSinglyLinkedList[T]) Length() int {
	return c.length
}

func (c *CircularSinglyLinkedList[T]) IsEmpty() bool {
	return c.length == 0
}

func (c *CircularSinglyLinkedList[T]) Prepend(value T) {
	newNode := &CircularSinglyListNode[T]{value: value}
	if c.head == nil {
		c.head = newNode
		newNode.next = c.head
	} else {
		current := c.head
		for current.next != c.head {
			current = current.next
		}
		newNode.next = c.head
		current.next = newNode
		c.head = newNode
	}
	c.length++
}

func (c *CircularSinglyLinkedList[T]) Append(value T) {
	newNode := &CircularSinglyListNode[T]{value: value}
	if c.head == nil {
		c.head = newNode
		newNode.next = c.head
	} else {
		current := c.head
		for current.next != c.head {
			current = current.next
		}
		current.next = newNode
		newNode.next = c.head
	}
	c.length++
}

func (c *CircularSinglyLinkedList[T]) RemoveFirst() (T, bool) {
	if c.head == nil {
		var zero T
		return zero, false
	}
	removedNode := c.head
	if c.length == 1 {
		c.head = nil
	} else {
		current := c.head
		for current.next != c.head {
			current = current.next
		}
		c.head = c.head.next
		current.next = c.head
	}
	c.length--
	return removedNode.value, true
}

func (c *CircularSinglyLinkedList[T]) RemoveLast() (T, bool) {
	if c.head == nil {
		var zero T
		return zero, false
	}
	if c.length == 1 {
		value := c.head.value
		c.head = nil
		c.length--
		return value, true
	}
	current := c.head
	var prev *CircularSinglyListNode[T]
	for current.next != c.head {
		prev = current
		current = current.next
	}
	prev.next = c.head
	c.length--
	return current.value, true
}

func (c *CircularSinglyLinkedList[T]) ToSlice() []T {
	result := []T{}
	if c.head == nil {
		return result
	}
	current := c.head
	for {
		result = append(result, current.value)
		current = current.next
		if current == c.head {
			break
		}
	}
	return result
}

func (c *CircularSinglyLinkedList[T]) Clear() {
	c.head = nil
	c.length = 0
}

type CircularDoublyListNode[T any] struct {
	value T
	next  *CircularDoublyListNode[T]
	prev  *CircularDoublyListNode[T]
}

type CircularDoublyLinkedList[T any] struct {
	head   *CircularDoublyListNode[T]
	length int
}

func NewCircularDoublyLinkedList[T any]() *CircularDoublyLinkedList[T] {
	return &CircularDoublyLinkedList[T]{}
}

func (c *CircularDoublyLinkedList[T]) Length() int {
	return c.length
}

func (c *CircularDoublyLinkedList[T]) IsEmpty() bool {
	return c.length == 0
}

func (c *CircularDoublyLinkedList[T]) Prepend(value T) {
	newNode := &CircularDoublyListNode[T]{value: value}
	if c.head == nil {
		c.head = newNode
		newNode.next = newNode
		newNode.prev = newNode
	} else {
		tail := c.head.prev
		newNode.next = c.head
		newNode.prev = tail
		tail.next = newNode
		c.head.prev = newNode
		c.head = newNode
	}
	c.length++
}

func (c *CircularDoublyLinkedList[T]) Append(value T) {
	newNode := &CircularDoublyListNode[T]{value: value}
	if c.head == nil {
		c.head = newNode
		newNode.next = newNode
		newNode.prev = newNode
	} else {
		tail := c.head.prev
		newNode.prev = tail
		newNode.next = c.head
		tail.next = newNode
		c.head.prev = newNode
	}
	c.length++
}

func (c *CircularDoublyLinkedList[T]) RemoveFirst() (T, bool) {
	if c.head == nil {
		var zero T
		return zero, false
	}
	removedNode := c.head
	if c.length == 1 {
		c.head = nil
	} else {
		tail := c.head.prev
		c.head = c.head.next
		tail.next = c.head
		c.head.prev = tail
	}
	c.length--
	return removedNode.value, true
}

func (c *CircularDoublyLinkedList[T]) RemoveLast() (T, bool) {
	if c.head == nil {
		var zero T
		return zero, false
	}
	tail := c.head.prev
	if c.length == 1 {
		c.head = nil
	} else {
		newTail := tail.prev
		newTail.next = c.head
		c.head.prev = newTail
	}
	c.length--
	return tail.value, true
}

func (c *CircularDoublyLinkedList[T]) ToSlice() []T {
	result := []T{}
	if c.head == nil {
		return result
	}
	current := c.head
	for {
		result = append(result, current.value)
		current = current.next
		if current == c.head {
			break
		}
	}
	return result
}

func (c *CircularDoublyLinkedList[T]) ToSliceReverse() []T {
	result := []T{}
	if c.head == nil {
		return result
	}
	current := c.head.prev
	for {
		result = append(result, current.value)
		current = current.prev
		if current == c.head.prev {
			break
		}
	}
	return result
}

func (c *CircularDoublyLinkedList[T]) Clear() {
	c.head = nil
	c.length = 0
}

type TrieNode struct {
	children    map[rune]*TrieNode
	isEndOfWord bool
}

type Trie struct {
	root *TrieNode
}

func NewTrie() *Trie {
	return &Trie{root: &TrieNode{children: make(map[rune]*TrieNode)}}
}

func (t *Trie) Insert(word string) {
	current := t.root
	for _, char := range word {
		if _, exists := current.children[char]; !exists {
			current.children[char] = &TrieNode{children: make(map[rune]*TrieNode)}
		}
		current = current.children[char]
	}
	current.isEndOfWord = true
}

func (t *Trie) Search(word string) bool {
	current := t.root
	for _, char := range word {
		if _, exists := current.children[char]; !exists {
			return false
		}
		current = current.children[char]
	}
	return current.isEndOfWord
}

func (t *Trie) StartsWith(prefix string) bool {
	current := t.root
	for _, char := range prefix {
		if _, exists := current.children[char]; !exists {
			return false
		}
		current = current.children[char]
	}
	return true
}

func (t *Trie) Delete(word string) {
	t.deleteHelper(t.root, word, 0)
}

func (t *Trie) deleteHelper(node *TrieNode, word string, index int) bool {
	if index == len(word) {
		if !node.isEndOfWord {
			return false
		}
		node.isEndOfWord = false
		return len(node.children) == 0
	}

	char := rune(word[index])
	if _, exists := node.children[char]; !exists {
		return false
	}

	shouldDeleteChild := t.deleteHelper(node.children[char], word, index+1)

	if shouldDeleteChild {
		delete(node.children, char)
		return len(node.children) == 0 && !node.isEndOfWord
	}

	return false
}

func (t *Trie) GetAllWords() []string {
	words := []string{}
	t.getAllWordsHelper(t.root, "", &words)
	return words
}

func (t *Trie) getAllWordsHelper(node *TrieNode, prefix string, words *[]string) {
	if node.isEndOfWord {
		*words = append(*words, prefix)
	}
	for char, child := range node.children {
		t.getAllWordsHelper(child, prefix+string(char), words)
	}
}

func (t *Trie) GetWordsWithPrefix(prefix string) []string {
	current := t.root
	for _, char := range prefix {
		if _, exists := current.children[char]; !exists {
			return []string{}
		}
		current = current.children[char]
	}
	words := []string{}
	t.getAllWordsHelper(current, prefix, &words)
	return words
}

type GraphNode[T comparable] struct {
	value     T
	neighbors []*GraphNode[T]
}

type Graph[T comparable] struct {
	nodes      map[T]*GraphNode[T]
	isDirected bool
}

func NewGraph[T comparable](isDirected bool) *Graph[T] {
	return &Graph[T]{
		nodes:      make(map[T]*GraphNode[T]),
		isDirected: isDirected,
	}
}

func (g *Graph[T]) AddVertex(value T) {
	if _, exists := g.nodes[value]; !exists {
		g.nodes[value] = &GraphNode[T]{value: value}
	}
}

func (g *Graph[T]) AddEdge(from, to T) {
	g.AddVertex(from)
	g.AddVertex(to)

	fromNode := g.nodes[from]
	toNode := g.nodes[to]

	fromNode.neighbors = append(fromNode.neighbors, toNode)
	if !g.isDirected {
		toNode.neighbors = append(toNode.neighbors, fromNode)
	}
}

func (g *Graph[T]) RemoveVertex(value T) {
	node, exists := g.nodes[value]
	if !exists {
		return
	}

	for _, neighbor := range node.neighbors {
		for i, n := range neighbor.neighbors {
			if n == node {
				neighbor.neighbors = append(neighbor.neighbors[:i], neighbor.neighbors[i+1:]...)
				break
			}
		}
	}

	delete(g.nodes, value)
}

func (g *Graph[T]) RemoveEdge(from, to T) {
	fromNode, fromExists := g.nodes[from]
	toNode, toExists := g.nodes[to]

	if !fromExists || !toExists {
		return
	}

	for i, n := range fromNode.neighbors {
		if n == toNode {
			fromNode.neighbors = append(fromNode.neighbors[:i], fromNode.neighbors[i+1:]...)
			break
		}
	}

	if !g.isDirected {
		for i, n := range toNode.neighbors {
			if n == fromNode {
				toNode.neighbors = append(toNode.neighbors[:i], toNode.neighbors[i+1:]...)
				break
			}
		}
	}
}

func (g *Graph[T]) BFS(start T) []T {
	result := []T{}
	visited := make(map[T]bool)
	startNode, exists := g.nodes[start]
	if !exists {
		return result
	}

	queue := []*GraphNode[T]{startNode}
	visited[start] = true

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]
		result = append(result, current.value)

		for _, neighbor := range current.neighbors {
			if !visited[neighbor.value] {
				visited[neighbor.value] = true
				queue = append(queue, neighbor)
			}
		}
	}

	return result
}

func (g *Graph[T]) DFS(start T) []T {
	result := []T{}
	visited := make(map[T]bool)
	startNode, exists := g.nodes[start]
	if !exists {
		return result
	}

	stack := []*GraphNode[T]{startNode}
	visited[start] = true

	for len(stack) > 0 {
		current := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		result = append(result, current.value)

		for i := len(current.neighbors) - 1; i >= 0; i-- {
			neighbor := current.neighbors[i]
			if !visited[neighbor.value] {
				visited[neighbor.value] = true
				stack = append(stack, neighbor)
			}
		}
	}

	return result
}

func (g *Graph[T]) GetVertices() []T {
	vertices := []T{}
	for v := range g.nodes {
		vertices = append(vertices, v)
	}
	return vertices
}

func (g *Graph[T]) HasVertex(value T) bool {
	_, exists := g.nodes[value]
	return exists
}

func (g *Graph[T]) HasEdge(from, to T) bool {
	fromNode, fromExists := g.nodes[from]
	_, toExists := g.nodes[to]

	if !fromExists || !toExists {
		return false
	}

	for _, neighbor := range fromNode.neighbors {
		if neighbor.value == to {
			return true
		}
	}
	return false
}

type UnionFind struct {
	parent []int
	rank   []int
}

func NewUnionFind(size int) *UnionFind {
	parent := make([]int, size)
	rank := make([]int, size)
	for i := range parent {
		parent[i] = i
	}
	return &UnionFind{parent: parent, rank: rank}
}

func (uf *UnionFind) Find(x int) int {
	if uf.parent[x] != x {
		uf.parent[x] = uf.Find(uf.parent[x])
	}
	return uf.parent[x]
}

func (uf *UnionFind) Union(x, y int) bool {
	rootX := uf.Find(x)
	rootY := uf.Find(y)

	if rootX == rootY {
		return false
	}

	if uf.rank[rootX] < uf.rank[rootY] {
		uf.parent[rootX] = rootY
	} else if uf.rank[rootX] > uf.rank[rootY] {
		uf.parent[rootY] = rootX
	} else {
		uf.parent[rootY] = rootX
		uf.rank[rootX]++
	}

	return true
}

func (uf *UnionFind) Connected(x, y int) bool {
	return uf.Find(x) == uf.Find(y)
}

func (uf *UnionFind) GetCount() int {
	roots := make(map[int]bool)
	for i := range uf.parent {
		roots[uf.Find(i)] = true
	}
	return len(roots)
}

const (
	maxLevel = 16
	p        = 0.5
)

type SkipListNode[K ordered] struct {
	value   K
	forward []*SkipListNode[K]
}

type SkipList[K ordered] struct {
	head  *SkipListNode[K]
	level int
}

func NewSkipList[K ordered]() *SkipList[K] {
	head := &SkipListNode[K]{forward: make([]*SkipListNode[K], maxLevel+1)}
	return &SkipList[K]{head: head, level: 0}
}

func (sl *SkipList[K]) randomLevel() int {
	level := 0
	for rand.Float64() < p && level < maxLevel-1 {
		level++
	}
	return level
}

func (sl *SkipList[K]) Search(value K) bool {
	current := sl.head

	for i := sl.level; i >= 0; i-- {
		for current.forward[i] != nil && current.forward[i].value < value {
			current = current.forward[i]
		}
	}

	current = current.forward[0]
	return current != nil && current.value == value
}

func (sl *SkipList[K]) Insert(value K) {
	update := make([]*SkipListNode[K], maxLevel+1)
	current := sl.head

	for i := sl.level; i >= 0; i-- {
		for current.forward[i] != nil && current.forward[i].value < value {
			current = current.forward[i]
		}
		update[i] = current
	}

	current = current.forward[0]

	if current == nil || current.value != value {
		newLevel := sl.randomLevel()

		if newLevel > sl.level {
			for i := sl.level + 1; i <= newLevel; i++ {
				update[i] = sl.head
			}
			sl.level = newLevel
		}

		newNode := &SkipListNode[K]{value: value, forward: make([]*SkipListNode[K], newLevel+1)}

		for i := 0; i <= newLevel; i++ {
			newNode.forward[i] = update[i].forward[i]
			update[i].forward[i] = newNode
		}
	}
}

func (sl *SkipList[K]) Delete(value K) bool {
	update := make([]*SkipListNode[K], maxLevel+1)
	current := sl.head

	for i := sl.level; i >= 0; i-- {
		for current.forward[i] != nil && current.forward[i].value < value {
			current = current.forward[i]
		}
		update[i] = current
	}

	current = current.forward[0]

	if current != nil && current.value == value {
		for i := 0; i <= sl.level; i++ {
			if update[i].forward[i] != current {
				break
			}
			update[i].forward[i] = current.forward[i]
		}

		for sl.level > 0 && sl.head.forward[sl.level] == nil {
			sl.level--
		}

		return true
	}

	return false
}

func (sl *SkipList[K]) ToSlice() []K {
	result := []K{}
	current := sl.head.forward[0]
	for current != nil {
		result = append(result, current.value)
		current = current.forward[0]
	}
	return result
}

type SegmentTree[T any] struct {
	n            int
	size         int
	tree         []T
	merge        func(a, b T) T
	defaultValue T
}

func NewSegmentTree[T any](data []T, merge func(a, b T) T, defaultValue T) *SegmentTree[T] {
	n := len(data)
	size := 1
	for size < n {
		size <<= 1
	}
	tree := make([]T, 2*size)
	for i := 0; i < size; i++ {
		tree[i] = defaultValue
	}
	for i := 0; i < n; i++ {
		tree[size+i] = data[i]
	}
	for i := size - 1; i > 0; i-- {
		tree[i] = merge(tree[2*i], tree[2*i+1])
	}
	return &SegmentTree[T]{
		n:            n,
		size:         size,
		tree:         tree,
		merge:        merge,
		defaultValue: defaultValue,
	}
}

func (st *SegmentTree[T]) Update(index int, value T) error {
	if index < 0 || index >= st.n {
		return errors.New("index out of bounds")
	}
	index += st.size
	st.tree[index] = value
	index >>= 1
	for index >= 1 {
		newVal := st.merge(st.tree[2*index], st.tree[2*index+1])
		st.tree[index] = newVal
		index >>= 1
	}
	return nil
}

func (st *SegmentTree[T]) Query(l, r int) (T, error) {
	var result T
	if l < 0 || r >= st.n || l > r {
		return st.defaultValue, errors.New("invalid query range")
	}
	resLeft := st.defaultValue
	resRight := st.defaultValue
	l += st.size
	r += st.size
	for l <= r {
		if l%2 == 1 {
			resLeft = st.merge(resLeft, st.tree[l])
			l++
		}
		if r%2 == 0 {
			resRight = st.merge(st.tree[r], resRight)
			r--
		}
		l >>= 1
		r >>= 1
	}
	result = st.merge(resLeft, resRight)
	return result, nil
}

func (st *SegmentTree[T]) Get(index int) (T, error) {
	if index < 0 || index >= st.n {
		return st.defaultValue, errors.New("index out of bounds")
	}
	return st.tree[st.size+index], nil
}

type FenwickTree struct {
	tree []int
	n    int
}

func NewFenwickTreeFromSize(size int) *FenwickTree {
	return &FenwickTree{
		tree: make([]int, size+1),
		n:    size,
	}
}

func NewFenwickTreeFromData(data []int) *FenwickTree {
	n := len(data)
	ft := &FenwickTree{
		tree: make([]int, n+1),
		n:    n,
	}
	for i := 0; i < n; i++ {
		ft.Update(i, data[i])
	}
	return ft
}

func (ft *FenwickTree) Update(index int, delta int) error {
	if index < 0 || index >= ft.n {
		return errors.New("index out of bounds")
	}
	index++
	for index <= ft.n {
		ft.tree[index] += delta
		index += index & -index
	}
	return nil
}

func (ft *FenwickTree) Set(index int, value int) error {
	current, err := ft.Query(index, index)
	if err != nil {
		return err
	}
	return ft.Update(index, value-current)
}

func (ft *FenwickTree) PrefixSum(index int) (int, error) {
	if index < 0 || index >= ft.n {
		return 0, errors.New("index out of bounds")
	}
	index++
	sum := 0
	for index > 0 {
		sum += ft.tree[index]
		index -= index & -index
	}
	return sum, nil
}

func (ft *FenwickTree) Query(l, r int) (int, error) {
	if l < 0 || r >= ft.n || l > r {
		return 0, errors.New("invalid query range")
	}
	if l == 0 {
		return ft.PrefixSum(r)
	}
	sumR, err := ft.PrefixSum(r)
	if err != nil {
		return 0, err
	}
	sumL, err := ft.PrefixSum(l - 1)
	if err != nil {
		return 0, err
	}
	return sumR - sumL, nil
}

func (ft *FenwickTree) Size() int {
	return ft.n
}

type BloomFilter struct {
	bitArray         []byte
	size             int
	numHashFunctions int
}

func NewBloomFilter(expectedItems int, falsePositiveRate float64) *BloomFilter {
	size := calculateBloomSize(expectedItems, falsePositiveRate)
	numHashFunctions := calculateNumHashFunctions(size, expectedItems)
	byteSize := (size + 7) / 8
	return &BloomFilter{
		bitArray:         make([]byte, byteSize),
		size:             size,
		numHashFunctions: numHashFunctions,
	}
}

func calculateBloomSize(n int, p float64) int {
	return int(math.Ceil(-float64(n) * math.Log(p) / (math.Ln2 * math.Ln2)))
}

func calculateNumHashFunctions(m int, n int) int {
	k := int(math.Round(float64(m) / float64(n) * math.Ln2))
	if k < 1 {
		k = 1
	}
	return k
}

func (bf *BloomFilter) hash(item string, seed int) int {
	hashVal := seed
	for _, char := range item {
		hashVal = (hashVal*31 + int(char)) % bf.size
	}
	if hashVal < 0 {
		hashVal += bf.size
	}
	return hashVal
}

func (bf *BloomFilter) Add(item string) {
	for i := 0; i < bf.numHashFunctions; i++ {
		hashVal := bf.hash(item, i)
		byteIndex := hashVal / 8
		bitIndex := hashVal % 8
		bf.bitArray[byteIndex] |= (1 << bitIndex)
	}
}

func (bf *BloomFilter) MightContain(item string) bool {
	for i := 0; i < bf.numHashFunctions; i++ {
		hashVal := bf.hash(item, i)
		byteIndex := hashVal / 8
		bitIndex := hashVal % 8
		if (bf.bitArray[byteIndex] & (1 << bitIndex)) == 0 {
			return false
		}
	}
	return true
}

func (bf *BloomFilter) Clear() {
	for i := range bf.bitArray {
		bf.bitArray[i] = 0
	}
}

type LRUCacheNode[K comparable, V any] struct {
	key   K
	value V
	prev  *LRUCacheNode[K, V]
	next  *LRUCacheNode[K, V]
}

type LRUCache[K comparable, V any] struct {
	capacity int
	cache    map[K]*LRUCacheNode[K, V]
	head     *LRUCacheNode[K, V]
	tail     *LRUCacheNode[K, V]
}

func NewLRUCache[K comparable, V any](capacity int) (*LRUCache[K, V], error) {
	if capacity <= 0 {
		return nil, errors.New("capacity must be positive")
	}
	head := &LRUCacheNode[K, V]{}
	tail := &LRUCacheNode[K, V]{}
	head.next = tail
	tail.prev = head
	return &LRUCache[K, V]{
		capacity: capacity,
		cache:    make(map[K]*LRUCacheNode[K, V]),
		head:     head,
		tail:     tail,
	}, nil
}

func (lru *LRUCache[K, V]) addToHead(node *LRUCacheNode[K, V]) {
	node.prev = lru.head
	node.next = lru.head.next
	lru.head.next.prev = node
	lru.head.next = node
}

func (lru *LRUCache[K, V]) removeNode(node *LRUCacheNode[K, V]) {
	node.prev.next = node.next
	node.next.prev = node.prev
}

func (lru *LRUCache[K, V]) moveToHead(node *LRUCacheNode[K, V]) {
	lru.removeNode(node)
	lru.addToHead(node)
}

func (lru *LRUCache[K, V]) removeTail() *LRUCacheNode[K, V] {
	node := lru.tail.prev
	lru.removeNode(node)
	return node
}

func (lru *LRUCache[K, V]) Get(key K) (V, bool) {
	var zero V
	node, exists := lru.cache[key]
	if !exists {
		return zero, false
	}
	lru.moveToHead(node)
	return node.value, true
}

func (lru *LRUCache[K, V]) Put(key K, value V) {
	node, exists := lru.cache[key]
	if exists {
		node.value = value
		lru.moveToHead(node)
		return
	}
	newNode := &LRUCacheNode[K, V]{key: key, value: value}
	lru.cache[key] = newNode
	lru.addToHead(newNode)
	if len(lru.cache) > lru.capacity {
		tail := lru.removeTail()
		delete(lru.cache, tail.key)
	}
}

func (lru *LRUCache[K, V]) Has(key K) bool {
	_, exists := lru.cache[key]
	return exists
}

func (lru *LRUCache[K, V]) Delete(key K) bool {
	node, exists := lru.cache[key]
	if !exists {
		return false
	}
	lru.removeNode(node)
	delete(lru.cache, key)
	return true
}

func (lru *LRUCache[K, V]) Clear() {
	lru.cache = make(map[K]*LRUCacheNode[K, V])
	lru.head.next = lru.tail
	lru.tail.prev = lru.head
}

func (lru *LRUCache[K, V]) Size() int {
	return len(lru.cache)
}

func (lru *LRUCache[K, V]) Keys() []K {
	keys := make([]K, 0)
	current := lru.head.next
	for current != lru.tail {
		keys = append(keys, current.key)
		current = current.next
	}
	return keys
}

func (lru *LRUCache[K, V]) Values() []V {
	values := make([]V, 0)
	current := lru.head.next
	for current != lru.tail {
		values = append(values, current.value)
		current = current.next
	}
	return values
}

type SuffixArray struct {
	text        string
	suffixArray []int
	lcpArray    []int
}

func NewSuffixArray(text string) *SuffixArray {
	sa := &SuffixArray{text: text}
	sa.suffixArray = sa.buildSuffixArray(text)
	return sa
}

func (sa *SuffixArray) buildSuffixArray(s string) []int {
	n := len(s)
	suffixArr := make([]int, n)
	for i := range suffixArr {
		suffixArr[i] = i
	}
	rank := make([]int, n)
	for i := range rank {
		rank[i] = int(s[i])
	}
	k := 1
	for k < n {
		sort.Slice(suffixArr, func(i, j int) bool {
			if rank[suffixArr[i]] != rank[suffixArr[j]] {
				return rank[suffixArr[i]] < rank[suffixArr[j]]
			}
			ra := -1
			if suffixArr[i]+k < n {
				ra = rank[suffixArr[i]+k]
			}
			rb := -1
			if suffixArr[j]+k < n {
				rb = rank[suffixArr[j]+k]
			}
			return ra < rb
		})
		newRank := make([]int, n)
		newRank[suffixArr[0]] = 0
		for i := 1; i < n; i++ {
			prev := suffixArr[i-1]
			curr := suffixArr[i]
			same := rank[prev] == rank[curr]
			if same {
				ra := -1
				if prev+k < n {
					ra = rank[prev+k]
				}
				rb := -1
				if curr+k < n {
					rb = rank[curr+k]
				}
				same = ra == rb
			}
			if same {
				newRank[curr] = newRank[prev]
			} else {
				newRank[curr] = newRank[prev] + 1
			}
		}
		rank = newRank
		k *= 2
	}
	return suffixArr
}

func (sa *SuffixArray) GetSuffixArray() []int {
	result := make([]int, len(sa.suffixArray))
	copy(result, sa.suffixArray)
	return result
}

func (sa *SuffixArray) GetSuffix(index int) (string, error) {
	if index < 0 || index >= len(sa.text) {
		return "", errors.New("index out of bounds")
	}
	return sa.text[index:], nil
}

func (sa *SuffixArray) buildLCPArray() []int {
	n := len(sa.text)
	rank := make([]int, n)
	for i := range rank {
		rank[sa.suffixArray[i]] = i
	}
	lcp := make([]int, n-1)
	k := 0
	for i := 0; i < n; i++ {
		if rank[i] == n-1 {
			k = 0
			continue
		}
		j := sa.suffixArray[rank[i]+1]
		for i+k < n && j+k < n && sa.text[i+k] == sa.text[j+k] {
			k++
		}
		lcp[rank[i]] = k
		if k > 0 {
			k--
		}
	}
	return lcp
}

func (sa *SuffixArray) GetLCPArray() []int {
	if sa.lcpArray == nil {
		sa.lcpArray = sa.buildLCPArray()
	}
	result := make([]int, len(sa.lcpArray))
	copy(result, sa.lcpArray)
	return result
}

func (sa *SuffixArray) Search(pattern string) []int {
	result := []int{}
	m := len(pattern)
	n := len(sa.text)
	low := 0
	high := n - 1
	for low <= high {
		mid := (low + high) / 2
		suffix, _ := sa.GetSuffix(sa.suffixArray[mid])
		end := min(m, len(suffix))
		suffixPrefix := suffix[:end]
		if pattern == suffixPrefix {
			result = append(result, sa.suffixArray[mid])
			left := mid - 1
			for left >= 0 {
				leftSuffix, _ := sa.GetSuffix(sa.suffixArray[left])
				if len(leftSuffix) >= m && leftSuffix[:m] == pattern {
					result = append(result, sa.suffixArray[left])
					left--
				} else {
					break
				}
			}
			right := mid + 1
			for right < n {
				rightSuffix, _ := sa.GetSuffix(sa.suffixArray[right])
				if len(rightSuffix) >= m && rightSuffix[:m] == pattern {
					result = append(result, sa.suffixArray[right])
					right++
				} else {
					break
				}
			}
			break
		} else if pattern < suffixPrefix {
			high = mid - 1
		} else {
			low = mid + 1
		}
	}
	sort.Ints(result)
	return result
}

func (sa *SuffixArray) GetLongestCommonPrefix() int {
	lcp := sa.GetLCPArray()
	if len(lcp) == 0 {
		return 0
	}
	maxLen := 0
	for _, v := range lcp {
		if v > maxLen {
			maxLen = v
		}
	}
	return maxLen
}

func (sa *SuffixArray) GetLongestRepeatedSubstring() string {
	lcp := sa.GetLCPArray()
	maxLen := 0
	maxIndex := 0
	for i, v := range lcp {
		if v > maxLen {
			maxLen = v
			maxIndex = i
		}
	}
	if maxLen == 0 {
		return ""
	}
	return sa.text[sa.suffixArray[maxIndex] : sa.suffixArray[maxIndex]+maxLen]
}

type KDPoint interface {
	Coordinates() []float64
}

type SimpleKDPoint struct {
	coordinates []float64
}

func NewSimpleKDPoint(coordinates []float64) *SimpleKDPoint {
	return &SimpleKDPoint{coordinates: coordinates}
}

func (p *SimpleKDPoint) Coordinates() []float64 {
	return p.coordinates
}

type KDNode[T KDPoint] struct {
	point T
	left  *KDNode[T]
	right *KDNode[T]
	axis  int
}

type KDTree[T KDPoint] struct {
	root       *KDNode[T]
	dimensions int
}

func NewKDTree[T KDPoint](points []T) *KDTree[T] {
	if len(points) == 0 {
		return &KDTree[T]{root: nil, dimensions: 0}
	}
	dimensions := len(points[0].Coordinates())
	return &KDTree[T]{
		root:       buildTree(points, 0, dimensions),
		dimensions: dimensions,
	}
}

func buildTree[T KDPoint](points []T, depth int, dimensions int) *KDNode[T] {
	if len(points) == 0 {
		return nil
	}
	axis := depth % dimensions
	sortedPoints := make([]T, len(points))
	copy(sortedPoints, points)
	sort.Slice(sortedPoints, func(i, j int) bool {
		return sortedPoints[i].Coordinates()[axis] < sortedPoints[j].Coordinates()[axis]
	})
	median := len(sortedPoints) / 2
	return &KDNode[T]{
		point: sortedPoints[median],
		left:  buildTree(sortedPoints[:median], depth+1, dimensions),
		right: buildTree(sortedPoints[median+1:], depth+1, dimensions),
		axis:  axis,
	}
}

func (kdt *KDTree[T]) Insert(point T) error {
	if kdt.root == nil {
		kdt.dimensions = len(point.Coordinates())
		kdt.root = &KDNode[T]{point: point, axis: 0}
		return nil
	}
	if len(point.Coordinates()) != kdt.dimensions {
		return errors.New("point must have the same dimensions as the tree")
	}
	current := kdt.root
	depth := 0
	for {
		axis := depth % kdt.dimensions
		if point.Coordinates()[axis] < current.point.Coordinates()[axis] {
			if current.left == nil {
				current.left = &KDNode[T]{point: point, axis: (depth + 1) % kdt.dimensions}
				break
			}
			current = current.left
		} else {
			if current.right == nil {
				current.right = &KDNode[T]{point: point, axis: (depth + 1) % kdt.dimensions}
				break
			}
			current = current.right
		}
		depth++
	}
	return nil
}

func distanceSquared(a, b []float64) float64 {
	sum := 0.0
	for i := range a {
		diff := a[i] - b[i]
		sum += diff * diff
	}
	return sum
}

func (kdt *KDTree[T]) NearestNeighbor(target []float64) (T, bool) {
	var zero T
	if kdt.root == nil || len(target) != kdt.dimensions {
		return zero, false
	}
	best := kdt.root
	bestDist := distanceSquared(kdt.root.point.Coordinates(), target)
	var search func(node *KDNode[T], depth int)
	search = func(node *KDNode[T], depth int) {
		if node == nil {
			return
		}
		dist := distanceSquared(node.point.Coordinates(), target)
		if dist < bestDist {
			bestDist = dist
			best = node
		}
		axis := depth % kdt.dimensions
		goLeft := target[axis] < node.point.Coordinates()[axis]
		if goLeft {
			search(node.left, depth+1)
		} else {
			search(node.right, depth+1)
		}
		planeDist := (target[axis] - node.point.Coordinates()[axis]) * (target[axis] - node.point.Coordinates()[axis])
		if planeDist < bestDist {
			if goLeft {
				search(node.right, depth+1)
			} else {
				search(node.left, depth+1)
			}
		}
	}
	search(kdt.root, 0)
	return best.point, true
}

func (kdt *KDTree[T]) RangeSearch(minCoords, maxCoords []float64) []T {
	result := []T{}
	if kdt.root == nil || len(minCoords) != kdt.dimensions || len(maxCoords) != kdt.dimensions {
		return result
	}
	var search func(node *KDNode[T])
	search = func(node *KDNode[T]) {
		if node == nil {
			return
		}
		point := node.point.Coordinates()
		inRange := true
		for i := range point {
			if point[i] < minCoords[i] || point[i] > maxCoords[i] {
				inRange = false
				break
			}
		}
		if inRange {
			result = append(result, node.point)
		}
		axis := node.axis
		if minCoords[axis] <= point[axis] {
			search(node.left)
		}
		if maxCoords[axis] >= point[axis] {
			search(node.right)
		}
	}
	search(kdt.root)
	return result
}

func (kdt *KDTree[T]) KNearestNeighbors(target []float64, k int) []T {
	result := []T{}
	if k <= 0 || kdt.root == nil || len(target) != kdt.dimensions {
		return result
	}
	type neighbor struct {
		point T
		dist  float64
	}
	neighbors := []neighbor{}
	var search func(node *KDNode[T], depth int)
	search = func(node *KDNode[T], depth int) {
		if node == nil {
			return
		}
		dist := distanceSquared(node.point.Coordinates(), target)
		if len(neighbors) < k {
			neighbors = append(neighbors, neighbor{point: node.point, dist: dist})
			sort.Slice(neighbors, func(i, j int) bool {
				return neighbors[i].dist < neighbors[j].dist
			})
		} else if dist < neighbors[len(neighbors)-1].dist {
			neighbors = neighbors[:len(neighbors)-1]
			neighbors = append(neighbors, neighbor{point: node.point, dist: dist})
			sort.Slice(neighbors, func(i, j int) bool {
				return neighbors[i].dist < neighbors[j].dist
			})
		}
		axis := depth % kdt.dimensions
		goLeft := target[axis] < node.point.Coordinates()[axis]
		if goLeft {
			search(node.left, depth+1)
		} else {
			search(node.right, depth+1)
		}
		planeDist := (target[axis] - node.point.Coordinates()[axis]) * (target[axis] - node.point.Coordinates()[axis])
		if len(neighbors) < k || planeDist < neighbors[len(neighbors)-1].dist {
			if goLeft {
				search(node.right, depth+1)
			} else {
				search(node.left, depth+1)
			}
		}
	}
	search(kdt.root, 0)
	for _, n := range neighbors {
		result = append(result, n.point)
	}
	return result
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
