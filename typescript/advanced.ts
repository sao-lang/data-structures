class CircularSinglyListNode<T> {
    value: T;
    next: CircularSinglyListNode<T> | null;

    constructor(value: T) {
        this.value = value;
        this.next = null;
    }
}

export class CircularSinglyLinkedList<T> {
    private _head: CircularSinglyListNode<T> | null;
    private _length: number;

    constructor() {
        this._head = null;
        this._length = 0;
    }

    get length(): number {
        return this._length;
    }

    get isEmpty(): boolean {
        return this._length === 0;
    }

    get head(): T | undefined {
        return this._head?.value;
    }

    prepend(value: T): void {
        const newNode = new CircularSinglyListNode(value);
        if (!this._head) {
            this._head = newNode;
            newNode.next = this._head;
        } else {
            let current = this._head;
            while (current.next !== this._head) {
                current = current.next!;
            }
            newNode.next = this._head;
            current.next = newNode;
            this._head = newNode;
        }
        this._length++;
    }

    append(value: T): void {
        const newNode = new CircularSinglyListNode(value);
        if (!this._head) {
            this._head = newNode;
            newNode.next = this._head;
        } else {
            let current = this._head;
            while (current.next !== this._head) {
                current = current.next!;
            }
            current.next = newNode;
            newNode.next = this._head;
        }
        this._length++;
    }

    removeFirst(): T | undefined {
        if (!this._head) {
            return undefined;
        }
        const removedNode = this._head;
        if (this._length === 1) {
            this._head = null;
        } else {
            let current = this._head;
            while (current.next !== this._head) {
                current = current.next!;
            }
            this._head = this._head.next;
            current.next = this._head;
        }
        this._length--;
        return removedNode.value;
    }

    removeLast(): T | undefined {
        if (!this._head) {
            return undefined;
        }
        if (this._length === 1) {
            const value = this._head.value;
            this._head = null;
            this._length--;
            return value;
        }
        let current = this._head;
        let prev: CircularSinglyListNode<T> | null = null;
        while (current.next !== this._head) {
            prev = current;
            current = current.next!;
        }
        const removedNode = current;
        prev!.next = this._head;
        this._length--;
        return removedNode.value;
    }

    toArray(): T[] {
        const result: T[] = [];
        if (!this._head) {
            return result;
        }
        let current = this._head;
        do {
            result.push(current.value);
            current = current.next!;
        } while (current !== this._head);
        return result;
    }

    clear(): void {
        this._head = null;
        this._length = 0;
    }
}

class CircularDoublyListNode<T> {
    value: T;
    next: CircularDoublyListNode<T> | null;
    prev: CircularDoublyListNode<T> | null;

    constructor(value: T) {
        this.value = value;
        this.next = null;
        this.prev = null;
    }
}

export class CircularDoublyLinkedList<T> {
    private _head: CircularDoublyListNode<T> | null;
    private _length: number;

    constructor() {
        this._head = null;
        this._length = 0;
    }

    get length(): number {
        return this._length;
    }

    get isEmpty(): boolean {
        return this._length === 0;
    }

    get head(): T | undefined {
        return this._head?.value;
    }

    get tail(): T | undefined {
        return this._head?.prev?.value;
    }

    prepend(value: T): void {
        const newNode = new CircularDoublyListNode(value);
        if (!this._head) {
            this._head = newNode;
            newNode.next = newNode;
            newNode.prev = newNode;
        } else {
            const tail = this._head.prev!;
            newNode.next = this._head;
            newNode.prev = tail;
            tail.next = newNode;
            this._head.prev = newNode;
            this._head = newNode;
        }
        this._length++;
    }

    append(value: T): void {
        const newNode = new CircularDoublyListNode(value);
        if (!this._head) {
            this._head = newNode;
            newNode.next = newNode;
            newNode.prev = newNode;
        } else {
            const tail = this._head.prev!;
            newNode.prev = tail;
            newNode.next = this._head;
            tail.next = newNode;
            this._head.prev = newNode;
        }
        this._length++;
    }

    removeFirst(): T | undefined {
        if (!this._head) {
            return undefined;
        }
        const removedNode = this._head;
        if (this._length === 1) {
            this._head = null;
        } else {
            const tail = this._head.prev!;
            this._head = this._head.next;
            tail.next = this._head;
            this._head!.prev = tail;
        }
        this._length--;
        return removedNode.value;
    }

    removeLast(): T | undefined {
        if (!this._head) {
            return undefined;
        }
        const tail = this._head.prev!;
        if (this._length === 1) {
            this._head = null;
        } else {
            const newTail = tail.prev!;
            newTail.next = this._head;
            this._head.prev = newTail;
        }
        this._length--;
        return tail.value;
    }

    toArray(): T[] {
        const result: T[] = [];
        if (!this._head) {
            return result;
        }
        let current = this._head;
        do {
            result.push(current.value);
            current = current.next!;
        } while (current !== this._head);
        return result;
    }

    toArrayReverse(): T[] {
        const result: T[] = [];
        if (!this._head) {
            return result;
        }
        let current = this._head.prev!;
        do {
            result.push(current.value);
            current = current.prev!;
        } while (current !== this._head.prev);
        return result;
    }

    clear(): void {
        this._head = null;
        this._length = 0;
    }
}

class TrieNode {
    children: { [key: string]: TrieNode };
    isEndOfWord: boolean;

    constructor() {
        this.children = {};
        this.isEndOfWord = false;
    }
}

export class Trie {
    private root: TrieNode;

    constructor() {
        this.root = new TrieNode();
    }

    insert(word: string): void {
        let current = this.root;
        for (const char of word) {
            if (!current.children[char]) {
                current.children[char] = new TrieNode();
            }
            current = current.children[char];
        }
        current.isEndOfWord = true;
    }

    search(word: string): boolean {
        let current = this.root;
        for (const char of word) {
            if (!current.children[char]) {
                return false;
            }
            current = current.children[char];
        }
        return current.isEndOfWord;
    }

    startsWith(prefix: string): boolean {
        let current = this.root;
        for (const char of prefix) {
            if (!current.children[char]) {
                return false;
            }
            current = current.children[char];
        }
        return true;
    }

    delete(word: string): void {
        this.deleteHelper(this.root, word, 0);
    }

    private deleteHelper(node: TrieNode, word: string, index: number): boolean {
        if (index === word.length) {
            if (!node.isEndOfWord) {
                return false;
            }
            node.isEndOfWord = false;
            return Object.keys(node.children).length === 0;
        }

        const char = word[index];
        if (!node.children[char]) {
            return false;
        }

        const shouldDeleteChild = this.deleteHelper(node.children[char], word, index + 1);

        if (shouldDeleteChild) {
            delete node.children[char];
            return Object.keys(node.children).length === 0 && !node.isEndOfWord;
        }

        return false;
    }

    getAllWords(): string[] {
        const words: string[] = [];
        this.getAllWordsHelper(this.root, '', words);
        return words;
    }

    private getAllWordsHelper(node: TrieNode, prefix: string, words: string[]): void {
        if (node.isEndOfWord) {
            words.push(prefix);
        }
        for (const char in node.children) {
            this.getAllWordsHelper(node.children[char], prefix + char, words);
        }
    }

    getWordsWithPrefix(prefix: string): string[] {
        let current = this.root;
        for (const char of prefix) {
            if (!current.children[char]) {
                return [];
            }
            current = current.children[char];
        }
        const words: string[] = [];
        this.getAllWordsHelper(current, prefix, words);
        return words;
    }
}

class GraphNode<T> {
    value: T;
    neighbors: GraphNode<T>[];

    constructor(value: T) {
        this.value = value;
        this.neighbors = [];
    }
}

export class Graph<T> {
    private nodes: Map<T, GraphNode<T>>;
    private isDirected: boolean;

    constructor(isDirected: boolean = false) {
        this.nodes = new Map();
        this.isDirected = isDirected;
    }

    addVertex(value: T): void {
        if (!this.nodes.has(value)) {
            this.nodes.set(value, new GraphNode(value));
        }
    }

    addEdge(from: T, to: T): void {
        this.addVertex(from);
        this.addVertex(to);

        const fromNode = this.nodes.get(from)!;
        const toNode = this.nodes.get(to)!;

        fromNode.neighbors.push(toNode);
        if (!this.isDirected) {
            toNode.neighbors.push(fromNode);
        }
    }

    removeVertex(value: T): void {
        const node = this.nodes.get(value);
        if (!node) return;

        for (const neighbor of node.neighbors) {
            const index = neighbor.neighbors.indexOf(node);
            if (index !== -1) {
                neighbor.neighbors.splice(index, 1);
            }
        }

        this.nodes.delete(value);
    }

    removeEdge(from: T, to: T): void {
        const fromNode = this.nodes.get(from);
        const toNode = this.nodes.get(to);

        if (!fromNode || !toNode) return;

        const fromIndex = fromNode.neighbors.indexOf(toNode);
        if (fromIndex !== -1) {
            fromNode.neighbors.splice(fromIndex, 1);
        }

        if (!this.isDirected) {
            const toIndex = toNode.neighbors.indexOf(fromNode);
            if (toIndex !== -1) {
                toNode.neighbors.splice(toIndex, 1);
            }
        }
    }

    bfs(start: T): T[] {
        const result: T[] = [];
        const visited = new Set<T>();
        const startNode = this.nodes.get(start);

        if (!startNode) return result;

        const queue: GraphNode<T>[] = [startNode];
        visited.add(start);

        while (queue.length > 0) {
            const current = queue.shift()!;
            result.push(current.value);

            for (const neighbor of current.neighbors) {
                if (!visited.has(neighbor.value)) {
                    visited.add(neighbor.value);
                    queue.push(neighbor);
                }
            }
        }

        return result;
    }

    dfs(start: T): T[] {
        const result: T[] = [];
        const visited = new Set<T>();
        const startNode = this.nodes.get(start);

        if (!startNode) return result;

        const stack: GraphNode<T>[] = [startNode];
        visited.add(start);

        while (stack.length > 0) {
            const current = stack.pop()!;
            result.push(current.value);

            for (let i = current.neighbors.length - 1; i >= 0; i--) {
                const neighbor = current.neighbors[i];
                if (!visited.has(neighbor.value)) {
                    visited.add(neighbor.value);
                    stack.push(neighbor);
                }
            }
        }

        return result;
    }

    getVertices(): T[] {
        return Array.from(this.nodes.keys());
    }

    hasVertex(value: T): boolean {
        return this.nodes.has(value);
    }

    hasEdge(from: T, to: T): boolean {
        const fromNode = this.nodes.get(from);
        const toNode = this.nodes.get(to);
        if (!fromNode || !toNode) return false;
        return fromNode.neighbors.some(n => n.value === to);
    }
}

export class UnionFind {
    private parent: number[];
    private rank: number[];

    constructor(size: number) {
        this.parent = Array.from({ length: size }, (_, i) => i);
        this.rank = Array(size).fill(0);
    }

    find(x: number): number {
        if (this.parent[x] !== x) {
            this.parent[x] = this.find(this.parent[x]);
        }
        return this.parent[x];
    }

    union(x: number, y: number): boolean {
        const rootX = this.find(x);
        const rootY = this.find(y);

        if (rootX === rootY) {
            return false;
        }

        if (this.rank[rootX] < this.rank[rootY]) {
            this.parent[rootX] = rootY;
        } else if (this.rank[rootX] > this.rank[rootY]) {
            this.parent[rootY] = rootX;
        } else {
            this.parent[rootY] = rootX;
            this.rank[rootX]++;
        }

        return true;
    }

    connected(x: number, y: number): boolean {
        return this.find(x) === this.find(y);
    }

    getCount(): number {
        const roots = new Set<number>();
        for (let i = 0; i < this.parent.length; i++) {
            roots.add(this.find(i));
        }
        return roots.size;
    }
}

class SkipListNode<T> {
    value: T;
    forward: SkipListNode<T>[];

    constructor(value: T, level: number) {
        this.value = value;
        this.forward = Array(level + 1).fill(null);
    }
}

export class SkipList<T extends number | string> {
    private static readonly MAX_LEVEL = 16;
    private static readonly P = 0.5;

    private head: SkipListNode<T>;
    private level: number;

    constructor() {
        this.level = 0;
        this.head = new SkipListNode<T>(null as any, SkipList.MAX_LEVEL);
    }

    private randomLevel(): number {
        let level = 0;
        while (Math.random() < SkipList.P && level < SkipList.MAX_LEVEL - 1) {
            level++;
        }
        return level;
    }

    search(value: T): boolean {
        let current = this.head;

        for (let i = this.level; i >= 0; i--) {
            while (current.forward[i] && current.forward[i].value < value) {
                current = current.forward[i];
            }
        }

        current = current.forward[0];
        return current !== null && current.value === value;
    }

    insert(value: T): void {
        const update: SkipListNode<T>[] = Array(SkipList.MAX_LEVEL).fill(null);
        let current = this.head;

        for (let i = this.level; i >= 0; i--) {
            while (current.forward[i] && current.forward[i].value < value) {
                current = current.forward[i];
            }
            update[i] = current;
        }

        current = current.forward[0];

        if (!current || current.value !== value) {
            const newLevel = this.randomLevel();

            if (newLevel > this.level) {
                for (let i = this.level + 1; i <= newLevel; i++) {
                    update[i] = this.head;
                }
                this.level = newLevel;
            }

            const newNode = new SkipListNode(value, newLevel);

            for (let i = 0; i <= newLevel; i++) {
                newNode.forward[i] = update[i].forward[i];
                update[i].forward[i] = newNode;
            }
        }
    }

    delete(value: T): boolean {
        const update: SkipListNode<T>[] = Array(SkipList.MAX_LEVEL).fill(null);
        let current = this.head;

        for (let i = this.level; i >= 0; i--) {
            while (current.forward[i] && current.forward[i].value < value) {
                current = current.forward[i];
            }
            update[i] = current;
        }

        current = current.forward[0];

        if (current && current.value === value) {
            for (let i = 0; i <= this.level; i++) {
                if (update[i].forward[i] !== current) {
                    break;
                }
                update[i].forward[i] = current.forward[i];
            }

            while (this.level > 0 && !this.head.forward[this.level]) {
                this.level--;
            }

            return true;
        }

        return false;
    }

    toArray(): T[] {
        const result: T[] = [];
        let current = this.head.forward[0];
        while (current) {
            result.push(current.value);
            current = current.forward[0];
        }
        return result;
    }
}

export class SegmentTree<T> {
    private n: number;
    private size: number;
    private tree: T[];
    private merge: (a: T, b: T) => T;
    private defaultValue: T;

    constructor(data: T[], merge: (a: T, b: T) => T, defaultValue: T) {
        this.n = data.length;
        this.merge = merge;
        this.defaultValue = defaultValue;
        
        this.size = 1;
        while (this.size < this.n) {
            this.size <<= 1;
        }
        
        this.tree = new Array(2 * this.size).fill(defaultValue);
        
        for (let i = 0; i < this.n; i++) {
            this.tree[this.size + i] = data[i];
        }
        
        for (let i = this.size - 1; i > 0; i--) {
            this.tree[i] = this.merge(this.tree[2 * i], this.tree[2 * i + 1]);
        }
    }

    update(index: number, value: T): void {
        if (index < 0 || index >= this.n) {
            throw new RangeError('Index out of bounds');
        }
        
        index += this.size;
        this.tree[index] = value;
        index >>= 1;
        
        while (index >= 1) {
            const newVal = this.merge(this.tree[2 * index], this.tree[2 * index + 1]);
            if (this.tree[index] === newVal) break;
            this.tree[index] = newVal;
            index >>= 1;
        }
    }

    query(l: number, r: number): T {
        if (l < 0 || r >= this.n || l > r) {
            throw new RangeError('Invalid query range');
        }
        
        let resLeft = this.defaultValue;
        let resRight = this.defaultValue;
        l += this.size;
        r += this.size;
        
        while (l <= r) {
            if (l % 2 === 1) {
                resLeft = this.merge(resLeft, this.tree[l]);
                l++;
            }
            if (r % 2 === 0) {
                resRight = this.merge(this.tree[r], resRight);
                r--;
            }
            l >>= 1;
            r >>= 1;
        }
        
        return this.merge(resLeft, resRight);
    }

    get(index: number): T {
        if (index < 0 || index >= this.n) {
            throw new RangeError('Index out of bounds');
        }
        return this.tree[this.size + index];
    }
}

export class FenwickTree {
    private tree: number[];
    private n: number;

    constructor(size: number);
    constructor(data: number[]);
    constructor(arg: number | number[]) {
        if (typeof arg === 'number') {
            this.n = arg;
            this.tree = new Array(this.n + 1).fill(0);
        } else {
            this.n = arg.length;
            this.tree = new Array(this.n + 1).fill(0);
            for (let i = 0; i < this.n; i++) {
                this.update(i, arg[i]);
            }
        }
    }

    update(index: number, delta: number): void {
        if (index < 0 || index >= this.n) {
            throw new RangeError('Index out of bounds');
        }
        index++;
        while (index <= this.n) {
            this.tree[index] += delta;
            index += index & -index;
        }
    }

    set(index: number, value: number): void {
        const current = this.query(index, index);
        this.update(index, value - current);
    }

    prefixSum(index: number): number {
        if (index < 0 || index >= this.n) {
            throw new RangeError('Index out of bounds');
        }
        index++;
        let sum = 0;
        while (index > 0) {
            sum += this.tree[index];
            index -= index & -index;
        }
        return sum;
    }

    query(l: number, r: number): number {
        if (l < 0 || r >= this.n || l > r) {
            throw new RangeError('Invalid query range');
        }
        if (l === 0) {
            return this.prefixSum(r);
        }
        return this.prefixSum(r) - this.prefixSum(l - 1);
    }

    get size(): number {
        return this.n;
    }
}

export class BloomFilter {
    private bitArray: Uint8Array;
    private size: number;
    private numHashFunctions: number;

    constructor(expectedItems: number, falsePositiveRate: number = 0.01) {
        this.size = this.calculateSize(expectedItems, falsePositiveRate);
        this.numHashFunctions = this.calculateNumHashFunctions(this.size, expectedItems);
        this.bitArray = new Uint8Array(Math.ceil(this.size / 8));
    }

    private calculateSize(n: number, p: number): number {
        return Math.ceil(-n * Math.log(p) / (Math.log(2) * Math.log(2)));
    }

    private calculateNumHashFunctions(m: number, n: number): number {
        return Math.max(1, Math.round((m / n) * Math.log(2)));
    }

    private hash(item: string, seed: number): number {
        let hash = seed;
        for (let i = 0; i < item.length; i++) {
            hash = (hash * 31 + item.charCodeAt(i)) % this.size;
        }
        return hash;
    }

    add(item: string): void {
        for (let i = 0; i < this.numHashFunctions; i++) {
            const hash = this.hash(item, i);
            const byteIndex = Math.floor(hash / 8);
            const bitIndex = hash % 8;
            this.bitArray[byteIndex] |= (1 << bitIndex);
        }
    }

    mightContain(item: string): boolean {
        for (let i = 0; i < this.numHashFunctions; i++) {
            const hash = this.hash(item, i);
            const byteIndex = Math.floor(hash / 8);
            const bitIndex = hash % 8;
            if (!(this.bitArray[byteIndex] & (1 << bitIndex))) {
                return false;
            }
        }
        return true;
    }

    clear(): void {
        this.bitArray.fill(0);
    }
}

class LRUCacheNode<K, V> {
    key: K;
    value: V;
    prev: LRUCacheNode<K, V> | null;
    next: LRUCacheNode<K, V> | null;

    constructor(key: K, value: V) {
        this.key = key;
        this.value = value;
        this.prev = null;
        this.next = null;
    }
}

export class LRUCache<K, V> {
    private capacity: number;
    private cache: Map<K, LRUCacheNode<K, V>>;
    private head: LRUCacheNode<K, V>;
    private tail: LRUCacheNode<K, V>;

    constructor(capacity: number) {
        if (capacity <= 0) {
            throw new Error('Capacity must be positive');
        }
        this.capacity = capacity;
        this.cache = new Map();
        this.head = new LRUCacheNode<K, V>(null as K, null as V);
        this.tail = new LRUCacheNode<K, V>(null as K, null as V);
        this.head.next = this.tail;
        this.tail.prev = this.head;
    }

    private addToHead(node: LRUCacheNode<K, V>): void {
        node.prev = this.head;
        node.next = this.head.next;
        this.head.next!.prev = node;
        this.head.next = node;
    }

    private removeNode(node: LRUCacheNode<K, V>): void {
        node.prev!.next = node.next;
        node.next!.prev = node.prev;
    }

    private moveToHead(node: LRUCacheNode<K, V>): void {
        this.removeNode(node);
        this.addToHead(node);
    }

    private removeTail(): LRUCacheNode<K, V> {
        const node = this.tail.prev!;
        this.removeNode(node);
        return node;
    }

    get(key: K): V | undefined {
        const node = this.cache.get(key);
        if (!node) {
            return undefined;
        }
        this.moveToHead(node);
        return node.value;
    }

    put(key: K, value: V): void {
        let node = this.cache.get(key);
        if (node) {
            node.value = value;
            this.moveToHead(node);
        } else {
            node = new LRUCacheNode(key, value);
            this.cache.set(key, node);
            this.addToHead(node);
            if (this.cache.size > this.capacity) {
                const tail = this.removeTail();
                this.cache.delete(tail.key);
            }
        }
    }

    has(key: K): boolean {
        return this.cache.has(key);
    }

    delete(key: K): boolean {
        const node = this.cache.get(key);
        if (!node) {
            return false;
        }
        this.removeNode(node);
        this.cache.delete(key);
        return true;
    }

    clear(): void {
        this.cache.clear();
        this.head.next = this.tail;
        this.tail.prev = this.head;
    }

    get size(): number {
        return this.cache.size;
    }

    keys(): K[] {
        const keys: K[] = [];
        let current = this.head.next;
        while (current !== this.tail) {
            keys.push(current!.key);
            current = current!.next;
        }
        return keys;
    }

    values(): V[] {
        const values: V[] = [];
        let current = this.head.next;
        while (current !== this.tail) {
            values.push(current!.value);
            current = current!.next;
        }
        return values;
    }
}

export class SuffixArray {
    private text: string;
    private suffixArray: number[];
    private lcpArray: number[] | null;

    constructor(text: string) {
        this.text = text;
        this.suffixArray = this.buildSuffixArray(text);
        this.lcpArray = null;
    }

    private buildSuffixArray(s: string): number[] {
        const n = s.length;
        let sa = Array.from({ length: n }, (_, i) => i);
        let rank = Array.from(s);
        let k = 1;

        while (k < n) {
            sa.sort((a, b) => {
                if (rank[a] !== rank[b]) {
                    return rank[a] < rank[b] ? -1 : 1;
                }
                const ra = a + k < n ? rank[a + k] : -1;
                const rb = b + k < n ? rank[b + k] : -1;
                return ra < rb ? -1 : 1;
            });

            const newRank = new Array(n).fill(0);
            newRank[sa[0]] = 0;
            for (let i = 1; i < n; i++) {
                const prev = sa[i - 1];
                const curr = sa[i];
                const same = rank[prev] === rank[curr] &&
                    (prev + k < n ? rank[prev + k] : -1) === (curr + k < n ? rank[curr + k] : -1);
                newRank[curr] = newRank[prev] + (same ? 0 : 1);
            }
            rank = newRank;
            k *= 2;
        }

        return sa;
    }

    getSuffixArray(): number[] {
        return [...this.suffixArray];
    }

    getSuffix(index: number): string {
        if (index < 0 || index >= this.text.length) {
            throw new RangeError('Index out of bounds');
        }
        return this.text.substring(index);
    }

    getLCPArray(): number[] {
        if (this.lcpArray === null) {
            this.lcpArray = this.buildLCPArray();
        }
        return [...this.lcpArray];
    }

    private buildLCPArray(): number[] {
        const n = this.text.length;
        const rank = new Array(n).fill(0);
        for (let i = 0; i < n; i++) {
            rank[this.suffixArray[i]] = i;
        }

        const lcp = new Array(n - 1).fill(0);
        let k = 0;
        for (let i = 0; i < n; i++) {
            if (rank[i] === n - 1) {
                k = 0;
                continue;
            }
            const j = this.suffixArray[rank[i] + 1];
            while (i + k < n && j + k < n && this.text[i + k] === this.text[j + k]) {
                k++;
            }
            lcp[rank[i]] = k;
            if (k > 0) k--;
        }
        return lcp;
    }

    search(pattern: string): number[] {
        const result: number[] = [];
        const m = pattern.length;
        const n = this.text.length;
        
        let low = 0;
        let high = n - 1;
        
        while (low <= high) {
            const mid = Math.floor((low + high) / 2);
            const suffix = this.getSuffix(this.suffixArray[mid]);
            const cmp = pattern.localeCompare(suffix.substring(0, Math.min(m, suffix.length)));
            
            if (cmp === 0) {
                result.push(this.suffixArray[mid]);
                let left = mid - 1;
                while (left >= 0) {
                    const leftSuffix = this.getSuffix(this.suffixArray[left]);
                    if (leftSuffix.startsWith(pattern)) {
                        result.push(this.suffixArray[left]);
                        left--;
                    } else {
                        break;
                    }
                }
                let right = mid + 1;
                while (right < n) {
                    const rightSuffix = this.getSuffix(this.suffixArray[right]);
                    if (rightSuffix.startsWith(pattern)) {
                        result.push(this.suffixArray[right]);
                        right++;
                    } else {
                        break;
                    }
                }
                break;
            } else if (cmp < 0) {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        
        return result.sort((a, b) => a - b);
    }

    getLongestCommonPrefix(): number {
        const lcp = this.getLCPArray();
        return lcp.length === 0 ? 0 : Math.max(...lcp);
    }

    getLongestRepeatedSubstring(): string {
        const lcp = this.getLCPArray();
        let maxLen = 0;
        let maxIndex = 0;
        
        for (let i = 0; i < lcp.length; i++) {
            if (lcp[i] > maxLen) {
                maxLen = lcp[i];
                maxIndex = i;
            }
        }
        
        if (maxLen === 0) return '';
        return this.text.substring(this.suffixArray[maxIndex], this.suffixArray[maxIndex] + maxLen);
    }
}

interface KDPoint {
    coordinates: number[];
}

class KDNode<T extends KDPoint> {
    point: T;
    left: KDNode<T> | null;
    right: KDNode<T> | null;
    axis: number;

    constructor(point: T, axis: number) {
        this.point = point;
        this.left = null;
        this.right = null;
        this.axis = axis;
    }
}

export class KDTree<T extends KDPoint> {
    private root: KDNode<T> | null;
    private dimensions: number;

    constructor(points: T[] = []) {
        if (points.length > 0) {
            this.dimensions = points[0].coordinates.length;
            this.root = this.buildTree(points, 0);
        } else {
            this.root = null;
            this.dimensions = 0;
        }
    }

    private buildTree(points: T[], depth: number): KDNode<T> | null {
        if (points.length === 0) return null;

        const axis = depth % this.dimensions;
        const sorted = [...points].sort((a, b) => a.coordinates[axis] - b.coordinates[axis]);
        const median = Math.floor(sorted.length / 2);

        const node = new KDNode(sorted[median], axis);
        node.left = this.buildTree(sorted.slice(0, median), depth + 1);
        node.right = this.buildTree(sorted.slice(median + 1), depth + 1);

        return node;
    }

    insert(point: T): void {
        if (this.root === null) {
            this.dimensions = point.coordinates.length;
            this.root = new KDNode(point, 0);
            return;
        }

        if (point.coordinates.length !== this.dimensions) {
            throw new Error(`Point must have ${this.dimensions} dimensions`);
        }

        let current = this.root;
        let depth = 0;

        while (true) {
            const axis = depth % this.dimensions;
            if (point.coordinates[axis] < current.point.coordinates[axis]) {
                if (current.left === null) {
                    current.left = new KDNode(point, (depth + 1) % this.dimensions);
                    break;
                }
                current = current.left;
            } else {
                if (current.right === null) {
                    current.right = new KDNode(point, (depth + 1) % this.dimensions);
                    break;
                }
                current = current.right;
            }
            depth++;
        }
    }

    private distanceSquared(a: number[], b: number[]): number {
        let sum = 0;
        for (let i = 0; i < a.length; i++) {
            const diff = a[i] - b[i];
            sum += diff * diff;
        }
        return sum;
    }

    nearestNeighbor(target: number[]): T | null {
        if (this.root === null || target.length !== this.dimensions) {
            return null;
        }

        let best: KDNode<T> | null = null;
        let bestDist = Infinity;

        const search = (node: KDNode<T> | null, depth: number): void => {
            if (node === null) return;

            const dist = this.distanceSquared(node.point.coordinates, target);
            if (dist < bestDist) {
                bestDist = dist;
                best = node;
            }

            const axis = depth % this.dimensions;
            const goLeft = target[axis] < node.point.coordinates[axis];

            search(goLeft ? node.left : node.right, depth + 1);

            const planeDist = (target[axis] - node.point.coordinates[axis]) ** 2;
            if (planeDist < bestDist) {
                search(goLeft ? node.right : node.left, depth + 1);
            }
        };

        search(this.root, 0);
        return best ? (best as KDNode<T>).point : null;
    }

    rangeSearch(min: number[], max: number[]): T[] {
        const result: T[] = [];
        
        if (this.root === null || min.length !== this.dimensions || max.length !== this.dimensions) {
            return result;
        }

        const search = (node: KDNode<T> | null): void => {
            if (node === null) return;

            const point = node.point.coordinates;
            let inRange = true;
            for (let i = 0; i < this.dimensions; i++) {
                if (point[i] < min[i] || point[i] > max[i]) {
                    inRange = false;
                    break;
                }
            }
            if (inRange) {
                result.push(node.point);
            }

            const axis = node.axis;
            if (min[axis] <= point[axis]) {
                search(node.left);
            }
            if (max[axis] >= point[axis]) {
                search(node.right);
            }
        };

        search(this.root);
        return result;
    }

    kNearestNeighbors(target: number[], k: number): T[] {
        if (k <= 0 || this.root === null || target.length !== this.dimensions) {
            return [];
        }

        const neighbors: { point: T; dist: number }[] = [];

        const search = (node: KDNode<T> | null, depth: number): void => {
            if (node === null) return;

            const dist = this.distanceSquared(node.point.coordinates, target);
            
            if (neighbors.length < k) {
                neighbors.push({ point: node.point, dist });
                neighbors.sort((a, b) => a.dist - b.dist);
            } else if (dist < neighbors[neighbors.length - 1].dist) {
                neighbors.pop();
                neighbors.push({ point: node.point, dist });
                neighbors.sort((a, b) => a.dist - b.dist);
            }

            const axis = depth % this.dimensions;
            const goLeft = target[axis] < node.point.coordinates[axis];

            search(goLeft ? node.left : node.right, depth + 1);

            const planeDist = (target[axis] - node.point.coordinates[axis]) ** 2;
            if (neighbors.length < k || planeDist < neighbors[neighbors.length - 1].dist) {
                search(goLeft ? node.right : node.left, depth + 1);
            }
        };

        search(this.root, 0);
        return neighbors.map(n => n.point);
    }
}
