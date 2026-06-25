from typing import TypeVar, Generic, List, Optional, Callable

T = TypeVar('T')

class FixedArray(Generic[T]):
    def __init__(self, capacity: int):
        if capacity <= 0:
            raise ValueError("Capacity must be positive")
        self._capacity: int = capacity
        self._length: int = 0
        self._data: List[Optional[T]] = [None] * capacity

    @property
    def capacity(self) -> int:
        return self._capacity

    @property
    def length(self) -> int:
        return self._length

    @property
    def is_empty(self) -> bool:
        return self._length == 0

    @property
    def is_full(self) -> bool:
        return self._length == self._capacity

    def at(self, index: int) -> Optional[T]:
        if index < 0 or index >= self._length:
            raise IndexError("Index out of bounds")
        return self._data[index]

    def set(self, index: int, value: T) -> None:
        if index < 0 or index >= self._length:
            raise IndexError("Index out of bounds")
        self._data[index] = value

    def push(self, value: T) -> None:
        if self.is_full:
            raise RuntimeError("Array is full")
        self._data[self._length] = value
        self._length += 1

    def pop(self) -> Optional[T]:
        if self.is_empty:
            raise RuntimeError("Array is empty")
        self._length -= 1
        value = self._data[self._length]
        self._data[self._length] = None
        return value

    def insert(self, index: int, value: T) -> None:
        if self.is_full:
            raise RuntimeError("Array is full")
        if index < 0 or index > self._length:
            raise IndexError("Index out of bounds")
        for i in range(self._length, index, -1):
            self._data[i] = self._data[i - 1]
        self._data[index] = value
        self._length += 1

    def remove(self, index: int) -> Optional[T]:
        if self.is_empty:
            raise RuntimeError("Array is empty")
        if index < 0 or index >= self._length:
            raise IndexError("Index out of bounds")
        value = self._data[index]
        for i in range(index, self._length - 1):
            self._data[i] = self._data[i + 1]
        self._length -= 1
        self._data[self._length] = None
        return value

    def find(self, value: T, equal: Callable[[T, T], bool] = lambda x, y: x == y) -> int:
        for i in range(self._length):
            if self._data[i] is not None and equal(self._data[i], value):
                return i
        return -1

    def to_list(self) -> List[Optional[T]]:
        return self._data[:self._length]

    def clear(self) -> None:
        self._data = [None] * self._capacity
        self._length = 0

    def __getitem__(self, index: int) -> Optional[T]:
        return self.at(index)

    def __setitem__(self, index: int, value: T) -> None:
        self.set(index, value)

    def __len__(self) -> int:
        return self._length

    def __iter__(self):
        for i in range(self._length):
            yield self._data[i]

    def __str__(self) -> str:
        return str(self.to_list())


class DynamicArray(Generic[T]):
    def __init__(self, initial_capacity: int = 10):
        if initial_capacity <= 0:
            raise ValueError("Initial capacity must be positive")
        self._capacity: int = initial_capacity
        self._length: int = 0
        self._data: List[Optional[T]] = [None] * initial_capacity
        self._growth_factor: int = 2

    @property
    def capacity(self) -> int:
        return self._capacity

    @property
    def length(self) -> int:
        return self._length

    @property
    def is_empty(self) -> bool:
        return self._length == 0

    def _resize(self) -> None:
        new_capacity = self._capacity * self._growth_factor
        new_data: List[Optional[T]] = [None] * new_capacity
        for i in range(self._length):
            new_data[i] = self._data[i]
        self._data = new_data
        self._capacity = new_capacity

    def at(self, index: int) -> Optional[T]:
        if index < 0 or index >= self._length:
            raise IndexError("Index out of bounds")
        return self._data[index]

    def set(self, index: int, value: T) -> None:
        if index < 0 or index >= self._length:
            raise IndexError("Index out of bounds")
        self._data[index] = value

    def push(self, value: T) -> None:
        if self._length >= self._capacity:
            self._resize()
        self._data[self._length] = value
        self._length += 1

    def pop(self) -> Optional[T]:
        if self.is_empty:
            raise RuntimeError("Array is empty")
        self._length -= 1
        value = self._data[self._length]
        self._data[self._length] = None
        return value

    def insert(self, index: int, value: T) -> None:
        if index < 0 or index > self._length:
            raise IndexError("Index out of bounds")
        if self._length >= self._capacity:
            self._resize()
        for i in range(self._length, index, -1):
            self._data[i] = self._data[i - 1]
        self._data[index] = value
        self._length += 1

    def remove(self, index: int) -> Optional[T]:
        if self.is_empty:
            raise RuntimeError("Array is empty")
        if index < 0 or index >= self._length:
            raise IndexError("Index out of bounds")
        value = self._data[index]
        for i in range(index, self._length - 1):
            self._data[i] = self._data[i + 1]
        self._length -= 1
        self._data[self._length] = None
        return value

    def find(self, value: T, equal: Callable[[T, T], bool] = lambda x, y: x == y) -> int:
        for i in range(self._length):
            if self._data[i] is not None and equal(self._data[i], value):
                return i
        return -1

    def to_list(self) -> List[Optional[T]]:
        return self._data[:self._length]

    def clear(self) -> None:
        self._data = [None] * 10
        self._capacity = 10
        self._length = 0

    def sort(self, key: Optional[Callable[[T], any]] = None, reverse: bool = False) -> None:
        arr = [x for x in self._data[:self._length] if x is not None]
        arr.sort(key=key, reverse=reverse)
        for i in range(len(arr)):
            self._data[i] = arr[i]
        for i in range(len(arr), self._length):
            self._data[i] = None

    def __getitem__(self, index: int) -> Optional[T]:
        return self.at(index)

    def __setitem__(self, index: int, value: T) -> None:
        self.set(index, value)

    def __len__(self) -> int:
        return self._length

    def __iter__(self):
        for i in range(self._length):
            yield self._data[i]

    def __str__(self) -> str:
        return str(self.to_list())


if __name__ == "__main__":
    print("FixedArray Example:")
    fa = FixedArray[int](5)
    fa.push(1)
    fa.push(2)
    fa.push(3)
    print(f"FixedArray: {fa}")
    print("Iterating with for loop:")
    for val in fa:
        print(val, end=' ')
    print()
    
    print("\nDynamicArray Example:")
    da = DynamicArray[int](2)
    da.push(10)
    da.push(20)
    da.push(30)
    print(f"DynamicArray: {da}")
    print(f"Capacity: {da.capacity}")
    print("Iterating with for loop:")
    for val in da:
        print(val, end=' ')
    print()
