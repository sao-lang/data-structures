export class FixedArray<T> {
    private _capacity: number;
    private _length: number;
    private _data: (T | undefined)[];

    constructor(capacity: number) {
        if (capacity <= 0) {
            throw new Error('Capacity must be positive');
        }
        this._capacity = capacity;
        this._length = 0;
        this._data = new Array(capacity);
    }

    get capacity(): number {
        return this._capacity;
    }

    get length(): number {
        return this._length;
    }

    get isEmpty(): boolean {
        return this._length === 0;
    }

    get isFull(): boolean {
        return this._length === this._capacity;
    }

    at(index: number): T | undefined {
        if (index < 0 || index >= this._length) {
            throw new Error('Index out of bounds');
        }
        return this._data[index];
    }

    set(index: number, value: T): void {
        if (index < 0 || index >= this._length) {
            throw new Error('Index out of bounds');
        }
        this._data[index] = value;
    }

    push(value: T): void {
        if (this.isFull) {
            throw new Error('Array is full');
        }
        this._data[this._length] = value;
        this._length++;
    }

    pop(): T | undefined {
        if (this.isEmpty) {
            throw new Error('Array is empty');
        }
        this._length--;
        const value = this._data[this._length];
        this._data[this._length] = undefined;
        return value;
    }

    insert(index: number, value: T): void {
        if (this.isFull) {
            throw new Error('Array is full');
        }
        if (index < 0 || index > this._length) {
            throw new Error('Index out of bounds');
        }
        for (let i = this._length; i > index; i--) {
            this._data[i] = this._data[i - 1];
        }
        this._data[index] = value;
        this._length++;
    }

    remove(index: number): T | undefined {
        if (this.isEmpty) {
            throw new Error('Array is empty');
        }
        if (index < 0 || index >= this._length) {
            throw new Error('Index out of bounds');
        }
        const value = this._data[index];
        for (let i = index; i < this._length - 1; i++) {
            this._data[i] = this._data[i + 1];
        }
        this._length--;
        this._data[this._length] = undefined;
        return value;
    }

    find(value: T): number {
        for (let i = 0; i < this._length; i++) {
            if (this._data[i] === value) {
                return i;
            }
        }
        return -1;
    }

    toArray(): (T | undefined)[] {
        return this._data.slice(0, this._length);
    }

    clear(): void {
        this._data = new Array(this._capacity);
        this._length = 0;
    }

    [Symbol.iterator](): Iterator<T | undefined> {
        let index = 0;
        const length = this._length;
        const data = this._data;
        return {
            next(): IteratorResult<T | undefined> {
                if (index >= length) {
                    return { done: true, value: undefined };
                }
                const value = data[index];
                index++;
                return { done: false, value };
            }
        };
    }
}

export class DynamicArray<T> {
    private _capacity: number;
    private _length: number;
    private _data: (T | undefined)[];
    private readonly _growthFactor: number = 2;

    constructor(initialCapacity: number = 10) {
        if (initialCapacity <= 0) {
            throw new Error('Initial capacity must be positive');
        }
        this._capacity = initialCapacity;
        this._length = 0;
        this._data = new Array(initialCapacity);
    }

    get capacity(): number {
        return this._capacity;
    }

    get length(): number {
        return this._length;
    }

    get isEmpty(): boolean {
        return this._length === 0;
    }

    private resize(): void {
        const newCapacity = this._capacity * this._growthFactor;
        const newData = new Array(newCapacity);
        for (let i = 0; i < this._length; i++) {
            newData[i] = this._data[i];
        }
        this._data = newData;
        this._capacity = newCapacity;
    }

    at(index: number): T | undefined {
        if (index < 0 || index >= this._length) {
            throw new Error('Index out of bounds');
        }
        return this._data[index];
    }

    set(index: number, value: T): void {
        if (index < 0 || index >= this._length) {
            throw new Error('Index out of bounds');
        }
        this._data[index] = value;
    }

    push(value: T): void {
        if (this._length >= this._capacity) {
            this.resize();
        }
        this._data[this._length] = value;
        this._length++;
    }

    pop(): T | undefined {
        if (this.isEmpty) {
            throw new Error('Array is empty');
        }
        this._length--;
        const value = this._data[this._length];
        this._data[this._length] = undefined;
        return value;
    }

    insert(index: number, value: T): void {
        if (index < 0 || index > this._length) {
            throw new Error('Index out of bounds');
        }
        if (this._length >= this._capacity) {
            this.resize();
        }
        for (let i = this._length; i > index; i--) {
            this._data[i] = this._data[i - 1];
        }
        this._data[index] = value;
        this._length++;
    }

    remove(index: number): T | undefined {
        if (this.isEmpty) {
            throw new Error('Array is empty');
        }
        if (index < 0 || index >= this._length) {
            throw new Error('Index out of bounds');
        }
        const value = this._data[index];
        for (let i = index; i < this._length - 1; i++) {
            this._data[i] = this._data[i + 1];
        }
        this._length--;
        this._data[this._length] = undefined;
        return value;
    }

    find(value: T): number {
        for (let i = 0; i < this._length; i++) {
            if (this._data[i] === value) {
                return i;
            }
        }
        return -1;
    }

    toArray(): (T | undefined)[] {
        return this._data.slice(0, this._length);
    }

    clear(): void {
        this._data = new Array(10);
        this._capacity = 10;
        this._length = 0;
    }

    sort(compareFn?: (a: T, b: T) => number): void {
        const arr = this.toArray() as T[];
        arr.sort(compareFn);
        for (let i = 0; i < this._length; i++) {
            this._data[i] = arr[i];
        }
    }

    [Symbol.iterator](): Iterator<T | undefined> {
        let index = 0;
        const length = this._length;
        const data = this._data;
        return {
            next(): IteratorResult<T | undefined> {
                if (index >= length) {
                    return { done: true, value: undefined };
                }
                const value = data[index];
                index++;
                return { done: false, value };
            }
        };
    }
}
