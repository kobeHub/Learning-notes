# [重读 Python 官方文档](https://docs.python.org/3/reference/index.html)

[TOC]

## 1. Lexical analysis

### 1.1 编码方式注释

```python
# -*- encoding:<encodeing-name> -*-
# vim:fileencoding=<encoding-name>
```

由多个物理行连接而成的逻辑行需要使用`\`链接。

### 1.2 保留的类的标识符

+ `_*`:不可以使用`from module import *`导入
+ `__*`:类的 private 成员
+ `__*__`:系统定义的标识符，由解释器定义以及实现

### 1.3 字符串字面值

python3 使用的字符串使用Unicode编码，对于ASCII编码转换为 byte 处理

#### 1.3.1 string 

`stringliteral   ::=  [stringprefix](shortstring | longstring)`

可以选择添加相应的字符串前缀，可选的字符串前缀，可选的值是："r" | "u" | "R" | "U" | "f" | "F" | "fr" | "Fr" | "fR" | "FR" | "rf" | "rF" | "Rf" | "RF"

+ r 或 R：定义一个原生字符串（raw strings），不使用 \ 转义字符串，在其中的字符都当做普通字符串。
+ u 或 U：定义一个使用Unicode编码的字符串
+ f 或 F：定义一个格式化的字符串
+ 其中 `r | R` 可以和 `f | F` 联合使用

**之前的format（）函数可以在一些场景下使用f''替代**

#### 1.3.2 byte

可以选择的 byte 前缀包括： "b" | "B" | "br" | "Br" | "bR" | "BR" | "rb" | "rB" | "Rb" | "RB" | “f”

代表含义与string相似，转换为 byte 必须加 `'b'`  

#### 1.3.3 格式化字符串

```python
>>> name = "Fred"
>>> f"He said his name is {name!r}."
"He said his name is 'Fred'."
>>> f"He said his name is {repr(name)}."  # repr() is equivalent to !r
"He said his name is 'Fred'."
>>> width = 10
>>> precision = 4
>>> value = decimal.Decimal("12.34567")
>>> f"result: {value:{width}.{precision}}"  # nested fields
'result:      12.35'
>>> today = datetime(year=2017, month=1, day=27)
>>> f"{today:%B %d, %Y}"  # using date format specifier
'January 27, 2017'
>>> number = 1024
>>> f"{number:#0x}"  # using integer format specifier
'0x400'
```

### 1.4 数字字面值

#### 1.4.1 不同进制

+ binary ::= 0 (b|B)[_] 0-1
+ oct ::= 0 (O|o) [_] 0-7
+ hex ::= 0(x|X) [_] 0-f
+ 指数： num (e|E) num
+ 虚数： float|digit (j|J)

可以在数字字面值中使用`_`作为分隔符，便于阅读，不可以位于数字末尾。可以位于`0b_10001_0`

## 2. 数据类型

### 2.1 基本数据类型

**一下数据类型可能包含一些特定参数，但是在以后的版本中可能变化**

+ None：内置空值

+ NotImplemented： 内置未实现类型

+ Ellipsis： 内置省略类型，可以通过`...`访问可能在数字计算中使用到，例如获取一个矩阵的一个列：`array[..., 0]` 

+ numbers.Integer

  + int
  + bool: False => 0, others => True (Not equal)

+ numbers.Real float

+ numbers.Complex 具有两个只读的属性:

  + real
  + imag

+ Sequences

  + 不可变序列

    > + String: 可表示范围 U+0000 - U+10FFFF，python 没有 char 类型，最基本的字符类型使用长度为1的string表示，`ord()`方法将一个码点转换为string,`chr()`将一个String转换为 int
    > + Tuple
    > + Bytes

  + 可变序列

    > + Lists
    > + bytes array

+ 集合类型：

  + sets： `set()`方法构建，`add()`添加
  + 不可变集合： frozenset() 

+ 映射类型：

  mapping

## 3. 包管理

python 定义了两种类型的包管理方式基本的包管理方式，以及命名域的包管理。基本的包管理策略，在每一个 package 下放置一个`__init__.py`的文件。导入常规包时，将隐式执行此__init__.py文件，并且它定义的对象将绑定到包命名空间中的名称。 __init__.py文件可以包含与任何其他模块可以包含的相同的Python代码，并且Python将在导入模块时向模块添加一些其他属性。

```
parent/
    __init__.py
    one/
        __init__.py
    two/
        __init__.py
    three/
        __init__.py
```

命名空间包是各个部分的组合，其中每个部分为父包提供子包。部分可以驻留在文件系统上的不同位置。也可以在zip文件，网络或Python导入期间搜索的任何其他位置找到部分。命名空间包可能或可能不直接对应于文件系统上的对象;它们可能是没有具体表示的虚拟模块。可以使用添加`sys.path`变量获取命名域,不需要使用`__init__.py`

```python
>>> import sys
>>> sys.path += ['Lib/test/namespace_pkgs/project1', 'Lib/test/namespace_pkgs/project2']
>>> import parent.child.one
>>> parent.__path__
_NamespacePath(['Lib/test/namespace_pkgs/project1/parent', 'Lib/test/namespace_pkgs/project2/parent'])
>>> parent.child.__path__
_NamespacePath(['Lib/test/namespace_pkgs/project1/parent/child', 'Lib/test/namespace_pkgs/project2/parent/child'])
>>> import parent.child.two
>>>
```

对应的目录结构：

```
Lib/test/namespace_pkgs
    project1
        parent
            child
                one.py
    project2
        parent
            child
                two.py
```

## 4. Generator

对于 python的可遍历类型，需要为每一个类实现`__iter__()`, `__next__()`方法，很耗时费力，可以使用生成器简化操作。可以在一个函数或者表达式中使用`yield`关键字构建一个generator，也可以使用generator 产生式:

```python
# Intialize the list
my_list = [1, 3, 6, 10]

a = (x**2 for x in my_list)
# Output: 1
print(next(a))

# Output: 9
print(next(a))

# Output: 36
print(next(a))

# Output: 100
print(next(a))

# Output: StopIteration
next(a)
sum(x**2 for x in my_list)
```

## 5.异步

### 5.1 协程

由于GIL的存在，导致python的多线程性能甚至比单线程还慢。

> GIL: 全局解释器锁（英语：Global Interpreter Lock，缩写GIL），是计算机程序设计语言解释器用于同步线程的一种机制，它使得任何时刻仅有一个线程在执行。[1]即便在多核心处理器上，使用 GIL 的解释器也只允许同一时间执行一个线程。

所以出现了协程的概念，与Golang中的Goroutines不同，python的coroutines是基于`yield`操作进行的，由于进行`yield`时空出了cpu的使用，可以切换到其他的操作，借以实现并发操作。

> 协程: 协程，又称微线程，纤程，英文名Coroutine。协程的作用，是在执行函数A时，可以随时中断，去执行函数B，然后中断继续执行函数A（可以自由切换）。但这一过程并不是函数调用（没有调用语句），这一整个过程看似像多线程，然而协程只有一个线程执行.

python 3.4中内置了 asychio库，在python 3.5 中使用了`asych`, `await`关键字用以替代`@coroutines`,`yield/yield from`. python对于携程的支持，是基于一些特定规则的生成器。一个生成器的主要操作有以下三个：

+ `close()`: 关闭一个生成器，并且raise GeneratorExit
+ `send(arg)`:把一个参数送入生成器
+ `next()`获取生成器的下一个元素

对于启动或者恢复一个生成器的执行，可以使用三方式：`next(), __next__(), g.send(None)`

### 5.2 `async`, `await`基于协程的异步调用 

对于一个基本的异步函数，可以使用关键字`async`定义一个异步函数，直接调用一个异步函数，不会执行相应的结果而是返回一个`coroutine`对象。称之为`Navie coroutine`，除此之外，还可以使用装饰器`types.coroutine`,`asycnio.coroutine`获得一个navie coroutine。**注意对于`await`操作只可以等待一个naive coroutine**

对于一个异步函数的调用可以使用`send(None)`进行唤醒，因为其本质是一个generator：

```python
print(async_function().send(None))

StopIteration: 1
```

但是这样的直接调用会造成一个停止迭代的异常，可以定义一个如下的函数进行异常处理：（或者直接使用`asyncio.run()`）

```python
def run(coroutine):
    try:
        coroutine.send(None)
    except StopIteration as e:
        return e.value
```

在一个协程函数中，使用`await`关键字将自身挂起，并且等待其后的协程，等待其指向的协程结束后继续执行。`await`等待的协程对象必须是一个`Awaitable`的子类，只需要实现了`__await__`方法即可,主要有以下三种`Awaitable`, coroutine, Tasks, Futures。

```python
class Awaitable(metaclass=ABCMeta):
    __slots__ = ()

    @abstractmethod
    def __await__(self):
        yield

    @classmethod
    def __subclasshook__(cls, C):
        if cls is Awaitable:
            return _check_methods(C, "__await__")
        return NotImplemented
```

而且可以看到，Coroutine类也继承了Awaitable，而且实现了send，throw和close方法。所以await一个调用异步函数返回的协程对象是合法的。

```python
class Coroutine(Awaitable):
    __slots__ = ()

    @abstractmethod
    def send(self, value):
        ...

    @abstractmethod
    def throw(self, typ, val=None, tb=None):
        ...

    def close(self):
        ...
        
    @classmethod
    def __subclasshook__(cls, C):
        if cls is Coroutine:
            return _check_methods(C, '__await__', 'send', 'throw', 'close')
        return NotImplemented
```

使用`event_loop`执行coroutines

```python
loop = asyncio.get_event_loop()
res = loop.run_until_complete(asyncio.wait([buy_potatos(), buy_tomatos()]))
loop.close()
```

### 5.3 couroutine 高级API

**Aeaitable**：

+ coroutine：可以指向两个相似的概念，协程函数（sync def）以及协程对象(由调用一个协程函数得到的对象)
+ Tasks： 用以调控协程并发执行，可以使用一个`asycnio.create_task()`函数将一个协程包装为一个Task对象并且自动按顺序执行
+ Futures： 低层API
+ `asyncio.creat_task(coro)`:
  创建一个Task并且调度其执行
+ `asyncio.sleep（ delay, result=None, *, loop=None）`:
  阻塞调用一定时间,如果给定了result，那么当该协程执行结束后将result返回给调用者。使用该操作可以挂起当前协程让其他协程获得cpu时间

**并发执行程序：**

+ `asyncio.gather(*aws, loop=None, return_exceptions=None)`

  序列化执行`aws`中的Awaitable 对象，如果其中的任何一个为coroutine，那么自动转换为Task对象，如果所有的Awaitable对象都成功执行，那么返回一个于输入参数对应的返回值列表。如果gather被取消了，那么所有的提交的 Awaitable 都会被取消执行，但是其中的一个协程或者Future取消执行并不影响其他的 Awaitables。

### 5.4 异步生成器

看下AsyncGenerator的定义，它需要实现__aiter__和__anext__两个核心方法，以及asend，athrow，aclose方法。

```python
class AsyncGenerator(AsyncIterator):
    __slots__ = ()

    async def __anext__(self):
        ...

    @abstractmethod
    async def asend(self, value):
        ...

    @abstractmethod
    async def athrow(self, typ, val=None, tb=None):
        ...

    async def aclose(self):
        ...

    @classmethod
    def __subclasshook__(cls, C):
        if cls is AsyncGenerator:
            return _check_methods(C, '__aiter__', '__anext__',
                                  'asend', 'athrow', 'aclose')
        return NotImplemented
```

### 5.5 异步方法

对于类的普通方法或者是类方法都可以使用`async`关键字，将其定义为异步的。

```python
class ThreeTwoOne:
    @classmethod
    async def begin(cls):
        print(3)
        await asyncio.sleep(1)
        print(2)
        await asyncio.sleep(1)
        print(1)        
        await asyncio.sleep(1)
        return
```

完成异步的代码不一定要用async/await，使用了async/await的代码也不一定能做到异步，async/await是协程的语法糖，使协程之间的调用变得更加清晰，使用async修饰的函数调用时会返回一个协程对象，await只能放在async修饰的函数里面使用，await后面必须要跟着一个协程对象或Awaitable，await的目的是等待协程控制流的返回，而实现暂停并挂起函数的操作是yield。