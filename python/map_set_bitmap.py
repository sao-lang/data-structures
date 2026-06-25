from typing import TypeVar, Generic, Optional, List, Tuple, Callable, Iterable, Any

K = TypeVar('K')
V = TypeVar('V')
T = TypeVar('T')


class HashNode(Generic[K, V]):
    def __init__(self, key: K, value: V):
        self.key: K = key
        self.value: V = value
        self.next: Optional[HashNode[K, V]] = None


class Map(Generic[K, V]):
    def __init__(self, initial_capacity: int = 16):
        self._capacity: int = initial_capacity
        self._size: int = 0
        self._buckets: List[Optional[HashNode[K, V]]] = [None] * initial_capacity
        self._load_factor: float = 0.7

    @property
    def size(self) -> int:
        return self._size

    @property
    def is_empty(self) -> bool:
        return self._size == 0

    @property
    def capacity(self) -> int:
        return self._capacity

    def _hash(self, key: K) -> int:
        hash_val = 0
        key_str = str(key)
        for char in key_str:
            hash_val = (hash_val << 5) - hash_val + ord(char)
            hash_val = hash_val & hash_val
        return abs(hash_val) % self._capacity

    def _resize(self) -> None:
        old_buckets = self._buckets
        self._capacity *= 2
        self._size = 0
        self._buckets = [None] * self._capacity

        for bucket in old_buckets:
            current = bucket
            while current:
                self.set(current.key, current.value)
                current = current.next

    def set(self, key: K, value: V) -> None:
        if self._size / self._capacity >= self._load_factor:
            self._resize()

        index = self._hash(key)
        current = self._buckets[index]

        while current:
            if current.key == key:
                current.value = value
                return
            current = current.next

        new_node = HashNode(key, value)
        new_node.next = self._buckets[index]
        self._buckets[index] = new_node
        self._size += 1

    def get(self, key: K) -> Optional[V]:
        index = self._hash(key)
        current = self._buckets[index]

        while current:
            if current.key == key:
                return current.value
            current = current.next

        return None

    def has(self, key: K) -> bool:
        return self.get(key) is not None

    def delete(self, key: K) -> bool:
        index = self._hash(key)
        current = self._buckets[index]
        prev: Optional[HashNode[K, V]] = None

        while current:
            if current.key == key:
                if prev:
                    prev.next = current.next
                else:
                    self._buckets[index] = current.next
                self._size -= 1
                return True
            prev = current
            current = current.next

        return False

    def keys(self) -> List[K]:
        keys_list: List[K] = []
        for bucket in self._buckets:
            current = bucket
            while current:
                keys_list.append(current.key)
                current = current.next
        return keys_list

    def values(self) -> List[V]:
        values_list: List[V] = []
        for bucket in self._buckets:
            current = bucket
            while current:
                values_list.append(current.value)
                current = current.next
        return values_list

    def entries(self) -> List[Tuple[K, V]]:
        entries_list: List[Tuple[K, V]] = []
        for bucket in self._buckets:
            current = bucket
            while current:
                entries_list.append((current.key, current.value))
                current = current.next
        return entries_list

    def clear(self) -> None:
        self._buckets = [None] * self._capacity
        self._size = 0

    def for_each(self, callback: Callable[[V, K, 'Map[K, V]'], None]) -> None:
        for bucket in self._buckets:
            current = bucket
            while current:
                callback(current.value, current.key, self)
                current = current.next

    def __getitem__(self, key: K) -> Optional[V]:
        return self.get(key)

    def __setitem__(self, key: K, value: V) -> None:
        self.set(key, value)

    def __contains__(self, key: K) -> bool:
        return self.has(key)

    def __len__(self) -> int:
        return self._size

    def __iter__(self):
        for key in self.keys():
            yield key

    def __repr__(self) -> str:
        items = [f"{repr(k)}: {repr(v)}" for k, v in self.entries()]
        return f"Map({{{', '.join(items)}}})"

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, Map):
            return False
        if self.size != other.size:
            return False
        for key in self:
            if key not in other or self[key] != other[key]:
                return False
        return True

    def update(self, other: 'Map[K, V]') -> None:
        for key, value in other.entries():
            self[key] = value

    def __add__(self, other: 'Map[K, V]') -> 'Map[K, V]':
        result = Map[K, V](max(self.capacity, other.capacity))
        result.update(self)
        result.update(other)
        return result


class BitMap:
    def __init__(self, size: int = 64):
        self._size = size
        self._bits = [0] * ((size + 63) // 64)

    @property
    def size(self) -> int:
        return self._size

    def _get_index_and_mask(self, bit: int) -> Tuple[int, int]:
        if bit < 0 or bit >= self._size:
            raise IndexError(f"Bit index out of range: {bit}")
        index = bit // 64
        mask = 1 << (bit % 64)
        return index, mask

    def set(self, bit: int) -> None:
        index, mask = self._get_index_and_mask(bit)
        self._bits[index] |= mask

    def clear(self, bit: int) -> None:
        index, mask = self._get_index_and_mask(bit)
        self._bits[index] &= ~mask

    def toggle(self, bit: int) -> None:
        index, mask = self._get_index_and_mask(bit)
        self._bits[index] ^= mask

    def get(self, bit: int) -> bool:
        index, mask = self._get_index_and_mask(bit)
        return (self._bits[index] & mask) != 0

    def set_all(self) -> None:
        for i in range(len(self._bits)):
            self._bits[i] = 0xFFFFFFFFFFFFFFFF

    def clear_all(self) -> None:
        for i in range(len(self._bits)):
            self._bits[i] = 0

    def count_set_bits(self) -> int:
        count = 0
        for word in self._bits:
            count += bin(word).count('1')
        return count

    def find_first_set(self) -> Optional[int]:
        for i in range(self._size):
            if self.get(i):
                return i
        return None

    def find_first_clear(self) -> Optional[int]:
        for i in range(self._size):
            if not self.get(i):
                return i
        return None

    def __getitem__(self, bit: int) -> bool:
        return self.get(bit)

    def __setitem__(self, bit: int, value: bool) -> None:
        if value:
            self.set(bit)
        else:
            self.clear(bit)

    def __and__(self, other: 'BitMap') -> 'BitMap':
        if self._size != other._size:
            raise ValueError("BitMaps must have the same size for AND operation")
        result = BitMap(self._size)
        for i in range(len(self._bits)):
            result._bits[i] = self._bits[i] & other._bits[i]
        return result

    def __or__(self, other: 'BitMap') -> 'BitMap':
        if self._size != other._size:
            raise ValueError("BitMaps must have the same size for OR operation")
        result = BitMap(self._size)
        for i in range(len(self._bits)):
            result._bits[i] = self._bits[i] | other._bits[i]
        return result

    def __xor__(self, other: 'BitMap') -> 'BitMap':
        if self._size != other._size:
            raise ValueError("BitMaps must have the same size for XOR operation")
        result = BitMap(self._size)
        for i in range(len(self._bits)):
            result._bits[i] = self._bits[i] ^ other._bits[i]
        return result

    def __invert__(self) -> 'BitMap':
        result = BitMap(self._size)
        for i in range(len(self._bits)):
            result._bits[i] = ~self._bits[i]
        return result

    def __len__(self) -> int:
        return self._size

    def __iter__(self):
        for i in range(self._size):
            yield self.get(i)

    def __repr__(self) -> str:
        bits_str = ''.join('1' if self.get(i) else '0' for i in range(self._size))
        return f"BitMap({bits_str})"


class Set(Generic[T]):
    def __init__(self, initial_capacity: int = 16):
        self._map: Map[T, bool] = Map(initial_capacity)

    @property
    def size(self) -> int:
        return self._map.size

    @property
    def is_empty(self) -> bool:
        return self._map.is_empty

    def add(self, item: T) -> None:
        self._map.set(item, True)

    def remove(self, item: T) -> bool:
        return self._map.delete(item)

    def has(self, item: T) -> bool:
        return self._map.has(item)

    def clear(self) -> None:
        self._map.clear()

    def items(self) -> List[T]:
        return self._map.keys()

    def for_each(self, callback: Callable[[T, 'Set[T]'], None]) -> None:
        def map_callback(value: bool, key: T, map_obj: Map[T, bool]) -> None:
            callback(key, self)
        self._map.for_each(map_callback)

    def union(self, other: 'Set[T]') -> 'Set[T]':
        result = Set[T](max(self.size, other.size) + 1)
        for item in self:
            result.add(item)
        for item in other:
            result.add(item)
        return result

    def intersection(self, other: 'Set[T]') -> 'Set[T]':
        result = Set[T]()
        smaller = self if self.size <= other.size else other
        larger = other if self.size <= other.size else self
        for item in smaller:
            if larger.has(item):
                result.add(item)
        return result

    def difference(self, other: 'Set[T]') -> 'Set[T]':
        result = Set[T]()
        for item in self:
            if not other.has(item):
                result.add(item)
        return result

    def symmetric_difference(self, other: 'Set[T]') -> 'Set[T]':
        result = Set[T]()
        for item in self:
            if not other.has(item):
                result.add(item)
        for item in other:
            if not self.has(item):
                result.add(item)
        return result

    def is_subset(self, other: 'Set[T]') -> bool:
        if self.size > other.size:
            return False
        for item in self:
            if not other.has(item):
                return False
        return True

    def is_superset(self, other: 'Set[T]') -> bool:
        return other.is_subset(self)

    def __contains__(self, item: T) -> bool:
        return self.has(item)

    def __len__(self) -> int:
        return self.size

    def __iter__(self):
        for item in self.items():
            yield item

    def __repr__(self) -> str:
        items = [repr(item) for item in self.items()]
        return f"Set({{{', '.join(items)}}})"

    def __add__(self, other: 'Set[T]') -> 'Set[T]':
        return self.union(other)

    def __sub__(self, other: 'Set[T]') -> 'Set[T]':
        return self.difference(other)

    def __and__(self, other: 'Set[T]') -> 'Set[T]':
        return self.intersection(other)

    def __or__(self, other: 'Set[T]') -> 'Set[T]':
        return self.union(other)

    def __xor__(self, other: 'Set[T]') -> 'Set[T]':
        return self.symmetric_difference(other)

    def __le__(self, other: 'Set[T]') -> bool:
        return self.is_subset(other)

    def __ge__(self, other: 'Set[T]') -> bool:
        return self.is_superset(other)

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, Set):
            return False
        if self.size != other.size:
            return False
        return self.is_subset(other)


if __name__ == "__main__":
    print("=" * 50)
    print("Map 示例:")
    print("=" * 50)
    map1 = Map[str, int]()
    map1["one"] = 1
    map1["two"] = 2
    map1["three"] = 3
    print(f"Map: {map1}")
    print(f"大小: {len(map1)}")
    print(f"包含 'two': {'two' in map1}")
    print(f"'two' 的值: {map1['two']}")
    print(f"键: {map1.keys()}")
    print(f"值: {map1.values()}")
    print()

    print("=" * 50)
    print("BitMap 示例:")
    print("=" * 50)
    bitmap = BitMap(10)
    bitmap[0] = True
    bitmap[2] = True
    bitmap[5] = True
    print(f"BitMap: {bitmap}")
    print(f"设置的位数: {bitmap.count_set_bits()}")
    print(f"第 2 位: {bitmap[2]}")
    bitmap.toggle(2)
    print(f"翻转第 2 位后: {bitmap}")
    print()

    print("=" * 50)
    print("Set 示例:")
    print("=" * 50)
    set1 = Set[int]()
    set1.add(1)
    set1.add(2)
    set1.add(3)
    set2 = Set[int]()
    set2.add(3)
    set2.add(4)
    set2.add(5)
    print(f"Set1: {set1}")
    print(f"Set2: {set2}")
    print(f"并集: {set1 | set2}")
    print(f"交集: {set1 & set2}")
    print(f"差集 (Set1 - Set2): {set1 - set2}")
    print(f"包含 2: {2 in set1}")
