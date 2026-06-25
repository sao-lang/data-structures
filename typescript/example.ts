import { Bool, Int, Float, Str, Null, nullValue, BigNumber, BigDecimal } from './index';

console.log('=== 重要说明 ===');
console.log('JavaScript/TypeScript 不支持真正的运算符重载！');
console.log('但通过 valueOf() 和 Symbol.toPrimitive，我们可以在很多场景下像原生类型一样使用这些类型');
console.log('');

console.log('=== Bool 示例 ===');
const t = Bool.true();
const f = Bool.false();
console.log('t.value:', t.value);
console.log('f.value:', f.value);
console.log('!t.value:', !t.value);
console.log('t.value && f.value:', t.value && f.value);
console.log('t.value || f.value:', t.value || f.value);
console.log('t.value === true:', t.value === true);
console.log('Boolean(t.value):', Boolean(t.value));
console.log('t.not():', t.not().value);
console.log('t.and(f):', t.and(f).value);
console.log('');

console.log('=== Int 示例 ===');
const a = new Int(10);
const b = new Int(3);
console.log('a.value:', a.value);
console.log('b.value:', b.value);
console.log('a.value + b.value:', a.value + b.value);
console.log('a.value - b.value:', a.value - b.value);
console.log('a.value * b.value:', a.value * b.value);
console.log('a.value / b.value:', a.value / b.value);
console.log('a.value % b.value:', a.value % b.value);
console.log('a.value ** b.value:', a.value ** b.value);
console.log('a.add(b):', a.add(b));
console.log('a.subtract(b):', a.subtract(b));
console.log('a.multiply(b):', a.multiply(b));
console.log('a.divide(b):', a.divide(b));
console.log('');

console.log('=== Float 示例 ===');
const x = new Float(3.14159);
const y = new Float(2.71828);
console.log('x.value:', x.value);
console.log('y.value:', y.value);
console.log('x.value + y.value:', x.value + y.value);
console.log('x.value - y.value:', x.value - y.value);
console.log('x.value * y.value:', x.value * y.value);
console.log('x.value / y.value:', x.value / y.value);
console.log('x.add(y):', x.add(y));
console.log('x.sqrt():', x.sqrt());
console.log('Math.sin(x.value):', Math.sin(x.value));
console.log('');

console.log('=== Str 示例 ===');
const hello = new Str('Hello');
const world = new Str('World');
const space = new Str(' ');
console.log('hello.value:', hello.value);
console.log('world.value:', world.value);
console.log('hello.value + space.value + world.value:', hello.value + space.value + world.value);
console.log('hello.value === "Hello":', hello.value === 'Hello');
console.log('hello.concat(space).concat(world):', hello.concat(space).concat(world));
console.log('hello.toUpperCase():', hello.toUpperCase());
console.log('');

console.log('=== Null 示例 ===');
const n = nullValue;
console.log('n:', n);
console.log('n.isNull():', n.isNull());
console.log('');

console.log('=== BigNumber 示例 ===');
const bigA = new BigNumber('123456789012345678901234567890');
const bigB = new BigNumber('987654321098765432109876543210');
console.log('bigA:', bigA.toString());
console.log('bigB:', bigB.toString());
console.log('bigA.add(bigB):', bigA.add(bigB).toString());
console.log('bigA.multiply(bigB):', bigA.multiply(bigB).toString());
console.log('');

console.log('=== BigDecimal 示例 ===');
const decA = new BigDecimal('123.456', 3);
const decB = new BigDecimal('78.901', 3);
console.log('decA:', decA.toString());
console.log('decB:', decB.toString());
console.log('decA.add(decB):', decA.add(decB).toString());
console.log('decA.subtract(decB):', decA.subtract(decB).toString());
console.log('decA.multiply(decB):', decA.multiply(decB).toString());
console.log('');

console.log('=== 数组示例 ===');
const nums = [new Int(1), new Int(2), new Int(3), new Int(4), new Int(5)];
console.log('nums:', nums.map(n => n.value));
console.log('sum:', nums.reduce((acc, n) => acc + n.value, 0));
console.log('doubled:', nums.map(n => n.multiply(new Int(2))));
console.log('');

console.log('=== 使用 as any 绕过类型检查（可选） ===');
const aa = new Int(10) as any;
const bb = new Int(3) as any;
console.log('aa + bb:', aa + bb);
console.log('aa * bb:', aa * bb);
console.log('');

console.log('=== 总结 ===');
console.log('1. 推荐使用 .value 获取原生值后再进行运算');
console.log('2. 或者使用提供的方法（add/subtract/multiply 等）保持类型');
console.log('3. valueOf() 和 Symbol.toPrimitive 在运行时有效，但 TypeScript 编译时会检查类型');
