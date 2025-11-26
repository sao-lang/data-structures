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
        let current = this.head;
        while (current?.next !== this.head) {
            current = current?.next!;
        }
        current.next = node;
        node.next = this.head;
        this.head = node;
        this.size++;
    }

    public insertAtTail(data: T) {
        const node = new CircularSingleLinkedNode(data);
        if (this.isEmpty()) {
            this.head = node;
            this.head.next = node;
            this.size++;
            return;
        }
        let current = this.head;
        while (current?.next !== this.head) {
            current = current?.next!;
        }
        current.next = node;
        node.next = this.head;
        this.size++;
    }

    public deleteAtHead(): T | null {
        if (this.isEmpty()) {
            return null;
        }
        const data = this.head?.data;
        if (this.head === this.head?.next) {
            this.head = null;
            return data!;
        }
        let current = this.head;
        while (current?.next !== this.head) {
            current = current!.next;
        }
        current.next = this.head?.next!;
        this.head = this.head!.next;
        this.size--;
        return data!;
    }

    public deleteAtTail(): T | null {
        if (this.isEmpty()) {
            return null;
        }
        if (this.head === this.head?.next) {
            this.head = null;
            this.size--;
            return this.head!.data;
        }
        let current = this.head;
        while (current?.next?.next !== this.head) {
            current = current!.next;
        }
        current.next = this.head;
        this.size--;
        return current.data;
    }

    public delete(data: T) {
        if (this.isEmpty()) {
            return null;
        }
        let current = this.head;
        let previous: CircularSingleLinkedNode<T> | null = null;
        if (current?.data === data) {
            return this.deleteAtHead();
        }
        previous = current;
        current = previous?.next!;
        while (current !== this.head) {
            if (current.data === data) {
                previous!.next = current.next;
                this.size--;
                return data;
            }
            previous = current;
            current = previous?.next!;
        }
        return null;
    }

    public display(): void {
        // 1. 处理空链表的情况
        if (this.isEmpty()) {
            console.log(`[Size: 0] List: (Empty)`);
            return;
        }
        let current = this.head; // 从头节点开始
        let result = '';
        // 使用 while 循环
        while (current !== this.head || result === '') {
            // 循环条件：
            // 1. current 不等于 this.head (即还没有绕回一圈)
            // 2. 或者 result 为空字符串 (仅用于确保第一次循环能够执行，即处理 head 节点)
            // 检查是否为第一次循环 (即处理 head 节点)
            if (result !== '') {
                // 如果不是第一次循环，且 current 已经回到 head
                if (current === this.head) {
                    break; // 结束循环，因为已经遍历完所有节点并回到 head
                }
                result += ' -> '; // 添加连接符
            }
            // 处理当前节点的数据
            result += `${current!.data}`;
            // 移动到下一个节点
            current = current!.next;
        }
        console.log(`[Size: ${this.size}] Circular List: ${result} -> (Head)`);
    }
}
