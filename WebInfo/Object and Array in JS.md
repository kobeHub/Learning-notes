# javascript 中的object array

[TOC]

## 1. Object 数据类型

与python中的dict类型一样，object数据类型也是由减值以及数值组成的集合，属于一种无序序列对象的 所有键值均为字符串，所以加不加引号效果都一样，但是在进行引用时必须要加引号，如果键值是数值类型，也会转化为字符串。

### 键值的注意事项

- 如果键值不符合标识名的条件，（第一个字符为数值，包含空格或者运算符）则键值必须加上引号，否则会报错，所以建议所有键值都加引号
- 对象的每一个键名又被称为属性，其value可以是任何数据类型，通常把这个属性叫做方法，可以像函数一样进行调用，如果属性的值还是一个对象就可以形成链式引用

```javascript
let function_sets = {
    double : (x) => {
        return 2 * x;
    },
    abs : (a) => {
        if(a < 0) return -a;
        else return a;
    },
    inner_func:{
        debug : () => console.log('You are using the inner function'),
    },
};

console.log(`Object test: ${function_sets.double(11)}, ${function_sets.abs(-1000)}`);
function_sets.inner_func.debug();
```

对象的每一个属性都可以进行动态指定，也可以进行动态更改,在进行引用时可以直接采用`.`  或者采用python dict的引用方式，但是注意对于键值标识名不合乎规定的需要采用后者引用方式

但是需要注意，如果使用方括号进行引用，必须加上引号，否则会被当做变量处理

```javascript
let dict = {
    1:'kobe bryant',
    test:123,
    'another':111
}
console.log(`The dict is :${dict['test']}`);
dict[1] = 123;
dict.wander = 'Amazing language';
console.log(`Change dict ${dict['1']}, ${dict['wander']}`);

let foo = 'bar';
let obj = {
    foo: 1,
    bar:2,
};

obj.foo // 1
obj[foo] // 2
```

### 表达式还是语句

由于对象采用大括号表示，就导致了一个问题，如果行首是一个大括号，到底是一个对象还是一个代码块。

```javascript
{ foo: 123 }
```

JavaScript 引擎读到上面这行代码，会发现可能有两种含义。第一种可能是，这是一个表达式，表示一个包含`foo`属性的对象；第二种可能是，这是一个语句，表示一个代码区块，里面有一个标签`foo`，指向表达式`123`。

为了避免这种歧义，V8 引擎规定，如果行首是大括号，一律解释为对象。不过，为了避免歧义，最好在大括号前加上圆括号。

```javascript
({ foo: 123})
```

这种差异在`eval`语句（作用是对字符串求值）中反映得最明显。

```javascript
eval('{foo: 123}') // 123
eval('({foo: 123})') // {foo: 123}
```

上面代码中，如果没有圆括号，`eval`将其理解为一个代码块；加上圆括号以后，就理解成一个对象。

### 对象的属性参数

通过对象的属性的参数可以设置对象属性的行为，基本参数如下：

+ [[value]]: 保存属性的值
+ [[Writable]]: 确定属性值是否可以被更改
+ [[get]]: 保存`getter` ,当一个属性被read时会调用`getter` 函数
+ [[set]]: 保存`setter` 当一个属性被设置为一个值的时候,调用该函数,该函数需要传入value参数
+ [[enumerable]]: 控制某个属性是否可以被迭代
+ [[configurable]]: 为假时该属性不可被删除,不可以改变属性的其他参数(除了value), 换言之,该参数控制属性元数据的读写铨叙

**默认值如下:**

| Attribute Key    | Default value |
| ---------------- | ------------- |
| [[Value]]        | `undefined`   |
| [[Get]]          | `undefined`   |
| [[Set]]          | `undefined`   |
| [[Writable]]     | `false`       |
| [[Enumerable]]   | `false`       |
| [[Configurable]] | false         |

### 对象属性的操作

#### 1. delete

`delete` 命令用于删除对象的属性，删除成功后会返回`true` ，使用该命令会删除指定的属性，这时再读取会得到`undefined` ,而且使用`Object.keys()` 函数进行属性的查看也找不到该属性。但删除一个不存在的属性时，不会报错，而是返回true

```javascript
var obj = { p: 1 };
Object.keys(obj) // ["p"]

delete obj.p // true
obj.p // undefined
Object.keys(obj) // []
```

只有一种情况，delete命令会返回false，就是使用`defineProperty`  函数设定的属性不可更改

**`Object.defineProperty(obj, propName, proDesc)`** :

> 该函数用于创建或者修改一个对象obj的属性proName，规定其描述符

```javascript
let obj = Object.defineProperty({}, 'foo', {
    value: 123,
    enumerable: true,
    writable: false,
    configurable: false
});
```

根据该设定，将该对象的该属性设置为只读状态，不可以进行删除以及更改，进行delete时会返回false

#### 2.属性存在

`in` 运算符用于判断属性是否存在,返回boolean类型,但是`in` 运算符不能判断某个属性时object本身的还是继承得到的,`toString` 属性是继承得到的,也会返回true

此时可以借助函数`Object.hasOwnProperty()` 判断是否为自身属性

```javascript
let obj = {p: 1};
'p' in obj // true
'toString' in obj  // true

> obj
{ pre: [Getter/Setter] }
> pre in obj;
ReferenceError: pre is not defined
> 'pre' in obj;
true
> 'toString' in obj;
true
> 'tString' in obj;
false
> obj.hasOwnProperty('toString')
false
```

#### 3.属性的遍历

使用`form...in` 进行遍历操作,但是需要注意的是,只有`enumerable` 为真的属性才可以进行遍历,会跳过为假的属性;不仅遍历自身属性,而且遍历继承的属性,例如每个object都继承了`toString` 属性,但是不会被遍历到

```javascript
var obj = {};

// toString 属性是存在的
obj.toString // toString() { [native code] }

for (var p in obj) {
  console.log(p);
} // 没有任何输出

var person = { name: '老张' };

for (var key in person) {
  if (person.hasOwnProperty(key)) {
    console.log(key);
  }
}
// name
```

#### 4. with 语句

`with`语句的格式如下：

```javascript
with (对象) {
  语句;
}
```

它的作用是操作同一个对象的多个属性时，提供一些书写的方便。

```javascript
// 例一
var obj = {
  p1: 1,
  p2: 2,
};
with (obj) {
  p1 = 4;
  p2 = 5;
}
// 等同于
obj.p1 = 4;
obj.p2 = 5;

// 例二
with (document.links[0]){
  console.log(href);
  console.log(title);
  console.log(style);
}
// 等同于
console.log(document.links[0].href);
console.log(document.links[0].title);
console.log(document.links[0].style);
```

注意，如果`with`区块内部有变量的赋值操作，必须是当前对象已经存在的属性，否则会创造一个当前作用域的全局变量。

```javascript
var obj = {};
with (obj) {
  p1 = 4;
  p2 = 5;
}

obj.p1 // undefined
p1 // 4
```

上面代码中，对象`obj`并没有`p1`属性，对`p1`赋值等于创造了一个全局变量`p1`。正确的写法应该是，先定义对象`obj`的属性`p1`，然后在`with`区块内操作它。

这是因为`with`区块没有改变作用域，它的内部依然是当前作用域。这造成了`with`语句的一个很大的弊病，就是绑定对象不明确。

```javascript
with (obj) {
  console.log(x);
}
```

单纯从上面的代码块，根本无法判断`x`到底是全局变量，还是对象`obj`的一个属性。这非常不利于代码的除错和模块化，编译器也无法对这段代码进行优化，只能留到运行时判断，这就拖慢了运行速度。因此，建议不要使用`with`语句，可以考虑用一个临时变量代替`with`。

```javascript
with(obj1.obj2.obj3) {
  console.log(p1 + p2);
}

// 可以写成
var temp = obj1.obj2.obj3;
console.log(temp.p1 + temp.p2);
```

### 用作属性描述符的函数

+ **`Object.defineProperty(obj, propName, proDesc)`** :

+ **`Object.defineProperties(obj, propDescObj)`**

  ```javascript
  var obj = Object.defineProperties({}, {
      foo: {value: 111, enumerable: true},
      bar: {value: 'cdsv', enumerable: true},
  });
  ```

+ **`Object.create(proto, propDescObj?)`**

  首先根据原型参数创建一个原型为`proto` 的object,如果给出第二个参数,将此加入到属性参数列表中

  ```javascript
  var obj = Object.create(Object.prototype, {
          foo: { value: 123, enumerable: true },
          bar: { value: "abc", enumerable: true }
      });
  ```

+ **`Object.getOwnPropertyDescriptor(obj, propName)`**

  Returns the descriptor of the own (non-inherited) property of 

  obj  whose name is propName. If there is no such property, undefined is returned.

  ```javascript
      > Object.getOwnPropertyDescriptor(Object.prototype, "toString")
      { value: [Function: toString],
        writable: true,
        enumerable: false,
        configurable: true }
  
      > Object.getOwnPropertyDescriptor({}, "toString")
      undefined
  ```


## 2. eval 命令

`eval()` 接受一个字符串作为参数,并且将这个字符串参数当做语句运行,如果无法作为语句运行就会报错,同时放在该命令中的字符串应该有独自存在的意义,不可以与其他语句配合使用

```javascript
eval('var a = 1;');
eval('dsa'); // 无意义会报错
eval('return;'); // 不可以单独使用
```

注意eval函数没有自己的作用域,都在当前作用域执行,因此可能会修改当前已有变量值,造成安全问题,但是在使用严格模式时,其内部声明的作用域不会影响到外部.但是仍然可以更改外部变量的值

```javascript
 > "use strict";
'use strict'
> eval('let a = 12;')
undefined
> a
ReferenceError: a is not defined
> b = 123;
123
> b
123
> eval('b = 1');
1
```

## 2. Array 数组

类似于python中的列表类型,数组可以存储任意数据类型 ,其类型是Object,是一种特殊类型的,只是每个属性的键值是数字而且是自增的 ,可以先定义后赋值，也可以在定义时赋值。可以使用`Object.keys()` 函数得到一个数组对象的键值，即为每个元素的下标

进行数组元素读取时可以采用对象的读取方式，但是不可以对数字型键值使用点操作符，只可以使用方括号结构

```javascript
> let arr1 = [
... {a: 1},
... [1, 2, 3,],
... () => {return true;}]
undefined
> arr1
[ { a: 1 }, [ 1, 2, 3 ], [Function] ]
> arr1[2]
[Function]
> arr1[2]()
true
> arr1[1]
[ 1, 2, 3 ]
> arr1[0]
{ a: 1 }
```

### Array.length 属性

返回数组元素数量，js使用一个32位整数，保存数组的元素个数，所以最大的存储数量为（2^32^ - 1） 只要是数组就有length属性，该值是一个动态值，等于最后一个元素的键值+1

```javascript
> arr1.length
3
> arr1.length = 2
2
> arr1
[ { a: 1 }, [ 1, 2, 3 ] ]
> arr1.length = 0
0
> arr1
[]
```

可以使用`array.length = 0` 将数组清空，将数组length属性人为设置为小于当前元素数量的值，将会自动删除后面多雨的元素，将其设置为不合法的值会报错。

### in 运算符

`in` 运算符可以检查元素是否属于对象，也可以用来判断数组元素的存在情况，但是只能检查键值是否在数组中，可以用来检查某个位置是否为空位。注意length属性不过滤空位，使用delete语句删除数组元素时会将对应的位置设置为`empty item` 数组长度属性不会改变

读取已经删除的元素时，返回值为`undefined` ,对于数组中的空位，使用`forEach` 方法，`for ... in ` 结构， `Object.keys()` 都会被过滤

```javascript
> delete bu[2]
true
> bu
[ 1, 23, <1 empty item> ]
> 2 in bu
false
> bu.length
3
> delete bu[0]
true
> bu
[ <1 empty item>, 23, <1 empty item> ]
> 0 in bu
false
> 1 in bu
true
> bu[0]
undefined

for (let a in bu) {
    ...
}
```

### 数组的遍历

`for... in` 循环可以以遍历对象的方式遍历数组，也可以使用for循环，以及`forEach()` 方法

```javascript
var a = [1, 2, 3];
a.foo = true;

for (var key in a) {
  console.log(key);
}

var colors = ['red', 'green', 'blue'];
colors.forEach(function (color) {
  console.log(color);
});
```

### 类似数组的对象

如果一个对象的所有键值都为正整数或者0，并且有`length ` 属性，可以将该对象称为类似数组的对象。但是该对象中没有数组的`push（） pop()` 等方法，而且length属性不是动态值，不会随着数组成员的变化而变化

常见的类似数组的对象包括函数的`Arguments` 对象，以及大多数的dom元素

```javascript
// arguments对象
function args() { return arguments; }
var arrayLike = args('a', 'b');

arrayLike[0] // 'a'
arrayLike.length // 2
arrayLike instanceof Array // false

// DOM元素集
var elts = document.getElementsByTagName('h3');
elts.length // 3
elts instanceof Array // false

// 字符串
'abc'[1] // 'b'
'abc'.length // 3
'abc' instanceof Array // false
```

可以使用`Array.prototype.slice.call(arrayLike)` 方法将类似数组的对象转化为数组；也可以通过`call（）` 函数将数组的方法放到对象的上面

```javascript
function print(value, index) {
  console.log(index + ' : ' + value);
}

Array.prototype.forEach.call(arrayLike, print);
// 对于string的类型也可以使用该方式得到进行类似操作,但是使用这种方式比直接使用原声的forEach要慢，所以需要将数组型对象转化为数组
```

### 数组方法

####**添加删除元素:**

+ `arr.push(...items)`  将元素加入到最后,并且返回数组长度

+ `arr.pop()` 删除并返回最后一个元素

+ `arr.shift()` 删除并返回第一个元素

+ `arr.unshift(...items)` 将一个或者多个元素加入数组头部,返回数组长度

+ `arr.splice(index[, deleteCount, elm1, elm2, ..., elmN])` : 从下标index的元素开始,删除deleteCount个元素,并且将之后的参数加入到数组中,返回被删除的元素

  此函数可以作为数组操作中的一种瑞士军刀方法,可以满足数组增删改的所有需求

  ```javascript
  // delete
  arr.splice(1, 1)
  arr.splice(0)   // 删除全部
  arr.splice(1, 5)
  
  // insert
  arr.splice(0, 0, ..items)  === arr.unshift(..items)
  arr.splice(-1, 0, ..items)   //在倒数第二位插入元素
  arr.splice(1, 3, ...items)   // 删除并且插入元素arr
  
  // edit
  arr.splice(i, i, ..item)
  ```

+ `arr.slice(start, end)` 

  复制index位于[start, end)之间的所有元素为新的数组

+ `arr.concat(arg1, arg2)` 

  连接两个数组或者数组和单个元素,返回一个新的数组,注意原数组无变化

####**数组元素查找**

+ `arr.indexOf(item, from)` :从f索引为from的位置开始向右查找,返回第一个找到的索引

+ `arr.lastIndexOf(item, from)` 从右开始查找

+ `arr.includes(item, from)` 是否包含该元素 

  需要注意的是index方法使用的是绝对等运算符`===`,所以如果进行NaN的查找时,由于NaN != NaN

  所以会返回-1,可以使用includes方法

  ```javascript
  arr.indexOf(NaN)  // -1
  arr.includes(NaN)  // true
  ```

+ `find and findIndex` 

  对于一个对象的数组,使用某些条件对与对象进行查找,可以使用find方法.该方法需要传入一个函数作为参数,返回第一个使得函数返回值为真的元素

  ```javascript
  let result = arr.find(function (item, index, array) {
      // 如果找到符合条件的元素则返回true
  });
  // item 为待查找的元素
  // index 为索引
  // array 为原数组
  ```

  ```javascript
  var array1 = [5, 12, 8, 130, 44];
  
  function findFirstLargeNumber(element) {
    return element > 13;
  }
  
  console.log(array1.findIndex(findFirstLargeNumber));
  // expected output: 3
  
  
  > arr
  [ 'dsad', 44, 423, 'dsad', 'dsad', 'd', 'dsad' ]
  > arr.find((item, index, array) => index == 4);
  'dsad'
  > arr.find((item, index) => index == 4);
  'dsad'
  > arr.find((item, index, array) => array[index] == 'd');
  'd'
  > arr.find((item, index, array) => array.length == 7);
  'dsad'    // 恒为真则返回第一个值
  ```

+ `filter`

  `filter` 方法返回一个使得参数函数返回值为真的元素的数组

  ```javascript
  let results = arr.filter(function (item, index, array) {
      
  })
  
  
  // [ 44, 423, 'dsad', 'dsad', 'd' ]
  > arr.filter(item => !isNaN(item))
  [ 44, 423 ]
  > arr.filter((item, index, array) => array.length == 5);
  [ 44, 423, 'dsad', 'dsad', 'd' ]
  
  ```

#### **数组元素变换**  

+ `map`

  可以使用map函数对于数组元素进行映射,通过传入一个函数作为参数,进行转化

  ```javascript
  let result = arr.map(function (item, index, array) {
      
  });
  
  var array1 = [1, 4, 9, 16];
  // pass a function to map
  const map1 = array1.map(x => x * 2);
  console.log(map1);
  // expected output: Array [2, 8, 18, 32]
  ```

+ `sort(fn)` 

  将数组中所有元素作为字符串类型进行排序,注意数字也会转化为字符串类型,对于数字进行排序时需要传入函数进行操作.

  由于数组对象可以包含任意类型的数据,为了便于进行统一的排序操作,将所有类型都看作字符串类型进行操作,其内部实现是一个优化后的快排算法.将会遍历数组每一个元素,使用给定的函数对于数据进行重排.实际上给定的函数只要在`a > b` 时返回正值,反之返回负值即可

  ```javascript
  let arr = [ 1, 2, 15 ];
  
  arr.sort(function(a, b) { return a - b; });
  
  alert(arr);  // 1, 2, 15
  ```

+ `reverse, split, join`

  数组的反向,分割以及拼接

  ```javascript
  let names = 'Bilbo, Gandalf, Nazgul';
  
  let arr = names.split(', ');
  
  for (let name of arr) {
    alert( `A message to ${name}.` ); // A message to Bilbo  (and other names)
  }
  let arr = 'Bilbo, Gandalf, Nazgul, Saruman'.split(', ', 2);
  // Bilbo, Gandalf
  
  
  > let str = 'vdsbs'
  undefined
  > str.split('')
  [ 'v', 'd', 's', 'b', 's' ]
  ```

+ `resuce/redueRight`

  生成方式,根据传入的函数生成相应的值,并且每一个item在调用该函数时可以使用上一个itme调用的返回值,可以实现递归的操作

  ```javascript
  let value=  arr.reduce(function (previousValue, item, index, arr)) {
                         
                         }, initial);
  ```

  参数与find map filter类似,但是多了一个初始值,用于作为第一次调用的值

  ```javascript
  let arr = [1, 2, 3, 4, 5];
  let result = arr.reduce((sum, current) => sum + current, 0);
  // 15  求元素累加和
  
  ["Bilbo", "Gandalf", "Nazgul"].forEach((item, index, array) => {
    alert(`${item} is at index ${index} in ${array}`);
  });
  ```

  
