# python3 str 对象的基本操作

## 大小写变化

:black_flag:`capitalize()`:首字母大写，返回一个相对应的改变后的字符串

:black_flag::`casefold()`: `lower()`所有字符改为小写  对应`upper()` 改为大写

:black_flag: `swapcase()` : 大小写翻转

`title()`以标题形式返回

```
>>> 'i love u'.title()
'I Love U'
```



## 位置变化

:black_flag:`center(width, fillchar)`:返回一个指定宽度居中的字符串，fillchar为填充字符，默认为空格

:black_flag:`ljust()` 左对齐，并以空格填充剩余长度 

   `rjust()` 右对齐，并以空格填充剩余长度

​    `zfill`  右对齐以以0填充  

:black_flag:`count(str, beg = 0, end = len(string))`:返回str在string中出现的次数，若beg，end指定则返回指定区间的次数

## 编码

:black_flag:`bytes.decode(encoding='utf-8', errors=strict'')      encode()         `:

**参数：**

- encoding -- 要使用的编码，如"UTF-8"。
- errors -- 设置不同错误的处理方案。默认为 'strict',意为编码错误引起一个UnicodeError。 其他可能的值有 'ignore', 'replace', 'xmlcharrefreplace', 'backslashreplace' 以及通过 codecs.register_error() 注册的任何值。

##起始位置判断##
:black_flag: `endswith(suffix, beg=0, end=len(string))`: 在指定区间检查字符串是否以obj结束

:black_flag:`startwith()`:于其上类似，判断str是否以子串开始

## 查找

:black_flag:`find(str,beg=0, end=len(string))`:`index(str,[,beg=0[,end=len(string)]])`:在指定区间查找字符串，含有则返回开始的索引，若未找		到则返回-1

`rfind(str,[,beg=0[,end=len(string)]])`:从右边开始查找，返回最右边的第一个子串

## 判断属性

```python
str2 = 'xi324#'
a = str2.isalnum() #是否是alpha和num的组合（字母+数字的组合）
b = str2.isalpha() #是否是alpha（字母）
c = str2.isnumeric() #是否是numeric（数字）
str2.isdecimal() #只包含十进制的数，返回True
str2.isdigit() #是否包含数字，哪怕只包含一个数字也可以
str2.islower()#是否都是小写
str2.isupper()#是否都是大写
str2.isspace()#只包含空格
str2.istitle() #是否每个单词都是首字母大写,符合标题的形式
print(a,b,c)
```

## join连接分隔符

join函数将字符串作为分隔符插入到另一个字符串中

```python
>>> 'scd'.join('|||')
'|scd|scd|'
```

## str分割为序列

`split(str='', num=string.count(str))` 分割符是必须参数，以sep分割，返回一个列表

`partition(mid)`: 转化为一个三元组，必须参数为中间元素

`rpartition()`  从右边开始转化为三元组

```python
>>> f.split('v')
['', 's', 'fs', 'fs', '']
>>> 'hgvdgvbhvhj'.split('v')
['hg', 'dg', 'bh', 'hj']
>>> 'cdscsdc'.partition('scs')
('cd', 'scs', 'dc')
>>> 'cdscsdc'.rpartition('sc')
('cd', 'sc', 'sdc')

split() 可以指定分割次数
>>> str.split(' ', 1)
['I', 'am very glad \nto view\n it']
>>> str.split('\n', 1)
['I am very glad ', 'to view\n it']
>>> str.split('\n', )
['I am very glad ', 'to view', ' it']
>>> df = 'dvsd;dvsdvsd\'vdvds\r'
>>> re.split(',|;|\'|\r', df)
['dvsd', 'dvsdvsd', 'vdvds', '']
```

## 子串替换

`replace(sub, des)`:

 ```python
>>> 'i love u'.replace('love', 'hate')
'i hate u'
 ```

## str剪切

`lstrip()          rstrip()       strip(char='', num = string.count(str))   `

默认去除空格，可以指定字符

```python
>>> 'ikok'.lstrip('i')
'kok'
>>> 'fvbd'.rstrip('bd')
'fv'
>>> '  vdsvds   '.strip()
'vdsvds'
```

