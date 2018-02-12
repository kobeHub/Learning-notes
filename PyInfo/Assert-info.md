# Assert 断言函数

## 简介：

+ pthon assert 断言是声明其布尔值必须为真的判定，如果为假则抛出异常AssertionError

  assert用法：

			assert expression [, arguments]

+ 使用assert断言是学习python一个非常好的习惯，python assert 断言句语格式及用法很简单。在没完善一个程序之前，我们不知道程序在哪里会出错，与其让它在运行最崩溃，不如在出现错误条件时就崩溃，这时候就需要assert断言的帮助。

# 用法：

```python
>>assert 1==1
>> assert 1 == 2
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
AssertionError

>>assert 2+2==2*2
>>assert len(['my boy',12])<10
>>assert range(4)==[0,1,2,3]
>>> mylist = ['item']
>>assert len(mylist) >= 1
>>mylist.pop()
'item'
>>assert len(mylist) >= 1
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
AssertionError
```

## 使用误区：

### 1. 以assert方式作为参数异常

```python
if not isinstance(x, int)
	raise AssertionError('not an int')
#	不应该抛出断言异常应该是TypeError
#	但是，更危险的是，有一个关于assert的困扰：它可以被编译好然后从来不执行，如果你用 –O 或 –	  oo 选项运行Python，结果不保证assert表达式会运行到。当适当的使用assert时，这是未来，但      是当assert不恰当的使用时，它会让代码用-O执行时出错。
```

### 2.适用情形

+ 防御性编程

+ 运行时检查程序逻辑

+ 检查约定

+ 检查常量

+ 检查文档

  ```
  在测试代码的时候使用断言也是可接受的，是一种很方便的单元测试方法，你接受这些测试在用-O标志运行时不会做任何事。我有时在代码里使用assert False来标记没有写完的代码分支，我希望这些代码运行失败。尽管抛出NotImplementedError可能会更好。
  ```

### 3.以断言代替不必要注释

例如函数希望在执行过程中一直有数据库连接，作为不变量可以采用断言

``` python
def some_fun(args):
    assert not DB.cloesd()
...
	# code goes here

    assert not DB.closed()
    return result
```

断言也是一种防御型编程，为了防止以后版本的升级可能带来的错误理想情况下，单元测试可以完成这样的工作，可是需要面对的现实是，它们通常是没有完成的。人们可能在提交代码前会忘了运行测试代码。有一个内部检查是另一个阻挡错误的防线，尤其是那些不明显的错误，却导致了代码出问题并且返回错误的结果。

**假如现有以下语句：**

```python
#target isexpected to be one of x, y, or z, and nothing else.

if target == x:
    run_x_code()
elif
    target == y:
    run_y_code()
else:
    run_z_code()
```

**可以以断言代替注释，在违反时返回一个干净的错误：**

```python
assert target in (x, y, z)

if target == x:
    run_x_code()
elif target == y:
    run_y_code()
else：
	assert target == z
    run_z_code()
```

**还有更好的方案：**

```python
if target == x:
    run_x_code()
elif target == y:
    run_y_code()
elif  target == z:
    run_z_code()
else:
    # This can never happen. But just incase it does...
	raise RuntimeError("anunexpected error occurred")
```

