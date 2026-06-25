class ListNode<T> {
  T value;
  ListNode<T>? next;

  ListNode(this.value);
}

class SinglyLinkedList<T> {
  ListNode<T>? _head;
  ListNode<T>? _tail;
  int _length = 0;

  int get length => _length;
  bool get isEmpty => _length == 0;

  T? get head {
    if (_head == null) throw StateError('list is empty');
    return _head?.value;
  }

  T? get tail {
    if (_tail == null) throw StateError('list is empty');
    return _tail?.value;
  }

  void prepend(T value) {
    ListNode<T> newNode = ListNode(value);
    if (_head == null) {
      _head = newNode;
      _tail = newNode;
    } else {
      newNode.next = _head;
      _head = newNode;
    }
    _length++;
  }

  void append(T value) {
    ListNode<T> newNode = ListNode(value);
    if (_tail == null) {
      _head = newNode;
      _tail = newNode;
    } else {
      _tail?.next = newNode;
      _tail = newNode;
    }
    _length++;
  }

  T? removeFirst() {
    if (_head == null) throw StateError('list is empty');
    ListNode<T> removedNode = _head!;
    _head = _head?.next;
    if (_head == null) {
      _tail = null;
    }
    _length--;
    return removedNode.value;
  }

  void clear() {
    _head = null;
    _tail = null;
    _length = 0;
  }

  List<T> toSlice() {
    List<T> result = [];
    ListNode<T>? current = _head;
    while (current != null) {
      result.add(current.value);
      current = current.next;
    }
    return result;
  }

  Iterator<T> iter() {
    return toSlice().iterator;
  }
}

class CircularLinkedList<T> {
  ListNode<T>? _tail;
  int _length = 0;

  int get length => _length;
  bool get isEmpty => _length == 0;

  T? get head {
    if (_tail == null) throw StateError('list is empty');
    return _tail?.next?.value;
  }

  T? get tail {
    if (_tail == null) throw StateError('list is empty');
    return _tail?.value;
  }

  void prepend(T value) {
    ListNode<T> newNode = ListNode(value);
    if (_tail == null) {
      newNode.next = newNode;
      _tail = newNode;
    } else {
      newNode.next = _tail?.next;
      _tail?.next = newNode;
    }
    _length++;
  }

  void append(T value) {
    ListNode<T> newNode = ListNode(value);
    if (_tail == null) {
      newNode.next = newNode;
      _tail = newNode;
    } else {
      newNode.next = _tail?.next;
      _tail?.next = newNode;
      _tail = newNode;
    }
    _length++;
  }

  T? removeFirst() {
    if (_tail == null) throw StateError('list is empty');
    ListNode<T> removedNode = _tail!.next!;
    if (_tail == removedNode) {
      _tail = null;
    } else {
      _tail?.next = removedNode.next;
    }
    _length--;
    return removedNode.value;
  }

  void clear() {
    _tail = null;
    _length = 0;
  }

  List<T> toSlice() {
    List<T> result = [];
    if (_tail == null) return result;
    ListNode<T>? current = _tail?.next;
    for (int i = 0; i < _length; i++) {
      result.add(current!.value);
      current = current.next;
    }
    return result;
  }

  Iterator<T> iter() {
    return toSlice().iterator;
  }
}

class DoublyListNode<T> {
  T value;
  DoublyListNode<T>? next;
  DoublyListNode<T>? prev;

  DoublyListNode(this.value);
}

class DoublyLinkedList<T> {
  DoublyListNode<T>? _head;
  DoublyListNode<T>? _tail;
  int _length = 0;

  int get length => _length;
  bool get isEmpty => _length == 0;

  T? get head {
    if (_head == null) throw StateError('list is empty');
    return _head?.value;
  }

  T? get tail {
    if (_tail == null) throw StateError('list is empty');
    return _tail?.value;
  }

  void prepend(T value) {
    DoublyListNode<T> newNode = DoublyListNode(value);
    if (_head == null) {
      _head = newNode;
      _tail = newNode;
    } else {
      newNode.next = _head;
      _head?.prev = newNode;
      _head = newNode;
    }
    _length++;
  }

  void append(T value) {
    DoublyListNode<T> newNode = DoublyListNode(value);
    if (_tail == null) {
      _head = newNode;
      _tail = newNode;
    } else {
      newNode.prev = _tail;
      _tail?.next = newNode;
      _tail = newNode;
    }
    _length++;
  }

  T? removeFirst() {
    if (_head == null) throw StateError('list is empty');
    DoublyListNode<T> removedNode = _head!;
    _head = _head?.next;
    if (_head == null) {
      _tail = null;
    } else {
      _head?.prev = null;
    }
    _length--;
    return removedNode.value;
  }

  T? removeLast() {
    if (_tail == null) throw StateError('list is empty');
    DoublyListNode<T> removedNode = _tail!;
    _tail = _tail?.prev;
    if (_tail == null) {
      _head = null;
    } else {
      _tail?.next = null;
    }
    _length--;
    return removedNode.value;
  }

  void clear() {
    _head = null;
    _tail = null;
    _length = 0;
  }

  List<T> toSlice() {
    List<T> result = [];
    DoublyListNode<T>? current = _head;
    while (current != null) {
      result.add(current.value);
      current = current.next;
    }
    return result;
  }

  List<T> toSliceReverse() {
    List<T> result = [];
    DoublyListNode<T>? current = _tail;
    while (current != null) {
      result.add(current.value);
      current = current.prev;
    }
    return result;
  }

  Iterator<T> iter() {
    return toSlice().iterator;
  }
}

class DoublyCircularLinkedList<T> {
  DoublyListNode<T>? _tail;
  int _length = 0;

  int get length => _length;
  bool get isEmpty => _length == 0;

  T? get head {
    if (_tail == null) throw StateError('list is empty');
    return _tail?.next?.value;
  }

  T? get tail {
    if (_tail == null) throw StateError('list is empty');
    return _tail?.value;
  }

  void prepend(T value) {
    DoublyListNode<T> newNode = DoublyListNode(value);
    if (_tail == null) {
      newNode.next = newNode;
      newNode.prev = newNode;
      _tail = newNode;
    } else {
      DoublyListNode<T>? tailNode = _tail;
      newNode.next = tailNode?.next;
      newNode.prev = tailNode;
      tailNode?.next?.prev = newNode;
      tailNode?.next = newNode;
      _tail?.next = newNode;
      _tail?.next?.prev = _tail;
    }
    _length++;
  }

  void append(T value) {
    DoublyListNode<T> newNode = DoublyListNode(value);
    if (_tail == null) {
      newNode.next = newNode;
      newNode.prev = newNode;
      _tail = newNode;
    } else {
      DoublyListNode<T>? tailNode = _tail;
      newNode.next = tailNode?.next;
      newNode.prev = tailNode;
      tailNode?.next?.prev = newNode;
      tailNode?.next = newNode;
      _tail = newNode;
    }
    _length++;
  }

  T? removeFirst() {
    if (_tail == null) throw StateError('list is empty');
    DoublyListNode<T> removedNode = _tail!.next!;
    if (_tail == removedNode) {
      _tail = null;
    } else {
      _tail?.next = removedNode.next;
      removedNode.next?.prev = _tail;
    }
    _length--;
    return removedNode.value;
  }

  T? removeLast() {
    if (_tail == null) throw StateError('list is empty');
    DoublyListNode<T> removedNode = _tail!;
    if (_tail == removedNode.next) {
      _tail = null;
    } else {
      _tail = removedNode.prev;
      _tail?.next = removedNode.next;
      removedNode.next?.prev = _tail;
    }
    _length--;
    return removedNode.value;
  }

  void clear() {
    _tail = null;
    _length = 0;
  }

  List<T> toSlice() {
    List<T> result = [];
    if (_tail == null) return result;
    DoublyListNode<T>? current = _tail?.next;
    for (int i = 0; i < _length; i++) {
      result.add(current!.value);
      current = current.next;
    }
    return result;
  }

  List<T> toSliceReverse() {
    List<T> result = [];
    if (_tail == null) return result;
    DoublyListNode<T>? current = _tail;
    for (int i = 0; i < _length; i++) {
      result.add(current!.value);
      current = current.prev;
    }
    return result;
  }

  Iterator<T> iter() {
    return toSlice().iterator;
  }
}
