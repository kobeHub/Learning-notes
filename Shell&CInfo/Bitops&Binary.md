# 位运算符操作

[TOC]

## 基本位运算符

- **&**  :逐位与运算符,对二进制数进行逐位与运算
- **|** :逐位或运算符
- **^**:逐位异或
- **~**:逐位取反
- **<<**  :左移n位,右补0
- **>>** :右移n位,左补0

```c
int a = 3;
int b = 5;
a&b -->  011&101=001

^运算如果与0b000000可以保持原来数值,与0b11111111可以进行取反操作
同是可以进行int值的交换
对于位运算符,可以得到组合赋值符
&=, |=, ^| <<| >>| 
a &= b <--> a = a&b
a<<=2 <-> a = a<<2
```

  ## 不同进制的表示

二进制可以直接用int 十进制表示,可以进直接进行二进制位运算,也可以添加`0b` 前缀用于标识二进制数字,

使用前缀`0`标志八进制数字, `0x` 标志十六进制数字

|          | short   | int   | long    |
| -------- | ------- | ----- | ------- |
| 八进制   | %ho     | %o    | %lo     |
| 十进制   | %hd     | %d    | %ld     |
| 十六进制 | %hx %hX | %X %x | %lx %lX |

在进行输出时,可以加上特定前缀,格式化输出前加`#` 即可 `%#lx` 