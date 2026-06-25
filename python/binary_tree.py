from typing import TypeVar, Generic, Optional, List

T = TypeVar('T')

class TreeNode(Generic[T]):
    def __init__(self, value: T):
        self.value: T = value
        self.left: Optional[TreeNode[T]] = None
        self.right: Optional[TreeNode[T]] = None

class BinaryTree(Generic[T]):
    def __init__(self):
        self._root: Optional[TreeNode[T]] = None

    @property
    def root(self) -> Optional[T]:
        return self._root.value if self._root else None

    @property
    def is_empty(self) -> bool:
        return self._root is None

    def pre_order_traversal(self) -> List[T]:
        result: List[T] = []
        self._pre_order_helper(self._root, result)
        return result

    def _pre_order_helper(self, node: Optional[TreeNode[T]], result: List[T]) -> None:
        if node:
            result.append(node.value)
            self._pre_order_helper(node.left, result)
            self._pre_order_helper(node.right, result)

    def in_order_traversal(self) -> List[T]:
        result: List[T] = []
        self._in_order_helper(self._root, result)
        return result

    def _in_order_helper(self, node: Optional[TreeNode[T]], result: List[T]) -> None:
        if node:
            self._in_order_helper(node.left, result)
            result.append(node.value)
            self._in_order_helper(node.right, result)

    def post_order_traversal(self) -> List[T]:
        result: List[T] = []
        self._post_order_helper(self._root, result)
        return result

    def _post_order_helper(self, node: Optional[TreeNode[T]], result: List[T]) -> None:
        if node:
            self._post_order_helper(node.left, result)
            self._post_order_helper(node.right, result)
            result.append(node.value)

    def level_order_traversal(self) -> List[T]:
        result: List[T] = []
        if not self._root:
            return result

        queue: List[TreeNode[T]] = [self._root]
        while queue:
            node = queue.pop(0)
            result.append(node.value)
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)
        return result

    def height(self) -> int:
        return self._height_helper(self._root)

    def _height_helper(self, node: Optional[TreeNode[T]]) -> int:
        if not node:
            return -1
        left_height = self._height_helper(node.left)
        right_height = self._height_helper(node.right)
        return max(left_height, right_height) + 1

    def size(self) -> int:
        return self._size_helper(self._root)

    def _size_helper(self, node: Optional[TreeNode[T]]) -> int:
        if not node:
            return 0
        return 1 + self._size_helper(node.left) + self._size_helper(node.right)

    def clear(self) -> None:
        self._root = None

class BinarySearchTree(BinaryTree[T]):
    def __init__(self):
        super().__init__()

    def insert(self, value: T) -> None:
        new_node = TreeNode(value)
        if not self._root:
            self._root = new_node
            return

        current = self._root
        while True:
            if value < current.value:
                if not current.left:
                    current.left = new_node
                    break
                current = current.left
            elif value > current.value:
                if not current.right:
                    current.right = new_node
                    break
                current = current.right
            else:
                break

    def search(self, value: T) -> bool:
        current = self._root
        while current:
            if value < current.value:
                current = current.left
            elif value > current.value:
                current = current.right
            else:
                return True
        return False

    def find_min(self) -> Optional[T]:
        if not self._root:
            return None
        current = self._root
        while current.left:
            current = current.left
        return current.value

    def find_max(self) -> Optional[T]:
        if not self._root:
            return None
        current = self._root
        while current.right:
            current = current.right
        return current.value

    def delete(self, value: T) -> bool:
        current: Optional[TreeNode[T]] = self._root
        parent: Optional[TreeNode[T]] = None

        while current and current.value != value:
            parent = current
            current = current.left if value < current.value else current.right

        if not current:
            return False

        if not current.left and not current.right:
            if not parent:
                self._root = None
            elif parent.left == current:
                parent.left = None
            else:
                parent.right = None
        elif not current.left:
            if not parent:
                self._root = current.right
            elif parent.left == current:
                parent.left = current.right
            else:
                parent.right = current.right
        elif not current.right:
            if not parent:
                self._root = current.left
            elif parent.left == current:
                parent.left = current.left
            else:
                parent.right = current.left
        else:
            successor_parent = current
            successor = current.right
            while successor.left:
                successor_parent = successor
                successor = successor.left

            current.value = successor.value

            if successor_parent == current:
                successor_parent.right = successor.right
            else:
                successor_parent.left = successor.right

        return True


class AVLTreeNode(Generic[T]):
    def __init__(self, value: T):
        self.value: T = value
        self.left: Optional[AVLTreeNode[T]] = None
        self.right: Optional[AVLTreeNode[T]] = None
        self.height: int = 1


class AVLTree(Generic[T]):
    def __init__(self):
        self._root: Optional[AVLTreeNode[T]] = None

    @property
    def root(self) -> Optional[T]:
        return self._root.value if self._root else None

    @property
    def is_empty(self) -> bool:
        return self._root is None

    def _get_height(self, node: Optional[AVLTreeNode[T]]) -> int:
        return node.height if node else 0

    def _get_balance(self, node: Optional[AVLTreeNode[T]]) -> int:
        return self._get_height(node.left) - self._get_height(node.right) if node else 0

    def _right_rotate(self, y: AVLTreeNode[T]) -> AVLTreeNode[T]:
        x = y.left
        T2 = x.right

        x.right = y
        y.left = T2

        y.height = max(self._get_height(y.left), self._get_height(y.right)) + 1
        x.height = max(self._get_height(x.left), self._get_height(x.right)) + 1

        return x

    def _left_rotate(self, x: AVLTreeNode[T]) -> AVLTreeNode[T]:
        y = x.right
        T2 = y.left

        y.left = x
        x.right = T2

        x.height = max(self._get_height(x.left), self._get_height(x.right)) + 1
        y.height = max(self._get_height(y.left), self._get_height(y.right)) + 1

        return y

    def _insert_helper(self, node: Optional[AVLTreeNode[T]], value: T) -> AVLTreeNode[T]:
        if not node:
            return AVLTreeNode(value)

        if value < node.value:
            node.left = self._insert_helper(node.left, value)
        elif value > node.value:
            node.right = self._insert_helper(node.right, value)
        else:
            return node

        node.height = max(self._get_height(node.left), self._get_height(node.right)) + 1

        balance = self._get_balance(node)

        if balance > 1 and value < node.left.value:
            return self._right_rotate(node)

        if balance < -1 and value > node.right.value:
            return self._left_rotate(node)

        if balance > 1 and value > node.left.value:
            node.left = self._left_rotate(node.left)
            return self._right_rotate(node)

        if balance < -1 and value < node.right.value:
            node.right = self._right_rotate(node.right)
            return self._left_rotate(node)

        return node

    def insert(self, value: T) -> None:
        self._root = self._insert_helper(self._root, value)

    def _get_min_value_node(self, node: AVLTreeNode[T]) -> AVLTreeNode[T]:
        current = node
        while current.left:
            current = current.left
        return current

    def _delete_helper(self, node: Optional[AVLTreeNode[T]], value: T) -> Optional[AVLTreeNode[T]]:
        if not node:
            return None

        if value < node.value:
            node.left = self._delete_helper(node.left, value)
        elif value > node.value:
            node.right = self._delete_helper(node.right, value)
        else:
            if not node.left or not node.right:
                temp = node.left if node.left else node.right
                if not temp:
                    return None
                else:
                    node = temp
            else:
                temp = self._get_min_value_node(node.right)
                node.value = temp.value
                node.right = self._delete_helper(node.right, temp.value)

        if not node:
            return None

        node.height = max(self._get_height(node.left), self._get_height(node.right)) + 1

        balance = self._get_balance(node)

        if balance > 1 and self._get_balance(node.left) >= 0:
            return self._right_rotate(node)

        if balance > 1 and self._get_balance(node.left) < 0:
            node.left = self._left_rotate(node.left)
            return self._right_rotate(node)

        if balance < -1 and self._get_balance(node.right) <= 0:
            return self._left_rotate(node)

        if balance < -1 and self._get_balance(node.right) > 0:
            node.right = self._right_rotate(node.right)
            return self._left_rotate(node)

        return node

    def delete(self, value: T) -> bool:
        if not self.search(value):
            return False
        self._root = self._delete_helper(self._root, value)
        return True

    def search(self, value: T) -> bool:
        current = self._root
        while current:
            if value < current.value:
                current = current.left
            elif value > current.value:
                current = current.right
            else:
                return True
        return False

    def pre_order_traversal(self) -> List[T]:
        result: List[T] = []
        self._pre_order_helper(self._root, result)
        return result

    def _pre_order_helper(self, node: Optional[AVLTreeNode[T]], result: List[T]) -> None:
        if node:
            result.append(node.value)
            self._pre_order_helper(node.left, result)
            self._pre_order_helper(node.right, result)

    def in_order_traversal(self) -> List[T]:
        result: List[T] = []
        self._in_order_helper(self._root, result)
        return result

    def _in_order_helper(self, node: Optional[AVLTreeNode[T]], result: List[T]) -> None:
        if node:
            self._in_order_helper(node.left, result)
            result.append(node.value)
            self._in_order_helper(node.right, result)

    def post_order_traversal(self) -> List[T]:
        result: List[T] = []
        self._post_order_helper(self._root, result)
        return result

    def _post_order_helper(self, node: Optional[AVLTreeNode[T]], result: List[T]) -> None:
        if node:
            self._post_order_helper(node.left, result)
            self._post_order_helper(node.right, result)
            result.append(node.value)

    def level_order_traversal(self) -> List[T]:
        result: List[T] = []
        if not self._root:
            return result

        queue: List[AVLTreeNode[T]] = [self._root]
        while queue:
            node = queue.pop(0)
            result.append(node.value)
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)
        return result

    def height(self) -> int:
        return self._get_height(self._root)

    def clear(self) -> None:
        self._root = None


class RedBlackTreeNode(Generic[T]):
    RED = True
    BLACK = False

    def __init__(self, value: T):
        self.value: T = value
        self.left: Optional[RedBlackTreeNode[T]] = None
        self.right: Optional[RedBlackTreeNode[T]] = None
        self.parent: Optional[RedBlackTreeNode[T]] = None
        self.color: bool = RedBlackTreeNode.RED


class RedBlackTree(Generic[T]):
    def __init__(self):
        self._root: Optional[RedBlackTreeNode[T]] = None
        self._TNULL: RedBlackTreeNode[T] = RedBlackTreeNode(None)
        self._TNULL.color = RedBlackTreeNode.BLACK
        self._TNULL.left = self._TNULL
        self._TNULL.right = self._TNULL
        self._TNULL.parent = self._TNULL

    @property
    def root(self) -> Optional[T]:
        return self._root.value if self._root and self._root != self._TNULL else None

    @property
    def is_empty(self) -> bool:
        return self._root is None or self._root == self._TNULL

    def _left_rotate(self, x: RedBlackTreeNode[T]) -> None:
        y = x.right
        x.right = y.left
        if y.left != self._TNULL:
            y.left.parent = x
        y.parent = x.parent
        if x.parent is None:
            self._root = y
        elif x == x.parent.left:
            x.parent.left = y
        else:
            x.parent.right = y
        y.left = x
        x.parent = y

    def _right_rotate(self, y: RedBlackTreeNode[T]) -> None:
        x = y.left
        y.left = x.right
        if x.right != self._TNULL:
            x.right.parent = y
        x.parent = y.parent
        if y.parent is None:
            self._root = x
        elif y == y.parent.right:
            y.parent.right = x
        else:
            y.parent.left = x
        x.right = y
        y.parent = x

    def _insert_fixup(self, k: RedBlackTreeNode[T]) -> None:
        while k.parent and k.parent.color == RedBlackTreeNode.RED:
            if k.parent.parent and k.parent == k.parent.parent.left:
                u = k.parent.parent.right
                if u and u.color == RedBlackTreeNode.RED:
                    k.parent.color = RedBlackTreeNode.BLACK
                    u.color = RedBlackTreeNode.BLACK
                    k.parent.parent.color = RedBlackTreeNode.RED
                    k = k.parent.parent
                else:
                    if k == k.parent.right:
                        k = k.parent
                        self._left_rotate(k)
                    if k.parent:
                        k.parent.color = RedBlackTreeNode.BLACK
                    if k.parent and k.parent.parent:
                        k.parent.parent.color = RedBlackTreeNode.RED
                        self._right_rotate(k.parent.parent)
            else:
                if k.parent and k.parent.parent:
                    u = k.parent.parent.left
                    if u and u.color == RedBlackTreeNode.RED:
                        k.parent.color = RedBlackTreeNode.BLACK
                        u.color = RedBlackTreeNode.BLACK
                        k.parent.parent.color = RedBlackTreeNode.RED
                        k = k.parent.parent
                    else:
                        if k == k.parent.left:
                            k = k.parent
                            self._right_rotate(k)
                        if k.parent:
                            k.parent.color = RedBlackTreeNode.BLACK
                        if k.parent and k.parent.parent:
                            k.parent.parent.color = RedBlackTreeNode.RED
                            self._left_rotate(k.parent.parent)
            if k == self._root:
                break
        if self._root:
            self._root.color = RedBlackTreeNode.BLACK

    def insert(self, value: T) -> None:
        node = RedBlackTreeNode(value)
        node.parent = None
        node.value = value
        node.left = self._TNULL
        node.right = self._TNULL
        node.color = RedBlackTreeNode.RED

        y = None
        x = self._root

        while x and x != self._TNULL:
            y = x
            if node.value < x.value:
                x = x.left
            else:
                x = x.right

        node.parent = y
        if y is None:
            self._root = node
        elif node.value < y.value:
            y.left = node
        else:
            y.right = node

        if node.parent is None:
            node.color = RedBlackTreeNode.BLACK
            return

        if node.parent.parent is None:
            return

        self._insert_fixup(node)

    def _transplant(self, u: RedBlackTreeNode[T], v: RedBlackTreeNode[T]) -> None:
        if u.parent is None:
            self._root = v
        elif u == u.parent.left:
            u.parent.left = v
        else:
            u.parent.right = v
        v.parent = u.parent

    def _minimum(self, node: RedBlackTreeNode[T]) -> RedBlackTreeNode[T]:
        while node.left != self._TNULL:
            node = node.left
        return node

    def _delete_fixup(self, x: RedBlackTreeNode[T]) -> None:
        while x != self._root and x.color == RedBlackTreeNode.BLACK:
            if x.parent and x == x.parent.left:
                w = x.parent.right
                if w and w.color == RedBlackTreeNode.RED:
                    w.color = RedBlackTreeNode.BLACK
                    x.parent.color = RedBlackTreeNode.RED
                    self._left_rotate(x.parent)
                    w = x.parent.right
                if w and w.left and w.left.color == RedBlackTreeNode.BLACK and w.right and w.right.color == RedBlackTreeNode.BLACK:
                    w.color = RedBlackTreeNode.RED
                    x = x.parent
                elif w:
                    if w.right and w.right.color == RedBlackTreeNode.BLACK:
                        if w.left:
                            w.left.color = RedBlackTreeNode.BLACK
                        w.color = RedBlackTreeNode.RED
                        self._right_rotate(w)
                        w = x.parent.right
                    if w:
                        w.color = x.parent.color
                        x.parent.color = RedBlackTreeNode.BLACK
                        if w.right:
                            w.right.color = RedBlackTreeNode.BLACK
                        self._left_rotate(x.parent)
                    x = self._root
            else:
                if x.parent:
                    w = x.parent.left
                    if w and w.color == RedBlackTreeNode.RED:
                        w.color = RedBlackTreeNode.BLACK
                        x.parent.color = RedBlackTreeNode.RED
                        self._right_rotate(x.parent)
                        w = x.parent.left
                    if w and w.right and w.right.color == RedBlackTreeNode.BLACK and w.left and w.left.color == RedBlackTreeNode.BLACK:
                        w.color = RedBlackTreeNode.RED
                        x = x.parent
                    elif w:
                        if w.left and w.left.color == RedBlackTreeNode.BLACK:
                            if w.right:
                                w.right.color = RedBlackTreeNode.BLACK
                            w.color = RedBlackTreeNode.RED
                            self._left_rotate(w)
                            w = x.parent.left
                        if w:
                            w.color = x.parent.color
                            x.parent.color = RedBlackTreeNode.BLACK
                            if w.left:
                                w.left.color = RedBlackTreeNode.BLACK
                            self._right_rotate(x.parent)
                        x = self._root
        x.color = RedBlackTreeNode.BLACK

    def _delete_node_helper(self, node: RedBlackTreeNode[T], key: T) -> None:
        z = self._TNULL
        while node != self._TNULL:
            if node.value == key:
                z = node
            if node.value <= key:
                node = node.right
            else:
                node = node.left

        if z == self._TNULL:
            return

        y = z
        y_original_color = y.color
        if z.left == self._TNULL:
            x = z.right
            self._transplant(z, z.right)
        elif z.right == self._TNULL:
            x = z.left
            self._transplant(z, z.left)
        else:
            y = self._minimum(z.right)
            y_original_color = y.color
            x = y.right
            if y.parent == z:
                x.parent = y
            else:
                self._transplant(y, y.right)
                y.right = z.right
                y.right.parent = y
            self._transplant(z, y)
            y.left = z.left
            y.left.parent = y
            y.color = z.color
        if y_original_color == RedBlackTreeNode.BLACK:
            self._delete_fixup(x)

    def delete(self, value: T) -> bool:
        if not self.search(value):
            return False
        if self._root:
            self._delete_node_helper(self._root, value)
        return True

    def search(self, value: T) -> bool:
        current = self._root
        while current and current != self._TNULL:
            if value < current.value:
                current = current.left
            elif value > current.value:
                current = current.right
            else:
                return True
        return False

    def _pre_order_helper(self, node: Optional[RedBlackTreeNode[T]], result: List[T]) -> None:
        if node and node != self._TNULL:
            result.append(node.value)
            self._pre_order_helper(node.left, result)
            self._pre_order_helper(node.right, result)

    def _in_order_helper(self, node: Optional[RedBlackTreeNode[T]], result: List[T]) -> None:
        if node and node != self._TNULL:
            self._in_order_helper(node.left, result)
            result.append(node.value)
            self._in_order_helper(node.right, result)

    def _post_order_helper(self, node: Optional[RedBlackTreeNode[T]], result: List[T]) -> None:
        if node and node != self._TNULL:
            self._post_order_helper(node.left, result)
            self._post_order_helper(node.right, result)
            result.append(node.value)

    def pre_order_traversal(self) -> List[T]:
        result: List[T] = []
        self._pre_order_helper(self._root, result)
        return result

    def in_order_traversal(self) -> List[T]:
        result: List[T] = []
        self._in_order_helper(self._root, result)
        return result

    def post_order_traversal(self) -> List[T]:
        result: List[T] = []
        self._post_order_helper(self._root, result)
        return result

    def level_order_traversal(self) -> List[T]:
        result: List[T] = []
        if not self._root or self._root == self._TNULL:
            return result

        queue: List[RedBlackTreeNode[T]] = [self._root]
        while queue:
            node = queue.pop(0)
            result.append(node.value)
            if node.left != self._TNULL:
                queue.append(node.left)
            if node.right != self._TNULL:
                queue.append(node.right)
        return result

    def find_min(self) -> Optional[T]:
        if not self._root or self._root == self._TNULL:
            return None
        current = self._root
        while current.left != self._TNULL:
            current = current.left
        return current.value

    def find_max(self) -> Optional[T]:
        if not self._root or self._root == self._TNULL:
            return None
        current = self._root
        while current.right != self._TNULL:
            current = current.right
        return current.value

    def clear(self) -> None:
        self._root = None


if __name__ == "__main__":
    print("Binary Search Tree Example:")
    bst = BinarySearchTree[int]()
    bst.insert(5)
    bst.insert(3)
    bst.insert(7)
    bst.insert(2)
    bst.insert(4)
    bst.insert(6)
    bst.insert(8)

    print(f"In-order traversal: {bst.in_order_traversal()}")
    print(f"Pre-order traversal: {bst.pre_order_traversal()}")
    print(f"Post-order traversal: {bst.post_order_traversal()}")
    print(f"Level-order traversal: {bst.level_order_traversal()}")
    print(f"Min: {bst.find_min()}")
    print(f"Max: {bst.find_max()}")
    print(f"Search for 4: {bst.search(4)}")
    print(f"Search for 9: {bst.search(9)}")
    print(f"Height: {bst.height()}")
    print(f"Size: {bst.size()}")

    print("\nRed-Black Tree Example:")
    rbt = RedBlackTree[int]()
    values = [10, 20, 30, 15, 25, 5]
    for val in values:
        rbt.insert(val)
        print(f"Inserted {val}, In-order: {rbt.in_order_traversal()}")

    print(f"\nMin: {rbt.find_min()}")
    print(f"Max: {rbt.find_max()}")
    print(f"Search for 15: {rbt.search(15)}")
    print(f"Search for 100: {rbt.search(100)}")
    print(f"Level-order traversal: {rbt.level_order_traversal()}")

    print("\nDeleting 20:")
    rbt.delete(20)
    print(f"In-order after delete: {rbt.in_order_traversal()}")
