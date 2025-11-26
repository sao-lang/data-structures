// 单向链表
class SingleLinkedNode<T> {
    public data: T;
    public next: SingleLinkedNode<T> | null;

    constructor(data: T, next: SingleLinkedNode<T> | null) {
        this.data = data;
        this.next = next;
    }
}

export class SingleLinkedList<T> {
    private head: SingleLinkedNode<T> | null;
    private size: number;
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
        const node = new SingleLinkedNode(data, this.head);
        node.next = this.head;
        this.head = node;
        this.size++;
    }

    public insertAtTail(data: T) {
        const node = new SingleLinkedNode(data, null);
        if (this.isEmpty()) {
            this.head = node;
            this.size++;
            return;
        }
        let currentNode = this.head;
        while (currentNode?.next !== null) {
            currentNode = currentNode?.next!;
        }
        currentNode.next = node;
        this.size++;
    }

    public find(data: T) {
        let index = 0;
        let currentNode = this.head;
        while (currentNode !== null) {
            if (currentNode.data === data) {
                return index;
            }
            currentNode = currentNode?.next!;
            index++;
        }
        return -1;
    }

    public deleteAtHead(): T | null {
        if (this.isEmpty()) {
            return null;
        }
        const data = this.head?.data;
        this.head = this.head?.next!;
        return data!;
    }

    public delete(data: T): T | null {
        if (this.isEmpty()) {
            return null;
        }
        let previous: SingleLinkedNode<T> | null = null;
        let current = this.head;
        if (data === current?.data) {
            this.head = this.head?.next!;
            this.size--;
            return data;
        }

        while (current !== null && current.data !== data) {
            previous = current;
            current = previous.next;
        }

        if (current !== null) {
            previous!.next = current.next;
            this.size--;
            return current.data;
        }

        return null;
    }

    public display() {
        let result = '';
        let current = this.head;
        while (current !== null) {
            result += `${current.data}`;
            if (current.next !== null) {
                result += '->';
            }
            current = current.next;
        }
        // return result;
        console.log(`[Size: ${this.size}, SingleList: ${result}`);
    }
}