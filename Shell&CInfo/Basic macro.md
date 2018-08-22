# Base of C macro

[TOC]

## 1.Intro

​    宏是一个已经给出名称的代码片段,无论何时使用该名称,都会替换为宏定义的内容.有两种形式的宏:

+ **Object-like:** 使用时类似于数据对象
+ **Function-like:** 使用时类似于函数调用

进行宏定义时,可以将任何有效标识符定义为宏,即使是关键字,因为预处理器不处理c关键字,但是定义的预处理程序运算符不可以定义为宏,宏不可以重复定义

```c
#if
#if defined MACRO
#ifdef Macro

#if defined BUFSIZE >= 1024
//可以简化为: #if BUFSIZEB >= 1024  
//对于未进行定义的宏,将会被解释为0
```

## 2. Object-like macro

类似对象的宏就是简单的将一个代码段替换为一个宏名最常用的就是给一个数字常量定义一个符号名称,宏定义时如果多行需要用`\` 连接

```c++
#define BUFFER 1024
#define M_PI 3.14159265358979323846264338327950288
```

在编译时,将宏名替换为代码段也称为宏展开

## 3.Function-like macro

类似函数的宏就是可以执行相应的函数操作,可以传递参数的宏,`#` 用于显示一个字符串的值,可以用来取参数的值,主要功能是给后面紧跟的一个宏参数加引号.同时为了放置在宏的使用中的错误,通常在函数式的宏中使用`do{}while(0)`从而获取一个独立的代码块,可以进行整体实现

```
#define SWITCH(a, b) \
    do{ \
        a = a^b;  \
        b = a^b;  \
        a = a^b;  \
    }while(0)     
#define 
    
    
```

在GUN C中,宏可以接受可变参数列表,就像函数一样`void printf(const char* format, ...)`

通过在宏的定义中,在参数列表中使用...,可以使用保留字`__VA_ARGS__` 使用可变参数列表,同时可以使用很多编译器内置宏

+ **\__LINE\__:**  代表当前行号
+ **\__FILE\__:** 当前源文件名
+ **\__DATE\__:** 当前编译日期
+ **\__TIME\__:** 当前编译时间
+ **\__STDC\__:** 当前程序严格遵守 ANSIC 标准时,该值

```c
#define LOG(module) fprintf(stderr, "{+}File:%s Line:%d "#module, __FILE__, __lINE__)

#define LOG_V2(...) \
  do{ \
     printf("Filename:%s, Function:%s, Line$d:", __FILE__, __FUNCTION__, __LINE__); \
     printf(__VA__ARGS__); \
     printf("\n"); \
}while(0);

#define LOG_V3(format, args...) printf("[File:%s, Line:%d]"format"\n", __FILE__, __LINE__, ##args);
```

对于第三个宏,使用了可变参数列表,同时不使用预定义的`__VA_ARGS__` 作为参数,类似于`printf()`的定义,format指定格式,args指定内容,同时根据c字符串连接的特性,多个""内容会被自动连接为一个字符串,可以以此简单的格式输出

## 4.对于MIN() 的认知

```c
#define MIN(A, B) A<B?A:B
```

对于最简单的min宏的实现,但是可能出现很多bug `int a = 2* MIN(3, 4) --> 4 ` 

```
// a = 2*3<4?3:4 --> 4 shame...\
可以尝试加括号
#define MIN(A, B) (A<B?A:B)
int a = MIN(3, 4<5?4:5);
int a = MIN(3, 4 < 5 ? 4 : 5);
// => int a = (3 < 4 < 5 ? 4 : 5 ? 3 : 4 < 5 ? 4 : 5);  
//  => int a = ((3 < (4 < 5 ? 4 : 5) ? 3 : 4) < 5 ? 4 : 5);  
//   => int a = ((3 < 4 ? 3 : 4) < 5 ? 4 : 5)
//    => int a = (3 < 5 ? 4 : 5)
//     => int a = 4

尝试再加括号..
//Version 3.0
#define MIN(A,B) ((A) < (B) ? (A) : (B))
too young too simple

float a = 1.0f;
float b = MIN(a++, 1.5f);
printf("a=%f, b=%f",a,b);
// => a=3.000000, b=2.000000

```

可以采用gunc中的拓展赋值:

```
int a = ({
    int b = 1; 
    int c = 2;
    b+c;
})
```

GUN C中的min()

```
#define MIN(A, B) ({__typeof__(A) __a = A; __typeof__(B) __b = B; __a <__b?__a:__b;})
```

```
#define __NSX_PASTE__(A,B) A##B

#define MIN(A,B) __NSMIN_IMPL__(A,B,__COUNTER__)

#define __NSMIN_IMPL__(A,B,L) ({ __typeof__(A) __NSX_PASTE__(__a,L) = (A); \
                                 __typeof__(B) __NSX_PASTE__(__b,L) = (B); \
                                 (__NSX_PASTE__(__a,L) < __NSX_PASTE__(__b,L)) ? __NSX_PASTE__(__a,L) : __NSX_PASTE__(__b,L); \
                              })
```

