class HashNode<K, V> {
  final K key;
  V value;
  HashNode<K, V>? next;

  HashNode(this.key, this.value);
}


class Map<K, V> {
  int _capacity;
  int _size = 0;
  List<HashNode<K, V>?> _buckets;
  final double _loadFactor = 0.7;

  Map(this._capacity) : _buckets = List.filled(_capacity, null);

  int get size => _size;
  bool get isEmpty => _size == 0;
  int get capacity => _capacity;

  int _hash(K key) {
    String keyStr = key.toString();
    int hash = 0;
    for (int i = 0; i < keyStr.length; i++) {
      hash = (hash << 5) - hash + keyStr.codeUnitAt(i);
    }
    if (hash < 0) hash = -hash;
    return hash % _capacity;
  }

  void _resize() {
    List<HashNode<K, V>?> oldBuckets = _buckets;
    _capacity *= 2;
    _size = 0;
    _buckets = List.filled(_capacity, null);

    for (HashNode<K, V>? bucket in oldBuckets) {
      HashNode<K, V>? current = bucket;
      while (current != null) {
        set(current.key, current.value);
        current = current.next;
      }
    }
  }

  void set(K key, V value) {
    if (_size / _capacity >= _loadFactor) {
      _resize();
    }

    int index = _hash(key);
    HashNode<K, V>? current = _buckets[index];

    while (current != null) {
      if (current.key == key) {
        current.value = value;
        return;
      }
      current = current.next;
    }

    HashNode<K, V> newNode = HashNode(key, value);
    newNode.next = _buckets[index];
    _buckets[index] = newNode;
    _size++;
  }

  V? get(K key) {
    int index = _hash(key);
    HashNode<K, V>? current = _buckets[index];

    while (current != null) {
      if (current.key == key) {
        return current.value;
      }
      current = current.next;
    }

    return null;
  }

  bool has(K key) {
    int index = _hash(key);
    HashNode<K, V>? current = _buckets[index];

    while (current != null) {
      if (current.key == key) {
        return true;
      }
      current = current.next;
    }

    return false;
  }

  bool delete(K key) {
    int index = _hash(key);
    HashNode<K, V>? current = _buckets[index];
    HashNode<K, V>? prev;

    while (current != null) {
      if (current.key == key) {
        if (prev != null) {
          prev.next = current.next;
        } else {
          _buckets[index] = current.next;
        }
        _size--;
        return true;
      }
      prev = current;
      current = current.next;
    }

    return false;
  }

  List<K> keys() {
    List<K> result = [];
    for (HashNode<K, V>? bucket in _buckets) {
      HashNode<K, V>? current = bucket;
      while (current != null) {
        result.add(current.key);
        current = current.next;
      }
    }
    return result;
  }

  List<V> values() {
    List<V> result = [];
    for (HashNode<K, V>? bucket in _buckets) {
      HashNode<K, V>? current = bucket;
      while (current != null) {
        result.add(current.value);
        current = current.next;
      }
    }
    return result;
  }

  List<List<dynamic>> entries() {
    List<List<dynamic>> result = [];
    for (HashNode<K, V>? bucket in _buckets) {
      HashNode<K, V>? current = bucket;
      while (current != null) {
        result.add([current.key, current.value]);
        current = current.next;
      }
    }
    return result;
  }

  void clear() {
    _buckets = List.filled(_capacity, null);
    _size = 0;
  }

  void update(Map<K, V> other) {
    for (List<dynamic> entry in other.entries()) {
      K key = entry[0] as K;
      V value = entry[1] as V;
      set(key, value);
    }
  }

  V? operator [](K key) => get(key);

  void operator []=(K key, V value) => set(key, value);

  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    if (other is! Map<K, V>) return false;
    if (_size != other._size) return false;
    for (K key in keys()) {
      if (!other.has(key) || get(key) != other.get(key)) {
        return false;
      }
    }
    return true;
  }

  @override
  int get hashCode => Object.hashAll([_size, ...keys()]);

  @override
  String toString() {
    List<String> items = [];
    for (List<dynamic> entry in entries()) {
      items.add('${entry[0]}: ${entry[1]}');
    }
    return '{${items.join(', ')}}';
  }

  // 实现Iterable接口，返回键值对的迭代器
  Iterable<MapEntry<K, V>> get entriesIterable sync* {
    for (HashNode<K, V>? bucket in _buckets) {
      HashNode<K, V>? current = bucket;
      while (current != null) {
        yield MapEntry(current.key, current.value);
        current = current.next;
      }
    }
  }

  // 实现Iterable接口，返回键的迭代器
  Iterable<K> get keysIterable sync* {
    for (HashNode<K, V>? bucket in _buckets) {
      HashNode<K, V>? current = bucket;
      while (current != null) {
        yield current.key;
        current = current.next;
      }
    }
  }

  // 实现Iterable接口，返回值的迭代器
  Iterable<V> get valuesIterable sync* {
    for (HashNode<K, V>? bucket in _buckets) {
      HashNode<K, V>? current = bucket;
      while (current != null) {
        yield current.value;
        current = current.next;
      }
    }
  }

  // 实现forEach方法
  void forEach(void Function(K key, V value) action) {
    for (HashNode<K, V>? bucket in _buckets) {
      HashNode<K, V>? current = bucket;
      while (current != null) {
        action(current.key, current.value);
        current = current.next;
      }
    }
  }
}

// 为了方便使用，定义MapEntry类
class MapEntry<K, V> {
  final K key;
  final V value;

  const MapEntry(this.key, this.value);

  @override
  String toString() => 'MapEntry($key: $value)';
}


class BitMap {
  final int size;
  final List<int> _bits;

  BitMap(this.size) : _bits = List.filled((size + 63) ~/ 64, 0);

  void _checkIndex(int bit) {
    if (bit < 0 || bit >= size) {
      throw RangeError.range(bit, 0, size - 1);
    }
  }

  void set(int bit) {
    _checkIndex(bit);
    int index = bit ~/ 64;
    int mask = 1 << (bit % 64);
    _bits[index] |= mask;
  }

  void clear(int bit) {
    _checkIndex(bit);
    int index = bit ~/ 64;
    int mask = 1 << (bit % 64);
    _bits[index] &= ~mask;
  }

  void toggle(int bit) {
    _checkIndex(bit);
    int index = bit ~/ 64;
    int mask = 1 << (bit % 64);
    _bits[index] ^= mask;
  }

  bool get(int bit) {
    _checkIndex(bit);
    int index = bit ~/ 64;
    int mask = 1 << (bit % 64);
    return (_bits[index] & mask) != 0;
  }

  void setAll() {
    for (int i = 0; i < _bits.length; i++) {
      _bits[i] = 0xFFFFFFFFFFFFFFFF;
    }
  }

  void clearAll() {
    for (int i = 0; i < _bits.length; i++) {
      _bits[i] = 0;
    }
  }

  int countSetBits() {
    int count = 0;
    for (int word in _bits) {
      count += word.toRadixString(2).replaceAll('0', '').length;
    }
    return count;
  }

  int? findFirstSet() {
    for (int i = 0; i < size; i++) {
      if (get(i)) {
        return i;
      }
    }
    return null;
  }

  int? findFirstClear() {
    for (int i = 0; i < size; i++) {
      if (!get(i)) {
        return i;
      }
    }
    return null;
  }

  bool operator [](int bit) => get(bit);

  void operator []=(int bit, bool value) {
    if (value) {
      set(bit);
    } else {
      clear(bit);
    }
  }

  BitMap operator &(BitMap other) {
    if (size != other.size) {
      throw ArgumentError('BitMaps must have the same size');
    }
    BitMap result = BitMap(size);
    for (int i = 0; i < _bits.length; i++) {
      result._bits[i] = _bits[i] & other._bits[i];
    }
    return result;
  }

  BitMap operator |(BitMap other) {
    if (size != other.size) {
      throw ArgumentError('BitMaps must have the same size');
    }
    BitMap result = BitMap(size);
    for (int i = 0; i < _bits.length; i++) {
      result._bits[i] = _bits[i] | other._bits[i];
    }
    return result;
  }

  BitMap operator ^(BitMap other) {
    if (size != other.size) {
      throw ArgumentError('BitMaps must have the same size');
    }
    BitMap result = BitMap(size);
    for (int i = 0; i < _bits.length; i++) {
      result._bits[i] = _bits[i] ^ other._bits[i];
    }
    return result;
  }

  BitMap operator ~() {
    BitMap result = BitMap(size);
    for (int i = 0; i < _bits.length; i++) {
      result._bits[i] = ~_bits[i];
    }
    return result;
  }

  @override
  String toString() {
    StringBuffer sb = StringBuffer();
    for (int i = 0; i < size; i++) {
      sb.write(get(i) ? '1' : '0');
    }
    return sb.toString();
  }

  // 实现Iterable接口，返回位的迭代器
  Iterable<bool> get bitsIterable sync* {
    for (int i = 0; i < size; i++) {
      yield get(i);
    }
  }

  // 实现Iterable接口，返回设置位的索引迭代器
  Iterable<int> get setBitsIterable sync* {
    for (int i = 0; i < size; i++) {
      if (get(i)) {
        yield i;
      }
    }
  }

  // 实现forEach方法
  void forEach(void Function(int index, bool value) action) {
    for (int i = 0; i < size; i++) {
      action(i, get(i));
    }
  }
}


class Set<T> {
  final Map<T, bool> _map;

  Set([int initialCapacity = 16]) : _map = Map<T, bool>(initialCapacity);

  int get size => _map.size;
  bool get isEmpty => _map.isEmpty;

  void add(T item) {
    _map.set(item, true);
  }

  bool remove(T item) {
    return _map.delete(item);
  }

  bool has(T item) {
    return _map.has(item);
  }

  void clear() {
    _map.clear();
  }

  List<T> items() {
    return _map.keys();
  }

  Set<T> union(Set<T> other) {
    Set<T> result = Set<T>(size + other.size);
    for (T item in items()) {
      result.add(item);
    }
    for (T item in other.items()) {
      result.add(item);
    }
    return result;
  }

  Set<T> intersection(Set<T> other) {
    Set<T> result = Set<T>();
    Set<T> smaller = size <= other.size ? this : other;
    Set<T> larger = size <= other.size ? other : this;
    for (T item in smaller.items()) {
      if (larger.has(item)) {
        result.add(item);
      }
    }
    return result;
  }

  Set<T> difference(Set<T> other) {
    Set<T> result = Set<T>();
    for (T item in items()) {
      if (!other.has(item)) {
        result.add(item);
      }
    }
    return result;
  }

  Set<T> symmetricDifference(Set<T> other) {
    Set<T> result = Set<T>();
    for (T item in items()) {
      if (!other.has(item)) {
        result.add(item);
      }
    }
    for (T item in other.items()) {
      if (!has(item)) {
        result.add(item);
      }
    }
    return result;
  }

  bool isSubset(Set<T> other) {
    if (size > other.size) {
      return false;
    }
    for (T item in items()) {
      if (!other.has(item)) {
        return false;
      }
    }
    return true;
  }

  bool isSuperset(Set<T> other) {
    return other.isSubset(this);
  }

  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    if (other is! Set<T>) return false;
    if (size != other.size) return false;
    return isSubset(other);
  }

  @override
  int get hashCode => Object.hashAll([size, ...items()]);

  bool operator <=(Set<T> other) => isSubset(other);

  bool operator >=(Set<T> other) => isSuperset(other);

  Set<T> operator +(Set<T> other) => union(other);

  Set<T> operator -(Set<T> other) => difference(other);

  Set<T> operator &(Set<T> other) => intersection(other);

  Set<T> operator |(Set<T> other) => union(other);

  Set<T> operator ^(Set<T> other) => symmetricDifference(other);

  @override
  String toString() {
    List<String> items = this.items().map((item) => item.toString()).toList();
    return '{${items.join(', ')}}';
  }

  // 提供可遍历的接口
  Iterable<T> get iterable sync* {
    for (T item in items()) {
      yield item;
    }
  }

  // 实现forEach方法
  void forEach(void Function(T item) action) {
    for (T item in items()) {
      action(item);
    }
  }
}


void main() {
  print('=' * 50);
  print('Map 示例:');
  print('=' * 50);
  Map<String, int> map1 = Map<String, int>(16);
  map1['one'] = 1;
  map1['two'] = 2;
  map1['three'] = 3;
  print('Map: $map1');
  print('大小: ${map1.size}');
  print('包含 "two": ${map1.has("two")}');
  print('"two" 的值: ${map1["two"]}');
  print('键: ${map1.keys()}');
  print('值: ${map1.values()}');
  print('');

  // 测试Map的可遍历结构
  print('=== 测试Map的可遍历结构 ===');
  // 使用forEach方法遍历
  print('使用forEach方法遍历Map:');
  map1.forEach((key, value) {
    print('$key: $value');
  });
  
  // 使用entriesIterable遍历
  print('使用entriesIterable遍历Map:');
  for (var entry in map1.entriesIterable) {
    print('${entry.key}: ${entry.value}');
  }
  
  // 使用keysIterable遍历
  print('使用keysIterable遍历Map键:');
  for (var key in map1.keysIterable) {
    print(key);
  }
  
  // 使用valuesIterable遍历
  print('使用valuesIterable遍历Map值:');
  for (var value in map1.valuesIterable) {
    print(value);
  }
  print('');

  print('=' * 50);
  print('BitMap 示例:');
  print('=' * 50);
  BitMap bitmap = BitMap(10);
  bitmap[0] = true;
  bitmap[2] = true;
  bitmap[5] = true;
  print('BitMap: $bitmap');
  print('设置的位数: ${bitmap.countSetBits()}');
  print('第 2 位: ${bitmap[2]}');
  bitmap.toggle(2);
  print('翻转第 2 位后: $bitmap');
  print('');

  // 测试BitMap的可遍历结构
  print('=== 测试BitMap的可遍历结构 ===');
  // 使用forEach方法遍历
  print('使用forEach方法遍历BitMap:');
  bitmap.forEach((index, value) {
    print('$index: $value');
  });
  
  // 使用bitsIterable遍历
  print('使用bitsIterable遍历BitMap:');
  for (var value in bitmap.bitsIterable) {
    print(value);
  }
  
  // 使用setBitsIterable遍历
  print('使用setBitsIterable遍历BitMap设置的位:');
  for (var index in bitmap.setBitsIterable) {
    print(index);
  }
  print('');

  print('=' * 50);
  print('Set 示例:');
  print('=' * 50);
  Set<int> set1 = Set<int>(16);
  set1.add(1);
  set1.add(2);
  set1.add(3);
  Set<int> set2 = Set<int>(16);
  set2.add(3);
  set2.add(4);
  set2.add(5);
  print('Set1: $set1');
  print('Set2: $set2');
  print('并集: ${set1 | set2}');
  print('交集: ${set1 & set2}');
  print('差集 (Set1 - Set2): ${set1 - set2}');
  print('包含 2: ${set1.has(2)}');
  print('');

  // 测试Set的可遍历结构
  print('=== 测试Set的可遍历结构 ===');
  // 使用forEach方法遍历
  print('使用forEach方法遍历Set:');
  set1.forEach((item) {
    print(item);
  });
  
  // 使用iterable遍历
  print('使用iterable遍历Set:');
  for (var item in set1.iterable) {
    print(item);
  }
  
  // 使用iterable遍历（通过for-in循环使用）
  print('使用for-in循环遍历Set:');
  for (var item in set1.iterable) {
    print(item);
  }
}
