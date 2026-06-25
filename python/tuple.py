from typing import TypeVar, Generic, List, Any, Tuple as _Tuple, Iterator, overload

T = TypeVar('T')
U = TypeVar('U')
V = TypeVar('V')


class Tuple(Generic[T]):
    def __init__(self, *items: T):
        self._items: List[T] = list(items)
    
    @property
    def size(self) -> int:
        return len(self._items)
    
    @property
    def is_empty(self) -> bool:
        return len(self._items) == 0
    
    def at(self, index: int) -> T:
        if index < 0 or index >= len(self._items):
            raise IndexError("Index out of bounds")
        return self._items[index]
    
    def first(self) -> T:
        if self.is_empty:
            raise IndexError("Tuple is empty")
        return self._items[0]
    
    def last(self) -> T:
        if self.is_empty:
            raise IndexError("Tuple is empty")
        return self._items[-1]
    
    def to_list(self) -> List[T]:
        return list(self._items)
    
    def to_python_tuple(self) -> _Tuple[T, ...]:
        return tuple(self._items)
    
    def map(self, func):
        return Tuple(*(func(item) for item in self._items))
    
    def filter(self, func):
        return Tuple(*(item for item in self._items if func(item)))
    
    def reduce(self, func, initial):
        result = initial
        for item in self._items:
            result = func(result, item)
        return result
    
    def zip(self, other: 'Tuple[U]') -> 'Tuple[_Tuple[T, U]]':
        min_len = min(len(self._items), len(other._items))
        return Tuple(*((self._items[i], other._items[i]) for i in range(min_len)))
    
    def concat(self, other: 'Tuple[T]') -> 'Tuple[T]':
        return Tuple(*(self._items + other._items))
    
    def slice(self, start: int, end: int) -> 'Tuple[T]':
        return Tuple(*self._items[start:end])
    
    def take(self, n: int) -> 'Tuple[T]':
        return Tuple(*self._items[:n])
    
    def drop(self, n: int) -> 'Tuple[T]':
        return Tuple(*self._items[n:])
    
    def contains(self, item: T) -> bool:
        return item in self._items
    
    def index(self, item: T) -> int:
        try:
            return self._items.index(item)
        except ValueError:
            return -1
    
    def count(self, item: T) -> int:
        return self._items.count(item)
    
    def reverse(self) -> 'Tuple[T]':
        return Tuple(*reversed(self._items))
    
    def __getitem__(self, index: int) -> T:
        return self.at(index)
    
    def __len__(self) -> int:
        return self.size
    
    def __iter__(self) -> Iterator[T]:
        return iter(self._items)
    
    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, Tuple):
            return False
        return self._items == other._items
    
    def __lt__(self, other: 'Tuple[T]') -> bool:
        return self._items < other._items
    
    def __le__(self, other: 'Tuple[T]') -> bool:
        return self._items <= other._items
    
    def __gt__(self, other: 'Tuple[T]') -> bool:
        return self._items > other._items
    
    def __ge__(self, other: 'Tuple[T]') -> bool:
        return self._items >= other._items
    
    def __hash__(self) -> int:
        return hash(tuple(self._items))
    
    def __add__(self, other: 'Tuple[T]') -> 'Tuple[T]':
        return self.concat(other)
    
    def __mul__(self, times: int) -> 'Tuple[T]':
        if times <= 0:
            return Tuple()
        return Tuple(*(self._items * times))
    
    def __rmul__(self, times: int) -> 'Tuple[T]':
        return self.__mul__(times)
    
    def __str__(self) -> str:
        return f"Tuple{tuple(self._items)}"
    
    def __repr__(self) -> str:
        return f"Tuple{tuple(self._items)}"


class Pair(Generic[T, U]):
    def __init__(self, first: T, second: U):
        self._first = first
        self._second = second
    
    @property
    def first(self) -> T:
        return self._first
    
    @property
    def second(self) -> U:
        return self._second
    
    def swap(self) -> 'Pair[U, T]':
        return Pair(self._second, self._first)
    
    def to_tuple(self) -> Tuple[Any]:
        return Tuple(self._first, self._second)
    
    def to_python_tuple(self) -> _Tuple[T, U]:
        return (self._first, self._second)
    
    def map_first(self, func):
        return Pair(func(self._first), self._second)
    
    def map_second(self, func):
        return Pair(self._first, func(self._second))
    
    def map_both(self, func1, func2):
        return Pair(func1(self._first), func2(self._second))
    
    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, Pair):
            return False
        return self._first == other._first and self._second == other._second
    
    def __hash__(self) -> int:
        return hash((self._first, self._second))
    
    def __iter__(self) -> Iterator[Any]:
        yield self._first
        yield self._second
    
    def __len__(self) -> int:
        return 2
    
    def __getitem__(self, index: int) -> Any:
        if index == 0:
            return self._first
        elif index == 1:
            return self._second
        raise IndexError("Pair index out of bounds")
    
    def __str__(self) -> str:
        return f"Pair({self._first}, {self._second})"
    
    def __repr__(self) -> str:
        return f"Pair({self._first}, {self._second})"


class Triple(Generic[T, U, V]):
    def __init__(self, first: T, second: U, third: V):
        self._first = first
        self._second = second
        self._third = third
    
    @property
    def first(self) -> T:
        return self._first
    
    @property
    def second(self) -> U:
        return self._second
    
    @property
    def third(self) -> V:
        return self._third
    
    def to_tuple(self) -> Tuple[Any]:
        return Tuple(self._first, self._second, self._third)
    
    def to_python_tuple(self) -> _Tuple[T, U, V]:
        return (self._first, self._second, self._third)
    
    def map_first(self, func):
        return Triple(func(self._first), self._second, self._third)
    
    def map_second(self, func):
        return Triple(self._first, func(self._second), self._third)
    
    def map_third(self, func):
        return Triple(self._first, self._second, func(self._third))
    
    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, Triple):
            return False
        return (self._first == other._first and 
                self._second == other._second and 
                self._third == other._third)
    
    def __hash__(self) -> int:
        return hash((self._first, self._second, self._third))
    
    def __iter__(self) -> Iterator[Any]:
        yield self._first
        yield self._second
        yield self._third
    
    def __len__(self) -> int:
        return 3
    
    def __getitem__(self, index: int) -> Any:
        if index == 0:
            return self._first
        elif index == 1:
            return self._second
        elif index == 2:
            return self._third
        raise IndexError("Triple index out of bounds")
    
    def __str__(self) -> str:
        return f"Triple({self._first}, {self._second}, {self._third})"
    
    def __repr__(self) -> str:
        return f"Triple({self._first}, {self._second}, {self._third})"


if __name__ == "__main__":
    print("=== Tuple Example ===")
    t1 = Tuple(1, 2, 3, 4, 5)
    print(f"Tuple: {t1}")
    print(f"Size: {t1.size}")
    print(f"First: {t1.first()}")
    print(f"Last: {t1.last()}")
    print(f"At index 2: {t1[2]}")
    print()
    
    print("=== Tuple Operations ===")
    t2 = Tuple("a", "b", "c")
    print(f"Tuple t2: {t2}")
    print(f"Concat t1 + t2: {t1.concat(t2)}")
    print(f"Slice t1[1:4]: {t1.slice(1, 4)}")
    print(f"Take 3 from t1: {t1.take(3)}")
    print(f"Drop 2 from t1: {t1.drop(2)}")
    print(f"Reverse t1: {t1.reverse()}")
    print(f"Map t1 (x * 2): {t1.map(lambda x: x * 2)}")
    print(f"Filter t1 (even): {t1.filter(lambda x: x % 2 == 0)}")
    print(f"Reduce t1 (sum): {t1.reduce(lambda a, b: a + b, 0)}")
    print()
    
    print("=== Pair Example ===")
    p = Pair(10, "hello")
    print(f"Pair: {p}")
    print(f"First: {p.first}")
    print(f"Second: {p.second}")
    print(f"Swap: {p.swap()}")
    print(f"Map first (+5): {p.map_first(lambda x: x + 5)}")
    print(f"Map second (upper): {p.map_second(lambda x: x.upper())}")
    print()
    
    print("=== Triple Example ===")
    tri = Triple("a", 100, True)
    print(f"Triple: {tri}")
    print(f"First: {tri.first}")
    print(f"Second: {tri.second}")
    print(f"Third: {tri.third}")
    print()
    
    print("=== Zip Example ===")
    t3 = Tuple(1, 2, 3)
    t4 = Tuple("x", "y", "z")
    zipped = t3.zip(t4)
    print(f"Zip {t3} and {t4}: {zipped}")
    print()
    
    print("=== Comparison Example ===")
    t5 = Tuple(1, 2, 3)
    t6 = Tuple(1, 2, 4)
    print(f"{t5} == {t6}: {t5 == t6}")
    print(f"{t5} < {t6}: {t5 < t6}")
