# python 修饰器的使用

## 一．万物皆对象

> 我相信真理，我更相信万物皆对象！
>
>  
>
> ​																	-------python

在python中一切都是对象，一个函数可以赋值给一个变量，一个类可以赋值给一个变量．因为函数是普通对象，所以可以作为参数传递，作为另一个函数的返回值

![class](http://media.innohub.top/180407-class.png)

**以函数作为返回值：**

```python
def outer_function():
         print "1. This is outer function!"
         def inner_function():
            print "2. This is inner function, inside outer function!"
         print "3. This is outside inner function, inside outer function!"
         return inner_function()
   

func_assign = outer_function()

#######################Output######################################
1. This is outer function!
3. This is outside inner function, inside outer function!
2. This is inner function, inside outer function!
```

## 二．函数修饰器的使用　(Function Decorator)

python函数修饰符通过连接一个函数对象`@decor`，在调用一个callable 的函数对象`A`时将`A`传入`decor`中并执行函数，可以嵌套执行

```python
def test(f):  
    print "before ..."  
    f()  
    print "after ..."  
   
@test  
def func():  
    print "func was called"  
    
#############################output##################################
#before ...  
#func was called  
#after ...  
```

____

**使用修饰符展现函数运行时间：**

```python
#!/usr/bin/python3

import time

def time_count(input_func):
	
	def timed(*args, **kwargs):
		print('进入修饰器！')
		start = time.time()
		result = input_func(*args, *kwargs)
		end = time.time()

		print('Method name:{0}, Arg:{1}, Kwargs{2}, time:{3:.3f}ms'.format(
			input_func.__name__,
			args,
			kwargs,
			(end-start)*1000))
		return result
	return timed


@time_count
def test(*args, **kwargs):
	#time.sleep(1)
	print('被修饰函数内部')
	print(args, kwargs)


test([56252, 'test'], name='ano', bar=9)

```

![decor](http://media.innohub.top/180407-decor.png)

将函数ｔｅｓｔ作为参数传递给`time_count()` 在修饰器内部，test是参数ｉｎｐｕｔ，ｔｅｓｔ的参数被传递给ｔｉｍｅｄ

## 三．类中方法修饰器（Method Decorator）

根据一切皆对象的概念，可以将类中的方法作为类的属性进行操作，方法装饰器允许通过装饰来覆盖类属性，而不必查找调用函数。

```python
#!/usr/bin/python3

# 类中方法的修饰符，在不需要找到调用函数时改写类的属性
# ｐｙｔｈｏｎ中一切皆对象，可以通过修饰符以函数为参数调用修饰器函数，
# 传入的函数一定是可调用的   callabled  

def method_decor(method):

	def inner(city):
		if city.name == 'LosA':
			print('It\'s a great city!')
		else:
			method(city)

	return inner


class City(object):

	def __init__(self, name):
		self.name = name

	@method_decor
	def print_test(self):
		print(self.name)


c1 = City('LosA')
c1.print_test()
c2 = City('Beijing')
c2.print_test()

####################output#####################################

It's a great city!
Beijing
```

依旧方法的参数被传入了装饰器的内部函数，而方法本身被作为参数传给装饰器函数，在这种情形下，不需要在类内修改方法即可达到目的

## 四．　类装饰器（Ｃlass Decorator）

如果需要装饰器返回一个对象，并且可以对被装饰的可调用对象添加属性或者进行操作，可以定义一个装饰器类

```python
class decoclass(object):

    def __init__(self, f):
    	self.f = f

    def __call__(self, *args, **kwargs):
    	# before f actions
    	print('decorator initialised')
    	self.f(*args, **kwargs)
    	print('decorator terminated')
    	# after f actions

@decoclass
def klass():
    print('class')

klass()

###############################output#########################
decorator initialised
class
decorator terminated

```



## 五．多装饰器同时操作　(Chain Decorators)

A function definition may be wrapped by one or more [decorator](https://docs.python.org/2/glossary.html#term-decorator) expressions. Decorator expressions are evaluated when the function is defined, in the scope that contains the function definition. The result must be a callable, which is invoked with the function object as the only argument. The returned value is bound to the function name instead of the function object. Multiple decorators are applied in nested fashion. For example, the following code:

```python
@f1(arg)
@f2
def func(): 
	pass
等价于：
def func(): pass
func = f1(arg)(f2(func))
```

即f1返回值作为函数，参数是f2执行完毕后的返回值，注意最先执行的函数是f2即从下往上执行s

```python
#!/usr/bin/python3
class decoclass(object):

    def __init__(self, f):
    	self.f = f

    def __call__(self, *args, **kwargs):
    	# before f actions
    	print('decorator initialised')
    	print('进入装饰器类')
    	self.f(*args, **kwargs)
    	print('decorator terminated')
    	# after f actions


def show(func):
	print('进入装饰器函数')
	def inner(*args, **kwargs):
		print('进入内部函数')
		print('参数为：', args, kwargs)
	return inner　　# 必须返回一个函数对象，因为其上层调用是一个装饰器

@decoclass
@show
def klass(*args, *))*kwargs):
    print('class')

klass(1, 2, 'class',[1, 3], name='test', id=1)
```

*输出：*

```
进入装饰器函数

decorator initialised

进入装饰器类

进入内部函数

参数为： (1, 2, 'class', [1, 3]) {'name': 'test', 'id': 1}

decorator terminated
```

## 六．装饰时防止信息丢失

如果直接使用装饰器可能导致原方法或者类的属性丢失，如下实例：

```python
#!/usr/bin/python3

def decorator(func):
	"""decorator docstring is here!"""
	def inner(*args):
		"""inner docstring is here!"""
		print(func.__name__+' was called')
		return func(*args)
	return inner


@decorator
def func(x):
	"""Trible times"""
	return x**3

print(func(3))
print(func.__name__)
print(func.__doc__)
```

**输出：**

![wrap](http://media.innohub.top/180408-wrap.png)

由于func函数在inner中调用，其信息被inner函数的属性覆盖，可以通过`functools.wraps` 来避免丢失



```python
#!/usr/bin/python3
from functools import wraps

def decorator(func):
	"""decorator docstring is here!"""
	@wraps(func)
	def inner(*args):
		"""inner docstring is here!"""
		print(func.__name__+' was called')
		return func(*args)
	return inner


@decorator
def func(x):
	"""Trible times"""
	return x**3


print(func(3))
print(func.__name__)
print(func.__doc__)
#####################################output###############################
func was called
27
func
Trible times
```

## 七．向装饰器传递参数

```python
#!/usr/bin/python3

class Decorator(object):

	def __init__(self, arg1, arg2):
		print('参数传递给装饰器类，分别为：{0} and {1}'.format(arg1, arg2))
		self.arg1 = arg1
		self.arg2 = arg2

	def __call__(self, func,  *args, **kwargs):

		def inner(*args, **kwargs):
			print('参数传递给内部方法：%s and %s' % (self.arg1, self.arg2))
			return func(*args, **kwargs)
		return inner


@Decorator('kobe', 'Brany')
def print_(*args):
	for arg in args:
		print(arg, end='')


print_(1,2,3)


参数传递给装饰器类，分别为：kobe and Brany
参数传递给内部方法：kobe and Brany
123
```





