class HashNode<K, V> {
  final K key;
  V value;
  HashNode<K, V>? next;

  HashNode(this.key, this.value);
}

class HashTable<K, V> {
  int _capacity;
  int _size = 0;
  List<HashNode<K, V>?> _buckets;
  final double _loadFactor = 0.7;

  HashTable(this._capacity) : _buckets = List.filled(_capacity, null);

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

    throw ArgumentError('key not found');
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

  void forEach(void Function(V value, K key, HashTable<K, V> ht) callback) {
    for (HashNode<K, V>? bucket in _buckets) {
      HashNode<K, V>? current = bucket;
      while (current != null) {
        callback(current.value, current.key, this);
        current = current.next;
      }
    }
  }
}
