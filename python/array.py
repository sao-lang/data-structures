from typing import TypeVar, Generic, List, Optional


T = TypeVar("T")


class FixedArray(Generic[T]):
    def __init__(self, capacity: int):
        if capacity <= 0:
            raise ValueError("Capacity must be a positive integer.")
        self._items: List[Optional[T]] = [None] * capacity
        self._size: int = 0
        self._capacity: int = capacity

    def _check_index(self, index: int):
        if not (0 <= index <= self._size):  # Allow index 0 as valid
            raise ValueError(
                f"Index out of bounds: {index}. Current size: {self._size}."
            )

    def _check_capacity(self):
        if self._size >= self._capacity:
            raise ValueError("Fixed Array is full. Cannot add more elements.")

    def add_at(self, index: int, element: T):
        if not (0 <= index < self._size):
            raise ValueError(
                "Insertion index is out of bounds.",
                f"Valid range: 0 to {self._size}. Requested: {index}.",
            )
        self._check_capacity()
        for i in range(self._size, index, -1):
            self._items[i] = self._items[i - 1]
        self._items[index] = element
        self._size += 1

    def append(self, element: T):
        self._check_capacity()
        self._items[self._size] = element
        self._size += 1

    def prepend(self, element: T):
        self.add_at(0, element)

    def remove(self, index: T) -> T:
        self._check_index(index)
        removed_element = self._items[index]
        for i in range(index, self._size):
            self._items[i] = self._items[i + 1]
        self._size -= 1
        return removed_element

    def pop_front(self):
        self.remove(0)

    def pop_back(self):
        self.remove(self._size - 1)

    def get(self, index: int) -> Optional[T]:
        self._check_index(index)
        return self._items[index]

    def set(self, index: int, element: T):
        self._check_index(index)
        self._items[index] = element

    def get_first(self) -> Optional[T]:
        return self._items[0]

    def get_last(self) -> Optional[T]:
        return self._items[self._size]

    def sort(self, key=None, reverse=False):
        """对数组中的元素进行排序。
        Python的list.sort()是in-place排序，这里我们使用它的特性。
        """
        # 只对实际包含元素的切片进行排序
        actual_elements = self._items[: self._size]
        actual_elements.sort(key=key, reverse=reverse)
        # 将排序后的元素复制回内部数组
        self._items[: self._size] = actual_elements

    def get_size(self) -> int:
        return self._size

    def get_capacity(self) -> int:
        return self._capacity

    def get_elements(self) -> List[Optional[T]]:
        return self._items

    def to_array(self):
        """返回实际包含元素的列表副本"""
        return self._items[: self._size]

    def __str__(self):
        """支持print(fixed_array)"""
        elements = self.to_array()
        return f"[Size: {self._size}, Capacity: {self._capacity}, {elements}]"


# fixed_array = FixedArray(10)

# for i in range(7):
#     fixed_array.append(i)

# print(fixed_array)
# fixed_array.add_at(2, 11)
# print(fixed_array)


class DynamicArray(Generic[T]):
    def __init__(self, initial_capacity: int):
        if initial_capacity <= 0:
            raise ValueError("Capacity must be a positive integer.")
        self._items: List[Optional[T]] = [None] * initial_capacity
        self._size: int = 0
        self._growth_factor: int = 2

    def _check_index(self, index: int):
        if not (0 <= index <= self._size):  # Allow index 0 as valid
            raise ValueError(
                f"Index out of bounds: {index}. Current size: {self._size}."
            )

    def _ensure_capacity(self):
        if self._size == len(self._items):
            new_capacity = len(self._items) * self._growth_factor
            if new_capacity <= 0:
                new_capacity = 1
            new_items: List[Optional[T]] = [None] * new_capacity
            for i in range(len(self._items)):
                new_items[i] = self._items[i]
            self._items = new_items

    def add_at(self, index: int, element: T):
        if not (0 <= index < self._size):
            raise ValueError(
                "Insertion index is out of bounds.",
                f"Valid range: 0 to {self._size}. Requested: {index}.",
            )
        self._ensure_capacity()
        for i in range(self._size, index, -1):
            self._items[i] = self._items[i - 1]
        self._items[index] = element
        self._size += 1

    def append(self, element: T):
        self._ensure_capacity()
        self._items[self._size] = element
        self._size += 1

    def prepend(self, element: T):
        self.add_at(0, element)

    def remove(self, index: T) -> T:
        self._check_index(index)
        removed_element = self._items[index]
        for i in range(index, self._size):
            self._items[i] = self._items[i + 1]
        self._size -= 1
        return removed_element

    def pop_front(self):
        self.remove(0)

    def pop_back(self):
        self.remove(self._size - 1)

    def get(self, index: int) -> Optional[T]:
        self._check_index(index)
        return self._items[index]

    def set(self, index: int, element: T):
        self._check_index(index)
        self._items[index] = element

    def get_first(self) -> Optional[T]:
        return self._items[0]

    def get_last(self) -> Optional[T]:
        return self._items[self._size]

    def sort(self, key=None, reverse=False):
        """对数组中的元素进行排序。
        Python的list.sort()是in-place排序，这里我们使用它的特性。
        """
        # 只对实际包含元素的切片进行排序
        actual_elements = self._items[: self._size]
        actual_elements.sort(key=key, reverse=reverse)
        # 将排序后的元素复制回内部数组
        self._items[: self._size] = actual_elements

    def get_size(self) -> int:
        return self._size

    def get_capacity(self) -> int:
        return self._capacity

    def get_elements(self) -> List[Optional[T]]:
        return self._items

    def to_array(self):
        """返回实际包含元素的列表副本"""
        return self._items[: self._size]

    def __str__(self):
        """支持print(fixed_array)"""
        elements = self.to_array()
        return f"[Size: {self._size}, {elements}]"


dynamic_array = DynamicArray(5)

for i in range(7):
    dynamic_array.append(i)

print(dynamic_array)
dynamic_array.add_at(2, 11)
print(dynamic_array)
