import { DynamicArray } from './array';
import { SinglyLinkedList } from './linked-list';

export class Stack<T> {
    private _items: DynamicArray<T>;

    constructor() {
        this._items = new DynamicArray<T>();
    }

    get size(): number {
        return this._items.length;
    }

    get isEmpty(): boolean {
        return this._items.isEmpty;
    }

    push(item: T): void {
        this._items.push(item);
    }

    pop(): T | undefined {
        return this._items.pop();
    }

    peek(): T | undefined {
        if (this.isEmpty) {
            return undefined;
        }
        return this._items.at(this._items.length - 1);
    }

    clear(): void {
        this._items.clear();
    }

    toArray(): (T | undefined)[] {
        return this._items.toArray();
    }

    [Symbol.iterator](): Iterator<T | undefined> {
        return this._items[Symbol.iterator]();
    }
}

export class Queue<T> {
    private _items: SinglyLinkedList<T>;

    constructor() {
        this._items = new SinglyLinkedList<T>();
    }

    get size(): number {
        return this._items.length;
    }

    get isEmpty(): boolean {
        return this._items.isEmpty;
    }

    enqueue(item: T): void {
        this._items.append(item);
    }

    dequeue(): T | undefined {
        return this._items.removeFirst();
    }

    peek(): T | undefined {
        return this._items.head;
    }

    clear(): void {
        this._items.clear();
    }

    toArray(): T[] {
        return this._items.toArray();
    }

    [Symbol.iterator](): Iterator<T> {
        return this._items[Symbol.iterator]();
    }
}

export class CircularQueue<T> {
    private _capacity: number;
    private _items: (T | undefined)[];
    private _front: number;
    private _rear: number;
    private _size: number;

    constructor(capacity: number) {
        if (capacity <= 0) {
            throw new Error('Capacity must be positive');
        }
        this._capacity = capacity;
        this._items = new Array(capacity);
        this._front = 0;
        this._rear = -1;
        this._size = 0;
    }

    get capacity(): number {
        return this._capacity;
    }

    get size(): number {
        return this._size;
    }

    get isEmpty(): boolean {
        return this._size === 0;
    }

    get isFull(): boolean {
        return this._size === this._capacity;
    }

    enqueue(item: T): boolean {
        if (this.isFull) {
            return false;
        }
        this._rear = (this._rear + 1) % this._capacity;
        this._items[this._rear] = item;
        this._size++;
        return true;
    }

    dequeue(): T | undefined {
        if (this.isEmpty) {
            return undefined;
        }
        const item = this._items[this._front];
        this._items[this._front] = undefined;
        this._front = (this._front + 1) % this._capacity;
        this._size--;
        return item;
    }

    peek(): T | undefined {
        if (this.isEmpty) {
            return undefined;
        }
        return this._items[this._front];
    }

    clear(): void {
        this._items = new Array(this._capacity);
        this._front = 0;
        this._rear = -1;
        this._size = 0;
    }

    toArray(): (T | undefined)[] {
        const result: (T | undefined)[] = [];
        for (let i = 0; i < this._size; i++) {
            const index = (this._front + i) % this._capacity;
            result.push(this._items[index]);
        }
        return result;
    }

    [Symbol.iterator](): Iterator<T | undefined> {
        let i = 0;
        const size = this._size;
        const front = this._front;
        const capacity = this._capacity;
        const items = this._items;
        return {
            next(): IteratorResult<T | undefined> {
                if (i >= size) {
                    return { done: true, value: undefined };
                }
                const index = (front + i) % capacity;
                const value = items[index];
                i++;
                return { done: false, value };
            }
        };
    }
}

export class Deque<T> {
    private _items: DynamicArray<T>;

    constructor() {
        this._items = new DynamicArray<T>();
    }

    get size(): number {
        return this._items.length;
    }

    get isEmpty(): boolean {
        return this._items.isEmpty;
    }

    addFront(item: T): void {
        this._items.insert(0, item);
    }

    addRear(item: T): void {
        this._items.push(item);
    }

    removeFront(): T | undefined {
        if (this.isEmpty) {
            return undefined;
        }
        return this._items.remove(0);
    }

    removeRear(): T | undefined {
        return this._items.pop();
    }

    peekFront(): T | undefined {
        if (this.isEmpty) {
            return undefined;
        }
        return this._items.at(0);
    }

    peekRear(): T | undefined {
        if (this.isEmpty) {
            return undefined;
        }
        return this._items.at(this._items.length - 1);
    }

    clear(): void {
        this._items.clear();
    }

    toArray(): (T | undefined)[] {
        return this._items.toArray();
    }

    [Symbol.iterator](): Iterator<T | undefined> {
        return this._items[Symbol.iterator]();
    }
}
