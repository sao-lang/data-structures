class FixedArray<T> {
    private items: T[];
    private readonly capacity: number;
    private size: number;
    constructor(capacity: number) {
        if (capacity <= 0) {
            throw new Error('Capacity must be a positive integer.');
        }
        this.capacity = capacity;
        this.items = new Array(capacity);
        this.size = 0;
    }

    private checkIndex(index: number) {
        if (index < 0 || index >= this.size) {
            throw new Error(`Index out of bounds: ${index}. Current size: ${this.size}.`);
        }
    }

    private checkCapacity() {
        if (this.size >= this.capacity) {
            throw new Error('Fixed Array is full.Cannot add more elements.');
        }
    }

    /**
     * @description 从尾部插入
     */
    public append(element: T) {
        this.checkCapacity();
        this.items[this.size] = element;
        this.size++;
    }

    /**
     * @description 从头部插入
     */
    public prepend(element: T) {
        this.checkCapacity();
        this.addAt(0, element);
    }

    /**
     * @description 从第几位添加一个元素
     */
    public addAt(index: number, element: T) {
        if (index < 0 || index > this.size) {
            throw new Error(
                `Insertion index is out of bounds. Valid range: 0 to ${this.size}. Requested: ${index}`
            );
        }
        this.checkCapacity();
        // 从index开始往后移动一位
        for (let i = this.size - 1; i >= index; i--) {
            this.items[i + 1] = this.items[i];
        }
        this.items[index] = element;
        this.size++;
    }

    /**
     * 从第几位删除一个元素
     */
    public remove(index: number): T {
        this.checkIndex(index);
        const removedElement = this.items[index];
        for (let i = index; i < this.size; i++) {
            this.items[i] = this.items[i + 1];
        }
        this.size--;
        return removedElement;
    }

    public popFront() {
        this.remove(0);
    }

    public popBack() {
        this.remove(this.size - 1);
    }

    public get(index: number) {
        this.checkIndex(index);
        return this.items[index] as (undefined | T);
    }

    public set(index: number, element: T) {
        this.checkIndex(index);
        this.items[index] = element;
    }

    public getFirst() {
        return this.get(0) as (undefined | T);
    }

    public getLast() {
        return this.get(this.size - 1) as (undefined | T);
    }

    public sort(comparator?: (a: T, b: T) => number) {
        if (this.size <= 1) return;
        const cmp =
            comparator ??
            ((a: any, b: any) => {
                if (a < b) return -1;
                if (a > b) return 1;
                return 0;
            });

        const arr = this.items;
        const quickSort = (left: number, right: number) => {
            if (left >= right) return;

            const pivotIndex = partition(left, right);
            quickSort(left, pivotIndex - 1);
            quickSort(pivotIndex + 1, right);
        };
        const partition = (left: number, right: number): number => {
            const pivot = arr[right];
            let i = left;

            for (let j = left; j < right; j++) {
                if (cmp(arr[j], pivot) < 0) {
                    [arr[i], arr[j]] = [arr[j], arr[i]];
                    i++;
                }
            }

            [arr[i], arr[right]] = [arr[right], arr[i]];
            return i;
        };
        quickSort(0, this.size - 1);
    }

    public getSize() {
        return this.size;
    }

    public getCapacity(){
        return this.capacity;
    }

    public getElements(){
        return this.items;
    }

    public toArray(){
        return this.items.slice(0, this.size);
    }

    public toString(){
        return `[Size: ${this.size}, Capacity: ${this.capacity}, ${this.toArray().join()}]`
    }
}

class DynamicArray<T> {
    private items: T[];
    private size: number;
    private growthFactor = 2;
    constructor(initialCapacity: number = 10) {
        if (initialCapacity <= 0) {
            throw new Error('Capacity must be a positive integer.');
        }
        this.items = new Array(initialCapacity);
        this.size = 0;
    }

    private ensureCapacity(){
        if (this.size === this.items.length) {
            const newItems: T[] = new Array(this.size * this.growthFactor);
            for (let i = 0; i < this.items.length; i++) {
                newItems[i] = this.items[i];
            }
            this.items = newItems;
        }
    }
    private checkIndex(index: number) {
        if (index < 0 || index >= this.size) {
            throw new Error(`Index out of bounds: ${index}. Current size: ${this.size}.`);
        }
    }

    /**
     * @description 从尾部插入
     */
    public append(element: T) {
        this.ensureCapacity();
        this.items[this.size] = element;
        this.size++;
    }

    /**
     * @description 从头部插入
     */
    public prepend(element: T) {
        this.addAt(0, element);
    }

    /**
     * @description 从第几位添加一个元素
     */
    public addAt(index: number, element: T) {
        if (index < 0 || index > this.size) {
            throw new Error(
                `Insertion index is out of bounds. Valid range: 0 to ${this.size}. Requested: ${index}`
            );
        }
        this.ensureCapacity();
        // 从index开始往后移动一位
        for (let i = this.size - 2; i > index; i--) {
            this.items[i + 1] = this.items[i];
        }
        this.items[index] = element;
        this.size++;
    }

    /**
     * 从第几位删除一个元素
     */
    public remove(index: number): T {
        this.checkIndex(index);
        const removedElement = this.items[index];
        for (let i = index; i < this.size; i++) {
            this.items[i] = this.items[i + 1];
        }
        this.size--;
        return removedElement;
    }

    public popFront() {
        this.remove(0);
    }

    public popBack() {
        this.remove(this.size - 1);
    }

    public get(index: number) {
        this.checkIndex(index);
        return this.items[index];
    }

    public set(index: number, element: T) {
        this.checkIndex(index);
        this.items[index] = element;
    }

    public getFirst() {
        return this.get(0);
    }

    public getLast() {
        return this.get(this.size - 1);
    }

    public sort(comparator?: (a: T, b: T) => number) {
        if (this.size <= 1) return;
        const cmp =
            comparator ??
            ((a: any, b: any) => {
                if (a < b) return -1;
                if (a > b) return 1;
                return 0;
            });

        const arr = this.items;
        const quickSort = (left: number, right: number) => {
            if (left >= right) return;

            const pivotIndex = partition(left, right);
            quickSort(left, pivotIndex - 1);
            quickSort(pivotIndex + 1, right);
        };
        const partition = (left: number, right: number): number => {
            const pivot = arr[right];
            let i = left;

            for (let j = left; j < right; j++) {
                if (cmp(arr[j], pivot) < 0) {
                    [arr[i], arr[j]] = [arr[j], arr[i]];
                    i++;
                }
            }

            [arr[i], arr[right]] = [arr[right], arr[i]];
            return i;
        };
        quickSort(0, this.size - 1);
    }

    public getSize() {
        return this.size;
    }

    public getElements(){
        return this.items;
    }

    public toArray(){
        return this.items.slice(0, this.size);
    }

    public toString(){
        return `[Size: ${this.size}, ${this.toArray().join()}]`
    }
}

const fixedArray = new FixedArray(10);
for (let i = 0; i < 8; i++) {
    fixedArray.append(i);
}
console.log(fixedArray.toString());
fixedArray.addAt(3, 11);
console.log(fixedArray.toString());
fixedArray.remove(3);
console.log(fixedArray.toString());
