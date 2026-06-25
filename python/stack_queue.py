from typing import TypeVar, Generic, Optional, List
from array import DynamicArray
from linked_list import DoublyLinkedList

T = TypeVar('T')

class Stack(Generic[T]):
    def __init__(self):
        self._items: DynamicArray[T] = DynamicArray[T]()

    @property
    def size(self) -> int:
        return self._items.length

    @property
    def is_empty(self) -> bool:
        return self._items.is_empty

    def push(self, item: T) -> None:
        self._items.push(item)

    def pop(self) -> Optional[T]:
        return self._items.pop()

    def peek(self) -> Optional[T]:
        if self.is_empty:
            return None
        return self._items.at(self._items.length - 1)

    def clear(self) -> None:
        self._items.clear()

    def to_list(self) -> List[Optional[T]]:
        return self._items.to_list()

    def __iter__(self):
        return iter(self._items)

    def __str__(self) -> str:
        return str(self.to_list())

class Queue(Generic[T]):
    def __init__(self):
        self._items: DoublyLinkedList[T] = DoublyLinkedList[T]()

    @property
    def size(self) -> int:
        return self._items.length

    @property
    def is_empty(self) -> bool:
        return self._items.is_empty

    def enqueue(self, item: T) -> None:
        self._items.append(item)

    def dequeue(self) -> Optional[T]:
        if self.is_empty:
            return None
        return self._items.remove_first()

    def peek(self) -> Optional[T]:
        return self._items.head

    def clear(self) -> None:
        self._items.clear()

    def to_list(self) -> List[T]:
        return self._items.to_list()

    def __iter__(self):
        return iter(self._items)

    def __str__(self) -> str:
        return str(self.to_list())

class CircularQueue(Generic[T]):
    def __init__(self, capacity: int):
        if capacity <= 0:
            raise ValueError("Capacity must be positive")
        self._capacity: int = capacity
        self._items: List[Optional[T]] = [None] * capacity
        self._front: int = 0
        self._rear: int = -1
        self._size: int = 0

    @property
    def capacity(self) -> int:
        return self._capacity

    @property
    def size(self) -> int:
        return self._size

    @property
    def is_empty(self) -> bool:
        return self._size == 0

    @property
    def is_full(self) -> bool:
        return self._size == self._capacity

    def enqueue(self, item: T) -> bool:
        if self.is_full:
            return False
        self._rear = (self._rear + 1) % self._capacity
        self._items[self._rear] = item
        self._size += 1
        return True

    def dequeue(self) -> Optional[T]:
        if self.is_empty:
            return None
        item = self._items[self._front]
        self._items[self._front] = None
        self._front = (self._front + 1) % self._capacity
        self._size -= 1
        return item

    def peek(self) -> Optional[T]:
        if self.is_empty:
            return None
        return self._items[self._front]

    def clear(self) -> None:
        self._items = [None] * self._capacity
        self._front = 0
        self._rear = -1
        self._size = 0

    def to_list(self) -> List[Optional[T]]:
        result: List[Optional[T]] = []
        for i in range(self._size):
            index = (self._front + i) % self._capacity
            result.append(self._items[index])
        return result

    def __iter__(self):
        for i in range(self._size):
            index = (self._front + i) % self._capacity
            yield self._items[index]

    def __str__(self) -> str:
        return str(self.to_list())

class Deque(Generic[T]):
    def __init__(self):
        self._items: DoublyLinkedList[T] = DoublyLinkedList[T]()

    @property
    def size(self) -> int:
        return self._items.length

    @property
    def is_empty(self) -> bool:
        return self._items.is_empty

    def add_first(self, item: T) -> None:
        self._items.prepend(item)

    def add_last(self, item: T) -> None:
        self._items.append(item)

    def remove_first(self) -> Optional[T]:
        if self.is_empty:
            return None
        return self._items.remove_first()

    def remove_last(self) -> Optional[T]:
        if self.is_empty:
            return None
        return self._items.remove_last()

    def peek_first(self) -> Optional[T]:
        return self._items.head

    def peek_last(self) -> Optional[T]:
        return self._items.tail

    def clear(self) -> None:
        self._items.clear()

    def to_list(self) -> List[T]:
        return self._items.to_list()

    def to_list_reverse(self) -> List[T]:
        return self._items.to_list_reverse()

    def __iter__(self):
        return iter(self._items)

    def __str__(self) -> str:
        return str(self.to_list())

if __name__ == "__main__":
    print("Stack Example:")
    stack = Stack[int]()
    stack.push(1)
    stack.push(2)
    stack.push(3)
    print(f"Stack: {stack}")
    print("Iterating with for loop:")
    for val in stack:
        print(val, end=' ')
    print()
    popped = stack.pop()
    print(f"Popped: {popped}")
    print(f"Stack after pop: {stack}")

    print("\nQueue Example:")
    queue = Queue[int]()
    queue.enqueue(10)
    queue.enqueue(20)
    queue.enqueue(30)
    print(f"Queue: {queue}")
    print("Iterating with for loop:")
    for val in queue:
        print(val, end=' ')
    print()
    dequeued = queue.dequeue()
    print(f"Dequeued: {dequeued}")
    print(f"Queue after dequeue: {queue}")

    print("\nCircular Queue Example:")
    cq = CircularQueue[int](3)
    cq.enqueue(100)
    cq.enqueue(200)
    cq.enqueue(300)
    print(f"Circular Queue: {cq}")
    print(f"Is Full: {cq.is_full}")
    print("Iterating with for loop:")
    for val in cq:
        print(val, end=' ')
    print()

    print("\nDeque Example:")
    dq = Deque[int]()
    dq.add_last(1)
    dq.add_last(2)
    dq.add_last(3)
    print(f"Deque: {dq}")
    print("Iterating with for loop:")
    for val in dq:
        print(val, end=' ')
    print()
    dq.add_first(0)
    print(f"Deque after add_first(0): {dq}")
    removed_first = dq.remove_first()
    print(f"Removed first: {removed_first}")
    removed_last = dq.remove_last()
    print(f"Removed last: {removed_last}")
    print(f"Deque after removes: {dq}")
