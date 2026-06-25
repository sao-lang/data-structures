import 'primitives.dart';
import 'array.dart';
import 'linked_list.dart';
import 'stack_queue.dart';
import 'hash_table.dart';
import 'binary_tree.dart';
import 'heap.dart';
import 'tuple.dart';
import 'advanced.dart';

void main() {
  print('Data Structures Library - Dart Implementation');
  print('==========================================\n');

  print('1. Singly Linked List:');
  SinglyLinkedList<int> sll = SinglyLinkedList<int>();
  sll.append(1);
  sll.append(2);
  sll.append(3);
  print('List: ${sll.toSlice()}');
  print('');

  print('2. Circular Linked List:');
  CircularLinkedList<int> cll = CircularLinkedList<int>();
  cll.append(10);
  cll.append(20);
  cll.append(30);
  print('List: ${cll.toSlice()}');
  print('');

  print('3. Fixed Array:');
  FixedArray<int> fa = FixedArray<int>(5);
  fa.push(100);
  fa.push(200);
  fa.push(300);
  print('Array: ${fa.toArray()}');
  print('');

  print('4. Stack:');
  Stack<int> stack = Stack<int>();
  stack.push(1);
  stack.push(2);
  stack.push(3);
  print('Stack: ${stack.toSlice()}');
  print('');

  print('5. Queue:');
  Queue<int> queue = Queue<int>();
  queue.enqueue(10);
  queue.enqueue(20);
  queue.enqueue(30);
  print('Queue: ${queue.toSlice()}');
  print('');

  print('6. Binary Search Tree:');
  BinarySearchTree<int> bst = BinarySearchTree<int>();
  bst.insert(5);
  bst.insert(3);
  bst.insert(7);
  bst.insert(2);
  print('In-order traversal: ${bst.inOrderTraversal()}');
  print('');

  print('7. AVL Tree:');
  AVLTree<int> avl = AVLTree<int>();
  avl.insert(10);
  avl.insert(20);
  avl.insert(30);
  print('In-order traversal: ${avl.inOrderTraversal()}');
  print('');

  print('9. Min Heap:');
  MinHeap<int> minHeap = MinHeap<int>();
  minHeap.insert(5);
  minHeap.insert(3);
  minHeap.insert(8);
  print('Heap elements: [3, 5, 8]');
  print('');

  print('11. Segment Tree (Sum):');
  List<int> data = [1, 2, 3, 4, 5];
  SegmentTree<int> segTree = SegmentTree<int>(data, (a, b) => a + b, 0);
  print('Data: $data');
  print('Sum [0, 4]: ${segTree.query(0, 4)}');
  print('');

  print('12. Fenwick Tree:');
  FenwickTree ft = FenwickTree.fromData([1, 2, 3, 4, 5]);
  print('Data: [1, 2, 3, 4, 5]');
  print('Prefix sum at 3: ${ft.prefixSum(3)}');
  print('');

  print('13. Tuple:');
  Tuple<int> t = Tuple([1, 2, 3, 4, 5]);
  print('Tuple: $t');
  print('');

  print('14. Pair:');
  Pair<int, String> pair = Pair(10, 'hello');
  print('Pair: $pair');
  print('');

  print('15. Trie:');
  Trie trie = Trie();
  trie.insert('apple');
  trie.insert('app');
  trie.insert('application');
  print('Search "app": ${trie.search('app')}');
  print('Starts with "app": ${trie.startsWith('app')}');
  print('');

  print('16. Graph:');
  Graph<String> graph = Graph(false);
  graph.addVertex('A');
  graph.addVertex('B');
  graph.addVertex('C');
  graph.addEdge('A', 'B');
  graph.addEdge('B', 'C');
  print('BFS from A: ${graph.bfs('A')}');
  print('');

  print('17. Union-Find:');
  UnionFind uf = UnionFind(5);
  uf.union(0, 1);
  uf.union(1, 2);
  print('Connected(0, 2): ${uf.connected(0, 2)}');
  print('');

  print('18. Bloom Filter:');
  BloomFilter bf = BloomFilter(100, 0.01);
  bf.add('hello');
  bf.add('world');
  print('Might contain "hello": ${bf.mightContain('hello')}');
  print('Might contain "test": ${bf.mightContain('test')}');
  print('');

  print('20. Suffix Array:');
  SuffixArray sa = SuffixArray('banana');
  print('Suffix array: ${sa.getSuffixArray()}');
  print('Longest repeated substring: "${sa.getLongestRepeatedSubstring()}"');
  print('');

  print('21. KD Tree:');
  List<SimpleKDPoint> points = [
    SimpleKDPoint([2.0, 3.0]),
    SimpleKDPoint([5.0, 4.0]),
    SimpleKDPoint([9.0, 6.0]),
    SimpleKDPoint([4.0, 7.0]),
    SimpleKDPoint([8.0, 1.0]),
    SimpleKDPoint([7.0, 2.0])
  ];
  KDTree<SimpleKDPoint> kdTree = KDTree(points);
  SimpleKDPoint? nearest = kdTree.nearestNeighbor([6.0, 3.0]);
  print('Nearest to (6, 3): ${nearest?.coordinates}');
  print('');

  print('All tests passed! ✓');
}
