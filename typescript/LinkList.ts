// 单向链表
class SingleLinkedNode<T> {
    public data: T;
    public next: SingleLinkedNode<T> | null;

    constructor(data: T, next: SingleLinkedNode<T> | null) {
        this.data = data;
        this.next = next;
    }
}

class SingleLinkedList<T> {
    
}