from typing import TypeVar, Generic, Optional, List, Tuple, Callable

K = TypeVar('K')
V = TypeVar('V')

class HashNode(Generic[K, V]):
    def __init__(self, key: K, value: V):
        self.key: K = key
        self.value: V = value
        self.next: Optional[HashNode[K, V]] = None

class HashTable(Generic[K, V]):
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

    def for_each(self, callback: Callable[[V, K, 'HashTable[K, V]'], None]) -> None:
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

if __name__ == "__main__":
    print("Hash Table Example:")
    ht = HashTable[str, int]()
    ht["one"] = 1
    ht["two"] = 2
    ht["three"] = 3
    print(f"Size: {len(ht)}")
    print(f"Value for 'two': {ht['two']}")
    print(f"Keys: {ht.keys()}")
    print(f"Values: {ht.values()}")
    print(f"Entries: {ht.entries()}")
