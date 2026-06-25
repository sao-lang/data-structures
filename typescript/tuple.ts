export class Tuple<T> {
    private _items: T[];

    constructor(...items: T[]) {
        this._items = [...items];
    }

    get size(): number {
        return this._items.length;
    }

    get isEmpty(): boolean {
        return this._items.length === 0;
    }

    at(index: number): T | undefined {
        if (index < 0 || index >= this._items.length) {
            return undefined;
        }
        return this._items[index];
    }

    first(): T | undefined {
        return this._items[0];
    }

    last(): T | undefined {
        return this._items[this._items.length - 1];
    }

    toArray(): T[] {
        return [...this._items];
    }

    map<U>(func: (item: T) => U): Tuple<U> {
        return new Tuple(...this._items.map(func));
    }

    filter(func: (item: T) => boolean): Tuple<T> {
        return new Tuple(...this._items.filter(func));
    }

    reduce<U>(func: (accumulator: U, current: T) => U, initial: U): U {
        return this._items.reduce(func, initial);
    }

    concat(other: Tuple<T>): Tuple<T> {
        return new Tuple(...this._items, ...other._items);
    }

    slice(start: number, end?: number): Tuple<T> {
        return new Tuple(...this._items.slice(start, end));
    }

    take(n: number): Tuple<T> {
        return this.slice(0, n);
    }

    drop(n: number): Tuple<T> {
        return this.slice(n);
    }

    contains(item: T, equal?: (a: T, b: T) => boolean): boolean {
        const eq = equal || ((a, b) => a === b);
        return this._items.some(i => eq(i, item));
    }

    indexOf(item: T, equal?: (a: T, b: T) => boolean): number {
        const eq = equal || ((a, b) => a === b);
        for (let i = 0; i < this._items.length; i++) {
            if (eq(this._items[i], item)) {
                return i;
            }
        }
        return -1;
    }

    count(item: T, equal?: (a: T, b: T) => boolean): number {
        const eq = equal || ((a, b) => a === b);
        return this._items.filter(i => eq(i, item)).length;
    }

    reverse(): Tuple<T> {
        return new Tuple(...this._items.reverse());
    }

    [Symbol.iterator](): Iterator<T> {
        return this._items[Symbol.iterator]();
    }

    toString(): string {
        return `Tuple(${this._items.join(', ')})`;
    }
}

export class Pair<T, U> {
    private _first: T;
    private _second: U;

    constructor(first: T, second: U) {
        this._first = first;
        this._second = second;
    }

    get first(): T {
        return this._first;
    }

    get second(): U {
        return this._second;
    }

    swap(): Pair<U, T> {
        return new Pair(this._second, this._first);
    }

    toTuple(): Tuple<any> {
        return new Tuple<any>(this._first, this._second);
    }

    mapFirst<V>(func: (item: T) => V): Pair<V, U> {
        return new Pair(func(this._first), this._second);
    }

    mapSecond<V>(func: (item: U) => V): Pair<T, V> {
        return new Pair(this._first, func(this._second));
    }

    mapBoth<V, W>(func1: (item: T) => V, func2: (item: U) => W): Pair<V, W> {
        return new Pair(func1(this._first), func2(this._second));
    }

    toString(): string {
        return `Pair(${this._first}, ${this._second})`;
    }
}

export class Triple<T, U, V> {
    private _first: T;
    private _second: U;
    private _third: V;

    constructor(first: T, second: U, third: V) {
        this._first = first;
        this._second = second;
        this._third = third;
    }

    get first(): T {
        return this._first;
    }

    get second(): U {
        return this._second;
    }

    get third(): V {
        return this._third;
    }

    toTuple(): Tuple<any> {
        return new Tuple<any>(this._first, this._second, this._third);
    }

    mapFirst<W>(func: (item: T) => W): Triple<W, U, V> {
        return new Triple(func(this._first), this._second, this._third);
    }

    mapSecond<W>(func: (item: U) => W): Triple<T, W, V> {
        return new Triple(this._first, func(this._second), this._third);
    }

    mapThird<W>(func: (item: V) => W): Triple<T, U, W> {
        return new Triple(this._first, this._second, func(this._third));
    }

    toString(): string {
        return `Triple(${this._first}, ${this._second}, ${this._third})`;
    }
}

export function zip<T, U>(t1: Tuple<T>, t2: Tuple<U>): Tuple<Pair<T, U>> {
    const minLength = Math.min(t1.size, t2.size);
    const result: Pair<T, U>[] = [];
    for (let i = 0; i < minLength; i++) {
        const item1 = t1.at(i)!;
        const item2 = t2.at(i)!;
        result.push(new Pair(item1, item2));
    }
    return new Tuple(...result);
}

// if (require.main === module) {
//     console.log("=== Tuple Example ===");
//     const t1 = new Tuple(1, 2, 3, 4, 5);
//     console.log(`Tuple: ${t1}`);
//     console.log(`Size: ${t1.size}`);
//     console.log(`First: ${t1.first()}`);
//     console.log(`Last: ${t1.last()}`);
//     console.log(`At index 2: ${t1.at(2)}`);
//     console.log();

//     console.log("=== Tuple Operations ===");
//     const t2 = new Tuple("a", "b", "c");
//     console.log(`Tuple t2: ${t2}`);
//     console.log(`Concat t1 + t2: ${t1.concat(t2 as any)}`);
//     console.log(`Slice t1[1:4]: ${t1.slice(1, 4)}`);
//     console.log(`Take 3 from t1: ${t1.take(3)}`);
//     console.log(`Drop 2 from t1: ${t1.drop(2)}`);
//     console.log(`Reverse t1: ${t1.reverse()}`);
//     console.log(`Map t1 (x * 2): ${t1.map(x => x * 2)}`);
//     console.log(`Filter t1 (even): ${t1.filter(x => x % 2 === 0)}`);
//     console.log(`Reduce t1 (sum): ${t1.reduce((a, b) => a + b, 0)}`);
//     console.log();

//     console.log("=== Pair Example ===");
//     const p = new Pair(10, "hello");
//     console.log(`Pair: ${p}`);
//     console.log(`First: ${p.first}`);
//     console.log(`Second: ${p.second}`);
//     console.log(`Swap: ${p.swap()}`);
//     console.log(`Map first (+5): ${p.mapFirst(x => x + 5)}`);
//     console.log(`Map second (upper): ${p.mapSecond(s => s.toUpperCase())}`);
//     console.log();

//     console.log("=== Triple Example ===");
//     const tri = new Triple("a", 100, true);
//     console.log(`Triple: ${tri}`);
//     console.log(`First: ${tri.first}`);
//     console.log(`Second: ${tri.second}`);
//     console.log(`Third: ${tri.third}`);
//     console.log();

//     console.log("=== Zip Example ===");
//     const t3 = new Tuple(1, 2, 3);
//     const t4 = new Tuple("x", "y", "z");
//     const zipped = zip(t3, t4);
//     console.log(`Zip ${t3} and ${t4}: ${zipped}`);
//     console.log();

//     console.log("=== Comparison Example ===");
//     const t5 = new Tuple(1, 2, 3);
//     console.log(`Contains 2 in t5: ${t5.contains(2)}`);
//     console.log(`Index of 3 in t5: ${t5.indexOf(3)}`);
// }
