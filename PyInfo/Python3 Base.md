# 一.在linux下python3 源码的执行：
>  ```
>1.直接运行python3 file 
>2.在代码前添加解释器路径 #！/usr/bin/python3     再修改文件的执行权限  chomd +x file.py  可  以直接运行PYHTON源码
>			可以在代码中添加该行，使用方式1 时该语句相当于注释
>  ```


# 二.基础语法 
 ## 1.编码
      python3 默认以UTF-8为编码方式存储字符串，所有字符串都是Unicode字符串，也可指定编码方式
      # -*- coding: cp-1252 -*-
       
      对于python2.x：
      # -*- coding:utf-8 -*-
       su = "人生苦短" #su是一个utf-8格式的字节串
       u  = s.decode("utf-8") #  s被解码为unicode对象，赋给u
       sg = u.encode("gbk") # u被编码为gbk格式的字节串，赋给sg
       print sg

![image](https://github.com/kobeHub/Hello-world/blob/master/pic/1.png)

     Note: 
     1)制定编码方式为utf-8  存储的是字节码 需要解码(decode)为unicode对象<class 'unicode'>，再进行编码为其他格式此时类型为<class 'str'>
     2)不能直接输出decode结果：u = s.decode("utf-8"); print u; 要encode后输出。
       如果需要转换字符串编码方式，可以对字符串对象先进行
     3)x 想要查看字符串各种编码序列，只能通过encode转化成bytes类型，然后输出。str是标准形式没办法直接查看其字节序列。。
       通过urllib.parse.quote 将字节序列转化成url的中文编码形式，逆过程是unquote函数。

##    2.注释与缩进
      1)单行注释以#，多行注释可以使用三引号'''  '''  或   """    """
      2)python最具特色的就是使用缩进来表示代码块，不需要使用大括号 {} 。
      3)缩进的空格数是可变的，但是同一个代码块的语句必须包含相同的缩进空格数。
      4)一个语句可以写多行   以反斜杠链接
    	total = item1+\
    	       item2+\
    	       item3
          在[] {} () 中的多行语句不需要反斜杠

##   3.基本数据结构
![image](https://github.com/kobeHub/Hello-world/blob/master/pic/pyData.png)

## 4.序列对象  string list tuple
``` python
#!/usr/bin/python3
# -*-coding:utf-8-*-
"""
字符串、列表、元组均属于序列
string、list和tuple都属于sequence（序列）
其取值操作一致
对任一sequence对象cha[i:j] 表示将cha的下标为i到j-1的元素复制形成一个新的序列对象
i缺省值为0，j缺省值为char.length
索引有两种方式：
从左到右是从0开始，从右到左从-1 开始

cha[i:j:s]   表示将序列进行复制后，再次以s为进位大小进行复制形成新的序列对象

"""

"""
关于print函数，可以输出多个不同对象，若以逗号隔开，则在同一行输出，并且每个对象间相隔一个空格
默认换行，若不换行，则添加end=""  其e中的内容是输出结束后附加输出的内容
"""
print('%ls' % 'Now test the \'str\' \'list\' \'tuple\'')
str1 = 'Rose is a beautiful girl'  # string 对象元素不可更改
list1 = ['Mark', 651532, True, 'Jack']  # list中元素可变，类似于栈顶在尾部的栈，可以进行赋值操作
tuple1 = ()  # 可以以这种方式创建一个空的元组
tuple3 = (785,)  # 创建单元元组必须后面加逗号
tuple2 = ('Great', 65313, True, 'Amazing')  # 元组中元素不可变，

print(str1), print(list1), print(tuple1), print(tuple3), print(tuple2)

print("输出字符串的某个元素：", end='\n')
print(str1[0], str1[-1], str1[-10])
print("倒序输出：\n"+str1[::-1])

print("\n\n列表操作：")
print('list:', list1)
print('First ele and last ele:', list1[0], list1[-1])
print('Reverse:', list1[::-1])
print('pop:', list1.pop())
print('Append 655:', list1.append(655), list1)


print('\n\n')

```

### 运行结果：
![image](https://github.com/kobeHub/Hello-world/blob/master/pic/test.png)


---
### 补充说明：
```   
1.如果一个函数要返回多个值时，要以元组形式返回
   例如：
    del example(a, b)
    	      return (a, b)
	type(example(a, b))
	<class, 'tuple'>
2.python中的函数还可以接收可变长参数，比如以 "*" 开头的的参数名，会将所有的参数收集到一个元组上。
	例如：
	    def test(*args):
		    print(args)
	        return args

	    print(type(test(1,2,3,4)))    #可以看见其函数的返回值是一个元组
```
    3.字典的输出
```
可以通过遍历字典中所有元素的键值输出字典
def example(dict)
 	for c in dict
         print(c)  # 输出的是所有键值以及值的元组形式
不可以这样遍历所有元素:
     {print(c, ':', dict[c]) ~~  ## 字典中的单个元素是以二元元组存储的，故dict[c]报错}#{错误}

使用 字典的内置items()函数
for k,v in dict.items():
	print(k, ':', v)
  k,v也可以写作序列组的形式，即写作列表[k, v]  元组(k, v)  但是不能写作集合的形式，因为集合中不允许有重复元素，而字典的键值和值可能相等
```
---


##   5.集合(set)与字典(dictionary)
         集合是一个无重复元素的的特殊序列，基本功能是进行成员关系测试和删除重复元素。可以使用大括号 { } 或者 set() 函数创建集合，
        注意：创建一个空集合必须用 set() 而不是 { }，因为 { } 是用来创建一个空字典。
###  创建方式：
```python
parame = {value01,value02,...}
pa = set(value)  # 一般使用set函数是，传入一个字符串，讲每个字符作为集合的元素，去掉重复即可形成集合
```
###  集合的基本操作：
```python
#!/usr/bin/python3

student = {'Tom', 'Jim', 'Mary', 'Tom', 'Jack', 'Rose'}

print(student)   # 输出集合，重复的元素被自动去掉

# 成员测试
if('Rose' in student) :
    print('Rose 在集合中')
else :
    print('Rose 不在集合中')
 

# set可以进行集合运算
a = set('abracadabra')
b = set('alacazam')

print(a)

print(a - b)     # a和b的差集

print(a | b)     # a和b的并集

print(a & b)     # a和b的交集

print(a ^ b)     # a和b中不同时存在的元素

```
       字典（dictionary）是Python中另一个非常有用的内置数据类型。列表是有序的对象结合，字典是无序的对象集合。
    两者之间的区别在于：字典当中的元素是通过键来存取的，而不是通过偏移存取。
    字典是一种映射类型，字典用"{ }"标识，它是一个无序的键(key) : 值(value)对集合。
    键(key)必须使用不可变类型。
    在同一个字典中，键(key)必须是唯一的。

### 字典的创建方式
```python
dict = {}  # 创建空字典
dict['one'] = "The choosen"  # 向字典中添加一个元素
dict[2]     = "Guy"  # 现在字典中包含两个元素

tinydict = {'name': 'runoob','code':1, 'site': 'www.runoob.com‘}  # 直接创建，key 与 value 之间用冒号隔开

dict1 = dict([('Runoob', 1), ('Google', 2), ('Taobao', 3)])  # 利用dict函数创建

“”“
字典是二元有序序列(sequence)的无序不重复序列：具有以下形式
>>> dict_1 = dict([('a',1),('b',2),('c',3)]) #元素为元组的列表
>>> dict_1
   ': 1, 'b': 2, 'c': 3}
>>> dict_2 = dict({('a',1),('b',2),('c',3)})#元素为元组的集合
>>> dict_2
   ': 2, 'c': 3, 'a': 1}
>>> dict_3 = dict([['a',1],['b',2],['c',3]])#元素为列表的列表
>>> dict_3
   ': 1, 'b': 2, 'c': 3}
>>> dict_4 = dict((('a',1),('b',2),('c',3)))#元素为元组的元组
>>> dict_4   
>>> {'a': 1, 'b': 2, 'c': 3}

```



# 三. 运算、数据及逻辑

##      1.算数运算符
​        变量a为10，变量b为21： 

| 运算符 | 描述                                                 | 实例                  |
| ------ | ---------------------------------------------------- | --------------------- |
| +      | 两个对象相加                                         | a+b = 31              |
| -      | 得到负数或者减去另一个数                             | a-b = -11             |
| *      | 两个字相乘得到的返回值，或者是一个序列重复若干次输出 | a*b = 210             |
| /      | 除法运算，默认得到浮点结果，注意格式化输出           | b/a = 2.1             |
| %      | 取模运算                                             | b%a = 1               |
| **     | 返回x的y次幂u                                        | a**b 结果为10的21次幂 |
| //     | 整除，返回商的整数部分                               | a//b = 2              |

**对应的赋值运算符：**

| 运算符 | 描述             | 实例                                  |
| ------ | ---------------- | ------------------------------------- |
| =      | 简单的赋值运算符 | c = a + b 将 a + b 的运算结果赋值为 c |
| +=     | 加法赋值运算符   | c += a 等效于 c = c + a               |
| -=     | 减法赋值运算符   | c -= a 等效于 c = c - a               |
| *=     | 乘法赋值运算符   | c *= a 等效于 c = c * a               |
| /=     | 除法赋值运算符   | c /= a 等效于 c = c / a               |
| %=     | 取模赋值运算符   | c %= a 等效于 c = c % a               |
| **=    | 幂赋值运算符     | c **= a 等效于 c = c ** a             |
| //=    | 取整除赋值运算符 | c //= a 等效于 c = c // a             |

```python
a = 21
b = 10
c = 0

c = a + b
print ("1 - c 的值为：", c)

c += a
print ("2 - c 的值为：", c)

c *= a
print ("3 - c 的值为：", c)

c /= a 
print ("4 - c 的值为：", c)

c = 2
c %= a
print ("5 - c 的值为：", c)

c **= a
print ("6 - c 的值为：", c)

c //= a
print ("7 - c 的值为：", c)
```

>输出结果：
>
>```
>1 - c 的值为： 31
>2 - c 的值为： 52
>3 - c 的值为： 1092
>4 - c 的值为： 52.0
>5 - c 的值为： 2
>6 - c 的值为： 2097152
>7 - c 的值为： 99864
>```



## 2.python的位运算符

```
     python的位运算符是把数字当做二进制数来运算的，数字可以以任一进制输入，但是进行位运算时以二进制形式。注意默认输出是以十进制形式，进行输出时，可以采用以下函数：
bin()	采用二进制
oct()	八进制
hex()	十六进制


输入时：
2 进制是以 0b 开头的: 例如: 0b11 则表示十进制的 3
8 进制是以 0o 开头的: 例如: 0o11 则表示十进制的 9
16 进制是以 0x 开头的: 例如: 0x11 则表示十进制的 17
```



| 运算符 | 描述                                                         |
| ------ | ------------------------------------------------------------ |
| &      | 按位与运算符：<br >           参与运算的两个值每一位进行“与运算”，得到结果以二进制表示</br > |
| \|     | 按位或运算符：<br />           每一位进行逻辑或运算，即二个位置中有1即为1 |
| ^      | 按位异或逻辑符：<br />           对应位置不同时结果相应位置为1 |
| ~      | 按位取反逻辑符：<br />           对数据的每个位二进制位取反，~x 类似于 -x-1 |
| <<     | 左移逻辑符：<br />           进行左移操作，右边参数为左移位数，每左移一位，数值扩大二倍 |
| \>>    | 右移逻辑运算符：<br />           进行右移操作，每右移一位，原数值相当于缩小一半 |

## 3.逻辑运算符、成员及身份运算符

### 1）逻辑运算符

```
	逻辑运算符包括 and or not
	运算规则如下：
		x and y:
			if x==false   return false;
			else	return y;
		x or y:
			if x==true	return x;
			else	return y;
		not x:
			return -x

```

----

**注意：**

x和y可以是字符串对象或者数字，其中0和“”为假，其他为真。逻辑运算符可以连接多个对象，则and返回值为第一个为假的值，若所有只为真则返回最后一个值；or返回第一个为真的的值，所有值为假则返回最后一值，即逻辑运算需要遍历参加运算的每一个对象

---

### 2）成员运算符

```
in:	若在指定序列中找到该值则返回True  否则返回False
not in：与in作用相反
```

### 3）身份运算符

```
is：判断两个标识符是否是同一对象的引用，是则返回True
not is：---
函数id() 用于获取对象的内存地址，is 的用法x is y与id(x) == id(y)方式等价

is与==的区别：
	is用于判断是否是同一对象的不同引用，而==用于比较对象的值是否相等

python具有intern机制在交互式环境中，会有一个小整数池对象，将[-5,256]这些整数对象，提前分配好空间，所有位于这个范围的引用公用一个对象，所以会出现：
	a = 7; b = 7
	(a is b) == True

```


