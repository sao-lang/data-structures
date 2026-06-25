interface HashNode<K, V> {
    key: K;
    value: V;
    next: HashNode<K, V> | null;
}

export class HashTable<K, V> {
    private _capacity: number;
    private _size: number;
    private _buckets: Array<HashNode<K, V> | null>;
    private readonly _loadFactor: number = 0.7;

    constructor(initialCapacity: number = 16) {
        this._capacity = initialCapacity;
        this._size = 0;
        this._buckets = new Array(initialCapacity).fill(null);
    }

    get size(): number {
        return this._size;
    }

    get isEmpty(): boolean {
        return this._size === 0;
    }

    get capacity(): number {
        return this._capacity;
    }

    private hash(key: K): number {
        let hash = 0;
        const str = String(key);
        for (let i = 0; i < str.length; i++) {
            hash = (hash << 5) - hash + str.charCodeAt(i);
            hash = hash & hash;
        }
        return Math.abs(hash) % this._capacity;
    }

    private resize(): void {
        const oldBuckets = this._buckets;
        this._capacity *= 2;
        this._size = 0;
        this._buckets = new Array(this._capacity).fill(null);

        for (const bucket of oldBuckets) {
            let current = bucket;
            while (current) {
                this.set(current.key, current.value);
                current = current.next;
            }
        }
    }

    set(key: K, value: V): void {
        if (this._size / this._capacity >= this._loadFactor) {
            this.resize();
        }

        const index = this.hash(key);
        let current = this._buckets[index];

        while (current) {
            if (current.key === key) {
                current.value = value;
                return;
            }
            current = current.next;
        }

        const newNode: HashNode<K, V> = { key, value, next: this._buckets[index] };
        this._buckets[index] = newNode;
        this._size++;
    }

    get(key: K): V | undefined {
        const index = this.hash(key);
        let current = this._buckets[index];

        while (current) {
            if (current.key === key) {
                return current.value;
            }
            current = current.next;
        }

        return undefined;
    }

    has(key: K): boolean {
        return this.get(key) !== undefined;
    }

    delete(key: K): boolean {
        const index = this.hash(key);
        let current = this._buckets[index];
        let prev: HashNode<K, V> | null = null;

        while (current) {
            if (current.key === key) {
                if (prev) {
                    prev.next = current.next;
                } else {
                    this._buckets[index] = current.next;
                }
                this._size--;
                return true;
            }
            prev = current;
            current = current.next;
        }

        return false;
    }

    keys(): K[] {
        const keys: K[] = [];
        for (const bucket of this._buckets) {
            let current = bucket;
            while (current) {
                keys.push(current.key);
                current = current.next;
            }
        }
        return keys;
    }

    values(): V[] {
        const values: V[] = [];
        for (const bucket of this._buckets) {
            let current = bucket;
            while (current) {
                values.push(current.value);
                current = current.next;
            }
        }
        return values;
    }

    entries(): Array<[K, V]> {
        const entries: Array<[K, V]> = [];
        for (const bucket of this._buckets) {
            let current = bucket;
            while (current) {
                entries.push([current.key, current.value]);
                current = current.next;
            }
        }
        return entries;
    }

    clear(): void {
        this._buckets = new Array(this._capacity).fill(null);
        this._size = 0;
    }

    forEach(callback: (value: V, key: K, hashTable: HashTable<K, V>) => void): void {
        for (const bucket of this._buckets) {
            let current = bucket;
            while (current) {
                callback(current.value, current.key, this);
                current = current.next;
            }
        }
    }
}
