# 常用修饰器

+ @classmethod

  > 用classmethod修饰表示这是一个类方法,如果没有用@classmethod修饰，则表示这是一个对象方法，使用时必须与一个对象绑定

  ```python
  >>> class MyClass():
  ...     def thisIsClassMethod(self):
  ...         print ("this is a class method")
  ... 

  >>> MyClass().thisIsClassMethod()
  this is a class method
  >>> MyClass.thisIsClassMethod()
  Traceback (most recent call last):
    File "<stdin>", line 1, in <module>
  TypeError: thisIsClassMethod() missing 1 required positional argument: 'self'
  ```

  > 非类方法不可以被类本身直接调用，因为需要传入一个实例作为参数，当然也可以直接传入任一参数，但这时操作的已经不是该类的实例如：

  ```python
  >>> class vc:
  ...     def te(self):
  ...             print('test')
  ... 
  >>> vc.te()
  Traceback (most recent call last):
    File "<stdin>", line 1, in <module>
  TypeError: te() missing 1 required positional argument: 'self'
  >>> vc.te(0)
  test

  虽然可以调用但是无任何意义
  ```

  **添加修饰符后即可直接调用类方法：**

  ```python
  >>> class xc:
  ...     @classmethod
  ...     def cv(self):
  ...             print('Direct use')
  ... 
  >>> xc.cv()
  Direct use
  ```

+ @staticmethod

  > 被staticmethod修饰符的表示这是一个类的静态方法，可以被类直接调用，与@classmethod的区别在于classmethod对应的方法的第一个参数为self,为类的实例，而staticmethod不是

```python
class MyClass:
    @staticmethod
    def thisIsStaticMethod():
        print("This is static method")

if __name__ == "__main__":
    MyClass.thisIsStaticMethod()
```

+  property修饰符

​       被property修饰符修饰的方法可以像属性一样被访问，如

```python
class MyClass:
    def __init__(self,num):
        self._Num = num
    @property
    def Num(self):
        return self._Num

if __name__ == "__main__":
    c = MyClass(100)
    print c.Num   #注意，这里的访问形式看起来像是访问一个属性，但其实是一个方法
```


**修饰器property类的使用：**

```python
#!/use/bin/python3

class Person(object):

    def __init__(self):
        
        self.__x = None
        
    @property    
    def x(self):
        return self.__x
    
    @x.setter
    def x(self, value):
        self.__x = value
    
    @x.deleter
    def x(self):
        del self.__x
        
p = Person()

p.x = 123  # 自动调用 setx 方法
print(p.x)  # 自动调用 getx 方法

del p.x    # 自动调用 delx 方法
print(p.x)
```

通过property类可以使得类的方法可以像属性一样使用，使代码更为简洁

｜deleter(...)
 |      Descriptor to change the deleter on a property.
 |  
 |  getter(...)
 |      Descriptor to change the getter on a property.
 |  
 |  setter(...)
 |      Descriptor to change the setter on a property.



# 类的私有和公有

### 类的私有属性

**__private_attrs**：两个下划线开头，声明该属性为私有，不能在类的外部被使用或直接访问。在类内部的方法中使用时 **self.__private_attrs**。

### 类的方法

在类的内部，使用 def 关键字可以为类定义一个方法，与一般函数定义不同，类方法必须包含参数 self,且为第一个参数

### 类的私有方法

**__private_method**：两个下划线开头，声明该方法为私有方法，不能在类地外部调用。在类的内部调用 **self.__private_methods**

## 实例

## 实例

```python
#!/usr/bin/python
# -*- coding: UTF-8 -*-
 
class JustCounter:
    __secretCount = 0  # 私有变量
    publicCount = 0    # 公开变量
 
    def count(self):
        self.__secretCount += 1
        self.publicCount += 1
        print self.__secretCount
 
counter = JustCounter()
counter.count()
counter.count()
print counter.publicCount
print counter.__secretCount  # 报错，实例不能访问私有变量
```

```
1
2
2
Traceback (most recent call last):
  File "test.py", line 17, in <module>
    print counter.__secretCount  # 报错，实例不能访问私有变量
AttributeError: JustCounter instance has no attribute '__secretCount'
```

Python不允许实例化的类访问私有数据，但你可以使用 **object._className__attrName** 访问属性，将如下代码替换以上代码的最后一行代码：

```
.........................
print counter._JustCounter__secretCount
```

执行以上代码，执行结果如下：

```
1
2
2
2
```

### 单下划线、双下划线、头尾双下划线说明：

- **__foo__**: 定义的是特殊方法，一般是系统定义名字 ，类似 __init__() 之类的。
- **_foo**: 以单下划线开头的表示的是 protected 类型的变量，即保护类型只能允许其本身与子类进行访问，不能用于 from module import *
- **__foo**: 双下划线的表示的是私有类型(private)的变量, 只能是允许这个类本身进行访问了。