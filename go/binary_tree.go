package main

import (
	"errors"
)

type TreeNode[T comparable] struct {
	value T
	left  *TreeNode[T]
	right *TreeNode[T]
}

type BinaryTree[T comparable] struct {
	root *TreeNode[T]
}

func NewBinaryTree[T comparable]() *BinaryTree[T] {
	return &BinaryTree[T]{root: nil}
}

func (bt *BinaryTree[T]) Root() (T, error) {
	var zero T
	if bt.root == nil {
		return zero, errors.New("tree is empty")
	}
	return bt.root.value, nil
}

func (bt *BinaryTree[T]) IsEmpty() bool {
	return bt.root == nil
}

func (bt *BinaryTree[T]) PreOrderTraversal() []T {
	result := make([]T, 0)
	bt.preOrderHelper(bt.root, &result)
	return result
}

func (bt *BinaryTree[T]) preOrderHelper(node *TreeNode[T], result *[]T) {
	if node != nil {
		*result = append(*result, node.value)
		bt.preOrderHelper(node.left, result)
		bt.preOrderHelper(node.right, result)
	}
}

func (bt *BinaryTree[T]) InOrderTraversal() []T {
	result := make([]T, 0)
	bt.inOrderHelper(bt.root, &result)
	return result
}

func (bt *BinaryTree[T]) inOrderHelper(node *TreeNode[T], result *[]T) {
	if node != nil {
		bt.inOrderHelper(node.left, result)
		*result = append(*result, node.value)
		bt.inOrderHelper(node.right, result)
	}
}

func (bt *BinaryTree[T]) PostOrderTraversal() []T {
	result := make([]T, 0)
	bt.postOrderHelper(bt.root, &result)
	return result
}

func (bt *BinaryTree[T]) postOrderHelper(node *TreeNode[T], result *[]T) {
	if node != nil {
		bt.postOrderHelper(node.left, result)
		bt.postOrderHelper(node.right, result)
		*result = append(*result, node.value)
	}
}

func (bt *BinaryTree[T]) LevelOrderTraversal() []T {
	result := make([]T, 0)
	if bt.root == nil {
		return result
	}

	queue := make([]*TreeNode[T], 0)
	queue = append(queue, bt.root)

	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]
		result = append(result, node.value)
		if node.left != nil {
			queue = append(queue, node.left)
		}
		if node.right != nil {
			queue = append(queue, node.right)
		}
	}
	return result
}

func (bt *BinaryTree[T]) Height() int {
	return bt.heightHelper(bt.root)
}

func (bt *BinaryTree[T]) heightHelper(node *TreeNode[T]) int {
	if node == nil {
		return -1
	}
	leftHeight := bt.heightHelper(node.left)
	rightHeight := bt.heightHelper(node.right)
	if leftHeight > rightHeight {
		return leftHeight + 1
	}
	return rightHeight + 1
}

func (bt *BinaryTree[T]) Size() int {
	return bt.sizeHelper(bt.root)
}

func (bt *BinaryTree[T]) sizeHelper(node *TreeNode[T]) int {
	if node == nil {
		return 0
	}
	return 1 + bt.sizeHelper(node.left) + bt.sizeHelper(node.right)
}

func (bt *BinaryTree[T]) Clear() {
	bt.root = nil
}

type BinarySearchTree[T ordered] struct {
	BinaryTree[T]
}

type ordered interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 |
		~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 | ~uintptr |
		~float32 | ~float64 | ~string
}

func NewBinarySearchTree[T ordered]() *BinarySearchTree[T] {
	return &BinarySearchTree[T]{
		BinaryTree: *NewBinaryTree[T](),
	}
}

func (bst *BinarySearchTree[T]) Insert(value T) {
	newNode := &TreeNode[T]{value: value, left: nil, right: nil}
	if bst.root == nil {
		bst.root = newNode
		return
	}

	current := bst.root
	for {
		if value < current.value {
			if current.left == nil {
				current.left = newNode
				break
			}
			current = current.left
		} else if value > current.value {
			if current.right == nil {
				current.right = newNode
				break
			}
			current = current.right
		} else {
			break
		}
	}
}

func (bst *BinarySearchTree[T]) Search(value T) bool {
	current := bst.root
	for current != nil {
		if value < current.value {
			current = current.left
		} else if value > current.value {
			current = current.right
		} else {
			return true
		}
	}
	return false
}

func (bst *BinarySearchTree[T]) FindMin() (T, error) {
	var zero T
	if bst.root == nil {
		return zero, errors.New("tree is empty")
	}
	current := bst.root
	for current.left != nil {
		current = current.left
	}
	return current.value, nil
}

func (bst *BinarySearchTree[T]) FindMax() (T, error) {
	var zero T
	if bst.root == nil {
		return zero, errors.New("tree is empty")
	}
	current := bst.root
	for current.right != nil {
		current = current.right
	}
	return current.value, nil
}

func (bst *BinarySearchTree[T]) Delete(value T) bool {
	var parent *TreeNode[T] = nil
	current := bst.root

	for current != nil && current.value != value {
		parent = current
		if value < current.value {
			current = current.left
		} else {
			current = current.right
		}
	}

	if current == nil {
		return false
	}

	if current.left == nil && current.right == nil {
		if parent == nil {
			bst.root = nil
		} else if parent.left == current {
			parent.left = nil
		} else {
			parent.right = nil
		}
	} else if current.left == nil {
		if parent == nil {
			bst.root = current.right
		} else if parent.left == current {
			parent.left = current.right
		} else {
			parent.right = current.right
		}
	} else if current.right == nil {
		if parent == nil {
			bst.root = current.left
		} else if parent.left == current {
			parent.left = current.left
		} else {
			parent.right = current.left
		}
	} else {
		successorParent := current
		successor := current.right
		for successor.left != nil {
			successorParent = successor
			successor = successor.left
		}

		current.value = successor.value

		if successorParent == current {
			successorParent.right = successor.right
		} else {
			successorParent.left = successor.right
		}
	}

	return true
}

type AVLTreeNode[T ordered] struct {
	value  T
	left   *AVLTreeNode[T]
	right  *AVLTreeNode[T]
	height int
}

type AVLTree[T ordered] struct {
	root *AVLTreeNode[T]
}

func NewAVLTree[T ordered]() *AVLTree[T] {
	return &AVLTree[T]{root: nil}
}

func (avl *AVLTree[T]) Root() (T, error) {
	var zero T
	if avl.root == nil {
		return zero, errors.New("tree is empty")
	}
	return avl.root.value, nil
}

func (avl *AVLTree[T]) IsEmpty() bool {
	return avl.root == nil
}

func (avl *AVLTree[T]) getHeight(node *AVLTreeNode[T]) int {
	if node == nil {
		return 0
	}
	return node.height
}

func (avl *AVLTree[T]) getBalance(node *AVLTreeNode[T]) int {
	if node == nil {
		return 0
	}
	return avl.getHeight(node.left) - avl.getHeight(node.right)
}

func (avl *AVLTree[T]) rightRotate(y *AVLTreeNode[T]) *AVLTreeNode[T] {
	x := y.left
	T2 := x.right

	x.right = y
	y.left = T2

	y.height = max(avl.getHeight(y.left), avl.getHeight(y.right)) + 1
	x.height = max(avl.getHeight(x.left), avl.getHeight(x.right)) + 1

	return x
}

func (avl *AVLTree[T]) leftRotate(x *AVLTreeNode[T]) *AVLTreeNode[T] {
	y := x.right
	T2 := y.left

	y.left = x
	x.right = T2

	x.height = max(avl.getHeight(x.left), avl.getHeight(x.right)) + 1
	y.height = max(avl.getHeight(y.left), avl.getHeight(y.right)) + 1

	return y
}

func (avl *AVLTree[T]) insertHelper(node *AVLTreeNode[T], value T) *AVLTreeNode[T] {
	if node == nil {
		return &AVLTreeNode[T]{value: value, left: nil, right: nil, height: 1}
	}

	if value < node.value {
		node.left = avl.insertHelper(node.left, value)
	} else if value > node.value {
		node.right = avl.insertHelper(node.right, value)
	} else {
		return node
	}

	node.height = max(avl.getHeight(node.left), avl.getHeight(node.right)) + 1

	balance := avl.getBalance(node)

	if balance > 1 && value < node.left.value {
		return avl.rightRotate(node)
	}

	if balance < -1 && value > node.right.value {
		return avl.leftRotate(node)
	}

	if balance > 1 && value > node.left.value {
		node.left = avl.leftRotate(node.left)
		return avl.rightRotate(node)
	}

	if balance < -1 && value < node.right.value {
		node.right = avl.rightRotate(node.right)
		return avl.leftRotate(node)
	}

	return node
}

func (avl *AVLTree[T]) Insert(value T) {
	avl.root = avl.insertHelper(avl.root, value)
}

func (avl *AVLTree[T]) getMinValueNode(node *AVLTreeNode[T]) *AVLTreeNode[T] {
	current := node
	for current.left != nil {
		current = current.left
	}
	return current
}

func (avl *AVLTree[T]) deleteHelper(node *AVLTreeNode[T], value T) *AVLTreeNode[T] {
	if node == nil {
		return nil
	}

	if value < node.value {
		node.left = avl.deleteHelper(node.left, value)
	} else if value > node.value {
		node.right = avl.deleteHelper(node.right, value)
	} else {
		if node.left == nil || node.right == nil {
			var temp *AVLTreeNode[T]
			if node.left != nil {
				temp = node.left
			} else {
				temp = node.right
			}

			if temp == nil {
				return nil
			} else {
				*node = *temp
			}
		} else {
			temp := avl.getMinValueNode(node.right)
			node.value = temp.value
			node.right = avl.deleteHelper(node.right, temp.value)
		}
	}

	node.height = max(avl.getHeight(node.left), avl.getHeight(node.right)) + 1

	balance := avl.getBalance(node)

	if balance > 1 && avl.getBalance(node.left) >= 0 {
		return avl.rightRotate(node)
	}

	if balance > 1 && avl.getBalance(node.left) < 0 {
		node.left = avl.leftRotate(node.left)
		return avl.rightRotate(node)
	}

	if balance < -1 && avl.getBalance(node.right) <= 0 {
		return avl.leftRotate(node)
	}

	if balance < -1 && avl.getBalance(node.right) > 0 {
		node.right = avl.rightRotate(node.right)
		return avl.leftRotate(node)
	}

	return node
}

func (avl *AVLTree[T]) Delete(value T) bool {
	if !avl.Search(value) {
		return false
	}
	avl.root = avl.deleteHelper(avl.root, value)
	return true
}

func (avl *AVLTree[T]) Search(value T) bool {
	current := avl.root
	for current != nil {
		if value < current.value {
			current = current.left
		} else if value > current.value {
			current = current.right
		} else {
			return true
		}
	}
	return false
}

func (avl *AVLTree[T]) PreOrderTraversal() []T {
	result := make([]T, 0)
	avl.preOrderHelper(avl.root, &result)
	return result
}

func (avl *AVLTree[T]) preOrderHelper(node *AVLTreeNode[T], result *[]T) {
	if node != nil {
		*result = append(*result, node.value)
		avl.preOrderHelper(node.left, result)
		avl.preOrderHelper(node.right, result)
	}
}

func (avl *AVLTree[T]) InOrderTraversal() []T {
	result := make([]T, 0)
	avl.inOrderHelper(avl.root, &result)
	return result
}

func (avl *AVLTree[T]) inOrderHelper(node *AVLTreeNode[T], result *[]T) {
	if node != nil {
		avl.inOrderHelper(node.left, result)
		*result = append(*result, node.value)
		avl.inOrderHelper(node.right, result)
	}
}

func (avl *AVLTree[T]) PostOrderTraversal() []T {
	result := make([]T, 0)
	avl.postOrderHelper(avl.root, &result)
	return result
}

func (avl *AVLTree[T]) postOrderHelper(node *AVLTreeNode[T], result *[]T) {
	if node != nil {
		avl.postOrderHelper(node.left, result)
		avl.postOrderHelper(node.right, result)
		*result = append(*result, node.value)
	}
}

func (avl *AVLTree[T]) LevelOrderTraversal() []T {
	result := make([]T, 0)
	if avl.root == nil {
		return result
	}

	queue := make([]*AVLTreeNode[T], 0)
	queue = append(queue, avl.root)

	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]
		result = append(result, node.value)
		if node.left != nil {
			queue = append(queue, node.left)
		}
		if node.right != nil {
			queue = append(queue, node.right)
		}
	}
	return result
}

func (avl *AVLTree[T]) Height() int {
	return avl.getHeight(avl.root)
}

func (avl *AVLTree[T]) Clear() {
	avl.root = nil
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

const (
	RED   = true
	BLACK = false
)

type RBTreeNode[T ordered] struct {
	value  T
	left   *RBTreeNode[T]
	right  *RBTreeNode[T]
	parent *RBTreeNode[T]
	color  bool
}

type RBTree[T ordered] struct {
	root *RBTreeNode[T]
	nil  *RBTreeNode[T]
}

func NewRBTree[T ordered]() *RBTree[T] {
	nilNode := &RBTreeNode[T]{color: BLACK}
	return &RBTree[T]{
		root: nilNode,
		nil:  nilNode,
	}
}

func (rbt *RBTree[T]) IsEmpty() bool {
	return rbt.root == rbt.nil
}

func (rbt *RBTree[T]) leftRotate(x *RBTreeNode[T]) {
	y := x.right
	x.right = y.left
	if y.left != rbt.nil {
		y.left.parent = x
	}
	y.parent = x.parent
	if x.parent == rbt.nil {
		rbt.root = y
	} else if x == x.parent.left {
		x.parent.left = y
	} else {
		x.parent.right = y
	}
	y.left = x
	x.parent = y
}

func (rbt *RBTree[T]) rightRotate(y *RBTreeNode[T]) {
	x := y.left
	y.left = x.right
	if x.right != rbt.nil {
		x.right.parent = y
	}
	x.parent = y.parent
	if y.parent == rbt.nil {
		rbt.root = x
	} else if y == y.parent.right {
		y.parent.right = x
	} else {
		y.parent.left = x
	}
	x.right = y
	y.parent = x
}

func (rbt *RBTree[T]) insertFixup(z *RBTreeNode[T]) {
	for z.parent.color {
		if z.parent == z.parent.parent.left {
			y := z.parent.parent.right
			if y.color {
				z.parent.color = BLACK
				y.color = BLACK
				z.parent.parent.color = RED
				z = z.parent.parent
			} else {
				if z == z.parent.right {
					z = z.parent
					rbt.leftRotate(z)
				}
				z.parent.color = BLACK
				z.parent.parent.color = RED
				rbt.rightRotate(z.parent.parent)
			}
		} else {
			y := z.parent.parent.left
			if y.color {
				z.parent.color = BLACK
				y.color = BLACK
				z.parent.parent.color = RED
				z = z.parent.parent
			} else {
				if z == z.parent.left {
					z = z.parent
					rbt.rightRotate(z)
				}
				z.parent.color = BLACK
				z.parent.parent.color = RED
				rbt.leftRotate(z.parent.parent)
			}
		}
	}
	rbt.root.color = BLACK
}

func (rbt *RBTree[T]) Insert(value T) {
	z := &RBTreeNode[T]{
		value:  value,
		left:   rbt.nil,
		right:  rbt.nil,
		parent: rbt.nil,
		color:  RED,
	}

	y := rbt.nil
	x := rbt.root

	for x != rbt.nil {
		y = x
		if z.value < x.value {
			x = x.left
		} else if z.value > x.value {
			x = x.right
		} else {
			return
		}
	}

	z.parent = y
	if y == rbt.nil {
		rbt.root = z
	} else if z.value < y.value {
		y.left = z
	} else {
		y.right = z
	}

	if z.parent == rbt.nil {
		z.color = BLACK
		return
	}

	if z.parent.parent == rbt.nil {
		return
	}

	rbt.insertFixup(z)
}

func (rbt *RBTree[T]) transplant(u, v *RBTreeNode[T]) {
	if u.parent == rbt.nil {
		rbt.root = v
	} else if u == u.parent.left {
		u.parent.left = v
	} else {
		u.parent.right = v
	}
	v.parent = u.parent
}

func (rbt *RBTree[T]) minimum(node *RBTreeNode[T]) *RBTreeNode[T] {
	for node.left != rbt.nil {
		node = node.left
	}
	return node
}

func (rbt *RBTree[T]) deleteFixup(x *RBTreeNode[T]) {
	for x != rbt.root && !x.color {
		if x == x.parent.left {
			w := x.parent.right
			if w.color {
				w.color = BLACK
				x.parent.color = RED
				rbt.leftRotate(x.parent)
				w = x.parent.right
			}
			if !w.left.color && !w.right.color {
				w.color = RED
				x = x.parent
			} else {
				if !w.right.color {
					w.left.color = BLACK
					w.color = RED
					rbt.rightRotate(w)
					w = x.parent.right
				}
				w.color = x.parent.color
				x.parent.color = BLACK
				w.right.color = BLACK
				rbt.leftRotate(x.parent)
				x = rbt.root
			}
		} else {
			w := x.parent.left
			if w.color {
				w.color = BLACK
				x.parent.color = RED
				rbt.rightRotate(x.parent)
				w = x.parent.left
			}
			if !w.right.color && !w.left.color {
				w.color = RED
				x = x.parent
			} else {
				if !w.left.color {
					w.right.color = BLACK
					w.color = RED
					rbt.leftRotate(w)
					w = x.parent.left
				}
				w.color = x.parent.color
				x.parent.color = BLACK
				w.left.color = BLACK
				rbt.rightRotate(x.parent)
				x = rbt.root
			}
		}
	}
	x.color = BLACK
}

func (rbt *RBTree[T]) deleteNodeHelper(node *RBTreeNode[T], value T) {
	z := rbt.nil
	for node != rbt.nil {
		if node.value == value {
			z = node
		}
		if node.value <= value {
			node = node.right
		} else {
			node = node.left
		}
	}

	if z == rbt.nil {
		return
	}

	y := z
	yOriginalColor := y.color
	var x *RBTreeNode[T]

	if z.left == rbt.nil {
		x = z.right
		rbt.transplant(z, z.right)
	} else if z.right == rbt.nil {
		x = z.left
		rbt.transplant(z, z.left)
	} else {
		y = rbt.minimum(z.right)
		yOriginalColor = y.color
		x = y.right
		if y.parent == z {
			x.parent = y
		} else {
			rbt.transplant(y, y.right)
			y.right = z.right
			y.right.parent = y
		}
		rbt.transplant(z, y)
		y.left = z.left
		y.left.parent = y
		y.color = z.color
	}

	if !yOriginalColor {
		rbt.deleteFixup(x)
	}
}

func (rbt *RBTree[T]) Delete(value T) {
	rbt.deleteNodeHelper(rbt.root, value)
}

func (rbt *RBTree[T]) Search(value T) bool {
	current := rbt.root
	for current != rbt.nil {
		if value < current.value {
			current = current.left
		} else if value > current.value {
			current = current.right
		} else {
			return true
		}
	}
	return false
}

func (rbt *RBTree[T]) FindMin() (T, error) {
	var zero T
	if rbt.IsEmpty() {
		return zero, errors.New("tree is empty")
	}
	node := rbt.minimum(rbt.root)
	return node.value, nil
}

func (rbt *RBTree[T]) FindMax() (T, error) {
	var zero T
	if rbt.IsEmpty() {
		return zero, errors.New("tree is empty")
	}
	node := rbt.root
	for node.right != rbt.nil {
		node = node.right
	}
	return node.value, nil
}

func (rbt *RBTree[T]) InOrderTraversal() []T {
	result := make([]T, 0)
	rbt.inOrderHelper(rbt.root, &result)
	return result
}

func (rbt *RBTree[T]) inOrderHelper(node *RBTreeNode[T], result *[]T) {
	if node != rbt.nil {
		rbt.inOrderHelper(node.left, result)
		*result = append(*result, node.value)
		rbt.inOrderHelper(node.right, result)
	}
}
