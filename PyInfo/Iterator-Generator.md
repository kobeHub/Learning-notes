# Python3 迭代器与生成器

# 一、关系架构

```
Python数据结构中常用的概念：
容器(container)、可迭代对象(iterable)、迭代器(itertor)、生成器(generator)的关系及概念如下
```

![image](https://github.com/kobeHub/Hello-world/blob/master/pic/relationships.png)


# 二、详细描述

## 1.容器 container

由关系图可知，容器由集合、元组、字典等产生，是一种可以把多个元素组织在一起的数据结构
通常把这些元素存储在内存中（但也有特例，比如迭代器和生成器对象）
常见的容器对象：
- list, deque, ….
- set, frozensets, ….
- dict, defaultdict, OrderedDict, Counter, ….
- tuple, namedtuple, …
- str

可以询问某元素是否在容器中：

```python
# 某元素是否在字典的键值中
>>> d = {'a':652, 'ds':565}
>>> assert 'a' in d
>>> assert 65226 not in d 
```

## 2.可迭代对象

很多容器是可迭代对象，所有可以返回一个迭代器的对象都是一个可迭代对象

比如一个打开的`file` ,`socket`

例如：

```python
>>> x = [1, 2, 3]
>>> y = iter(x)
>>> z = iter(x)
>>> next(y)
1
>>> next(y)
2
>>> next(z)
1
>>> type(x)
<class 'list'>
>>> type(y)
<class 'list_iterator'>
```

​	这里`x`是一个可迭代对象，可迭代对象和容器一样是一种通俗的叫法，并不是指某种具体的数据类型，list是可迭代对象，dict是可迭代对象，set也是可迭代对象。`y`和`z`是两个独立的迭代器，迭代器内部持有一个状态，该状态用于记录当前迭代所在的位置，以方便下次迭代的时候获取正确的元素。迭代器有一种具体的迭代器类型，比如`list_iterator`，`set_iterator`。可迭代对象实现了`__iter__`方法，该方法返回一个迭代器对象。

```python
# 当运行代码
x =  [1, 2, 3]
for ele in x:
    pass
```

**实际执行情况：**

![image](https://github.com/kobeHub/Hello-world/blob/master/pic/iterable-vs-iterator.png)


## 3.迭代器

>迭代器(itertor)是可以记住遍历位置的对象，是访问集合元素的一种方式，从第一个元素开始直到遍历结束     任何实现了`__iter__` `__next__` 方法的都是迭代器前者返回迭代器自身，后者返回下一个值，若无其他元素则抛出`StopIteration`异常

>基本方法：
>
>:black_flag:  iter():传入一个序列对象，建立其对应的迭代器对象
>
>:black_flag:  next():访问下一个元素

**实例：**

```python
list = [1, 2, 3, 4]
it = iter(list)
print(next(it))
>>> 1
print(next(it))
>>>2

list=[1,2,3,4]
it = iter(list)    # 创建迭代器对象
for x in it:
    print (x, end=" ")
输出：
1 2 3 4

import sys         # 引入 sys 模块
 
list=[1,2,3,4]
it = iter(list)    # 创建迭代器对象
 
while True:
    try:
        print (next(it))
    except StopIteration:
        sys.exit()
```

**生成无限序列：**

```python
>>> from itertools import cycle
>>> col = cycle(['red', 'yellow', 'blue'])
>>> col
<itertools.cycle object at 0x7f060aeeb438>
>>> next(col)
'red'
>>> next(col)
'yellow'
>>> next(col)
'blue'
>>> next(col)
'red'
>>> next(col)
'yellow'
>>> next(col)
'blue'
```

**由无限序列截取有限序列：**

```python
>>> from itertools import islice
>>> co = cycle(['red', 'yellow', 'blue'])
>>> lim = islice(co, 0, 5)
>>> for x in lim:
...     print(x, end=' ')
... 
red yellow blue red yellow 
```

**以斐波那契数列定义迭代器：**

```python
class Fib:
    def __init__(self):
        self.prev = 0
        self.curr = 1
 
    def __iter__(self):
        return self
 
    def __next__(self):
        value = self.curr
        self.curr += self.prev
        self.prev = value
        return value
 
>>> f = Fib()
>>> list(islice(f, 0, 10))
[1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
```

> Fib既是一个可迭代对象（因为它实现了`__iter__`方法），又是一个迭代器（因为实现了`__next__`方法）。实例变量`prev`和`curr`用户维护迭代器内部的状态。每次调用`next()`方法的时候做两件事：
>
> 1. 为下一次调用`next()`方法修改状态
> 2. 为当前这次调用生成返回结果
>
> 迭代器就像一个懒加载的工厂，等到有人需要的时候才给它生成值返回，没调用的时候就处于休眠状态等待下一次调用。

## 4.生成器

​	生成器算得上是Python语言中最吸引人的特性之一，生成器其实是一种特殊的迭代器，不过这种迭代器更加优雅。它不需要再像上面的类一样写`__iter__()`和`__next__()`方法了，只需要一个`yiled`关键字。

 生成器一定是迭代器（反之不成立），因此任何生成器也是以一种懒加载的模式生成值。用生成器来实现斐波那契数列的例子是：

```python
def fib():
    prev, curr = 0, 1
    while True:
        yield curr
        prev, curr = curr, curr + prev
 
>>> f = fib()
>>> list(islice(f, 0, 10))
[1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
```

`fib`就是一个普通的python函数，它特殊的地方在于函数体中没有`return`关键字，函数的返回值是一个生成器对象。当执行`f=fib()`返回的是一个生成器对象，此时函数体中的代码并不会执行，只有显示或隐示地调用next的时候才会真正执行里面的代码。

生成器在Python中是一个非常强大的编程结构，可以用更少地中间变量写流式代码，此外，相比其它容器对象它更能节省内存和CPU，当然它可以用更少的代码来实现相似的功能。现在就可以动手重构你的代码了，但凡看到类似：

```python
def something():
    result = []
    for ... in ...:
        result.append(x)
    return result
## 转换成生成器函数

def iter_something():
    for ... in ...:
        yield x
```

## 5.生成器表达式

生成器表达式是以列表推导式的方式建立生成器的版本，但是返回的是一个生成器对象

```python
>>> a = (a**2 for a in range(10))
>>> a
<generator object <genexpr> at 0x7f060c5a17d8>
>>> a = (a**3 for a in range(10))
>>> a
<generator object <genexpr> at 0x7f0607ea83b8>
>>> a = (a for a in range(10))
>>> a
<generator object <genexpr> at 0x7f060c5a17d8>


>>> a = [a**3 for a in range(10)]
>>> a
[0, 1, 8, 27, 64, 125, 216, 343, 512, 729]
>>> b = tuple(a)
>>> b
(0, 1, 8, 27, 64, 125, 216, 343, 512, 729)
```

