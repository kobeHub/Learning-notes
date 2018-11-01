# 错误处理以及异常捕获

[TOC]

## 1. Error 基本错误类型

在javascript中`Error` 实例对象是最一般的错误类型,基于该基本错误类型派生出6个具体的错误类型,同时可以定义自定义的错误类型,对于一个Error对象必须具有`message` 属性,但是对于大多数的js引擎而言,一般都会定义`name` `stack` 属性

```javascript
function throwit() {
  throw new Error('');
}

function catchit() {
  try {
    throwit();
  } catch(e) {
    console.log(e.stack); // print stack trace
  }
}

catchit()
// Error
//    at throwit (~/examples/throwcatch.js:9:11)
//    at catchit (~/examples/throwcatch.js:3:9)
//    at repl:1:5
```

## 2. 派生错误类型

+ `SyntaxError` 

  js引擎解析代码时发生的语法错误

+ `RefferenceError`

  对象引用一个不存在的变量时发生的错误,或者将将一个值分配给不可以分配的对象时`this = 1`

+ `RangeError`

  对象是一个值超出有效范围时发生的错误。主要有几种情况，一是数组长度为负数，二是`Number`对象的方法参数超出范围，以及函数堆栈超过最大值。

+ `TypeError`

  `TypeError`对象是变量或参数不是预期类型时发生的错误。比如，对字符串、布尔值、数值等原始类型的值使用`new`命令，就会抛出这种错误，因为`new`命令的参数应该是一个构造函数。

  ```javascript
  new 123
  // Uncaught TypeError: number is not a func
  
  var obj = {};
  obj.unknownMethod()
  // Uncaught TypeError: obj.unknownMethod is not a function
  ```

  上面代码的第二种情况，调用对象不存在的方法，也会抛出`TypeError`错误，因为`obj.unknownMethod`的值是`undefined`，而不是一个函数。

+ `URIError`

  `URIError`对象是 URI 相关函数的参数不正确时抛出的错误，主要涉及`encodeURI()`、`decodeURI()`、`encodeURIComponent()`、`decodeURIComponent()`、`escape()`和`unescape()`这六个函数。

  ```javascript
  decodeURI('%2')
  // URIError: URI malformed
  ```

+ `EvalError`

  `eval`函数没有被正确执行时，会抛出`EvalError`错误。该错误类型已经不再使用了，只是为了保证与以前代码兼容，才继续保留。

## 3. 自定义错误

除了 JavaScript 原生提供的七种错误对象，还可以定义自己的错误对象。

```javascript
function UserError(message) {
  this.message = message || '默认信息';
  this.name = 'UserError';
}

UserError.prototype = new Error();
UserError.prototype.constructor = UserError;
```

上面代码自定义一个错误对象`UserError`，让它继承`Error`对象。然后，就可以生成这种自定义类型的错误了。

```javascript
new UserError('这是自定义的错误！');
```

 