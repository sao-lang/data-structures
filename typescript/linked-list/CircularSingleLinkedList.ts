class CircularSingleLinkedNode<T> {
    public data: T;
    public next: CircularSingleLinkedNode<T> | null;

    constructor(data: T, next: CircularSingleLinkedNode<T> | null = null) {
        this.data = data;
        this.next = next;
    }
}

export class CircularSingleLinkedList<T> {
    private size: number;
    private head: CircularSingleLinkedNode<T> | null;

    constructor() {
        this.size = 0;
        this.head = null;
    }

    public isEmpty() {
        return this.head === null;
    }

    public getSize() {
        return this.size;
    }

    public insertAtHead(data: T) {
        const node = new CircularSingleLinkedNode(data);
        if (this.isEmpty()) {
            this.head = node;
            this.head.next = node;
            this.size++;
            return;
        }
        if (this.head?.next === this.head) {
            node.next = this.head;
            this.head = node;
            this.head.next = node.next;
            this.size++;
            return;
        }

    }
}