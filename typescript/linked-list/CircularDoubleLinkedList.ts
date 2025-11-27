class CircularDoubleLinkedNode<T> {
    public data: T;
    public prev: CircularDoubleLinkedNode<T> | null;
    public next: CircularDoubleLinkedNode<T> | null;

    constructor(
        data: T,
        next: CircularDoubleLinkedNode<T> | null = null,
        prev: CircularDoubleLinkedNode<T> | null = null
    ) {
        this.data = data;
        this.next = next;
        this.prev = prev;
    }
}

class CircularDoubleLinkedList<T> {
    private size: number;
    private head: CircularDoubleLinkedNode<T> | null;
    private tail: CircularDoubleLinkedNode<T> | null;
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
        const node = new CircularDoubleLinkedNode(data);
        if (this.isEmpty()) {
            this.head = node;
            this.tail = node;
            node.prev = node;
            node.next = node;
            this.size++;
            return;
        }
        node.next = this.head;
        this.head!.prev = node;
        this.tail!.next = node;
        node.prev = this.tail;
        this.head = node;
        this.size++;
    }

    public insertAtTail(data: T) {
        const node = new CircularDoubleLinkedNode(data);
        if (this.isEmpty()) {
            this.head = node;
            this.tail = node;
            node.next = node;
            node.prev = node;
            this.size++;
            return;
        }
        node.prev = this.tail;
        node.next = this.head;
        this.tail!.next = node;
        this.head!.prev = node;
        this.tail = node;
        this.size++;
    }

    public deleteAtHead(): T | null {
        if (this.isEmpty()) {
            return null;
        }
        const data = this.head?.data;
        if (this.head === this.tail) {
            this.tail = null;
            this.head = null;
            this.size--;
            return data!;
        }
        this.head!.next!.prev = this.tail;
        this.tail!.next = this.head?.next!;
        this.head = this.head?.next!;
        this.size--;
        return data!;
    }

    public deleteAtTail(): T | null {
        if (this.isEmpty()) {
            return null;
        }
        const data = this.tail?.data;
        if (this.head === this.tail) {
            this.head = null;
            this.tail = null;
            this.size--;
            return data!;
        }
        this.tail!.prev!.next = this.head;
        this.head!.prev = this.tail?.prev!;
        this.tail = this.tail?.prev!;
        this.size--;
        return data!;
    }

    public displayForward(): void {
        if (this.isEmpty()) {
            console.log(`[Size: 0] List: (Empty)`);
            return;
        }

        let current = this.head;
        let result = '';

        do {
            result += `${current!.data}`;
            if (current!.next !== this.head) {
                result += ' <-> ';
            }
            current = current!.next;
        } while (current !== this.head);

        console.log(`[Size: ${this.size}] Forward: ${result} <-> (Head)`);
    }

    /**
     * 8. 打印链表（从尾向头）
     */
    public displayBackward(): void {
        if (this.isEmpty()) {
            console.log(`[Size: 0] List: (Empty)`);
            return;
        }

        let current = this.tail;
        let result = '';

        do {
            result += `${current!.data}`;
            if (current!.prev !== this.tail) {
                result += ' <-> ';
            }
            current = current!.prev;
        } while (current !== this.tail);

        console.log(`[Size: ${this.size}] Backward: ${result} <-> (Tail)`);
    }
}
