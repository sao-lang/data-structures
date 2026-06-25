class FixedArray<T> {
  final int capacity;
  int _length = 0;
  final List<T?> _data;

  FixedArray(this.capacity) : _data = List<T?>.filled(capacity, null);

  int get length => _length;
  bool get isEmpty => _length == 0;
  bool get isFull => _length == capacity;

  T? at(int index) {
    if (index < 0 || index >= _length) {
      throw ArgumentError('index out of bounds');
    }
    return _data[index];
  }

  void set(int index, T value) {
    if (index < 0 || index >= _length) {
      throw ArgumentError('index out of bounds');
    }
    _data[index] = value;
  }

  void push(T value) {
    if (isFull) {
      throw StateError('array is full');
    }
    _data[_length] = value;
    _length++;
  }

  T? pop() {
    if (isEmpty) {
      throw StateError('array is empty');
    }
    _length--;
    T? value = _data[_length];
    _data[_length] = null;
    return value;
  }

  void insert(int index, T value) {
    if (isFull) {
      throw StateError('array is full');
    }
    if (index < 0 || index > _length) {
      throw ArgumentError('index out of bounds');
    }
    for (int i = _length; i > index; i--) {
      _data[i] = _data[i - 1];
    }
    _data[index] = value;
    _length++;
  }

  T? remove(int index) {
    if (isEmpty) {
      throw StateError('array is empty');
    }
    if (index < 0 || index >= _length) {
      throw ArgumentError('index out of bounds');
    }
    T? value = _data[index];
    for (int i = index; i < _length - 1; i++) {
      _data[i] = _data[i + 1];
    }
    _length--;
    _data[_length] = null;
    return value;
  }

  int find(T value, bool Function(T a, T b) equal) {
    for (int i = 0; i < _length; i++) {
      if (equal(_data[i] as T, value)) {
        return i;
      }
    }
    return -1;
  }

  List<T> toArray() {
    return List<T>.from(_data.sublist(0, _length));
  }

  void clear() {
    for (int i = 0; i < capacity; i++) {
      _data[i] = null;
    }
    _length = 0;
  }

  Iterator<T> iter() {
    return toArray().iterator;
  }
}

class DynamicArray<T> {
  int capacity;
  int _length = 0;
  List<T?> _data;
  final int growthFactor;

  DynamicArray(this.capacity, [this.growthFactor = 2])
    : _data = List<T?>.filled(capacity, null);

  int get length => _length;
  bool get isEmpty => _length == 0;

  void _resize() {
    int newCapacity = capacity * growthFactor;
    List<T?> newData = List<T?>.filled(newCapacity, null);
    for (int i = 0; i < _length; i++) {
      newData[i] = _data[i];
    }
    _data = newData;
    capacity = newCapacity;
  }

  T? at(int index) {
    if (index < 0 || index >= _length) {
      throw ArgumentError('index out of bounds');
    }
    return _data[index];
  }

  void set(int index, T value) {
    if (index < 0 || index >= _length) {
      throw ArgumentError('index out of bounds');
    }
    _data[index] = value;
  }

  void push(T value) {
    if (_length >= capacity) {
      _resize();
    }
    _data[_length] = value;
    _length++;
  }

  T? pop() {
    if (isEmpty) {
      throw StateError('array is empty');
    }
    _length--;
    T? value = _data[_length];
    _data[_length] = null;
    return value;
  }

  void insert(int index, T value) {
    if (index < 0 || index > _length) {
      throw ArgumentError('index out of bounds');
    }
    if (_length >= capacity) {
      _resize();
    }
    for (int i = _length; i > index; i--) {
      _data[i] = _data[i - 1];
    }
    _data[index] = value;
    _length++;
  }

  T? remove(int index) {
    if (isEmpty) {
      throw StateError('array is empty');
    }
    if (index < 0 || index >= _length) {
      throw ArgumentError('index out of bounds');
    }
    T? value = _data[index];
    for (int i = index; i < _length - 1; i++) {
      _data[i] = _data[i + 1];
    }
    _length--;
    _data[_length] = null;
    return value;
  }

  int find(T value, bool Function(T a, T b) equal) {
    for (int i = 0; i < _length; i++) {
      if (equal(_data[i] as T, value)) {
        return i;
      }
    }
    return -1;
  }

  List<T> toArray() {
    return List<T>.from(_data.sublist(0, _length));
  }

  void clear() {
    _data = List<T?>.filled(10, null);
    capacity = 10;
    _length = 0;
  }

  void sort(int Function(T a, T b) compare) {
    List<T> arr = toArray();
    arr.sort(compare);
    for (int i = 0; i < _length; i++) {
      _data[i] = arr[i];
    }
  }

  Iterator<T> iter() {
    return toArray().iterator;
  }
}
