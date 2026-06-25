class Tuple<T> {
  final List<T> _items;

  Tuple(List<T> items) : _items = List.unmodifiable(items);

  int get size => _items.length;
  bool get isEmpty => _items.isEmpty;

  T at(int index) {
    if (index < 0 || index >= _items.length) {
      throw RangeError('index out of bounds');
    }
    return _items[index];
  }

  T first() {
    if (isEmpty) throw StateError('tuple is empty');
    return _items.first;
  }

  T last() {
    if (isEmpty) throw StateError('tuple is empty');
    return _items.last;
  }

  List<T> toSlice() => List.from(_items);

  Tuple<T> map(T Function(T) f) {
    return Tuple(_items.map(f).toList());
  }

  Tuple<T> filter(bool Function(T) f) {
    return Tuple(_items.where(f).toList());
  }

  T reduce(T Function(T, T) f, T initial) {
    return _items.fold(initial, f);
  }

  Tuple<T> concat(Tuple<T> other) {
    return Tuple([..._items, ...other._items]);
  }

  Tuple<T> slice(int start, int end) {
    if (start < 0) start = 0;
    if (end > _items.length) end = _items.length;
    if (start > end) return Tuple([]);
    return Tuple(_items.sublist(start, end));
  }

  Tuple<T> take(int n) {
    if (n <= 0) return Tuple([]);
    if (n > _items.length) n = _items.length;
    return Tuple(_items.sublist(0, n));
  }

  Tuple<T> drop(int n) {
    if (n <= 0) return Tuple(List.from(_items));
    if (n >= _items.length) return Tuple([]);
    return Tuple(_items.sublist(n));
  }

  bool contains(T item, bool Function(T, T) equal) {
    return _items.any((i) => equal(i, item));
  }

  int indexOf(T item, bool Function(T, T) equal) {
    for (int i = 0; i < _items.length; i++) {
      if (equal(_items[i], item)) {
        return i;
      }
    }
    return -1;
  }

  int count(T item, bool Function(T, T) equal) {
    int count = 0;
    for (var i in _items) {
      if (equal(i, item)) {
        count++;
      }
    }
    return count;
  }

  Tuple<T> reverse() {
    return Tuple(_items.reversed.toList());
  }

  @override
  String toString() => 'Tuple$_items';
}

class Pair<T, U> {
  final T first;
  final U second;

  Pair(this.first, this.second);

  Pair<U, T> swap() {
    return Pair(second, first);
  }

  Tuple<dynamic> toTuple() {
    return Tuple([first, second]);
  }

  Pair<T, U> mapFirst(T Function(T) f) {
    return Pair(f(first), second);
  }

  Pair<T, U> mapSecond(U Function(U) f) {
    return Pair(first, f(second));
  }

  Pair<T, U> mapBoth(T Function(T) f1, U Function(U) f2) {
    return Pair(f1(first), f2(second));
  }

  @override
  String toString() => 'Pair($first, $second)';

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Pair &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second;

  @override
  int get hashCode => first.hashCode ^ second.hashCode;
}

class Triple<T, U, V> {
  final T first;
  final U second;
  final V third;

  Triple(this.first, this.second, this.third);

  Tuple<dynamic> toTuple() {
    return Tuple([first, second, third]);
  }

  Triple<T, U, V> mapFirst(T Function(T) f) {
    return Triple(f(first), second, third);
  }

  Triple<T, U, V> mapSecond(U Function(U) f) {
    return Triple(first, f(second), third);
  }

  Triple<T, U, V> mapThird(V Function(V) f) {
    return Triple(first, second, f(third));
  }

  @override
  String toString() => 'Triple($first, $second, $third)';

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Triple &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second &&
          third == other.third;

  @override
  int get hashCode => first.hashCode ^ second.hashCode ^ third.hashCode;
}

Tuple<Pair<T, U>> zip<T, U>(Tuple<T> t1, Tuple<U> t2) {
  int minLen = t1.size < t2.size ? t1.size : t2.size;
  List<Pair<T, U>> result = [];
  for (int i = 0; i < minLen; i++) {
    result.add(Pair(t1.at(i), t2.at(i)));
  }
  return Tuple(result);
}

class Tuple2<T1, T2> {
  final T1 first;
  final T2 second;

  Tuple2(this.first, this.second);

  @override
  String toString() {
    return '($first, $second)';
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Tuple2 &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second;

  @override
  int get hashCode => first.hashCode ^ second.hashCode;
}

class Tuple3<T1, T2, T3> {
  final T1 first;
  final T2 second;
  final T3 third;

  Tuple3(this.first, this.second, this.third);

  @override
  String toString() {
    return '($first, $second, $third)';
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Tuple3 &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second &&
          third == other.third;

  @override
  int get hashCode => first.hashCode ^ second.hashCode ^ third.hashCode;
}

class Tuple4<T1, T2, T3, T4> {
  final T1 first;
  final T2 second;
  final T3 third;
  final T4 fourth;

  Tuple4(this.first, this.second, this.third, this.fourth);

  @override
  String toString() {
    return '($first, $second, $third, $fourth)';
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Tuple4 &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second &&
          third == other.third &&
          fourth == other.fourth;

  @override
  int get hashCode => first.hashCode ^ second.hashCode ^ third.hashCode ^ fourth.hashCode;
}
