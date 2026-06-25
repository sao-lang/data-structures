export class MinHeap<T extends number | string> {
    private _heap: T[];

    constructor() {
        this._heap = [];
    }

    get size(): number {
        return this._heap.length;
    }

    get isEmpty(): boolean {
        return this._heap.length === 0;
    }

    private getParentIndex(index: number): number {
        return Math.floor((index - 1) / 2);
    }

    private getLeftChildIndex(index: number): number {
        return 2 * index + 1;
    }

    private getRightChildIndex(index: number): number {
        return 2 * index + 2;
    }

    private hasParent(index: number): boolean {
        return this.getParentIndex(index) >= 0;
    }

    private hasLeftChild(index: number): boolean {
        return this.getLeftChildIndex(index) < this._heap.length;
    }

    private hasRightChild(index: number): boolean {
        return this.getRightChildIndex(index) < this._heap.length;
    }

    private parent(index: number): T {
        return this._heap[this.getParentIndex(index)];
    }

    private leftChild(index: number): T {
        return this._heap[this.getLeftChildIndex(index)];
    }

    private rightChild(index: number): T {
        return this._heap[this.getRightChildIndex(index)];
    }

    private swap(indexOne: number, indexTwo: number): void {
        const temp = this._heap[indexOne];
        this._heap[indexOne] = this._heap[indexTwo];
        this._heap[indexTwo] = temp;
    }

    private heapifyUp(): void {
        let index = this._heap.length - 1;
        while (this.hasParent(index) && this.parent(index) > this._heap[index]) {
            this.swap(this.getParentIndex(index), index);
            index = this.getParentIndex(index);
        }
    }

    private heapifyDown(): void {
        let index = 0;
        while (this.hasLeftChild(index)) {
            let smallerChildIndex = this.getLeftChildIndex(index);
            if (this.hasRightChild(index) && this.rightChild(index) < this.leftChild(index)) {
                smallerChildIndex = this.getRightChildIndex(index);
            }

            if (this._heap[index] < this._heap[smallerChildIndex]) {
                break;
            } else {
                this.swap(index, smallerChildIndex);
            }
            index = smallerChildIndex;
        }
    }

    peek(): T | undefined {
        if (this.isEmpty) {
            return undefined;
        }
        return this._heap[0];
    }

    poll(): T | undefined {
        if (this.isEmpty) {
            return undefined;
        }
        const item = this._heap[0];
        const lastItem = this._heap.pop();
        if (this._heap.length > 0 && lastItem !== undefined) {
            this._heap[0] = lastItem;
            this.heapifyDown();
        }
        return item;
    }

    add(item: T): void {
        this._heap.push(item);
        this.heapifyUp();
    }

    toArray(): T[] {
        return [...this._heap];
    }

    clear(): void {
        this._heap = [];
    }
}

export class MaxHeap<T extends number | string> {
    private _heap: T[];

    constructor() {
        this._heap = [];
    }

    get size(): number {
        return this._heap.length;
    }

    get isEmpty(): boolean {
        return this._heap.length === 0;
    }

    private getParentIndex(index: number): number {
        return Math.floor((index - 1) / 2);
    }

    private getLeftChildIndex(index: number): number {
        return 2 * index + 1;
    }

    private getRightChildIndex(index: number): number {
        return 2 * index + 2;
    }

    private hasParent(index: number): boolean {
        return this.getParentIndex(index) >= 0;
    }

    private hasLeftChild(index: number): boolean {
        return this.getLeftChildIndex(index) < this._heap.length;
    }

    private hasRightChild(index: number): boolean {
        return this.getRightChildIndex(index) < this._heap.length;
    }

    private parent(index: number): T {
        return this._heap[this.getParentIndex(index)];
    }

    private leftChild(index: number): T {
        return this._heap[this.getLeftChildIndex(index)];
    }

    private rightChild(index: number): T {
        return this._heap[this.getRightChildIndex(index)];
    }

    private swap(indexOne: number, indexTwo: number): void {
        const temp = this._heap[indexOne];
        this._heap[indexOne] = this._heap[indexTwo];
        this._heap[indexTwo] = temp;
    }

    private heapifyUp(): void {
        let index = this._heap.length - 1;
        while (this.hasParent(index) && this.parent(index) < this._heap[index]) {
            this.swap(this.getParentIndex(index), index);
            index = this.getParentIndex(index);
        }
    }

    private heapifyDown(): void {
        let index = 0;
        while (this.hasLeftChild(index)) {
            let largerChildIndex = this.getLeftChildIndex(index);
            if (this.hasRightChild(index) && this.rightChild(index) > this.leftChild(index)) {
                largerChildIndex = this.getRightChildIndex(index);
            }

            if (this._heap[index] > this._heap[largerChildIndex]) {
                break;
            } else {
                this.swap(index, largerChildIndex);
            }
            index = largerChildIndex;
        }
    }

    peek(): T | undefined {
        if (this.isEmpty) {
            return undefined;
        }
        return this._heap[0];
    }

    poll(): T | undefined {
        if (this.isEmpty) {
            return undefined;
        }
        const item = this._heap[0];
        const lastItem = this._heap.pop();
        if (this._heap.length > 0 && lastItem !== undefined) {
            this._heap[0] = lastItem;
            this.heapifyDown();
        }
        return item;
    }

    add(item: T): void {
        this._heap.push(item);
        this.heapifyUp();
    }

    toArray(): T[] {
        return [...this._heap];
    }

    clear(): void {
        this._heap = [];
    }
}
