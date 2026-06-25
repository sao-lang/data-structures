class MinHeap<T extends Comparable> {
  final List<T> _heap = [];

  int get size => _heap.length;
  bool get isEmpty => _heap.isEmpty;

  int _getParentIndex(int index) => (index - 1) ~/ 2;
  int _getLeftChildIndex(int index) => 2 * index + 1;
  int _getRightChildIndex(int index) => 2 * index + 2;
  bool _hasParent(int index) => _getParentIndex(index) >= 0;
  bool _hasLeftChild(int index) => _getLeftChildIndex(index) < _heap.length;
  bool _hasRightChild(int index) => _getRightChildIndex(index) < _heap.length;
  T _parent(int index) => _heap[_getParentIndex(index)];
  T _leftChild(int index) => _heap[_getLeftChildIndex(index)];
  T _rightChild(int index) => _heap[_getRightChildIndex(index)];

  void _swap(int indexOne, int indexTwo) {
    T temp = _heap[indexOne];
    _heap[indexOne] = _heap[indexTwo];
    _heap[indexTwo] = temp;
  }

  void _heapifyUp() {
    int index = _heap.length - 1;
    while (_hasParent(index) && _parent(index).compareTo(_heap[index]) > 0) {
      _swap(_getParentIndex(index), index);
      index = _getParentIndex(index);
    }
  }

  void _heapifyDown() {
    int index = 0;
    while (_hasLeftChild(index)) {
      int smallerChildIndex = _getLeftChildIndex(index);
      if (_hasRightChild(index) && _rightChild(index).compareTo(_leftChild(index)) < 0) {
        smallerChildIndex = _getRightChildIndex(index);
      }

      if (_heap[index].compareTo(_heap[smallerChildIndex]) < 0) {
        break;
      } else {
        _swap(index, smallerChildIndex);
      }
      index = smallerChildIndex;
    }
  }

  T peek() {
    if (isEmpty) throw StateError('heap is empty');
    return _heap[0];
  }

  T poll() {
    if (isEmpty) throw StateError('heap is empty');
    T item = _heap[0];
    int lastIndex = _heap.length - 1;
    T lastItem = _heap[lastIndex];
    _heap.removeLast();
    if (_heap.isNotEmpty) {
      _heap[0] = lastItem;
      _heapifyDown();
    }
    return item;
  }

  void add(T item) {
    _heap.add(item);
    _heapifyUp();
  }

  void insert(T item) {
    add(item);
  }

  List<T> toSlice() {
    return List.from(_heap);
  }

  void clear() {
    _heap.clear();
  }
}

class MaxHeap<T extends Comparable> {
  final List<T> _heap = [];

  int get size => _heap.length;
  bool get isEmpty => _heap.isEmpty;

  int _getParentIndex(int index) => (index - 1) ~/ 2;
  int _getLeftChildIndex(int index) => 2 * index + 1;
  int _getRightChildIndex(int index) => 2 * index + 2;
  bool _hasParent(int index) => _getParentIndex(index) >= 0;
  bool _hasLeftChild(int index) => _getLeftChildIndex(index) < _heap.length;
  bool _hasRightChild(int index) => _getRightChildIndex(index) < _heap.length;
  T _parent(int index) => _heap[_getParentIndex(index)];
  T _leftChild(int index) => _heap[_getLeftChildIndex(index)];
  T _rightChild(int index) => _heap[_getRightChildIndex(index)];

  void _swap(int indexOne, int indexTwo) {
    T temp = _heap[indexOne];
    _heap[indexOne] = _heap[indexTwo];
    _heap[indexTwo] = temp;
  }

  void _heapifyUp() {
    int index = _heap.length - 1;
    while (_hasParent(index) && _parent(index).compareTo(_heap[index]) < 0) {
      _swap(_getParentIndex(index), index);
      index = _getParentIndex(index);
    }
  }

  void _heapifyDown() {
    int index = 0;
    while (_hasLeftChild(index)) {
      int largerChildIndex = _getLeftChildIndex(index);
      if (_hasRightChild(index) && _rightChild(index).compareTo(_leftChild(index)) > 0) {
        largerChildIndex = _getRightChildIndex(index);
      }

      if (_heap[index].compareTo(_heap[largerChildIndex]) > 0) {
        break;
      } else {
        _swap(index, largerChildIndex);
      }
      index = largerChildIndex;
    }
  }

  T peek() {
    if (isEmpty) throw StateError('heap is empty');
    return _heap[0];
  }

  T poll() {
    if (isEmpty) throw StateError('heap is empty');
    T item = _heap[0];
    int lastIndex = _heap.length - 1;
    T lastItem = _heap[lastIndex];
    _heap.removeLast();
    if (_heap.isNotEmpty) {
      _heap[0] = lastItem;
      _heapifyDown();
    }
    return item;
  }

  void add(T item) {
    _heap.add(item);
    _heapifyUp();
  }

  void insert(T item) {
    add(item);
  }

  List<T> toSlice() {
    return List.from(_heap);
  }

  void clear() {
    _heap.clear();
  }
}
