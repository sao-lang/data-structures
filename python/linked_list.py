from typing import TypeVar, Generic, Optional, List

T = TypeVar('T')

class ListNode(Generic[T]):
    def __init__(self, value: T):
        self.value: T = value
        self.next: Optional[ListNode[T]] = None

class SinglyLinkedList(Generic[T]):
    def __init__(self):
        self._head: Optional[ListNode[T]] = None
        self._tail: Optional[ListNode[T]] = None
        self._length: int = 0

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
        return self._tail.value if self._tail else None

    def prepend(self, value: T) -> None:
        new_node = ListNode(value)
        if not self._head:
            self._head = new_node
            self._tail = new_node
        else:
            new_node.next = self._head
            self._head = new_node
        self._length += 1

    def append(self, value: T) -> None:
        new_node = ListNode(value)
        if not self._tail:
            self._head = new_node
            self._tail = new_node
        else:
            self._tail.next = new_node
            self._tail = new_node
        self._length += 1

    def remove_first(self) -> Optional[T]:
        if not self._head:
            return None
        removed_node = self._head
        self._head = self._head.next
        if not self._head:
            self._tail = None
        self._length -= 1
        return removed_node.value

    def clear(self) -> None:
        self._head = None
        self._tail = None
        self._length = 0

    def to_list(self) -> List[T]:
        result: List[T] = []
        current = self._head
        while current:
            result.append(current.value)
            current = current.next
        return result

    def __iter__(self):
        class Iterator:
            def __init__(self, head):
                self.current = head
            
            def __next__(self):
                if self.current is None:
                    raise StopIteration
                value = self.current.value
                self.current = self.current.next
                return value
        
        return Iterator(self._head)

    def __str__(self) -> str:
        return str(self.to_list())

class DoublyListNode(Generic[T]):
    def __init__(self, value: T):
        self.value: T = value
        self.next: Optional[DoublyListNode[T]] = None
        self.prev: Optional[DoublyListNode[T]] = None

class DoublyLinkedList(Generic[T]):
    def __init__(self):
        self._head: Optional[DoublyListNode[T]] = None
        self._tail: Optional[DoublyListNode[T]] = None
        self._length: int = 0

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
        return self._tail.value if self._tail else None

    def prepend(self, value: T) -> None:
        new_node = DoublyListNode(value)
        if not self._head:
            self._head = new_node
            self._tail = new_node
        else:
            new_node.next = self._head
            self._head.prev = new_node
            self._head = new_node
        self._length += 1

    def append(self, value: T) -> None:
        new_node = DoublyListNode(value)
        if not self._tail:
            self._head = new_node
            self._tail = new_node
        else:
            new_node.prev = self._tail
            self._tail.next = new_node
            self._tail = new_node
        self._length += 1

    def remove_first(self) -> Optional[T]:
        if not self._head:
            return None
        removed_node = self._head
        self._head = self._head.next
        if not self._head:
            self._tail = None
        else:
            self._head.prev = None
        self._length -= 1
        return removed_node.value

    def remove_last(self) -> Optional[T]:
        if not self._tail:
            return None
        removed_node = self._tail
        self._tail = self._tail.prev
        if not self._tail:
            self._head = None
        else:
            self._tail.next = None
        self._length -= 1
        return removed_node.value

    def clear(self) -> None:
        self._head = None
        self._tail = None
        self._length = 0

    def to_list(self) -> List[T]:
        result: List[T] = []
        current = self._head
        while current:
            result.append(current.value)
            current = current.next
        return result

    def to_list_reverse(self) -> List[T]:
        result: List[T] = []
        current = self._tail
        while current:
            result.append(current.value)
            current = current.prev
        return result

    def __iter__(self):
        class Iterator:
            def __init__(self, head):
                self.current = head
            
            def __next__(self):
                if self.current is None:
                    raise StopIteration
                value = self.current.value
                self.current = self.current.next
                return value
        
        return Iterator(self._head)

    def __str__(self) -> str:
        return str(self.to_list())

if __name__ == "__main__":
    print("Singly Linked List Example")
    sll = SinglyLinkedList[int]()
    sll.append(1)
    sll.append(2)
    sll.append(3)
    print(f"List: {sll}")
    print("Iterating with for loop:")
    for val in sll:
        print(val, end=' ')
    print()

    print("\nDoubly Linked List Example")
    dll = DoublyLinkedList[int]()
    dll.append(1)
    dll.append(2)
    dll.append(3)
    print(f"List: {dll}")
    print(f"Reverse: {dll.to_list_reverse()}")
    print("Iterating with for loop:")
    for val in dll:
        print(val, end=' ')
    print()
