class DoubleLinkedNode<T> {
    public data: T;
    public next: DoubleLinkedNode<T> | null;
    public prev: DoubleLinkedNode<T> | null;

    constructor(
        data: T,
        next: DoubleLinkedNode<T> | null = null,
        prev: DoubleLinkedNode<T> | null = null
    ) {
        this.data = data;
        this.next = next;
        this.prev = prev;
    }
}

export class DoubleLinkedList<T> {
    private size: number;
    private head: DoubleLinkedNode<T> | null;
    private tail: DoubleLinkedNode<T> | null;
    constructor() {
        this.size = 0;
        this.head = null;
        this.tail = null;
    }

    public isEmpty() {
        return this.head === null;
    }

    public getSize() {
        return this.size;
    }

    public insertAtHead(data: T) {
        const node = new DoubleLinkedNode(data);
        if (this.isEmpty()) {
            this.head = node;
            this.tail = node;
            this.size++;
            return;
        }
        node.next = this.head;
        this.head!.prev = node;
        this.head = node;
        this.size++;
    }

    public insertAtTail(data: T) {
        const node = new DoubleLinkedNode(data);
        if (this.isEmpty()) {
            this.tail = node;
            this.head = node;
            this.size++;
            return;
        }
        node.prev = this.tail;
        this.tail!.next = node;
        this.tail = node;
        this.size++;
    }

    public deleteAtHead(): T | null {
        if (this.isEmpty()) {
            return null;
        }
        const data = this.head?.data;
        if (this.head === this.tail) {
            this.head = null;
            this.tail = null;
            this.size--;
            return data!;
        }
        this.head = this.head?.next!;
        this.head.prev = null;
        this.size--;
        return data!;
    }

    public deleteAtTail(): T | null {
        if (this.isEmpty()) {
            return null;
        }
        const data = this.head?.data;
        if (this.head === this.tail) {
            this.head = null;
            this.tail = null;
            this.size--;
            return data!;
        }
        this.tail = this.tail?.prev!;
        this.tail.next = null;
        this.size--;
        return data!;
    }

    public find(data: T) {
        let current = this.head;
        let index = 0;
        while (current !== null) {
            if (current.data === data) {
                return index;
            }
            current = current.next;
            index++;
        }
        return -1;
    }

    public delete(data: T): T | null {
        if (this.isEmpty()) {
            return null;
        }
        if (this.head === this.tail) {
            const data = this.head?.data!;
            this.size--;
            this.head = null;
            this.tail = null;
            return data;
        }
        let current = this.head;
        while (current !== null) {
            if (current.data === data) {
                if (current === this.head) {
                    this.head = this.head.next;
                    this.head!.prev = null;
                    this.size--;
                    return current.data;
                }
                if (current === this.tail) {
                    this.tail = this.tail.prev;
                    this.tail!.next = null;
                    this.size--;
                    return current.data;
                }
                current.prev!.next = current.next;
                current.next!.prev = current.prev;
                this.size--;
                return current.data;
            }
        }
        return null;
    }

    public displayForward() {
        let current = this.head;
        let result = '';
        while (current !== null) {
            result += current.data;
            if (current.next) {
                result += '<=>';
            }
            current = current.next;
        }
        console.log(`[Size: ${this.size}, Forward List: ${result}]`);
    }

    public displayBack() {
        let current = this.tail;
        let result = '';
        while (current !== null) {
            result += current.data;
            if (current.prev) {
                result += '<=>';
            }
            current = current.prev;
        }
        console.log(`[Size: ${this.size}, Back List: ${result}]`);
    }
}