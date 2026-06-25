class Stack<T> {
  final List<T> _items = [];

  int get size => _items.length;
  bool get isEmpty => _items.isEmpty;

  void push(T item) {
    _items.add(item);
  }

  T pop() {
    if (isEmpty) throw StateError('stack is empty');
    return _items.removeLast();
  }

  T peek() {
    if (isEmpty) throw StateError('stack is empty');
    return _items.last;
  }

  void clear() {
    _items.clear();
  }

  List<T> toSlice() {
    return List.from(_items);
  }

  Iterator<T> iter() {
    return _items.iterator;
  }
}

class Queue<T> {
  final List<T> _items = [];

  int get size => _items.length;
  bool get isEmpty => _items.isEmpty;

  void enqueue(T item) {
    _items.add(item);
  }

  T dequeue() {
    if (isEmpty) throw StateError('queue is empty');
    return _items.removeAt(0);
  }

  T peek() {
    if (isEmpty) throw StateError('queue is empty');
    return _items.first;
  }

  void clear() {
    _items.clear();
  }

  List<T> toSlice() {
    return List.from(_items);
  }

  Iterator<T> iter() {
    return _items.iterator;
  }
}

class CircularQueue<T> {
  final int capacity;
  final List<T?> _items;
  int _front = 0;
  int _rear = -1;
  int _size = 0;

  CircularQueue(this.capacity) : _items = List.filled(capacity, null);

  int get size => _size;
  bool get isEmpty => _size == 0;
  bool get isFull => _size == capacity;

  bool enqueue(T item) {
    if (isFull) return false;
    _rear = (_rear + 1) % capacity;
    _items[_rear] = item;
    _size++;
    return true;
  }

  T dequeue() {
    if (isEmpty) throw StateError('queue is empty');
    T? item = _items[_front];
    _items[_front] = null;
    _front = (_front + 1) % capacity;
    _size--;
    return item as T;
  }

  T peek() {
    if (isEmpty) throw StateError('queue is empty');
    return _items[_front] as T;
  }

  void clear() {
    for (int i = 0; i < capacity; i++) {
      _items[i] = null;
    }
    _front = 0;
    _rear = -1;
    _size = 0;
  }

  List<T> toSlice() {
    List<T> result = [];
    for (int i = 0; i < _size; i++) {
      int index = (_front + i) % capacity;
      result.add(_items[index] as T);
    }
    return result;
  }

  Iterator<T> iter() {
    return toSlice().iterator;
  }
}

class Deque<T> {
  final List<T> _items = [];

  int get size => _items.length;
  bool get isEmpty => _items.isEmpty;

  void addFront(T item) {
    _items.insert(0, item);
  }

  void addRear(T item) {
    _items.add(item);
  }

  T removeFront() {
    if (isEmpty) throw StateError('deque is empty');
    return _items.removeAt(0);
  }

  T removeRear() {
    if (isEmpty) throw StateError('deque is empty');
    return _items.removeLast();
  }

  T peekFront() {
    if (isEmpty) throw StateError('deque is empty');
    return _items.first;
  }

  T peekRear() {
    if (isEmpty) throw StateError('deque is empty');
    return _items.last;
  }

  void clear() {
    _items.clear();
  }

  List<T> toSlice() {
    return List.from(_items);
  }

  Iterator<T> iter() {
    return _items.iterator;
  }
}
