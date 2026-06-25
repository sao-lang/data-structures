class ListNode<T> {
    value: T;
    next: ListNode<T> | null;

    constructor(value: T) {
        this.value = value;
        this.next = null;
    }
}

export class SinglyLinkedList<T> {
    private _head: ListNode<T> | null;
    private _tail: ListNode<T> | null;
    private _length: number;

    constructor() {
        this._head = null;
        this._tail = null;
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
        return this._tail?.value;
    }

    prepend(value: T): void {
        const newNode = new ListNode(value);
        if (!this._head) {
            this._head = newNode;
            this._tail = newNode;
        } else {
            newNode.next = this._head;
            this._head = newNode;
        }
        this._length++;
    }

    append(value: T): void {
        const newNode = new ListNode(value);
        if (!this._tail) {
            this._head = newNode;
            this._tail = newNode;
        } else {
            this._tail.next = newNode;
            this._tail = newNode;
        }
        this._length++;
    }

    removeFirst(): T | undefined {
        if (!this._head) {
            return undefined;
        }
        const removedNode = this._head;
        this._head = this._head.next;
        if (!this._head) {
            this._tail = null;
        }
        this._length--;
        return removedNode.value;
    }

    clear(): void {
        this._head = null;
        this._tail = null;
        this._length = 0;
    }

    toArray(): T[] {
        const result: T[] = [];
        let current = this._head;
        while (current) {
            result.push(current.value);
            current = current.next;
        }
        return result;
    }

    [Symbol.iterator](): Iterator<T> {
        let current = this._head;
        return {
            next(): IteratorResult<T> {
                if (current === null) {
                    return { done: true, value: undefined };
                }
                const value = current.value;
                current = current.next;
                return { done: false, value };
            }
        };
    }
}

export class CircularLinkedList<T> {
    private _tail: ListNode<T> | null;
    private _length: number;

    constructor() {
        this._tail = null;
        this._length = 0;
    }

    get length(): number {
        return this._length;
    }

    get isEmpty(): boolean {
        return this._length === 0;
    }

    get head(): T | undefined {
        return this._tail?.next?.value;
    }

    get tail(): T | undefined {
        return this._tail?.value;
    }

    prepend(value: T): void {
        const newNode = new ListNode(value);
        if (!this._tail) {
            newNode.next = newNode;
            this._tail = newNode;
        } else {
            newNode.next = this._tail.next;
            this._tail.next = newNode;
        }
        this._length++;
    }

    append(value: T): void {
        const newNode = new ListNode(value);
        if (!this._tail) {
            newNode.next = newNode;
            this._tail = newNode;
        } else {
            newNode.next = this._tail.next;
            this._tail.next = newNode;
            this._tail = newNode;
        }
        this._length++;
    }

    removeFirst(): T | undefined {
        if (!this._tail) {
            return undefined;
        }
        const removedNode = this._tail.next!;
        if (this._tail === removedNode) {
            this._tail = null;
        } else {
            this._tail.next = removedNode.next;
        }
        this._length--;
        return removedNode.value;
    }

    clear(): void {
        this._tail = null;
        this._length = 0;
    }

    toArray(): T[] {
        const result: T[] = [];
        if (!this._tail) {
            return result;
        }
        let current = this._tail.next;
        for (let i = 0; i < this._length; i++) {
            if (current) {
                result.push(current.value);
                current = current.next;
            }
        }
        return result;
    }

    [Symbol.iterator](): Iterator<T> {
        let current = this._tail?.next || null;
        let count = 0;
        const length = this._length;
        return {
            next(): IteratorResult<T> {
                if (count >= length || current === null) {
                    return { done: true, value: undefined };
                }
                const value = current.value;
                current = current.next;
                count++;
                return { done: false, value };
            }
        };
    }
}

class DoublyListNode<T> {
    value: T;
    next: DoublyListNode<T> | null;
    prev: DoublyListNode<T> | null;

    constructor(value: T) {
        this.value = value;
        this.next = null;
        this.prev = null;
    }
}

export class DoublyLinkedList<T> {
    private _head: DoublyListNode<T> | null;
    private _tail: DoublyListNode<T> | null;
    private _length: number;

    constructor() {
        this._head = null;
        this._tail = null;
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
        return this._tail?.value;
    }

    prepend(value: T): void {
        const newNode = new DoublyListNode(value);
        if (!this._head) {
            this._head = newNode;
            this._tail = newNode;
        } else {
            newNode.next = this._head;
            this._head.prev = newNode;
            this._head = newNode;
        }
        this._length++;
    }

    append(value: T): void {
        const newNode = new DoublyListNode(value);
        if (!this._tail) {
            this._head = newNode;
            this._tail = newNode;
        } else {
            newNode.prev = this._tail;
            this._tail.next = newNode;
            this._tail = newNode;
        }
        this._length++;
    }

    removeFirst(): T | undefined {
        if (!this._head) {
            return undefined;
        }
        const removedNode = this._head;
        this._head = this._head.next;
        if (this._head) {
            this._head.prev = null;
        } else {
            this._tail = null;
        }
        this._length--;
        return removedNode.value;
    }

    removeLast(): T | undefined {
        if (!this._tail) {
            return undefined;
        }
        const removedNode = this._tail;
        this._tail = this._tail.prev;
        if (this._tail) {
            this._tail.next = null;
        } else {
            this._head = null;
        }
        this._length--;
        return removedNode.value;
    }

    clear(): void {
        this._head = null;
        this._tail = null;
        this._length = 0;
    }

    toArray(): T[] {
        const result: T[] = [];
        let current = this._head;
        while (current) {
            result.push(current.value);
            current = current.next;
        }
        return result;
    }

    toArrayReverse(): T[] {
        const result: T[] = [];
        let current = this._tail;
        while (current) {
            result.push(current.value);
            current = current.prev;
        }
        return result;
    }

    [Symbol.iterator](): Iterator<T> {
        let current = this._head;
        return {
            next(): IteratorResult<T> {
                if (current === null) {
                    return { done: true, value: undefined };
                }
                const value = current.value;
                current = current.next;
                return { done: false, value };
            }
        };
    }
}

export class DoublyCircularLinkedList<T> {
    private _tail: DoublyListNode<T> | null;
    private _length: number;

    constructor() {
        this._tail = null;
        this._length = 0;
    }

    get length(): number {
        return this._length;
    }

    get isEmpty(): boolean {
        return this._length === 0;
    }

    get head(): T | undefined {
        return this._tail?.next?.value;
    }

    get tail(): T | undefined {
        return this._tail?.value;
    }

    prepend(value: T): void {
        const newNode = new DoublyListNode(value);
        if (!this._tail) {
            newNode.next = newNode;
            newNode.prev = newNode;
            this._tail = newNode;
        } else {
            newNode.next = this._tail.next;
            newNode.prev = this._tail;
            this._tail.next!.prev = newNode;
            this._tail.next = newNode;
        }
        this._length++;
    }

    append(value: T): void {
        const newNode = new DoublyListNode(value);
        if (!this._tail) {
            newNode.next = newNode;
            newNode.prev = newNode;
            this._tail = newNode;
        } else {
            newNode.next = this._tail.next;
            newNode.prev = this._tail;
            this._tail.next!.prev = newNode;
            this._tail.next = newNode;
            this._tail = newNode;
        }
        this._length++;
    }

    removeFirst(): T | undefined {
        if (!this._tail) {
            return undefined;
        }
        const removedNode = this._tail.next!;
        if (this._tail === removedNode) {
            this._tail = null;
        } else {
            this._tail.next = removedNode.next;
            removedNode.next!.prev = this._tail;
        }
        this._length--;
        return removedNode.value;
    }

    removeLast(): T | undefined {
        if (!this._tail) {
            return undefined;
        }
        const removedNode = this._tail;
        if (this._tail === removedNode.next) {
            this._tail = null;
        } else {
            this._tail = removedNode.prev!;
            this._tail.next = removedNode.next;
            removedNode.next!.prev = this._tail;
        }
        this._length--;
        return removedNode.value;
    }

    clear(): void {
        this._tail = null;
        this._length = 0;
    }

    toArray(): T[] {
        const result: T[] = [];
        if (!this._tail) {
            return result;
        }
        let current = this._tail.next;
        for (let i = 0; i < this._length; i++) {
            if (current) {
                result.push(current.value);
                current = current.next;
            }
        }
        return result;
    }

    toArrayReverse(): T[] {
        const result: T[] = [];
        if (!this._tail) {
            return result;
        }
        let current: DoublyListNode<T> | null = this._tail;
        for (let i = 0; i < this._length; i++) {
            if (current) {
                result.push(current.value);
                current = current.prev;
            }
        }
        return result;
    }

    [Symbol.iterator](): Iterator<T> {
        let current = this._tail?.next || null;
        let count = 0;
        const length = this._length;
        return {
            next(): IteratorResult<T> {
                if (count >= length || current === null) {
                    return { done: true, value: undefined };
                }
                const value = current.value;
                current = current.next;
                count++;
                return { done: false, value };
            }
        };
    }
}

console.log("Singly Linked List Example");
const sll = new SinglyLinkedList<number>();
sll.append(1);
sll.append(2);
sll.append(3);
console.log(`List: [${sll.toArray().join(", ")}]`);
console.log("Iterating with for loop:");
let output = '';
for (const val of sll) {
    output += val + ' ';
}
console.log(output.trim());

console.log("\nCircular Linked List Example");
const cll = new CircularLinkedList<number>();
cll.append(1);
cll.append(2);
cll.append(3);
console.log(`List: [${cll.toArray().join(", ")}]`);
console.log("Iterating with for loop:");
output = '';
for (const val of cll) {
    output += val + ' ';
}
console.log(output.trim());

console.log("\nDoubly Linked List Example");
const dll = new DoublyLinkedList<number>();
dll.append(1);
dll.append(2);
dll.append(3);
console.log(`List: [${dll.toArray().join(", ")}]`);
console.log(`Reverse: [${dll.toArrayReverse().join(", ")}]`);
console.log("Iterating with for loop:");
output = '';
for (const val of dll) {
    output += val + ' ';
}
console.log(output.trim());

console.log("\nDoubly Circular Linked List Example");
const dcll = new DoublyCircularLinkedList<number>();
dcll.append(1);
dcll.append(2);
dcll.append(3);
console.log(`List: [${dcll.toArray().join(", ")}]`);
console.log(`Reverse: [${dcll.toArrayReverse().join(", ")}]`);
console.log("Iterating with for loop:");
output = '';
for (const val of dcll) {
    output += val + ' ';
}
console.log(output.trim());
