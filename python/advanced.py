from typing import Generic, TypeVar, Optional, List, Dict, Set, Deque as _Deque, Callable, Protocol
from collections import deque
import random
import math

T = TypeVar('T')
K = TypeVar('K', int, str, float)
V = TypeVar('V')


class CircularSinglyListNode(Generic[T]):
    def __init__(self, value: T):
        self.value = value
        self.next: Optional['CircularSinglyListNode[T]'] = None


class CircularSinglyLinkedList(Generic[T]):
    def __init__(self):
        self._head: Optional[CircularSinglyListNode[T]] = None
        self._length = 0

    @property
    def length(self) -> int:
        return self._length

    @property
    def is_empty(self) -> bool:
        return self._length == 0

    @property
    def head(self) -> Optional[T]:
        return self._head.value if self._head else None

    def prepend(self, value: T) -> None:
        new_node = CircularSinglyListNode(value)
        if not self._head:
            self._head = new_node
            new_node.next = self._head
        else:
            current = self._head
            while current.next != self._head:
                current = current.next
            new_node.next = self._head
            current.next = new_node
            self._head = new_node
        self._length += 1

    def append(self, value: T) -> None:
        new_node = CircularSinglyListNode(value)
        if not self._head:
            self._head = new_node
            new_node.next = self._head
        else:
            current = self._head
            while current.next != self._head:
                current = current.next
            current.next = new_node
            new_node.next = self._head
        self._length += 1

    def remove_first(self) -> Optional[T]:
        if not self._head:
            return None
        removed_node = self._head
        if self._length == 1:
            self._head = None
        else:
            current = self._head
            while current.next != self._head:
                current = current.next
            self._head = self._head.next
            current.next = self._head
        self._length -= 1
        return removed_node.value

    def remove_last(self) -> Optional[T]:
        if not self._head:
            return None
        if self._length == 1:
            value = self._head.value
            self._head = None
            self._length -= 1
            return value
        current = self._head
        prev = None
        while current.next != self._head:
            prev = current
            current = current.next
        prev.next = self._head
        self._length -= 1
        return current.value

    def to_list(self) -> List[T]:
        result = []
        if not self._head:
            return result
        current = self._head
        while True:
            result.append(current.value)
            current = current.next
            if current == self._head:
                break
        return result

    def clear(self) -> None:
        self._head = None
        self._length = 0


class CircularDoublyListNode(Generic[T]):
    def __init__(self, value: T):
        self.value = value
        self.next: Optional['CircularDoublyListNode[T]'] = None
        self.prev: Optional['CircularDoublyListNode[T]'] = None


class CircularDoublyLinkedList(Generic[T]):
    def __init__(self):
        self._head: Optional[CircularDoublyListNode[T]] = None
        self._length = 0

    @property
    def length(self) -> int:
        return self._length

    @property
    def is_empty(self) -> bool:
        return self._length == 0

    @property
    def head(self) -> Optional[T]:
        return self._head.value if self._head else None

    @property
    def tail(self) -> Optional[T]:
        return self._head.prev.value if self._head and self._head.prev else None

    def prepend(self, value: T) -> None:
        new_node = CircularDoublyListNode(value)
        if not self._head:
            self._head = new_node
            new_node.next = new_node
            new_node.prev = new_node
        else:
            tail = self._head.prev
            new_node.next = self._head
            new_node.prev = tail
            tail.next = new_node
            self._head.prev = new_node
            self._head = new_node
        self._length += 1

    def append(self, value: T) -> None:
        new_node = CircularDoublyListNode(value)
        if not self._head:
            self._head = new_node
            new_node.next = new_node
            new_node.prev = new_node
        else:
            tail = self._head.prev
            new_node.prev = tail
            new_node.next = self._head
            tail.next = new_node
            self._head.prev = new_node
        self._length += 1

    def remove_first(self) -> Optional[T]:
        if not self._head:
            return None
        removed_node = self._head
        if self._length == 1:
            self._head = None
        else:
            tail = self._head.prev
            self._head = self._head.next
            tail.next = self._head
            self._head.prev = tail
        self._length -= 1
        return removed_node.value

    def remove_last(self) -> Optional[T]:
        if not self._head:
            return None
        tail = self._head.prev
        if self._length == 1:
            self._head = None
        else:
            new_tail = tail.prev
            new_tail.next = self._head
            self._head.prev = new_tail
        self._length -= 1
        return tail.value

    def to_list(self) -> List[T]:
        result = []
        if not self._head:
            return result
        current = self._head
        while True:
            result.append(current.value)
            current = current.next
            if current == self._head:
                break
        return result

    def to_list_reverse(self) -> List[T]:
        result = []
        if not self._head:
            return result
        current = self._head.prev
        while True:
            result.append(current.value)
            current = current.prev
            if current == self._head.prev:
                break
        return result

    def clear(self) -> None:
        self._head = None
        self._length = 0


class TrieNode:
    def __init__(self):
        self.children: Dict[str, 'TrieNode'] = {}
        self.is_end_of_word = False


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        current = self.root
        for char in word:
            if char not in current.children:
                current.children[char] = TrieNode()
            current = current.children[char]
        current.is_end_of_word = True

    def search(self, word: str) -> bool:
        current = self.root
        for char in word:
            if char not in current.children:
                return False
            current = current.children[char]
        return current.is_end_of_word

    def starts_with(self, prefix: str) -> bool:
        current = self.root
        for char in prefix:
            if char not in current.children:
                return False
            current = current.children[char]
        return True

    def delete(self, word: str) -> None:
        def _delete_helper(node: TrieNode, word: str, index: int) -> bool:
            if index == len(word):
                if not node.is_end_of_word:
                    return False
                node.is_end_of_word = False
                return len(node.children) == 0

            char = word[index]
            if char not in node.children:
                return False

            should_delete_child = _delete_helper(node.children[char], word, index + 1)

            if should_delete_child:
                del node.children[char]
                return len(node.children) == 0 and not node.is_end_of_word

            return False

        _delete_helper(self.root, word, 0)

    def get_all_words(self) -> List[str]:
        words = []

        def _get_all_words_helper(node: TrieNode, prefix: str) -> None:
            if node.is_end_of_word:
                words.append(prefix)
            for char, child in node.children.items():
                _get_all_words_helper(child, prefix + char)

        _get_all_words_helper(self.root, '')
        return words

    def get_words_with_prefix(self, prefix: str) -> List[str]:
        current = self.root
        for char in prefix:
            if char not in current.children:
                return []
            current = current.children[char]

        words = []

        def _get_words_helper(node: TrieNode, current_prefix: str) -> None:
            if node.is_end_of_word:
                words.append(current_prefix)
            for char, child in node.children.items():
                _get_words_helper(child, current_prefix + char)

        _get_words_helper(current, prefix)
        return words


class GraphNode(Generic[T]):
    def __init__(self, value: T):
        self.value = value
        self.neighbors: List['GraphNode[T]'] = []


class Graph(Generic[T]):
    def __init__(self, is_directed: bool = False):
        self._nodes: Dict[T, GraphNode[T]] = {}
        self._is_directed = is_directed

    def add_vertex(self, value: T) -> None:
        if value not in self._nodes:
            self._nodes[value] = GraphNode(value)

    def add_edge(self, from_value: T, to_value: T) -> None:
        self.add_vertex(from_value)
        self.add_vertex(to_value)

        from_node = self._nodes[from_value]
        to_node = self._nodes[to_value]

        from_node.neighbors.append(to_node)
        if not self._is_directed:
            to_node.neighbors.append(from_node)

    def remove_vertex(self, value: T) -> None:
        if value not in self._nodes:
            return

        node = self._nodes[value]
        for neighbor in node.neighbors:
            if node in neighbor.neighbors:
                neighbor.neighbors.remove(node)

        del self._nodes[value]

    def remove_edge(self, from_value: T, to_value: T) -> None:
        if from_value not in self._nodes or to_value not in self._nodes:
            return

        from_node = self._nodes[from_value]
        to_node = self._nodes[to_value]

        if to_node in from_node.neighbors:
            from_node.neighbors.remove(to_node)

        if not self._is_directed and from_node in to_node.neighbors:
            to_node.neighbors.remove(from_node)

    def bfs(self, start: T) -> List[T]:
        result = []
        visited = set()
        if start not in self._nodes:
            return result

        queue: _Deque[GraphNode[T]] = deque([self._nodes[start]])
        visited.add(start)

        while queue:
            current = queue.popleft()
            result.append(current.value)
            for neighbor in current.neighbors:
                if neighbor.value not in visited:
                    visited.add(neighbor.value)
                    queue.append(neighbor)

        return result

    def dfs(self, start: T) -> List[T]:
        result = []
        visited = set()
        if start not in self._nodes:
            return result

        stack = [self._nodes[start]]
        visited.add(start)

        while stack:
            current = stack.pop()
            result.append(current.value)
            for neighbor in reversed(current.neighbors):
                if neighbor.value not in visited:
                    visited.add(neighbor.value)
                    stack.append(neighbor)

        return result

    def get_vertices(self) -> List[T]:
        return list(self._nodes.keys())

    def has_vertex(self, value: T) -> bool:
        return value in self._nodes

    def has_edge(self, from_value: T, to_value: T) -> bool:
        if from_value not in self._nodes or to_value not in self._nodes:
            return False
        from_node = self._nodes[from_value]
        return any(neighbor.value == to_value for neighbor in from_node.neighbors)


class UnionFind:
    def __init__(self, size: int):
        self.parent = list(range(size))
        self.rank = [0] * size

    def find(self, x: int) -> int:
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, x: int, y: int) -> bool:
        root_x = self.find(x)
        root_y = self.find(y)

        if root_x == root_y:
            return False

        if self.rank[root_x] < self.rank[root_y]:
            self.parent[root_x] = root_y
        elif self.rank[root_x] > self.rank[root_y]:
            self.parent[root_y] = root_x
        else:
            self.parent[root_y] = root_x
            self.rank[root_x] += 1

        return True

    def connected(self, x: int, y: int) -> bool:
        return self.find(x) == self.find(y)

    def get_count(self) -> int:
        roots = set()
        for i in range(len(self.parent)):
            roots.add(self.find(i))
        return len(roots)


class SkipListNode(Generic[K]):
    def __init__(self, value: K, level: int):
        self.value = value
        self.forward: List[Optional['SkipListNode[K]']] = [None] * (level + 1)


class SkipList(Generic[K]):
    MAX_LEVEL = 16
    P = 0.5

    def __init__(self):
        self._level = 0
        self._head = SkipListNode(None, self.MAX_LEVEL)

    def _random_level(self) -> int:
        level = 0
        while random.random() < self.P and level < self.MAX_LEVEL - 1:
            level += 1
        return level

    def search(self, value: K) -> bool:
        current = self._head

        for i in range(self._level, -1, -1):
            while current.forward[i] and current.forward[i].value < value:
                current = current.forward[i]

        current = current.forward[0]
        return current is not None and current.value == value

    def insert(self, value: K) -> None:
        update: List[Optional[SkipListNode[K]]] = [None] * (self.MAX_LEVEL + 1)
        current = self._head

        for i in range(self._level, -1, -1):
            while current.forward[i] and current.forward[i].value < value:
                current = current.forward[i]
            update[i] = current

        current = current.forward[0]

        if current is None or current.value != value:
            new_level = self._random_level()

            if new_level > self._level:
                for i in range(self._level + 1, new_level + 1):
                    update[i] = self._head
                self._level = new_level

            new_node = SkipListNode(value, new_level)

            for i in range(new_level + 1):
                new_node.forward[i] = update[i].forward[i]
                update[i].forward[i] = new_node

    def delete(self, value: K) -> bool:
        update: List[Optional[SkipListNode[K]]] = [None] * (self.MAX_LEVEL + 1)
        current = self._head

        for i in range(self._level, -1, -1):
            while current.forward[i] and current.forward[i].value < value:
                current = current.forward[i]
            update[i] = current

        current = current.forward[0]

        if current and current.value == value:
            for i in range(self._level + 1):
                if update[i].forward[i] != current:
                    break
                update[i].forward[i] = current.forward[i]

            while self._level > 0 and self._head.forward[self._level] is None:
                self._level -= 1

            return True

        return False

    def to_list(self) -> List[K]:
        result = []
        current = self._head.forward[0]
        while current:
            result.append(current.value)
            current = current.forward[0]
        return result


class SegmentTree(Generic[T]):
    def __init__(self, data: List[T], merge: Callable[[T, T], T], default_value: T):
        self._n = len(data)
        self._merge = merge
        self._default_value = default_value
        self._size = 1
        while self._size < self._n:
            self._size <<= 1
        self._tree: List[T] = [default_value] * (2 * self._size)
        
        for i in range(self._n):
            self._tree[self._size + i] = data[i]
        
        for i in range(self._size - 1, 0, -1):
            self._tree[i] = self._merge(self._tree[2 * i], self._tree[2 * i + 1])

    def update(self, index: int, value: T) -> None:
        if index < 0 or index >= self._n:
            raise IndexError("Index out of bounds")
        
        index += self._size
        self._tree[index] = value
        index >>= 1
        
        while index >= 1:
            new_val = self._merge(self._tree[2 * index], self._tree[2 * index + 1])
            if self._tree[index] == new_val:
                break
            self._tree[index] = new_val
            index >>= 1

    def query(self, l: int, r: int) -> T:
        if l < 0 or r >= self._n or l > r:
            raise IndexError("Invalid query range")
        
        res_left = self._default_value
        res_right = self._default_value
        l += self._size
        r += self._size
        
        while l <= r:
            if l % 2 == 1:
                res_left = self._merge(res_left, self._tree[l])
                l += 1
            if r % 2 == 0:
                res_right = self._merge(self._tree[r], res_right)
                r -= 1
            l >>= 1
            r >>= 1
        
        return self._merge(res_left, res_right)

    def get(self, index: int) -> T:
        if index < 0 or index >= self._n:
            raise IndexError("Index out of bounds")
        return self._tree[self._size + index]


class FenwickTree:
    def __init__(self, arg):
        if isinstance(arg, int):
            self._n = arg
            self._tree = [0] * (self._n + 1)
        elif isinstance(arg, list):
            self._n = len(arg)
            self._tree = [0] * (self._n + 1)
            for i in range(self._n):
                self.update(i, arg[i])
        else:
            raise TypeError("Argument must be an integer or a list")

    def update(self, index: int, delta: int) -> None:
        if index < 0 or index >= self._n:
            raise IndexError("Index out of bounds")
        index += 1
        while index <= self._n:
            self._tree[index] += delta
            index += index & -index

    def set(self, index: int, value: int) -> None:
        current = self.query(index, index)
        self.update(index, value - current)

    def prefix_sum(self, index: int) -> int:
        if index < 0 or index >= self._n:
            raise IndexError("Index out of bounds")
        index += 1
        sum_val = 0
        while index > 0:
            sum_val += self._tree[index]
            index -= index & -index
        return sum_val

    def query(self, l: int, r: int) -> int:
        if l < 0 or r >= self._n or l > r:
            raise IndexError("Invalid query range")
        if l == 0:
            return self.prefix_sum(r)
        return self.prefix_sum(r) - self.prefix_sum(l - 1)

    @property
    def size(self) -> int:
        return self._n


class BloomFilter:
    def __init__(self, expected_items: int, false_positive_rate: float = 0.01):
        self._size = self._calculate_size(expected_items, false_positive_rate)
        self._num_hash_functions = self._calculate_num_hash_functions(self._size, expected_items)
        self._bit_array = bytearray(math.ceil(self._size / 8))

    def _calculate_size(self, n: int, p: float) -> int:
        return math.ceil(-n * math.log(p) / (math.log(2) ** 2))

    def _calculate_num_hash_functions(self, m: int, n: int) -> int:
        return max(1, round((m / n) * math.log(2)))

    def _hash(self, item: str, seed: int) -> int:
        hash_val = seed
        for char in item:
            hash_val = (hash_val * 31 + ord(char)) % self._size
        return hash_val

    def add(self, item: str) -> None:
        for i in range(self._num_hash_functions):
            hash_val = self._hash(item, i)
            byte_index = hash_val // 8
            bit_index = hash_val % 8
            self._bit_array[byte_index] |= (1 << bit_index)

    def might_contain(self, item: str) -> bool:
        for i in range(self._num_hash_functions):
            hash_val = self._hash(item, i)
            byte_index = hash_val // 8
            bit_index = hash_val % 8
            if not (self._bit_array[byte_index] & (1 << bit_index)):
                return False
        return True

    def clear(self) -> None:
        for i in range(len(self._bit_array)):
            self._bit_array[i] = 0


class LRUCacheNode(Generic[K, V]):
    def __init__(self, key: K, value: V):
        self.key = key
        self.value = value
        self.prev: Optional['LRUCacheNode[K, V]'] = None
        self.next: Optional['LRUCacheNode[K, V]'] = None


class LRUCache(Generic[K, V]):
    def __init__(self, capacity: int):
        if capacity <= 0:
            raise ValueError("Capacity must be positive")
        self._capacity = capacity
        self._cache: Dict[K, LRUCacheNode[K, V]] = {}
        self._head = LRUCacheNode(None, None)
        self._tail = LRUCacheNode(None, None)
        self._head.next = self._tail
        self._tail.prev = self._head

    def _add_to_head(self, node: LRUCacheNode[K, V]) -> None:
        node.prev = self._head
        node.next = self._head.next
        self._head.next.prev = node
        self._head.next = node

    def _remove_node(self, node: LRUCacheNode[K, V]) -> None:
        node.prev.next = node.next
        node.next.prev = node.prev

    def _move_to_head(self, node: LRUCacheNode[K, V]) -> None:
        self._remove_node(node)
        self._add_to_head(node)

    def _remove_tail(self) -> LRUCacheNode[K, V]:
        node = self._tail.prev
        self._remove_node(node)
        return node

    def get(self, key: K) -> Optional[V]:
        node = self._cache.get(key)
        if not node:
            return None
        self._move_to_head(node)
        return node.value

    def put(self, key: K, value: V) -> None:
        node = self._cache.get(key)
        if node:
            node.value = value
            self._move_to_head(node)
        else:
            new_node = LRUCacheNode(key, value)
            self._cache[key] = new_node
            self._add_to_head(new_node)
            if len(self._cache) > self._capacity:
                tail = self._remove_tail()
                del self._cache[tail.key]

    def has(self, key: K) -> bool:
        return key in self._cache

    def delete(self, key: K) -> bool:
        node = self._cache.get(key)
        if not node:
            return False
        self._remove_node(node)
        del self._cache[key]
        return True

    def clear(self) -> None:
        self._cache.clear()
        self._head.next = self._tail
        self._tail.prev = self._head

    @property
    def size(self) -> int:
        return len(self._cache)

    def keys(self) -> List[K]:
        keys = []
        current = self._head.next
        while current != self._tail:
            keys.append(current.key)
            current = current.next
        return keys

    def values(self) -> List[V]:
        values = []
        current = self._head.next
        while current != self._tail:
            values.append(current.value)
            current = current.next
        return values


class SuffixArray:
    def __init__(self, text: str):
        self._text = text
        self._suffix_array = self._build_suffix_array(text)
        self._lcp_array = None

    def _build_suffix_array(self, s: str) -> List[int]:
        n = len(s)
        sa = list(range(n))
        rank = [ord(c) for c in s]
        k = 1

        while k < n:
            sa.sort(key=lambda x: (rank[x], rank[x + k] if x + k < n else -1))
            
            new_rank = [0] * n
            new_rank[sa[0]] = 0
            for i in range(1, n):
                prev = sa[i - 1]
                curr = sa[i]
                same = rank[prev] == rank[curr] and \
                       (rank[prev + k] if prev + k < n else -1) == (rank[curr + k] if curr + k < n else -1)
                new_rank[curr] = new_rank[prev] + (0 if same else 1)
            rank = new_rank
            k *= 2

        return sa

    def get_suffix_array(self) -> List[int]:
        return list(self._suffix_array)

    def get_suffix(self, index: int) -> str:
        if index < 0 or index >= len(self._text):
            raise IndexError("Index out of bounds")
        return self._text[index:]

    def get_lcp_array(self) -> List[int]:
        if self._lcp_array is None:
            self._lcp_array = self._build_lcp_array()
        return list(self._lcp_array)

    def _build_lcp_array(self) -> List[int]:
        n = len(self._text)
        rank = [0] * n
        for i in range(n):
            rank[self._suffix_array[i]] = i

        lcp = [0] * (n - 1)
        k = 0
        for i in range(n):
            if rank[i] == n - 1:
                k = 0
                continue
            j = self._suffix_array[rank[i] + 1]
            while i + k < n and j + k < n and self._text[i + k] == self._text[j + k]:
                k += 1
            lcp[rank[i]] = k
            if k > 0:
                k -= 1
        return lcp

    def search(self, pattern: str) -> List[int]:
        result = []
        m = len(pattern)
        n = len(self._text)
        
        low = 0
        high = n - 1
        
        while low <= high:
            mid = (low + high) // 2
            suffix = self.get_suffix(self._suffix_array[mid])
            suffix_prefix = suffix[:min(m, len(suffix))]
            
            if pattern == suffix_prefix:
                result.append(self._suffix_array[mid])
                left = mid - 1
                while left >= 0:
                    left_suffix = self.get_suffix(self._suffix_array[left])
                    if left_suffix.startswith(pattern):
                        result.append(self._suffix_array[left])
                        left -= 1
                    else:
                        break
                right = mid + 1
                while right < n:
                    right_suffix = self.get_suffix(self._suffix_array[right])
                    if right_suffix.startswith(pattern):
                        result.append(self._suffix_array[right])
                        right += 1
                    else:
                        break
                break
            elif pattern < suffix_prefix:
                high = mid - 1
            else:
                low = mid + 1
        
        return sorted(result)

    def get_longest_common_prefix(self) -> int:
        lcp = self.get_lcp_array()
        return 0 if len(lcp) == 0 else max(lcp)

    def get_longest_repeated_substring(self) -> str:
        lcp = self.get_lcp_array()
        max_len = 0
        max_index = 0
        
        for i in range(len(lcp)):
            if lcp[i] > max_len:
                max_len = lcp[i]
                max_index = i
        
        if max_len == 0:
            return ''
        return self._text[self._suffix_array[max_index]:self._suffix_array[max_index] + max_len]


class KDPoint(Protocol):
    @property
    def coordinates(self) -> List[float]:
        ...


class SimpleKDPoint:
    def __init__(self, coordinates: List[float]):
        self._coordinates = coordinates
    
    @property
    def coordinates(self) -> List[float]:
        return self._coordinates


class KDNode(Generic[T]):
    def __init__(self, point: T, axis: int):
        self.point = point
        self.left: Optional['KDNode[T]'] = None
        self.right: Optional['KDNode[T]'] = None
        self.axis = axis


class KDTree(Generic[T]):
    def __init__(self, points: Optional[List[T]] = None):
        if points and len(points) > 0:
            self._dimensions = len(points[0].coordinates)
            self._root = self._build_tree(points, 0)
        else:
            self._root = None
            self._dimensions = 0

    def _build_tree(self, points: List[T], depth: int) -> Optional[KDNode[T]]:
        if len(points) == 0:
            return None

        axis = depth % self._dimensions
        sorted_points = sorted(points, key=lambda p: p.coordinates[axis])
        median = len(sorted_points) // 2

        node = KDNode(sorted_points[median], axis)
        node.left = self._build_tree(sorted_points[:median], depth + 1)
        node.right = self._build_tree(sorted_points[median + 1:], depth + 1)

        return node

    def insert(self, point: T) -> None:
        if self._root is None:
            self._dimensions = len(point.coordinates)
            self._root = KDNode(point, 0)
            return

        if len(point.coordinates) != self._dimensions:
            raise ValueError(f"Point must have {self._dimensions} dimensions")

        current = self._root
        depth = 0

        while True:
            axis = depth % self._dimensions
            if point.coordinates[axis] < current.point.coordinates[axis]:
                if current.left is None:
                    current.left = KDNode(point, (depth + 1) % self._dimensions)
                    break
                current = current.left
            else:
                if current.right is None:
                    current.right = KDNode(point, (depth + 1) % self._dimensions)
                    break
                current = current.right
            depth += 1

    def _distance_squared(self, a: List[float], b: List[float]) -> float:
        return sum((x - y) ** 2 for x, y in zip(a, b))

    def nearest_neighbor(self, target: List[float]) -> Optional[T]:
        if self._root is None or len(target) != self._dimensions:
            return None

        best = [None]
        best_dist = [float('inf')]

        def search(node: Optional[KDNode[T]], depth: int) -> None:
            if node is None:
                return

            dist = self._distance_squared(node.point.coordinates, target)
            if dist < best_dist[0]:
                best_dist[0] = dist
                best[0] = node

            axis = depth % self._dimensions
            go_left = target[axis] < node.point.coordinates[axis]

            search(node.left if go_left else node.right, depth + 1)

            plane_dist = (target[axis] - node.point.coordinates[axis]) ** 2
            if plane_dist < best_dist[0]:
                search(node.right if go_left else node.left, depth + 1)

        search(self._root, 0)
        return best[0].point if best[0] else None

    def range_search(self, min_coords: List[float], max_coords: List[float]) -> List[T]:
        result = []
        
        if self._root is None or len(min_coords) != self._dimensions or len(max_coords) != self._dimensions:
            return result

        def search(node: Optional[KDNode[T]]) -> None:
            if node is None:
                return

            point = node.point.coordinates
            in_range = True
            for i in range(self._dimensions):
                if point[i] < min_coords[i] or point[i] > max_coords[i]:
                    in_range = False
                    break
            if in_range:
                result.append(node.point)

            axis = node.axis
            if min_coords[axis] <= point[axis]:
                search(node.left)
            if max_coords[axis] >= point[axis]:
                search(node.right)

        search(self._root)
        return result

    def k_nearest_neighbors(self, target: List[float], k: int) -> List[T]:
        if k <= 0 or self._root is None or len(target) != self._dimensions:
            return []

        neighbors = []

        def search(node: Optional[KDNode[T]], depth: int) -> None:
            if node is None:
                return

            dist = self._distance_squared(node.point.coordinates, target)
            
            if len(neighbors) < k:
                neighbors.append((node.point, dist))
                neighbors.sort(key=lambda x: x[1])
            elif dist < neighbors[-1][1]:
                neighbors.pop()
                neighbors.append((node.point, dist))
                neighbors.sort(key=lambda x: x[1])

            axis = depth % self._dimensions
            go_left = target[axis] < node.point.coordinates[axis]

            search(node.left if go_left else node.right, depth + 1)

            plane_dist = (target[axis] - node.point.coordinates[axis]) ** 2
            if len(neighbors) < k or plane_dist < neighbors[-1][1]:
                search(node.right if go_left else node.left, depth + 1)

        search(self._root, 0)
        return [n[0] for n in neighbors]


if __name__ == "__main__":
    print("=== Segment Tree Example ===")
    data = [1, 3, 5, 7, 9, 11]
    seg_tree = SegmentTree(data, lambda a, b: a + b, 0)
    print(f"Sum [0..5]: {seg_tree.query(0, 5)}")
    seg_tree.update(2, 10)
    print(f"Sum [0..5] after update: {seg_tree.query(0, 5)}")
    print()

    print("=== Fenwick Tree Example ===")
    ft = FenwickTree([1, 2, 3, 4, 5])
    print(f"Prefix sum at 3: {ft.prefix_sum(3)}")
    print(f"Sum [1..3]: {ft.query(1, 3)}")
    ft.update(2, 2)
    print(f"Sum [1..3] after update: {ft.query(1, 3)}")
    print()

    print("=== Bloom Filter Example ===")
    bf = BloomFilter(10)
    bf.add("apple")
    bf.add("banana")
    bf.add("cherry")
    print(f"Might contain 'apple': {bf.might_contain('apple')}")
    print(f"Might contain 'date': {bf.might_contain('date')}")
    print()

    print("=== LRU Cache Example ===")
    lru = LRUCache[str, int](3)
    lru.put("a", 1)
    lru.put("b", 2)
    lru.put("c", 3)
    print(f"Cache keys: {lru.keys()}")
    print(f"Get 'a': {lru.get('a')}")
    lru.put("d", 4)
    print(f"Cache keys after putting 'd': {lru.keys()}")
    print(f"Has 'b'? {lru.has('b')}")
    print()

    print("=== Suffix Array Example ===")
    text = "banana"
    sa = SuffixArray(text)
    print(f"Suffix array: {sa.get_suffix_array()}")
    print(f"Search 'ana': {sa.search('ana')}")
    print(f"Longest repeated substring: '{sa.get_longest_repeated_substring()}'")
    print()

    print("=== KD Tree Example ===")
    points = [
        SimpleKDPoint([2, 3]),
        SimpleKDPoint([5, 4]),
        SimpleKDPoint([9, 6]),
        SimpleKDPoint([4, 7]),
        SimpleKDPoint([8, 1]),
        SimpleKDPoint([7, 2])
    ]
    kdtree = KDTree(points)
    target = [5, 5]
    nearest = kdtree.nearest_neighbor(target)
    print(f"Nearest to {target}: {nearest.coordinates}")
    range_result = kdtree.range_search([3, 1], [9, 5])
    print(f"Range query [3,1] to [9,5]: {[p.coordinates for p in range_result]}")
    k_nearest = kdtree.k_nearest_neighbors(target, 3)
    print(f"3 nearest to {target}: {[p.coordinates for p in k_nearest]}")
