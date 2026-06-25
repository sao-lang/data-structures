class TreeNode<T> {
  T value;
  TreeNode<T>? left;
  TreeNode<T>? right;

  TreeNode(this.value);
}

class BinaryTree<T> {
  TreeNode<T>? _root;

  T? get root {
    if (_root == null) throw StateError('tree is empty');
    return _root?.value;
  }

  bool get isEmpty => _root == null;

  List<T> preOrderTraversal() {
    List<T> result = [];
    _preOrderHelper(_root, result);
    return result;
  }

  void _preOrderHelper(TreeNode<T>? node, List<T> result) {
    if (node != null) {
      result.add(node.value);
      _preOrderHelper(node.left, result);
      _preOrderHelper(node.right, result);
    }
  }

  List<T> inOrderTraversal() {
    List<T> result = [];
    _inOrderHelper(_root, result);
    return result;
  }

  void _inOrderHelper(TreeNode<T>? node, List<T> result) {
    if (node != null) {
      _inOrderHelper(node.left, result);
      result.add(node.value);
      _inOrderHelper(node.right, result);
    }
  }

  List<T> postOrderTraversal() {
    List<T> result = [];
    _postOrderHelper(_root, result);
    return result;
  }

  void _postOrderHelper(TreeNode<T>? node, List<T> result) {
    if (node != null) {
      _postOrderHelper(node.left, result);
      _postOrderHelper(node.right, result);
      result.add(node.value);
    }
  }

  List<T> levelOrderTraversal() {
    List<T> result = [];
    if (_root == null) return result;

    List<TreeNode<T>> queue = [];
    queue.add(_root!);

    while (queue.isNotEmpty) {
      TreeNode<T> node = queue.removeAt(0);
      result.add(node.value);
      if (node.left != null) queue.add(node.left!);
      if (node.right != null) queue.add(node.right!);
    }
    return result;
  }

  int get height => _heightHelper(_root);

  int _heightHelper(TreeNode<T>? node) {
    if (node == null) return -1;
    int leftHeight = _heightHelper(node.left);
    int rightHeight = _heightHelper(node.right);
    return (leftHeight > rightHeight ? leftHeight : rightHeight) + 1;
  }

  int get size => _sizeHelper(_root);

  int _sizeHelper(TreeNode<T>? node) {
    if (node == null) return 0;
    return 1 + _sizeHelper(node.left) + _sizeHelper(node.right);
  }

  void clear() {
    _root = null;
  }
}

class BinarySearchTree<T extends Comparable> {
  final BinaryTree<T> _tree = BinaryTree<T>();

  T? get root => _tree.root;
  bool get isEmpty => _tree.isEmpty;

  void insert(T value) {
    TreeNode<T> newNode = TreeNode(value);
    if (_tree._root == null) {
      _tree._root = newNode;
      return;
    }

    TreeNode<T>? current = _tree._root;
    while (true) {
      int comparison = value.compareTo(current!.value);
      if (comparison < 0) {
        if (current.left == null) {
          current.left = newNode;
          break;
        }
        current = current.left;
      } else if (comparison > 0) {
        if (current.right == null) {
          current.right = newNode;
          break;
        }
        current = current.right;
      } else {
        break;
      }
    }
  }

  bool search(T value) {
    TreeNode<T>? current = _tree._root;
    while (current != null) {
      int comparison = value.compareTo(current.value);
      if (comparison < 0) {
        current = current.left;
      } else if (comparison > 0) {
        current = current.right;
      } else {
        return true;
      }
    }
    return false;
  }

  T? findMin() {
    if (_tree._root == null) throw StateError('tree is empty');
    TreeNode<T> current = _tree._root!;
    while (current.left != null) {
      current = current.left!;
    }
    return current.value;
  }

  T? findMax() {
    if (_tree._root == null) throw StateError('tree is empty');
    TreeNode<T> current = _tree._root!;
    while (current.right != null) {
      current = current.right!;
    }
    return current.value;
  }

  bool delete(T value) {
    TreeNode<T>? parent;
    TreeNode<T>? current = _tree._root;

    while (current != null && current.value != value) {
      parent = current;
      int comparison = value.compareTo(current.value);
      if (comparison < 0) {
        current = current.left;
      } else {
        current = current.right;
      }
    }

    if (current == null) return false;

    if (current.left == null && current.right == null) {
      if (parent == null) {
        _tree._root = null;
      } else if (parent.left == current) {
        parent.left = null;
      } else {
        parent.right = null;
      }
    } else if (current.left == null) {
      if (parent == null) {
        _tree._root = current.right;
      } else if (parent.left == current) {
        parent.left = current.right;
      } else {
        parent.right = current.right;
      }
    } else if (current.right == null) {
      if (parent == null) {
        _tree._root = current.left;
      } else if (parent.left == current) {
        parent.left = current.left;
      } else {
        parent.right = current.left;
      }
    } else {
      TreeNode<T>? successorParent = current;
      TreeNode<T>? successor = current.right;
      while (successor!.left != null) {
        successorParent = successor;
        successor = successor.left;
      }

      current.value = successor.value;

      if (successorParent == current) {
        successorParent?.right = successor.right;
      } else {
        successorParent?.left = successor.right;
      }
    }

    return true;
  }

  List<T> preOrderTraversal() => _tree.preOrderTraversal();
  List<T> inOrderTraversal() => _tree.inOrderTraversal();
  List<T> postOrderTraversal() => _tree.postOrderTraversal();
  List<T> levelOrderTraversal() => _tree.levelOrderTraversal();
  int get height => _tree.height;
  int get size => _tree.size;
  void clear() => _tree.clear();
}

class AVLTreeNode<T extends Comparable> {
  T value;
  AVLTreeNode<T>? left;
  AVLTreeNode<T>? right;
  int height;

  AVLTreeNode(this.value) : height = 1;
}

class AVLTree<T extends Comparable> {
  AVLTreeNode<T>? _root;

  T? get root {
    if (_root == null) throw StateError('tree is empty');
    return _root?.value;
  }

  bool get isEmpty => _root == null;

  int _getHeight(AVLTreeNode<T>? node) {
    return node?.height ?? 0;
  }

  int _getBalance(AVLTreeNode<T>? node) {
    return node == null ? 0 : _getHeight(node.left) - _getHeight(node.right);
  }

  int _max(int a, int b) {
    return a > b ? a : b;
  }

  AVLTreeNode<T> _rightRotate(AVLTreeNode<T> y) {
    AVLTreeNode<T> x = y.left!;
    AVLTreeNode<T>? t2 = x.right;

    x.right = y;
    y.left = t2;

    y.height = _max(_getHeight(y.left), _getHeight(y.right)) + 1;
    x.height = _max(_getHeight(x.left), _getHeight(x.right)) + 1;

    return x;
  }

  AVLTreeNode<T> _leftRotate(AVLTreeNode<T> x) {
    AVLTreeNode<T> y = x.right!;
    AVLTreeNode<T>? t2 = y.left;

    y.left = x;
    x.right = t2;

    x.height = _max(_getHeight(x.left), _getHeight(x.right)) + 1;
    y.height = _max(_getHeight(y.left), _getHeight(y.right)) + 1;

    return y;
  }

  AVLTreeNode<T>? _insertHelper(AVLTreeNode<T>? node, T value) {
    if (node == null) {
      return AVLTreeNode(value);
    }

    int comparison = value.compareTo(node.value);
    if (comparison < 0) {
      node.left = _insertHelper(node.left, value);
    } else if (comparison > 0) {
      node.right = _insertHelper(node.right, value);
    } else {
      return node;
    }

    node.height = _max(_getHeight(node.left), _getHeight(node.right)) + 1;

    int balance = _getBalance(node);

    if (balance > 1 && value.compareTo(node.left!.value) < 0) {
      return _rightRotate(node);
    }

    if (balance < -1 && value.compareTo(node.right!.value) > 0) {
      return _leftRotate(node);
    }

    if (balance > 1 && value.compareTo(node.left!.value) > 0) {
      node.left = _leftRotate(node.left!);
      return _rightRotate(node);
    }

    if (balance < -1 && value.compareTo(node.right!.value) < 0) {
      node.right = _rightRotate(node.right!);
      return _leftRotate(node);
    }

    return node;
  }

  void insert(T value) {
    _root = _insertHelper(_root, value);
  }

  AVLTreeNode<T>? _getMinValueNode(AVLTreeNode<T>? node) {
    AVLTreeNode<T>? current = node;
    while (current?.left != null) {
      current = current!.left;
    }
    return current;
  }

  AVLTreeNode<T>? _deleteHelper(AVLTreeNode<T>? node, T value) {
    if (node == null) {
      return null;
    }

    int comparison = value.compareTo(node.value);
    if (comparison < 0) {
      node.left = _deleteHelper(node.left, value);
    } else if (comparison > 0) {
      node.right = _deleteHelper(node.right, value);
    } else {
      if (node.left == null || node.right == null) {
        AVLTreeNode<T>? temp;
        if (node.left != null) {
          temp = node.left;
        } else {
          temp = node.right;
        }

        if (temp == null) {
          return null;
        } else {
          node = temp;
        }
      } else {
        AVLTreeNode<T>? temp = _getMinValueNode(node.right);
        node.value = temp!.value;
        node.right = _deleteHelper(node.right, temp.value);
      }
    }

    if (node == null) {
      return null;
    }

    node.height = _max(_getHeight(node.left), _getHeight(node.right)) + 1;

    int balance = _getBalance(node);

    if (balance > 1 && _getBalance(node.left) >= 0) {
      return _rightRotate(node);
    }

    if (balance > 1 && _getBalance(node.left) < 0) {
      node.left = _leftRotate(node.left!);
      return _rightRotate(node);
    }

    if (balance < -1 && _getBalance(node.right) <= 0) {
      return _leftRotate(node);
    }

    if (balance < -1 && _getBalance(node.right) > 0) {
      node.right = _rightRotate(node.right!);
      return _leftRotate(node);
    }

    return node;
  }

  bool delete(T value) {
    if (!search(value)) {
      return false;
    }
    _root = _deleteHelper(_root, value);
    return true;
  }

  bool search(T value) {
    AVLTreeNode<T>? current = _root;
    while (current != null) {
      int comparison = value.compareTo(current.value);
      if (comparison < 0) {
        current = current.left;
      } else if (comparison > 0) {
        current = current.right;
      } else {
        return true;
      }
    }
    return false;
  }

  List<T> preOrderTraversal() {
    List<T> result = [];
    _preOrderHelper(_root, result);
    return result;
  }

  void _preOrderHelper(AVLTreeNode<T>? node, List<T> result) {
    if (node != null) {
      result.add(node.value);
      _preOrderHelper(node.left, result);
      _preOrderHelper(node.right, result);
    }
  }

  List<T> inOrderTraversal() {
    List<T> result = [];
    _inOrderHelper(_root, result);
    return result;
  }

  void _inOrderHelper(AVLTreeNode<T>? node, List<T> result) {
    if (node != null) {
      _inOrderHelper(node.left, result);
      result.add(node.value);
      _inOrderHelper(node.right, result);
    }
  }

  List<T> postOrderTraversal() {
    List<T> result = [];
    _postOrderHelper(_root, result);
    return result;
  }

  void _postOrderHelper(AVLTreeNode<T>? node, List<T> result) {
    if (node != null) {
      _postOrderHelper(node.left, result);
      _postOrderHelper(node.right, result);
      result.add(node.value);
    }
  }

  List<T> levelOrderTraversal() {
    List<T> result = [];
    if (_root == null) return result;

    List<AVLTreeNode<T>> queue = [];
    queue.add(_root!);

    while (queue.isNotEmpty) {
      AVLTreeNode<T> node = queue.removeAt(0);
      result.add(node.value);
      if (node.left != null) queue.add(node.left!);
      if (node.right != null) queue.add(node.right!);
    }
    return result;
  }

  int get height => _getHeight(_root);

  void clear() {
    _root = null;
  }
}

const bool _RED = true;
const bool _BLACK = false;

class RBTreeNode<T extends Comparable> {
  T value;
  RBTreeNode<T>? left;
  RBTreeNode<T>? right;
  RBTreeNode<T>? parent;
  bool color;

  RBTreeNode(this.value) : color = _RED;
}

class RBTree<T extends Comparable> {
  late RBTreeNode<T> _nil;
  RBTreeNode<T>? _root;

  RBTree() {
    _nil = RBTreeNode(T as T);
    _nil.color = _BLACK;
    _root = _nil;
  }

  bool get isEmpty => _root == _nil;

  void _leftRotate(RBTreeNode<T> x) {
    RBTreeNode<T> y = x.right!;
    x.right = y.left;
    if (y.left != _nil) {
      y.left!.parent = x;
    }
    y.parent = x.parent;
    if (x.parent == _nil) {
      _root = y;
    } else if (x == x.parent!.left) {
      x.parent!.left = y;
    } else {
      x.parent!.right = y;
    }
    y.left = x;
    x.parent = y;
  }

  void _rightRotate(RBTreeNode<T> y) {
    RBTreeNode<T> x = y.left!;
    y.left = x.right;
    if (x.right != _nil) {
      x.right!.parent = y;
    }
    x.parent = y.parent;
    if (y.parent == _nil) {
      _root = x;
    } else if (y == y.parent!.right) {
      y.parent!.right = x;
    } else {
      y.parent!.left = x;
    }
    x.right = y;
    y.parent = x;
  }

  void _insertFixup(RBTreeNode<T> z) {
    while (z.parent!.color) {
      if (z.parent == z.parent!.parent!.left) {
        RBTreeNode<T> y = z.parent!.parent!.right!;
        if (y.color) {
          z.parent!.color = _BLACK;
          y.color = _BLACK;
          z.parent!.parent!.color = _RED;
          z = z.parent!.parent!;
        } else {
          if (z == z.parent!.right) {
            z = z.parent!;
            _leftRotate(z);
          }
          z.parent!.color = _BLACK;
          z.parent!.parent!.color = _RED;
          _rightRotate(z.parent!.parent!);
        }
      } else {
        RBTreeNode<T> y = z.parent!.parent!.left!;
        if (y.color) {
          z.parent!.color = _BLACK;
          y.color = _BLACK;
          z.parent!.parent!.color = _RED;
          z = z.parent!.parent!;
        } else {
          if (z == z.parent!.left) {
            z = z.parent!;
            _rightRotate(z);
          }
          z.parent!.color = _BLACK;
          z.parent!.parent!.color = _RED;
          _leftRotate(z.parent!.parent!);
        }
      }
    }
    _root!.color = _BLACK;
  }

  void insert(T value) {
    RBTreeNode<T> z = RBTreeNode(value);
    z.left = _nil;
    z.right = _nil;
    z.parent = _nil;

    RBTreeNode<T>? y = _nil;
    RBTreeNode<T>? x = _root;

    while (x != _nil) {
      y = x;
      int comparison = z.value.compareTo(x!.value);
      if (comparison < 0) {
        x = x.left;
      } else if (comparison > 0) {
        x = x.right;
      } else {
        return;
      }
    }

    z.parent = y;
    if (y == _nil) {
      _root = z;
    } else {
      int comparison = z.value.compareTo(y!.value);
      if (comparison < 0) {
        y.left = z;
      } else {
        y.right = z;
      }
    }

    if (z.parent == _nil) {
      z.color = _BLACK;
      return;
    }

    if (z.parent!.parent == _nil) {
      return;
    }

    _insertFixup(z);
  }

  void _transplant(RBTreeNode<T> u, RBTreeNode<T> v) {
    if (u.parent == _nil) {
      _root = v;
    } else if (u == u.parent!.left) {
      u.parent!.left = v;
    } else {
      u.parent!.right = v;
    }
    v.parent = u.parent;
  }

  RBTreeNode<T> _minimum(RBTreeNode<T> node) {
    while (node.left != _nil) {
      node = node.left!;
    }
    return node;
  }

  void _deleteFixup(RBTreeNode<T> x) {
    while (x != _root && !x.color) {
      if (x == x.parent!.left) {
        RBTreeNode<T> w = x.parent!.right!;
        if (w.color) {
          w.color = _BLACK;
          x.parent!.color = _RED;
          _leftRotate(x.parent!);
          w = x.parent!.right!;
        }
        if (!w.left!.color && !w.right!.color) {
          w.color = _RED;
          x = x.parent!;
        } else {
          if (!w.right!.color) {
            w.left!.color = _BLACK;
            w.color = _RED;
            _rightRotate(w);
            w = x.parent!.right!;
          }
          w.color = x.parent!.color;
          x.parent!.color = _BLACK;
          w.right!.color = _BLACK;
          _leftRotate(x.parent!);
          x = _root!;
        }
      } else {
        RBTreeNode<T> w = x.parent!.left!;
        if (w.color) {
          w.color = _BLACK;
          x.parent!.color = _RED;
          _rightRotate(x.parent!);
          w = x.parent!.left!;
        }
        if (!w.right!.color && !w.left!.color) {
          w.color = _RED;
          x = x.parent!;
        } else {
          if (!w.left!.color) {
            w.right!.color = _BLACK;
            w.color = _RED;
            _leftRotate(w);
            w = x.parent!.left!;
          }
          w.color = x.parent!.color;
          x.parent!.color = _BLACK;
          w.left!.color = _BLACK;
          _rightRotate(x.parent!);
          x = _root!;
        }
      }
    }
    x.color = _BLACK;
  }

  void _deleteNodeHelper(RBTreeNode<T> node, T value) {
    RBTreeNode<T> z = _nil;
    while (node != _nil) {
      int comparison = node.value.compareTo(value);
      if (comparison == 0) {
        z = node;
      }
      if (comparison <= 0) {
        node = node.right!;
      } else {
        node = node.left!;
      }
    }

    if (z == _nil) {
      return;
    }

    RBTreeNode<T> y = z;
    bool yOriginalColor = y.color;
    RBTreeNode<T> x;

    if (z.left == _nil) {
      x = z.right!;
      _transplant(z, z.right!);
    } else if (z.right == _nil) {
      x = z.left!;
      _transplant(z, z.left!);
    } else {
      y = _minimum(z.right!);
      yOriginalColor = y.color;
      x = y.right!;
      if (y.parent == z) {
        x.parent = y;
      } else {
        _transplant(y, y.right!);
        y.right = z.right;
        y.right!.parent = y;
      }
      _transplant(z, y);
      y.left = z.left;
      y.left!.parent = y;
      y.color = z.color;
    }

    if (!yOriginalColor) {
      _deleteFixup(x);
    }
  }

  void delete(T value) {
    _deleteNodeHelper(_root!, value);
  }

  bool search(T value) {
    RBTreeNode<T>? current = _root;
    while (current != _nil) {
      int comparison = value.compareTo(current!.value);
      if (comparison < 0) {
        current = current.left;
      } else if (comparison > 0) {
        current = current.right;
      } else {
        return true;
      }
    }
    return false;
  }

  T? findMin() {
    if (isEmpty) throw StateError('tree is empty');
    RBTreeNode<T> node = _minimum(_root!);
    return node.value;
  }

  T? findMax() {
    if (isEmpty) throw StateError('tree is empty');
    RBTreeNode<T>? node = _root;
    while (node!.right != _nil) {
      node = node.right;
    }
    return node.value;
  }

  List<T> inOrderTraversal() {
    List<T> result = [];
    _inOrderHelper(_root, result);
    return result;
  }

  void _inOrderHelper(RBTreeNode<T>? node, List<T> result) {
    if (node != _nil && node != null) {
      _inOrderHelper(node.left, result);
      result.add(node.value);
      _inOrderHelper(node.right, result);
    }
  }
}
