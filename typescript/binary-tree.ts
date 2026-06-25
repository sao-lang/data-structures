class TreeNode<T> {
    value: T;
    left: TreeNode<T> | null;
    right: TreeNode<T> | null;

    constructor(value: T) {
        this.value = value;
        this.left = null;
        this.right = null;
    }
}

export class BinaryTree<T> {
    protected _root: TreeNode<T> | null;

    constructor() {
        this._root = null;
    }

    get root(): T | undefined {
        return this._root?.value;
    }

    get isEmpty(): boolean {
        return this._root === null;
    }

    preOrderTraversal(): T[] {
        const result: T[] = [];
        this.preOrderHelper(this._root, result);
        return result;
    }

    private preOrderHelper(node: TreeNode<T> | null, result: T[]): void {
        if (node) {
            result.push(node.value);
            this.preOrderHelper(node.left, result);
            this.preOrderHelper(node.right, result);
        }
    }

    inOrderTraversal(): T[] {
        const result: T[] = [];
        this.inOrderHelper(this._root, result);
        return result;
    }

    private inOrderHelper(node: TreeNode<T> | null, result: T[]): void {
        if (node) {
            this.inOrderHelper(node.left, result);
            result.push(node.value);
            this.inOrderHelper(node.right, result);
        }
    }

    postOrderTraversal(): T[] {
        const result: T[] = [];
        this.postOrderHelper(this._root, result);
        return result;
    }

    private postOrderHelper(node: TreeNode<T> | null, result: T[]): void {
        if (node) {
            this.postOrderHelper(node.left, result);
            this.postOrderHelper(node.right, result);
            result.push(node.value);
        }
    }

    levelOrderTraversal(): T[] {
        const result: T[] = [];
        if (!this._root) {
            return result;
        }

        const queue: TreeNode<T>[] = [this._root];
        while (queue.length > 0) {
            const node = queue.shift()!;
            result.push(node.value);
            if (node.left) {
                queue.push(node.left);
            }
            if (node.right) {
                queue.push(node.right);
            }
        }
        return result;
    }

    height(): number {
        return this.heightHelper(this._root);
    }

    private heightHelper(node: TreeNode<T> | null): number {
        if (!node) {
            return -1;
        }
        const leftHeight = this.heightHelper(node.left);
        const rightHeight = this.heightHelper(node.right);
        return Math.max(leftHeight, rightHeight) + 1;
    }

    size(): number {
        return this.sizeHelper(this._root);
    }

    private sizeHelper(node: TreeNode<T> | null): number {
        if (!node) {
            return 0;
        }
        return 1 + this.sizeHelper(node.left) + this.sizeHelper(node.right);
    }

    clear(): void {
        this._root = null;
    }
}

export class BinarySearchTree<T extends number | string> extends BinaryTree<T> {
    constructor() {
        super();
    }

    insert(value: T): void {
        const newNode = new TreeNode(value);
        if (!this._root) {
            this._root = newNode;
            return;
        }

        let current = this._root;
        while (true) {
            if (value < current.value) {
                if (!current.left) {
                    current.left = newNode;
                    break;
                }
                current = current.left;
            } else if (value > current.value) {
                if (!current.right) {
                    current.right = newNode;
                    break;
                }
                current = current.right;
            } else {
                break;
            }
        }
    }

    search(value: T): boolean {
        let current = this._root;
        while (current) {
            if (value < current.value) {
                current = current.left;
            } else if (value > current.value) {
                current = current.right;
            } else {
                return true;
            }
        }
        return false;
    }

    findMin(): T | undefined {
        if (!this._root) {
            return undefined;
        }
        let current = this._root;
        while (current.left) {
            current = current.left;
        }
        return current.value;
    }

    findMax(): T | undefined {
        if (!this._root) {
            return undefined;
        }
        let current = this._root;
        while (current.right) {
            current = current.right;
        }
        return current.value;
    }

    delete(value: T): boolean {
        let current: TreeNode<T> | null = this._root;
        let parent: TreeNode<T> | null = null;

        while (current && current.value !== value) {
            parent = current;
            current = value < current.value ? current.left : current.right;
        }

        if (!current) {
            return false;
        }

        if (!current.left && !current.right) {
            if (!parent) {
                this._root = null;
            } else if (parent.left === current) {
                parent.left = null;
            } else {
                parent.right = null;
            }
        } else if (!current.left) {
            if (!parent) {
                this._root = current.right;
            } else if (parent.left === current) {
                parent.left = current.right;
            } else {
                parent.right = current.right;
            }
        } else if (!current.right) {
            if (!parent) {
                this._root = current.left;
            } else if (parent.left === current) {
                parent.left = current.left;
            } else {
                parent.right = current.left;
            }
        } else {
            let successorParent = current;
            let successor = current.right;
            while (successor.left) {
                successorParent = successor;
                successor = successor.left;
            }

            current.value = successor.value;

            if (successorParent === current) {
                successorParent.right = successor.right;
            } else {
                successorParent.left = successor.right;
            }
        }

        return true;
    }
}

class AVLTreeNode<T extends number | string> {
    value: T;
    left: AVLTreeNode<T> | null;
    right: AVLTreeNode<T> | null;
    height: number;

    constructor(value: T) {
        this.value = value;
        this.left = null;
        this.right = null;
        this.height = 1;
    }
}

export class AVLTree<T extends number | string> {
    private _root: AVLTreeNode<T> | null;

    constructor() {
        this._root = null;
    }

    get root(): T | undefined {
        return this._root?.value;
    }

    get isEmpty(): boolean {
        return this._root === null;
    }

    private getHeight(node: AVLTreeNode<T> | null): number {
        return node ? node.height : 0;
    }

    private getBalance(node: AVLTreeNode<T> | null): number {
        return node ? this.getHeight(node.left) - this.getHeight(node.right) : 0;
    }

    private rightRotate(y: AVLTreeNode<T>): AVLTreeNode<T> {
        const x = y.left!;
        const T2 = x.right;

        x.right = y;
        y.left = T2;

        y.height = Math.max(this.getHeight(y.left), this.getHeight(y.right)) + 1;
        x.height = Math.max(this.getHeight(x.left), this.getHeight(x.right)) + 1;

        return x;
    }

    private leftRotate(x: AVLTreeNode<T>): AVLTreeNode<T> {
        const y = x.right!;
        const T2 = y.left;

        y.left = x;
        x.right = T2;

        x.height = Math.max(this.getHeight(x.left), this.getHeight(x.right)) + 1;
        y.height = Math.max(this.getHeight(y.left), this.getHeight(y.right)) + 1;

        return y;
    }

    private insertHelper(node: AVLTreeNode<T> | null, value: T): AVLTreeNode<T> {
        if (!node) {
            return new AVLTreeNode(value);
        }

        if (value < node.value) {
            node.left = this.insertHelper(node.left, value);
        } else if (value > node.value) {
            node.right = this.insertHelper(node.right, value);
        } else {
            return node;
        }

        node.height = Math.max(this.getHeight(node.left), this.getHeight(node.right)) + 1;

        const balance = this.getBalance(node);

        if (balance > 1 && value < node.left!.value) {
            return this.rightRotate(node);
        }

        if (balance < -1 && value > node.right!.value) {
            return this.leftRotate(node);
        }

        if (balance > 1 && value > node.left!.value) {
            node.left = this.leftRotate(node.left!);
            return this.rightRotate(node);
        }

        if (balance < -1 && value < node.right!.value) {
            node.right = this.rightRotate(node.right!);
            return this.leftRotate(node);
        }

        return node;
    }

    insert(value: T): void {
        this._root = this.insertHelper(this._root, value);
    }

    private getMinValueNode(node: AVLTreeNode<T>): AVLTreeNode<T> {
        let current = node;
        while (current.left) {
            current = current.left;
        }
        return current;
    }

    private deleteHelper(node: AVLTreeNode<T> | null, value: T): AVLTreeNode<T> | null {
        if (!node) {
            return null;
        }

        if (value < node.value) {
            node.left = this.deleteHelper(node.left, value);
        } else if (value > node.value) {
            node.right = this.deleteHelper(node.right, value);
        } else {
            if (!node.left || !node.right) {
                const temp = node.left ? node.left : node.right;
                if (!temp) {
                    return null;
                } else {
                    node = temp;
                }
            } else {
                const temp = this.getMinValueNode(node.right);
                node.value = temp.value;
                node.right = this.deleteHelper(node.right, temp.value);
            }
        }

        if (!node) {
            return null;
        }

        node.height = Math.max(this.getHeight(node.left), this.getHeight(node.right)) + 1;

        const balance = this.getBalance(node);

        if (balance > 1 && this.getBalance(node.left) >= 0) {
            return this.rightRotate(node);
        }

        if (balance > 1 && this.getBalance(node.left) < 0) {
            node.left = this.leftRotate(node.left!);
            return this.rightRotate(node);
        }

        if (balance < -1 && this.getBalance(node.right) <= 0) {
            return this.leftRotate(node);
        }

        if (balance < -1 && this.getBalance(node.right) > 0) {
            node.right = this.rightRotate(node.right!);
            return this.leftRotate(node);
        }

        return node;
    }

    delete(value: T): boolean {
        if (!this.search(value)) {
            return false;
        }
        this._root = this.deleteHelper(this._root, value);
        return true;
    }

    search(value: T): boolean {
        let current = this._root;
        while (current) {
            if (value < current.value) {
                current = current.left;
            } else if (value > current.value) {
                current = current.right;
            } else {
                return true;
            }
        }
        return false;
    }

    preOrderTraversal(): T[] {
        const result: T[] = [];
        this.preOrderHelper(this._root, result);
        return result;
    }

    private preOrderHelper(node: AVLTreeNode<T> | null, result: T[]): void {
        if (node) {
            result.push(node.value);
            this.preOrderHelper(node.left, result);
            this.preOrderHelper(node.right, result);
        }
    }

    inOrderTraversal(): T[] {
        const result: T[] = [];
        this.inOrderHelper(this._root, result);
        return result;
    }

    private inOrderHelper(node: AVLTreeNode<T> | null, result: T[]): void {
        if (node) {
            this.inOrderHelper(node.left, result);
            result.push(node.value);
            this.inOrderHelper(node.right, result);
        }
    }

    postOrderTraversal(): T[] {
        const result: T[] = [];
        this.postOrderHelper(this._root, result);
        return result;
    }

    private postOrderHelper(node: AVLTreeNode<T> | null, result: T[]): void {
        if (node) {
            this.postOrderHelper(node.left, result);
            this.postOrderHelper(node.right, result);
            result.push(node.value);
        }
    }

    levelOrderTraversal(): T[] {
        const result: T[] = [];
        if (!this._root) {
            return result;
        }

        const queue: AVLTreeNode<T>[] = [this._root];
        while (queue.length > 0) {
            const node = queue.shift()!;
            result.push(node.value);
            if (node.left) {
                queue.push(node.left);
            }
            if (node.right) {
                queue.push(node.right);
            }
        }
        return result;
    }

    height(): number {
        return this.getHeight(this._root);
    }

    clear(): void {
        this._root = null;
    }
}

enum Color {
    RED,
    BLACK
}

class RBTreeNode<T extends number | string> {
    value: T | null;
    left: RBTreeNode<T>;
    right: RBTreeNode<T>;
    parent: RBTreeNode<T>;
    color: Color;

    constructor(value: T | null) {
        this.value = value;
        this.left = this as any;
        this.right = this as any;
        this.parent = this as any;
        this.color = Color.RED;
    }
}

export class RedBlackTree<T extends number | string> {
    private _root: RBTreeNode<T>;
    private _TNULL: RBTreeNode<T>;

    constructor() {
        this._TNULL = new RBTreeNode<T>(null);
        this._TNULL.color = Color.BLACK;
        this._TNULL.left = this._TNULL;
        this._TNULL.right = this._TNULL;
        this._TNULL.parent = this._TNULL;
        this._root = this._TNULL;
    }

    get root(): T | undefined {
        if (this._root === this._TNULL) {
            return undefined;
        }
        return this._root.value as T;
    }

    get isEmpty(): boolean {
        return this._root === this._TNULL;
    }

    private leftRotate(x: RBTreeNode<T>): void {
        const y = x.right;
        x.right = y.left;
        if (y.left !== this._TNULL) {
            y.left.parent = x;
        }
        y.parent = x.parent;
        if (x.parent === this._TNULL) {
            this._root = y;
        } else if (x === x.parent.left) {
            x.parent.left = y;
        } else {
            x.parent.right = y;
        }
        y.left = x;
        x.parent = y;
    }

    private rightRotate(x: RBTreeNode<T>): void {
        const y = x.left;
        x.left = y.right;
        if (y.right !== this._TNULL) {
            y.right.parent = x;
        }
        y.parent = x.parent;
        if (x.parent === this._TNULL) {
            this._root = y;
        } else if (x === x.parent.right) {
            x.parent.right = y;
        } else {
            x.parent.left = y;
        }
        y.right = x;
        x.parent = y;
    }

    private insertFixup(k: RBTreeNode<T>): void {
        while (k.parent.color === Color.RED) {
            if (k.parent === k.parent.parent.left) {
                const u = k.parent.parent.right;
                if (u.color === Color.RED) {
                    k.parent.color = Color.BLACK;
                    u.color = Color.BLACK;
                    k.parent.parent.color = Color.RED;
                    k = k.parent.parent;
                } else {
                    if (k === k.parent.right) {
                        k = k.parent;
                        this.leftRotate(k);
                    }
                    k.parent.color = Color.BLACK;
                    k.parent.parent.color = Color.RED;
                    this.rightRotate(k.parent.parent);
                }
            } else {
                const u = k.parent.parent.left;
                if (u.color === Color.RED) {
                    k.parent.color = Color.BLACK;
                    u.color = Color.BLACK;
                    k.parent.parent.color = Color.RED;
                    k = k.parent.parent;
                } else {
                    if (k === k.parent.left) {
                        k = k.parent;
                        this.rightRotate(k);
                    }
                    k.parent.color = Color.BLACK;
                    k.parent.parent.color = Color.RED;
                    this.leftRotate(k.parent.parent);
                }
            }
            if (k === this._root) {
                break;
            }
        }
        this._root.color = Color.BLACK;
    }

    private insertHelper(node: RBTreeNode<T>, value: T): void {
        let y = this._TNULL;
        let x = this._root;

        while (x !== this._TNULL) {
            y = x;
            const xValue = x.value as T;
            if (value < xValue) {
                x = x.left;
            } else if (value > xValue) {
                x = x.right;
            } else {
                return;
            }
        }

        node.parent = y;
        if (y === this._TNULL) {
            this._root = node;
        } else {
            const yValue = y.value as T;
            if (value < yValue) {
                y.left = node;
            } else {
                y.right = node;
            }
        }

        node.left = this._TNULL;
        node.right = this._TNULL;
        node.color = Color.RED;

        this.insertFixup(node);
    }

    insert(value: T): void {
        const node = new RBTreeNode(value);
        this.insertHelper(node, value);
    }

    private transplant(u: RBTreeNode<T>, v: RBTreeNode<T>): void {
        if (u.parent === this._TNULL) {
            this._root = v;
        } else if (u === u.parent.left) {
            u.parent.left = v;
        } else {
            u.parent.right = v;
        }
        v.parent = u.parent;
    }

    private minimum(node: RBTreeNode<T>): RBTreeNode<T> {
        while (node.left !== this._TNULL) {
            node = node.left;
        }
        return node;
    }

    private deleteFixup(x: RBTreeNode<T>): void {
        while (x !== this._root && x.color === Color.BLACK) {
            if (x === x.parent.left) {
                let w = x.parent.right;
                if (w.color === Color.RED) {
                    w.color = Color.BLACK;
                    x.parent.color = Color.RED;
                    this.leftRotate(x.parent);
                    w = x.parent.right;
                }
                if (w.left.color === Color.BLACK && w.right.color === Color.BLACK) {
                    w.color = Color.RED;
                    x = x.parent;
                } else {
                    if (w.right.color === Color.BLACK) {
                        w.left.color = Color.BLACK;
                        w.color = Color.RED;
                        this.rightRotate(w);
                        w = x.parent.right;
                    }
                    w.color = x.parent.color;
                    x.parent.color = Color.BLACK;
                    w.right.color = Color.BLACK;
                    this.leftRotate(x.parent);
                    x = this._root;
                }
            } else {
                let w = x.parent.left;
                if (w.color === Color.RED) {
                    w.color = Color.BLACK;
                    x.parent.color = Color.RED;
                    this.rightRotate(x.parent);
                    w = x.parent.left;
                }
                if (w.right.color === Color.BLACK && w.left.color === Color.BLACK) {
                    w.color = Color.RED;
                    x = x.parent;
                } else {
                    if (w.left.color === Color.BLACK) {
                        w.right.color = Color.BLACK;
                        w.color = Color.RED;
                        this.leftRotate(w);
                        w = x.parent.left;
                    }
                    w.color = x.parent.color;
                    x.parent.color = Color.BLACK;
                    w.left.color = Color.BLACK;
                    this.rightRotate(x.parent);
                    x = this._root;
                }
            }
        }
        x.color = Color.BLACK;
    }

    private deleteNodeHelper(_node: RBTreeNode<T>, value: T): void {
        let z = this._TNULL;
        let x, y;
        let current = this._root;

        while (current !== this._TNULL) {
            const currentValue = current.value as T;
            if (currentValue === value) {
                z = current;
            }
            if (currentValue <= value) {
                current = current.right;
            } else {
                current = current.left;
            }
        }

        if (z === this._TNULL) {
            return;
        }

        y = z;
        let yOriginalColor = y.color;

        if (z.left === this._TNULL) {
            x = z.right;
            this.transplant(z, z.right);
        } else if (z.right === this._TNULL) {
            x = z.left;
            this.transplant(z, z.left);
        } else {
            y = this.minimum(z.right);
            yOriginalColor = y.color;
            x = y.right;
            if (y.parent === z) {
                x.parent = y;
            } else {
                this.transplant(y, y.right);
                y.right = z.right;
                y.right.parent = y;
            }
            this.transplant(z, y);
            y.left = z.left;
            y.left.parent = y;
            y.color = z.color;
        }

        if (yOriginalColor === Color.BLACK) {
            this.deleteFixup(x);
        }
    }

    delete(value: T): boolean {
        if (!this.search(value)) {
            return false;
        }
        this.deleteNodeHelper(this._root, value);
        return true;
    }

    search(value: T): boolean {
        let current = this._root;
        while (current !== this._TNULL) {
            const currentValue = current.value as T;
            if (value < currentValue) {
                current = current.left;
            } else if (value > currentValue) {
                current = current.right;
            } else {
                return true;
            }
        }
        return false;
    }

    findMin(): T | undefined {
        if (this._root === this._TNULL) {
            return undefined;
        }
        let current = this._root;
        while (current.left !== this._TNULL) {
            current = current.left;
        }
        return current.value as T;
    }

    findMax(): T | undefined {
        if (this._root === this._TNULL) {
            return undefined;
        }
        let current = this._root;
        while (current.right !== this._TNULL) {
            current = current.right;
        }
        return current.value as T;
    }

    private preOrderHelper(node: RBTreeNode<T>, result: T[]): void {
        if (node !== this._TNULL) {
            result.push(node.value as T);
            this.preOrderHelper(node.left, result);
            this.preOrderHelper(node.right, result);
        }
    }

    preOrderTraversal(): T[] {
        const result: T[] = [];
        this.preOrderHelper(this._root, result);
        return result;
    }

    private inOrderHelper(node: RBTreeNode<T>, result: T[]): void {
        if (node !== this._TNULL) {
            this.inOrderHelper(node.left, result);
            result.push(node.value as T);
            this.inOrderHelper(node.right, result);
        }
    }

    inOrderTraversal(): T[] {
        const result: T[] = [];
        this.inOrderHelper(this._root, result);
        return result;
    }

    private postOrderHelper(node: RBTreeNode<T>, result: T[]): void {
        if (node !== this._TNULL) {
            this.postOrderHelper(node.left, result);
            this.postOrderHelper(node.right, result);
            result.push(node.value as T);
        }
    }

    postOrderTraversal(): T[] {
        const result: T[] = [];
        this.postOrderHelper(this._root, result);
        return result;
    }

    levelOrderTraversal(): T[] {
        const result: T[] = [];
        if (this._root === this._TNULL) {
            return result;
        }
        const queue: RBTreeNode<T>[] = [this._root];
        while (queue.length > 0) {
            const node = queue.shift()!;
            result.push(node.value as T);
            if (node.left !== this._TNULL) {
                queue.push(node.left);
            }
            if (node.right !== this._TNULL) {
                queue.push(node.right);
            }
        }
        return result;
    }

    clear(): void {
        this._root = this._TNULL;
    }
}

console.log("Red-Black Tree Example");
const rbt = new RedBlackTree<number>();
console.log("Inserting: 10, 5, 15, 3, 7, 12, 18");
rbt.insert(10);
rbt.insert(5);
rbt.insert(15);
rbt.insert(3);
rbt.insert(7);
rbt.insert(12);
rbt.insert(18);
console.log(`In-order traversal: [${rbt.inOrderTraversal().join(", ")}]`);
console.log(`Pre-order traversal: [${rbt.preOrderTraversal().join(", ")}]`);
console.log(`Level-order traversal: [${rbt.levelOrderTraversal().join(", ")}]`);
console.log(`Search 7: ${rbt.search(7) ? "Found" : "Not found"}`);
console.log(`Min: ${rbt.findMin()}, Max: ${rbt.findMax()}`);
console.log("\nDelete 15");
rbt.delete(15);
console.log(`In-order traversal after delete: [${rbt.inOrderTraversal().join(", ")}]`);
