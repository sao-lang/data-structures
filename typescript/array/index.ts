import { FixedArray } from './FixedArray';

const fixedArray = new FixedArray(10);
for (let i = 0; i < 8; i++) {
    fixedArray.append(i);
}
console.log(fixedArray.toString());
fixedArray.addAt(3, 11);
console.log(fixedArray.toString());
fixedArray.remove(3);
console.log(fixedArray.toString());
