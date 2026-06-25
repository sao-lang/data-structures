from typing import TypeVar, Generic, List, Optional

T = TypeVar('T', int, str)

class MinHeap(Generic[T]):
    def __init__(self):
        self._heap: List[T] = []

    @property
    def size(self) -> int:
        return len(self._heap)

    @property
    def is_empty(self) -> bool:
        return len(self._heap) == 0

    def _get_parent_index(self, index: int) -> int:
        return (index - 1) // 2

    def _get_left_child_index(self, index: int) -> int:
        return 2 * index + 1

    def _get_right_child_index(self, index: int) -> int:
        return 2 * index + 2

    def _has_parent(self, index: int) -> bool:
        return self._get_parent_index(index) >= 0

    def _has_left_child(self, index: int) -> bool:
        return self._get_left_child_index(index) < len(self._heap)

    def _has_right_child(self, index: int) -> bool:
        return self._get_right_child_index(index) < len(self._heap)

    def _parent(self, index: int) -> T:
        return self._heap[self._get_parent_index(index)]

    def _left_child(self, index: int) -> T:
        return self._heap[self._get_left_child_index(index)]

    def _right_child(self, index: int) -> T:
        return self._heap[self._get_right_child_index(index)]

    def _swap(self, index_one: int, index_two: int) -> None:
        self._heap[index_one], self._heap[index_two] = self._heap[index_two], self._heap[index_one]

    def _heapify_up(self) -> None:
        index = len(self._heap) - 1
        while self._has_parent(index) and self._parent(index) > self._heap[index]:
            self._swap(self._get_parent_index(index), index)
            index = self._get_parent_index(index)

    def _heapify_down(self) -> None:
        index = 0
        while self._has_left_child(index):
            smaller_child_index = self._get_left_child_index(index)
            if self._has_right_child(index) and self._right_child(index) < self._left_child(index):
                smaller_child_index = self._get_right_child_index(index)

            if self._heap[index] < self._heap[smaller_child_index]:
                break
            else:
                self._swap(index, smaller_child_index)
            index = smaller_child_index

    def peek(self) -> Optional[T]:
        if self.is_empty:
            return None
        return self._heap[0]

    def poll(self) -> Optional[T]:
        if self.is_empty:
            return None
        item = self._heap[0]
        last_item = self._heap.pop()
        if self._heap:
            self._heap[0] = last_item
            self._heapify_down()
        return item

    def add(self, item: T) -> None:
        self._heap.append(item)
        self._heapify_up()

    def to_list(self) -> List[T]:
        return list(self._heap)

    def clear(self) -> None:
        self._heap = []

    def __str__(self) -> str:
        return str(self._heap)

class MaxHeap(Generic[T]):
    def __init__(self):
        self._heap: List[T] = []

    @property
    def size(self) -> int:
        return len(self._heap)

    @property
    def is_empty(self) -> bool:
        return len(self._heap) == 0

    def _get_parent_index(self, index: int) -> int:
        return (index - 1) // 2

    def _get_left_child_index(self, index: int) -> int:
        return 2 * index + 1

    def _get_right_child_index(self, index: int) -> int:
        return 2 * index + 2

    def _has_parent(self, index: int) -> bool:
        return self._get_parent_index(index) >= 0

    def _has_left_child(self, index: int) -> bool:
        return self._get_left_child_index(index) < len(self._heap)

    def _has_right_child(self, index: int) -> bool:
        return self._get_right_child_index(index) < len(self._heap)

    def _parent(self, index: int) -> T:
        return self._heap[self._get_parent_index(index)]

    def _left_child(self, index: int) -> T:
        return self._heap[self._get_left_child_index(index)]

    def _right_child(self, index: int) -> T:
        return self._heap[self._get_right_child_index(index)]

    def _swap(self, index_one: int, index_two: int) -> None:
        self._heap[index_one], self._heap[index_two] = self._heap[index_two], self._heap[index_one]

    def _heapify_up(self) -> None:
        index = len(self._heap) - 1
        while self._has_parent(index) and self._parent(index) < self._heap[index]:
            self._swap(self._get_parent_index(index), index)
            index = self._get_parent_index(index)

    def _heapify_down(self) -> None:
        index = 0
        while self._has_left_child(index):
            larger_child_index = self._get_left_child_index(index)
            if self._has_right_child(index) and self._right_child(index) > self._left_child(index):
                larger_child_index = self._get_right_child_index(index)

            if self._heap[index] > self._heap[larger_child_index]:
                break
            else:
                self._swap(index, larger_child_index)
            index = larger_child_index

    def peek(self) -> Optional[T]:
        if self.is_empty:
            return None
        return self._heap[0]

    def poll(self) -> Optional[T]:
        if self.is_empty:
            return None
        item = self._heap[0]
        last_item = self._heap.pop()
        if self._heap:
            self._heap[0] = last_item
            self._heapify_down()
        return item

    def add(self, item: T) -> None:
        self._heap.append(item)
        self._heapify_up()

    def to_list(self) -> List[T]:
        return list(self._heap)

    def clear(self) -> None:
        self._heap = []

    def __str__(self) -> str:
        return str(self._heap)

if __name__ == "__main__":
    print("Min Heap Example:")
    min_heap = MinHeap[int]()
    min_heap.add(5)
    min_heap.add(3)
    min_heap.add(7)
    min_heap.add(1)
    print(f"Min Heap: {min_heap}")
    print(f"Peek: {min_heap.peek()}")
    print(f"Poll: {min_heap.poll()}")
    print(f"Min Heap after poll: {min_heap}")

    print("\nMax Heap Example:")
    max_heap = MaxHeap[int]()
    max_heap.add(5)
    max_heap.add(3)
    max_heap.add(7)
    max_heap.add(1)
    print(f"Max Heap: {max_heap}")
    print(f"Peek: {max_heap.peek()}")
    print(f"Poll: {max_heap.poll()}")
    print(f"Max Heap after poll: {max_heap}")
