# 一.在linux下python3 源码的执行：
    1.直接运行python3 file 
    2.在代码前添加解释器路径 #！/usr/bin/python3   可以直接运行PYHTON源码
    故可以在代码中添加该行，使用方式1 时该语句相当于注释
   
   
# 二.基础语法#
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
##   4.序列对象  string list tuple
``` c
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
    
    
    
