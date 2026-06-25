class HashNode<K, V> {
  constructor(public key: K, public value: V) {}
  next: HashNode<K, V> | null = null;
}

class HashMap<K, V> {
  private _capacity: number;
  private _size: number = 0;
  private _buckets: Array<HashNode<K, V> | null>;
  private readonly _loadFactor: number = 0.7;

  constructor(initialCapacity: number = 16) {
    this._capacity = initialCapacity;
    this._buckets = new Array<HashNode<K, V> | null>(initialCapacity).fill(null);
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

  private _hash(key: K): number {
    const keyStr = String(key);
    let hash = 0;
    for (const char of keyStr) {
      hash = (hash << 5) - hash + char.charCodeAt(0);
      hash = hash & hash;
    }
    return Math.abs(hash) % this._capacity;
  }

  private _resize(): void {
    const oldBuckets = this._buckets;
    this._capacity *= 2;
    this._size = 0;
    this._buckets = new Array<HashNode<K, V> | null>(this._capacity).fill(null);

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
      this._resize();
    }

    const index = this._hash(key);
    let current = this._buckets[index];

    while (current) {
      if (current.key === key) {
        current.value = value;
        return;
      }
      current = current.next;
    }

    const newNode = new HashNode(key, value);
    newNode.next = this._buckets[index];
    this._buckets[index] = newNode;
    this._size++;
  }

  get(key: K): V | undefined {
    const index = this._hash(key);
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
    const index = this._hash(key);
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
    const keysList: K[] = [];
    for (const bucket of this._buckets) {
      let current = bucket;
      while (current) {
        keysList.push(current.key);
        current = current.next;
      }
    }
    return keysList;
  }

  values(): V[] {
    const valuesList: V[] = [];
    for (const bucket of this._buckets) {
      let current = bucket;
      while (current) {
        valuesList.push(current.value);
        current = current.next;
      }
    }
    return valuesList;
  }

  entries(): [K, V][] {
    const entriesList: [K, V][] = [];
    for (const bucket of this._buckets) {
      let current = bucket;
      while (current) {
        entriesList.push([current.key, current.value]);
        current = current.next;
      }
    }
    return entriesList;
  }

  clear(): void {
    this._buckets = new Array<HashNode<K, V> | null>(this._capacity).fill(null);
    this._size = 0;
  }

  update(other: HashMap<K, V>): void {
    for (const [key, value] of other.entries()) {
      this.set(key, value);
    }
  }

  forEach(callback: (value: V, key: K, map: HashMap<K, V>) => void): void {
    for (const bucket of this._buckets) {
      let current = bucket;
      while (current) {
        callback(current.value, current.key, this);
        current = current.next;
      }
    }
  }

  [Symbol.iterator](): Iterator<[K, V]> {
    let bucketIndex = -1;
    let current: HashNode<K, V> | null = null;
    const buckets = this._buckets;

    return {
      next(): IteratorResult<[K, V]> {
        if (current?.next) {
          current = current.next;
          return { value: [current.key, current.value], done: false };
        }

        bucketIndex++;
        while (bucketIndex < buckets.length) {
          if (buckets[bucketIndex]) {
            current = buckets[bucketIndex];
            return { value: [current!.key, current!.value], done: false };
          }
          bucketIndex++;
        }

        return { value: undefined, done: true };
      }
    };
  }

  keysIterable(): Iterable<K> {
    return {
      [Symbol.iterator]: () => {
        let bucketIndex = -1;
        let current: HashNode<K, V> | null = null;
        const buckets = this._buckets;

        return {
          next(): IteratorResult<K> {
            if (current?.next) {
              current = current.next;
              return { value: current.key, done: false };
            }

            bucketIndex++;
            while (bucketIndex < buckets.length) {
              if (buckets[bucketIndex]) {
                current = buckets[bucketIndex];
                return { value: current!.key, done: false };
              }
              bucketIndex++;
            }

            return { value: undefined, done: true };
          }
        };
      }
    };
  }

  valuesIterable(): Iterable<V> {
    return {
      [Symbol.iterator]: () => {
        let bucketIndex = -1;
        let current: HashNode<K, V> | null = null;
        const buckets = this._buckets;

        return {
          next(): IteratorResult<V> {
            if (current?.next) {
              current = current.next;
              return { value: current.value, done: false };
            }

            bucketIndex++;
            while (bucketIndex < buckets.length) {
              if (buckets[bucketIndex]) {
                current = buckets[bucketIndex];
                return { value: current!.value, done: false };
              }
              bucketIndex++;
            }

            return { value: undefined, done: true };
          }
        };
      }
    };
  }

  entriesIterable(): Iterable<[K, V]> {
    return this;
  }

  toString(): string {
    const items = this.entries().map(([k, v]) => `${JSON.stringify(k)}: ${JSON.stringify(v)}`);
    return `HashMap({${items.join(', ')}})`;
  }
}

class BitMap {
  private readonly _size: number;
  private _bits: number[];

  constructor(size: number = 64) {
    this._size = size;
    this._bits = new Array<number>((size + 63) >> 6).fill(0);
  }

  get size(): number {
    return this._size;
  }

  private _getIndexAndMask(bit: number): [number, number] {
    if (bit < 0 || bit >= this._size) {
      throw new RangeError(`Bit index out of range: ${bit}`);
    }
    const index = bit >> 6;
    const mask = 1 << (bit & 63);
    return [index, mask];
  }

  set(bit: number): void {
    const [index, mask] = this._getIndexAndMask(bit);
    this._bits[index] |= mask;
  }

  clear(bit: number): void {
    const [index, mask] = this._getIndexAndMask(bit);
    this._bits[index] &= ~mask;
  }

  toggle(bit: number): void {
    const [index, mask] = this._getIndexAndMask(bit);
    this._bits[index] ^= mask;
  }

  get(bit: number): boolean {
    const [index, mask] = this._getIndexAndMask(bit);
    return (this._bits[index] & mask) !== 0;
  }

  setAll(): void {
    for (let i = 0; i < this._bits.length; i++) {
      this._bits[i] = 0xFFFFFFFFFFFFFFFF;
    }
  }

  clearAll(): void {
    for (let i = 0; i < this._bits.length; i++) {
      this._bits[i] = 0;
    }
  }

  countSetBits(): number {
    let count = 0;
    for (const word of this._bits) {
      count += word.toString(2).replace(/0/g, '').length;
    }
    return count;
  }

  findFirstSet(): number | null {
    for (let i = 0; i < this._size; i++) {
      if (this.get(i)) {
        return i;
      }
    }
    return null;
  }

  findFirstClear(): number | null {
    for (let i = 0; i < this._size; i++) {
      if (!this.get(i)) {
        return i;
      }
    }
    return null;
  }

  forEach(callback: (index: number, value: boolean) => void): void {
    for (let i = 0; i < this._size; i++) {
      callback(i, this.get(i));
    }
  }

  [Symbol.iterator](): Iterator<boolean> {
    let current = -1;
    const size = this._size;
    const get = this.get.bind(this);

    return {
      next(): IteratorResult<boolean> {
        current++;
        if (current < size) {
          return { value: get(current), done: false };
        }
        return { value: undefined, done: true };
      }
    };
  }

  bitsIterable(): Iterable<boolean> {
    return this;
  }

  setBitsIterable(): Iterable<number> {
    return {
      [Symbol.iterator]: () => {
        let current = -1;
        const size = this._size;
        const get = this.get.bind(this);

        return {
          next(): IteratorResult<number> {
            current++;
            while (current < size) {
              if (get(current)) {
                return { value: current, done: false };
              }
              current++;
            }
            return { value: undefined, done: true };
          }
        };
      }
    };
  }

  toString(): string {
    let bitsStr = '';
    for (let i = 0; i < this._size; i++) {
      bitsStr += this.get(i) ? '1' : '0';
    }
    return `BitMap(${bitsStr})`;
  }
}

class HashSet<T> {
  private _map: HashMap<T, boolean>;

  constructor(initialCapacity: number = 16) {
    this._map = new HashMap<T, boolean>(initialCapacity);
  }

  get size(): number {
    return this._map.size;
  }

  get isEmpty(): boolean {
    return this._map.isEmpty;
  }

  add(item: T): void {
    this._map.set(item, true);
  }

  remove(item: T): boolean {
    return this._map.delete(item);
  }

  has(item: T): boolean {
    return this._map.has(item);
  }

  clear(): void {
    this._map.clear();
  }

  items(): T[] {
    return this._map.keys();
  }

  forEach(callback: (item: T, set: HashSet<T>) => void): void {
    this._map.forEach((_, key) => {
      callback(key, this);
    });
  }

  union(other: HashSet<T>): HashSet<T> {
    const result = new HashSet<T>(Math.max(this.size, other.size) + 1);
    for (const item of this) {
      result.add(item);
    }
    for (const item of other) {
      result.add(item);
    }
    return result;
  }

  intersection(other: HashSet<T>): HashSet<T> {
    const result = new HashSet<T>();
    const smaller = this.size <= other.size ? this : other;
    const larger = this.size <= other.size ? other : this;
    for (const item of smaller) {
      if (larger.has(item)) {
        result.add(item);
      }
    }
    return result;
  }

  difference(other: HashSet<T>): HashSet<T> {
    const result = new HashSet<T>();
    for (const item of this) {
      if (!other.has(item)) {
        result.add(item);
      }
    }
    return result;
  }

  symmetricDifference(other: HashSet<T>): HashSet<T> {
    const result = new HashSet<T>();
    for (const item of this) {
      if (!other.has(item)) {
        result.add(item);
      }
    }
    for (const item of other) {
      if (!this.has(item)) {
        result.add(item);
      }
    }
    return result;
  }

  isSubset(other: HashSet<T>): boolean {
    if (this.size > other.size) {
      return false;
    }
    for (const item of this) {
      if (!other.has(item)) {
        return false;
      }
    }
    return true;
  }

  isSuperset(other: HashSet<T>): boolean {
    return other.isSubset(this);
  }

  [Symbol.iterator](): Iterator<T> {
    return this._map.keysIterable()[Symbol.iterator]();
  }

  iterable(): Iterable<T> {
    return this;
  }

  toString(): string {
    const items = this.items().map(item => JSON.stringify(item));
    return `HashSet({${items.join(', ')}})`;
  }
}

// 测试代码
function testHashMap() {
  console.log('=' .repeat(50));
  console.log('HashMap 示例:');
  console.log('=' .repeat(50));
  const map1 = new HashMap<string, number>();
  map1.set('one', 1);
  map1.set('two', 2);
  map1.set('three', 3);
  console.log(`HashMap: ${map1}`);
  console.log(`大小: ${map1.size}`);
  console.log(`包含 'two': ${map1.has('two')}`);
  console.log(`'two' 的值: ${map1.get('two')}`);
  console.log(`键: ${JSON.stringify(map1.keys())}`);
  console.log(`值: ${JSON.stringify(map1.values())}`);
  console.log();

  // 测试HashMap的可遍历结构
  console.log('=== 测试HashMap的可遍历结构 ===');
  // 使用forEach方法遍历
  console.log('使用forEach方法遍历HashMap:');
  map1.forEach((value, key) => {
    console.log(`${key}: ${value}`);
  });
  
  // 使用for-in循环遍历（通过Symbol.iterator）
  console.log('使用for-in循环遍历HashMap:');
  for (const [key, value] of map1) {
    console.log(`${key}: ${value}`);
  }
  
  // 使用keysIterable遍历
  console.log('使用keysIterable遍历HashMap键:');
  for (const key of map1.keysIterable()) {
    console.log(key);
  }
  
  // 使用valuesIterable遍历
  console.log('使用valuesIterable遍历HashMap值:');
  for (const value of map1.valuesIterable()) {
    console.log(value);
  }
  console.log();
}

function testBitMap() {
  console.log('=' .repeat(50));
  console.log('BitMap 示例:');
  console.log('=' .repeat(50));
  const bitmap = new BitMap(10);
  bitmap.set(0);
  bitmap.set(2);
  bitmap.set(5);
  console.log(`BitMap: ${bitmap}`);
  console.log(`设置的位数: ${bitmap.countSetBits()}`);
  console.log(`第 2 位: ${bitmap.get(2)}`);
  bitmap.toggle(2);
  console.log(`翻转第 2 位后: ${bitmap}`);
  console.log();

  // 测试BitMap的可遍历结构
  console.log('=== 测试BitMap的可遍历结构 ===');
  // 使用forEach方法遍历
  console.log('使用forEach方法遍历BitMap:');
  bitmap.forEach((index, value) => {
    console.log(`${index}: ${value}`);
  });
  
  // 使用for-in循环遍历（通过Symbol.iterator）
  console.log('使用for-in循环遍历BitMap:');
  for (const value of bitmap) {
    console.log(value);
  }
  
  // 使用setBitsIterable遍历
  console.log('使用setBitsIterable遍历BitMap设置的位:');
  for (const index of bitmap.setBitsIterable()) {
    console.log(index);
  }
  console.log();
}

function testHashSet() {
  console.log('=' .repeat(50));
  console.log('HashSet 示例:');
  console.log('=' .repeat(50));
  const set1 = new HashSet<number>();
  set1.add(1);
  set1.add(2);
  set1.add(3);
  const set2 = new HashSet<number>();
  set2.add(3);
  set2.add(4);
  set2.add(5);
  console.log(`HashSet1: ${set1}`);
  console.log(`HashSet2: ${set2}`);
  console.log(`并集: ${set1.union(set2)}`);
  console.log(`交集: ${set1.intersection(set2)}`);
  console.log(`差集 (HashSet1 - HashSet2): ${set1.difference(set2)}`);
  console.log(`包含 2: ${set1.has(2)}`);
  console.log();

  // 测试HashSet的可遍历结构
  console.log('=== 测试HashSet的可遍历结构 ===');
  // 使用forEach方法遍历
  console.log('使用forEach方法遍历HashSet:');
  set1.forEach((item) => {
    console.log(item);
  });
  
  // 使用for-in循环遍历（通过Symbol.iterator）
  console.log('使用for-in循环遍历HashSet:');
  for (const item of set1) {
    console.log(item);
  }
  
  // 使用iterable遍历
  console.log('使用iterable遍历HashSet:');
  for (const item of set1.iterable()) {
    console.log(item);
  }
  console.log();
}

// 运行测试
testHashMap();
testBitMap();
testHashSet();
