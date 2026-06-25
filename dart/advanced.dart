import 'dart:math' as math;

const int _maxLevel = 16;
const double _p = 0.5;

class SkipListNode<T extends Comparable> {
  final T value;
  final List<SkipListNode<T>?> forward;

  SkipListNode(this.value, int level) : forward = List<SkipListNode<T>?>.filled(level + 1, null);
}

class SkipList<T extends Comparable> {
  final SkipListNode<T> _head;
  int _level;
  final math.Random _random = math.Random();

  SkipList()
      : _head = SkipListNode<T>(null as T, _maxLevel),
        _level = 0;

  int _randomLevel() {
    int level = 0;
    while (_random.nextDouble() < _p && level < _maxLevel - 1) {
      level++;
    }
    return level;
  }

  bool search(T value) {
    SkipListNode<T>? current = _head;

    for (int i = _level; i >= 0; i--) {
      while (current!.forward[i] != null && current.forward[i]!.value.compareTo(value) < 0) {
        current = current.forward[i];
      }
    }

    current = current!.forward[0];
    return current != null && current.value == value;
  }

  void insert(T value) {
    List<SkipListNode<T>?> update = List<SkipListNode<T>?>.filled(_maxLevel + 1, null);
    SkipListNode<T>? current = _head;

    for (int i = _level; i >= 0; i--) {
      while (current!.forward[i] != null && current.forward[i]!.value.compareTo(value) < 0) {
        current = current.forward[i];
      }
      update[i] = current;
    }

    current = current!.forward[0];

    if (current == null || current.value != value) {
      int newLevel = _randomLevel();

      if (newLevel > _level) {
        for (int i = _level + 1; i <= newLevel; i++) {
          update[i] = _head;
        }
        _level = newLevel;
      }

      SkipListNode<T> newNode = SkipListNode<T>(value, newLevel);

      for (int i = 0; i <= newLevel; i++) {
        newNode.forward[i] = update[i]!.forward[i];
        update[i]!.forward[i] = newNode;
      }
    }
  }

  bool delete(T value) {
    List<SkipListNode<T>?> update = List<SkipListNode<T>?>.filled(_maxLevel + 1, null);
    SkipListNode<T>? current = _head;

    for (int i = _level; i >= 0; i--) {
      while (current!.forward[i] != null && current.forward[i]!.value.compareTo(value) < 0) {
        current = current.forward[i];
      }
      update[i] = current;
    }

    current = current!.forward[0];

    if (current != null && current.value == value) {
      for (int i = 0; i <= _level; i++) {
        if (update[i]!.forward[i] != current) {
          break;
        }
        update[i]!.forward[i] = current.forward[i];
      }

      while (_level > 0 && _head.forward[_level] == null) {
        _level--;
      }

      return true;
    }

    return false;
  }

  List<T> toSlice() {
    List<T> result = [];
    SkipListNode<T>? current = _head.forward[0];
    while (current != null) {
      result.add(current.value);
      current = current.forward[0];
    }
    return result;
  }
}

class SegmentTree<T> {
  final int _n;
  final int _size;
  final List<T> _tree;
  final T Function(T, T) _merge;
  final T _defaultValue;

  SegmentTree(List<T> data, this._merge, this._defaultValue)
      : _n = data.length,
        _size = _calculateSize(data.length),
        _tree = List<T>.filled(2 * _calculateSize(data.length), _defaultValue) {
    for (int i = 0; i < _n; i++) {
      _tree[_size + i] = data[i];
    }
    for (int i = _size - 1; i > 0; i--) {
      _tree[i] = _merge(_tree[2 * i], _tree[2 * i + 1]);
    }
  }

  static int _calculateSize(int n) {
    int size = 1;
    while (size < n) {
      size <<= 1;
    }
    return size;
  }

  void update(int index, T value) {
    if (index < 0 || index >= _n) {
      throw RangeError('index out of bounds');
    }
    index += _size;
    _tree[index] = value;
    index >>= 1;
    while (index >= 1) {
      T newVal = _merge(_tree[2 * index], _tree[2 * index + 1]);
      _tree[index] = newVal;
      index >>= 1;
    }
  }

  T query(int l, int r) {
    if (l < 0 || r >= _n || l > r) {
      throw RangeError('invalid query range');
    }
    T resLeft = _defaultValue;
    T resRight = _defaultValue;
    l += _size;
    r += _size;
    while (l <= r) {
      if (l % 2 == 1) {
        resLeft = _merge(resLeft, _tree[l]);
        l++;
      }
      if (r % 2 == 0) {
        resRight = _merge(_tree[r], resRight);
        r--;
      }
      l >>= 1;
      r >>= 1;
    }
    return _merge(resLeft, resRight);
  }

  T get(int index) {
    if (index < 0 || index >= _n) {
      throw RangeError('index out of bounds');
    }
    return _tree[_size + index];
  }
}

class FenwickTree {
  final List<int> _tree;
  final int _n;

  FenwickTree.fromSize(int size)
      : _tree = List<int>.filled(size + 1, 0),
        _n = size;

  FenwickTree.fromData(List<int> data)
      : _tree = List<int>.filled(data.length + 1, 0),
        _n = data.length {
    for (int i = 0; i < data.length; i++) {
      update(i, data[i]);
    }
  }

  void update(int index, int delta) {
    if (index < 0 || index >= _n) {
      throw RangeError('index out of bounds');
    }
    index++;
    while (index <= _n) {
      _tree[index] += delta;
      index += index & -index;
    }
  }

  void set(int index, int value) {
    int current = query(index, index);
    update(index, value - current);
  }

  int prefixSum(int index) {
    if (index < 0 || index >= _n) {
      throw RangeError('index out of bounds');
    }
    index++;
    int sum = 0;
    while (index > 0) {
      sum += _tree[index];
      index -= index & -index;
    }
    return sum;
  }

  int query(int l, int r) {
    if (l < 0 || r >= _n || l > r) {
      throw RangeError('invalid query range');
    }
    if (l == 0) {
      return prefixSum(r);
    }
    return prefixSum(r) - prefixSum(l - 1);
  }

  int get size => _n;
}

class SuffixArray {
  final String _text;
  late final List<int> _suffixArray;
  List<int>? _lcpArray;

  SuffixArray(this._text) {
    _suffixArray = _buildSuffixArray(_text);
  }

  List<int> _buildSuffixArray(String s) {
    int n = s.length;
    List<int> suffixArr = List<int>.generate(n, (i) => i);
    List<int> rank = List<int>.generate(n, (i) => s.codeUnitAt(i));
    int k = 1;

    while (k < n) {
      suffixArr.sort((i, j) {
        if (rank[i] != rank[j]) {
          return rank[i].compareTo(rank[j]);
        }
        int ra = (i + k < n) ? rank[i + k] : -1;
        int rb = (j + k < n) ? rank[j + k] : -1;
        return ra.compareTo(rb);
      });

      List<int> newRank = List<int>.filled(n, 0);
      newRank[suffixArr[0]] = 0;
      for (int i = 1; i < n; i++) {
        int prev = suffixArr[i - 1];
        int curr = suffixArr[i];
        bool same = rank[prev] == rank[curr];
        if (same) {
          int ra = (prev + k < n) ? rank[prev + k] : -1;
          int rb = (curr + k < n) ? rank[curr + k] : -1;
          same = ra == rb;
        }
        newRank[curr] = same ? newRank[prev] : newRank[prev] + 1;
      }
      rank = newRank;
      k *= 2;
    }
    return suffixArr;
  }

  List<int> getSuffixArray() {
    return List<int>.from(_suffixArray);
  }

  String getSuffix(int index) {
    if (index < 0 || index >= _text.length) {
      throw RangeError('index out of bounds');
    }
    return _text.substring(index);
  }

  List<int> _buildLCPArray() {
    int n = _text.length;
    List<int> rank = List<int>.filled(n, 0);
    for (int i = 0; i < n; i++) {
      rank[_suffixArray[i]] = i;
    }
    List<int> lcp = List<int>.filled(n - 1, 0);
    int k = 0;
    for (int i = 0; i < n; i++) {
      if (rank[i] == n - 1) {
        k = 0;
        continue;
      }
      int j = _suffixArray[rank[i] + 1];
      while (i + k < n && j + k < n && _text[i + k] == _text[j + k]) {
        k++;
      }
      lcp[rank[i]] = k;
      if (k > 0) {
        k--;
      }
    }
    return lcp;
  }

  List<int> getLCPArray() {
    _lcpArray ??= _buildLCPArray();
    return List<int>.from(_lcpArray!);
  }

  List<int> search(String pattern) {
    List<int> result = [];
    int m = pattern.length;
    int n = _text.length;
    int low = 0;
    int high = n - 1;

    while (low <= high) {
      int mid = (low + high) ~/ 2;
      String suffix = getSuffix(_suffixArray[mid]);
      int end = m < suffix.length ? m : suffix.length;
      String suffixPrefix = suffix.substring(0, end);

      if (pattern == suffixPrefix) {
        result.add(_suffixArray[mid]);
        int left = mid - 1;
        while (left >= 0) {
          String leftSuffix = getSuffix(_suffixArray[left]);
          if (leftSuffix.length >= m && leftSuffix.substring(0, m) == pattern) {
            result.add(_suffixArray[left]);
            left--;
          } else {
            break;
          }
        }
        int right = mid + 1;
        while (right < n) {
          String rightSuffix = getSuffix(_suffixArray[right]);
          if (rightSuffix.length >= m && rightSuffix.substring(0, m) == pattern) {
            result.add(_suffixArray[right]);
            right++;
          } else {
            break;
          }
        }
        break;
      } else if (pattern.compareTo(suffixPrefix) < 0) {
        high = mid - 1;
      } else {
        low = mid + 1;
      }
    }
    result.sort();
    return result;
  }

  int getLongestCommonPrefix() {
    List<int> lcp = getLCPArray();
    if (lcp.isEmpty) return 0;
    int maxLen = 0;
    for (int v in lcp) {
      if (v > maxLen) {
        maxLen = v;
      }
    }
    return maxLen;
  }

  String getLongestRepeatedSubstring() {
    List<int> lcp = getLCPArray();
    int maxLen = 0;
    int maxIndex = 0;
    for (int i = 0; i < lcp.length; i++) {
      if (lcp[i] > maxLen) {
        maxLen = lcp[i];
        maxIndex = i;
      }
    }
    if (maxLen == 0) return '';
    return _text.substring(_suffixArray[maxIndex], _suffixArray[maxIndex] + maxLen);
  }
}

abstract class KDPoint {
  List<double> get coordinates;
}

class SimpleKDPoint implements KDPoint {
  @override
  final List<double> coordinates;

  SimpleKDPoint(this.coordinates);
}

class KDNode<T extends KDPoint> {
  final T point;
  KDNode<T>? left;
  KDNode<T>? right;
  final int axis;

  KDNode(this.point, this.axis);
}

class KDTree<T extends KDPoint> {
  KDNode<T>? _root;
  final int _dimensions;

  KDTree(List<T> points)
      : _dimensions = points.isEmpty ? 0 : points[0].coordinates.length,
        _root = points.isEmpty ? null : _buildTree(points, 0, points[0].coordinates.length);

  static KDNode<T>? _buildTree<T extends KDPoint>(List<T> points, int depth, int dimensions) {
    if (points.isEmpty) return null;
    int axis = depth % dimensions;
    List<T> sortedPoints = List<T>.from(points);
    sortedPoints.sort((a, b) => a.coordinates[axis].compareTo(b.coordinates[axis]));
    int median = sortedPoints.length ~/ 2;
    KDNode<T> node = KDNode<T>(sortedPoints[median], axis);
    node.left = _buildTree(sortedPoints.sublist(0, median), depth + 1, dimensions);
    node.right = _buildTree(sortedPoints.sublist(median + 1), depth + 1, dimensions);
    return node;
  }

  void insert(T point) {
    if (_root == null) {
      _dimensionsCheck(point.coordinates.length);
      _root = KDNode<T>(point, 0);
      return;
    }
    _dimensionsCheck(point.coordinates.length);
    KDNode<T>? current = _root;
    int depth = 0;
    while (true) {
      int axis = depth % _dimensions;
      if (point.coordinates[axis] < current!.point.coordinates[axis]) {
        if (current.left == null) {
          current.left = KDNode<T>(point, (depth + 1) % _dimensions);
          break;
        }
        current = current.left;
      } else {
        if (current.right == null) {
          current.right = KDNode<T>(point, (depth + 1) % _dimensions);
          break;
        }
        current = current.right;
      }
      depth++;
    }
  }

  void _dimensionsCheck(int dims) {
    if (dims != _dimensions) {
      throw ArgumentError('point must have the same dimensions as the tree');
    }
  }

  double _distanceSquared(List<double> a, List<double> b) {
    double sum = 0.0;
    for (int i = 0; i < a.length; i++) {
      double diff = a[i] - b[i];
      sum += diff * diff;
    }
    return sum;
  }

  T? nearestNeighbor(List<double> target) {
    if (_root == null || target.length != _dimensions) return null;
    KDNode<T>? best = _root;
    double bestDist = _distanceSquared(_root!.point.coordinates, target);

    void search(KDNode<T>? node, int depth) {
      if (node == null) return;
      double dist = _distanceSquared(node.point.coordinates, target);
      if (dist < bestDist) {
        bestDist = dist;
        best = node;
      }
      int axis = depth % _dimensions;
      bool goLeft = target[axis] < node.point.coordinates[axis];
      if (goLeft) {
        search(node.left, depth + 1);
      } else {
        search(node.right, depth + 1);
      }
      double planeDist = (target[axis] - node.point.coordinates[axis]) * (target[axis] - node.point.coordinates[axis]);
      if (planeDist < bestDist) {
        if (goLeft) {
          search(node.right, depth + 1);
        } else {
          search(node.left, depth + 1);
        }
      }
    }

    search(_root, 0);
    return best?.point;
  }

  List<T> rangeSearch(List<double> minCoords, List<double> maxCoords) {
    List<T> result = [];
    if (_root == null || minCoords.length != _dimensions || maxCoords.length != _dimensions) {
      return result;
    }

    void search(KDNode<T>? node) {
      if (node == null) return;
      List<double> point = node.point.coordinates;
      bool inRange = true;
      for (int i = 0; i < point.length; i++) {
        if (point[i] < minCoords[i] || point[i] > maxCoords[i]) {
          inRange = false;
          break;
        }
      }
      if (inRange) {
        result.add(node.point);
      }
      int axis = node.axis;
      if (minCoords[axis] <= point[axis]) {
        search(node.left);
      }
      if (maxCoords[axis] >= point[axis]) {
        search(node.right);
      }
    }

    search(_root);
    return result;
  }

  List<T> kNearestNeighbors(List<double> target, int k) {
    List<T> result = [];
    if (k <= 0 || _root == null || target.length != _dimensions) {
      return result;
    }

    List<_Neighbor<T>> neighbors = [];

    void search(KDNode<T>? node, int depth) {
      if (node == null) return;
      double dist = _distanceSquared(node.point.coordinates, target);
      if (neighbors.length < k) {
        neighbors.add(_Neighbor(node.point, dist));
        neighbors.sort((a, b) => a.dist.compareTo(b.dist));
      } else if (dist < neighbors.last.dist) {
        neighbors.removeLast();
        neighbors.add(_Neighbor(node.point, dist));
        neighbors.sort((a, b) => a.dist.compareTo(b.dist));
      }
      int axis = depth % _dimensions;
      bool goLeft = target[axis] < node.point.coordinates[axis];
      if (goLeft) {
        search(node.left, depth + 1);
      } else {
        search(node.right, depth + 1);
      }
      double planeDist = (target[axis] - node.point.coordinates[axis]) * (target[axis] - node.point.coordinates[axis]);
      if (neighbors.length < k || planeDist < neighbors.last.dist) {
        if (goLeft) {
          search(node.right, depth + 1);
        } else {
          search(node.left, depth + 1);
        }
      }
    }

    search(_root, 0);
    for (var n in neighbors) {
      result.add(n.point);
    }
    return result;
  }
}

class _Neighbor<T extends KDPoint> {
  final T point;
  final double dist;

  _Neighbor(this.point, this.dist);
}

class TrieNode {
  final Map<String, TrieNode> children = {};
  bool isEndOfWord = false;
}

class Trie {
  final TrieNode _root = TrieNode();

  void insert(String word) {
    TrieNode current = _root;
    for (int i = 0; i < word.length; i++) {
      String char = word[i];
      if (!current.children.containsKey(char)) {
        current.children[char] = TrieNode();
      }
      current = current.children[char]!;
    }
    current.isEndOfWord = true;
  }

  bool search(String word) {
    TrieNode? current = _root;
    for (int i = 0; i < word.length; i++) {
      String char = word[i];
      if (!current!.children.containsKey(char)) {
        return false;
      }
      current = current.children[char];
    }
    return current!.isEndOfWord;
  }

  bool startsWith(String prefix) {
    TrieNode? current = _root;
    for (int i = 0; i < prefix.length; i++) {
      String char = prefix[i];
      if (!current!.children.containsKey(char)) {
        return false;
      }
      current = current.children[char];
    }
    return true;
  }

  bool _deleteHelper(TrieNode node, String word, int index) {
    if (index == word.length) {
      if (!node.isEndOfWord) return false;
      node.isEndOfWord = false;
      return node.children.isEmpty;
    }

    String char = word[index];
    if (!node.children.containsKey(char)) return false;

    bool shouldDeleteChild = _deleteHelper(node.children[char]!, word, index + 1);

    if (shouldDeleteChild) {
      node.children.remove(char);
      return node.children.isEmpty && !node.isEndOfWord;
    }

    return false;
  }

  void delete(String word) {
    _deleteHelper(_root, word, 0);
  }

  List<String> getAllWords() {
    List<String> words = [];
    _getAllWordsHelper(_root, '', words);
    return words;
  }

  void _getAllWordsHelper(TrieNode node, String prefix, List<String> words) {
    if (node.isEndOfWord) {
      words.add(prefix);
    }
    node.children.forEach((char, child) {
      _getAllWordsHelper(child, prefix + char, words);
    });
  }

  List<String> getWordsWithPrefix(String prefix) {
    TrieNode? current = _root;
    for (int i = 0; i < prefix.length; i++) {
      String char = prefix[i];
      if (!current!.children.containsKey(char)) {
        return [];
      }
      current = current.children[char];
    }
    List<String> words = [];
    _getAllWordsHelper(current!, prefix, words);
    return words;
  }
}

class GraphNode<T> {
  final T value;
  final List<GraphNode<T>> neighbors = [];

  GraphNode(this.value);
}

class Graph<T> {
  final Map<T, GraphNode<T>> _nodes = {};
  final bool isDirected;

  Graph(this.isDirected);

  void addVertex(T value) {
    if (!_nodes.containsKey(value)) {
      _nodes[value] = GraphNode(value);
    }
  }

  void addEdge(T from, T to) {
    addVertex(from);
    addVertex(to);

    GraphNode<T> fromNode = _nodes[from]!;
    GraphNode<T> toNode = _nodes[to]!;

    fromNode.neighbors.add(toNode);
    if (!isDirected) {
      toNode.neighbors.add(fromNode);
    }
  }

  void removeVertex(T value) {
    GraphNode<T>? node = _nodes[value];
    if (node == null) return;

    for (GraphNode<T> neighbor in node.neighbors) {
      neighbor.neighbors.remove(node);
    }

    _nodes.remove(value);
  }

  void removeEdge(T from, T to) {
    GraphNode<T>? fromNode = _nodes[from];
    GraphNode<T>? toNode = _nodes[to];

    if (fromNode == null || toNode == null) return;

    fromNode.neighbors.remove(toNode);
    if (!isDirected) {
      toNode.neighbors.remove(fromNode);
    }
  }

  List<T> bfs(T start) {
    List<T> result = [];
    Map<T, bool> visited = {};
    GraphNode<T>? startNode = _nodes[start];
    if (startNode == null) return result;

    List<GraphNode<T>> queue = [startNode];
    visited[start] = true;

    while (queue.isNotEmpty) {
      GraphNode<T> current = queue.removeAt(0);
      result.add(current.value);
      for (GraphNode<T> neighbor in current.neighbors) {
        if (!visited.containsKey(neighbor.value)) {
          visited[neighbor.value] = true;
          queue.add(neighbor);
        }
      }
    }

    return result;
  }

  List<T> dfs(T start) {
    List<T> result = [];
    Map<T, bool> visited = {};
    GraphNode<T>? startNode = _nodes[start];
    if (startNode == null) return result;

    List<GraphNode<T>> stack = [startNode];
    visited[start] = true;

    while (stack.isNotEmpty) {
      GraphNode<T> current = stack.removeLast();
      result.add(current.value);
      for (int i = current.neighbors.length - 1; i >= 0; i--) {
        GraphNode<T> neighbor = current.neighbors[i];
        if (!visited.containsKey(neighbor.value)) {
          visited[neighbor.value] = true;
          stack.add(neighbor);
        }
      }
    }

    return result;
  }

  List<T> getVertices() => _nodes.keys.toList();
  bool hasVertex(T value) => _nodes.containsKey(value);
  
  bool hasEdge(T from, T to) {
    GraphNode<T>? fromNode = _nodes[from];
    GraphNode<T>? toNode = _nodes[to];

    if (fromNode == null || toNode == null) return false;

    return fromNode.neighbors.any((n) => n.value == to);
  }
}

class UnionFind {
  final List<int> _parent;
  final List<int> _rank;

  UnionFind(int size)
      : _parent = List<int>.generate(size, (i) => i),
        _rank = List<int>.filled(size, 0);

  int find(int x) {
    if (_parent[x] != x) {
      _parent[x] = find(_parent[x]);
    }
    return _parent[x];
  }

  bool union(int x, int y) {
    int rootX = find(x);
    int rootY = find(y);

    if (rootX == rootY) return false;

    if (_rank[rootX] < _rank[rootY]) {
      _parent[rootX] = rootY;
    } else if (_rank[rootX] > _rank[rootY]) {
      _parent[rootY] = rootX;
    } else {
      _parent[rootY] = rootX;
      _rank[rootX]++;
    }

    return true;
  }

  bool connected(int x, int y) => find(x) == find(y);

  int get count {
    Set<int> roots = {};
    for (int i = 0; i < _parent.length; i++) {
      roots.add(find(i));
    }
    return roots.length;
  }
}

class BloomFilter {
  late List<bool> _bitArray;
  final int _size;
  final int _numHashFunctions;

  BloomFilter(int expectedItems, double falsePositiveRate)
      : _size = _calculateBloomSize(expectedItems, falsePositiveRate),
        _numHashFunctions = _calculateNumHashFunctions(_calculateBloomSize(expectedItems, falsePositiveRate), expectedItems) {
    _bitArray = List<bool>.filled(_size, false);
  }

  static int _calculateBloomSize(int n, double p) {
    double ln2 = math.log(2);
    double ln2Squared = ln2 * ln2;
    return (-n * math.log(p) / ln2Squared).ceil();
  }

  static int _calculateNumHashFunctions(int m, int n) {
    int k = ((m / n) * math.log(2)).round();
    return k < 1 ? 1 : k;
  }

  int _hash(String item, int seed) {
    int hashVal = seed;
    for (int i = 0; i < item.length; i++) {
      hashVal = (hashVal * 31 + item.codeUnitAt(i)) % _size;
    }
    if (hashVal < 0) hashVal += _size;
    return hashVal;
  }

  void add(String item) {
    for (int i = 0; i < _numHashFunctions; i++) {
      int hashVal = _hash(item, i);
      _bitArray[hashVal] = true;
    }
  }

  bool mightContain(String item) {
    for (int i = 0; i < _numHashFunctions; i++) {
      int hashVal = _hash(item, i);
      if (!_bitArray[hashVal]) {
        return false;
      }
    }
    return true;
  }

  void clear() {
    for (int i = 0; i < _size; i++) {
      _bitArray[i] = false;
    }
  }
}

class LRUCacheNode<K, V> {
  final K key;
  V value;
  LRUCacheNode<K, V>? prev;
  LRUCacheNode<K, V>? next;

  LRUCacheNode(this.key, this.value);
}

class LRUCache<K, V> {
  final int _capacity;
  final Map<K, LRUCacheNode<K, V>> _cache = {};
  final LRUCacheNode<K, V> _head;
  final LRUCacheNode<K, V> _tail;

  LRUCache(this._capacity)
      : _head = LRUCacheNode<K, V>(null as K, null as V),
        _tail = LRUCacheNode<K, V>(null as K, null as V) {
    _head.next = _tail;
    _tail.prev = _head;
  }

  int get size => _cache.length;
  int get capacity => _capacity;

  void _addToHead(LRUCacheNode<K, V> node) {
    node.prev = _head;
    node.next = _head.next;
    _head.next?.prev = node;
    _head.next = node;
  }

  void _removeNode(LRUCacheNode<K, V> node) {
    node.prev?.next = node.next;
    node.next?.prev = node.prev;
  }

  void _moveToHead(LRUCacheNode<K, V> node) {
    _removeNode(node);
    _addToHead(node);
  }

  LRUCacheNode<K, V>? _removeTail() {
    LRUCacheNode<K, V>? node = _tail.prev;
    if (node != _head) {
      _removeNode(node!);
      return node;
    }
    return null;
  }

  V? get(K key) {
    LRUCacheNode<K, V>? node = _cache[key];
    if (node == null) return null;
    _moveToHead(node);
    return node.value;
  }

  void put(K key, V value) {
    LRUCacheNode<K, V>? node = _cache[key];
    if (node != null) {
      node.value = value;
      _moveToHead(node);
      return;
    }

    LRUCacheNode<K, V> newNode = LRUCacheNode(key, value);
    _cache[key] = newNode;
    _addToHead(newNode);

    if (_cache.length > _capacity) {
      LRUCacheNode<K, V>? tail = _removeTail();
      if (tail != null) {
        _cache.remove(tail.key);
      }
    }
  }

  bool has(K key) => _cache.containsKey(key);

  bool delete(K key) {
    LRUCacheNode<K, V>? node = _cache[key];
    if (node == null) return false;
    _removeNode(node);
    _cache.remove(key);
    return true;
  }

  void clear() {
    _cache.clear();
    _head.next = _tail;
    _tail.prev = _head;
  }

  List<K> keys() {
    List<K> keys = [];
    LRUCacheNode<K, V>? current = _head.next;
    while (current != _tail) {
      keys.add(current!.key);
      current = current.next;
    }
    return keys;
  }

  List<V> values() {
    List<V> values = [];
    LRUCacheNode<K, V>? current = _head.next;
    while (current != _tail) {
      values.add(current!.value);
      current = current.next;
    }
    return values;
  }
}
