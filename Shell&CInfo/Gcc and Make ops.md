# GCC 与make的使用

[TOC]

## 1. GCC 概述

GNU gcc/g++ 可以将c /c++ 源代码，汇编程序和目标程序编译链接称为可执行程序， 对于c进行编译时，需要以下4步：

+ **Preprocessing:** 预处理生成 `.i` 文件
+ **Compilation:** 编译，将预处理后的文件转换为汇编语言生成`.s`文件
+ **Assembly:** 汇编，由汇编文件生成机器代码，即`.o` 文件
+ **LInking**： 链接，链接目标代码以及静态库，生成可执行文件

**通过gcc编译的文件后缀以及类型**

| 文件后缀    | 类型                                         |
| ----------- | -------------------------------------------- |
| .c          | c 源代码                                     |
| .cc .C .cpp | c++ 源代码                                   |
| .a          | 目标文件构成的函数库文件                     |
| .i          | 预处理过的c语言文件                          |
| .ii         | 预处理过的c++ 文件                           |
| .h          | 头文件声明                                   |
| .o          | 编译后的目标文件                             |
| .s  .S      | 汇编语言源代码文件    预处理后的汇编语言文件 |

## 2. GCC参数详解

```shell
gcc -[option] [filename]
```

- `-x language`:

  设定文件所使用的语言，使后缀名无效，该参数对于其后面的以及下一个`-x` 参数前的所有文件均有效，可以使用的语言包括

  > c, objective-c, c-header, c++, cpp-output, assembler, assembler-with-spp

  ```shell
  gcc -x c hello.pig -x none hello_.c
  # -x none 关掉上一个选项，让gcc根据后缀名自动判断文件类型
  ```

- `-c` ：

  只激活预处理，编译以及汇编，生成目标文件`.o`，不连接成为可执行文件，该选项通常用于编译不包含主程序的子文件，最后再与其他文件一起生成主程序

- `-S`:

  只进行预处理以及编译，生成相应的汇编代码

  ```shell
  gcc -S hello.c # 生成.s 汇编代码
  ```

- `-E` :

  只进行预处理操作，不生成文件，可以将其重定向到另一个文件中查看

  ```shell
  gcc -E hello.c | more
  ```

- `-o output_file`:

  确定生成的可执行文件的名称，如果不指定默认为a.out

- `-g`    `-ggdb` :

  产生gdb调试所必须的符号信息，如果要对源代码进行调试，必选加入这个选项,后一个选项会尽可能生成gdb可用的额外调试信息

- `-O0 -O1 -O2 -O3`:

  编译器的4个优化级别，0为没有优化，1是默认优化等级，3优化级别最高

- `-static`:

  禁止使用动态库，将所有的库文件都包含进去，编译出来的文件一般都比较大

- `-share`:

  尽可能的使用动态库，编译出的文件较小，对于所使用的系统要求较高

- `-ansi`:

  仅支持ansi的标准c的语法，禁用 GNU c 的特色，如： asm， typeof

- `-Idirname`:

  将dirname指定的目录路径加到程序的头文件目录列表中，在进行预处理时使用的参数`#include ` 会进行以下搜索**优先在指定的目录寻找，然后再到系统目录找**：

  > - \#include \<stdio.h>   在系统预设的文件目录例如/usr/include 中搜寻相应文件
  > - \#include "my.h"    在当前目录下寻找该文件，如果没有则到 `-I` 参数指定的文件夹下找  

- `idirafter dirname`:

  在 -I 的目录寻找失败，到这个目录找

- `-Ldirname`:

  额外的库文件目录列表,**该参数指明的路径会作为寻找库文件的优先路径**，然后再会到系统默认路径寻找，如果库文件放在多个路径下，要依次给出路径的优先顺序

- `-lname`:

  连接时装载名字为“libname.a” 的函数库，该函数库位于系统预设文件夹下或者是 -L参数指明的文件夹

  ```shell
  gcc -lm -o test test.c
  ```

- `-M`:

  生成文件的关联信息 -MD 将该信息输出到  .d 文件中

- `Wa.option`     `Wl.option`

  分别给汇编程序和连接程序传递参数

   

## 3.makefile 书写规范

### 1. makefile 简介

​	make 在执行时,需要一个命名为 Makefile 的文件。这个文件告诉 make 以何种方
式编译源代码和链接程序。典型地,可执行文件可由一些.o 文件按照一定的顺序生成
或者更新。如果在你的工程中已经存在一个活着多个正确的 Makefile。当对工程中的若
干源文件修改以后,需要根据修改来更新可执行文件或者库文件,正如前面提到的你只
需要在 shell 下执行“make”。

make 会自动根据修改情况完成源文件的对应.o 文件的
更新、库文件的更新、最终的可执行程序的更新。
make 通过比较对应文件(规则的目标和依赖,)的最后修改时间,来决定哪些文
件需要更新、那些文件不需要更新。对需要更新的文件 make 就执行数据库中所记录的
相应命令(在 make 读取 Makefile 以后会建立一个编译过程的描述数据库。此数据库
中记录了所有各个文件之间的相互关系,以及它们的关系描述)来重建它,对于不需要
重建的文件 make 什么也不做。

### 2.基本语法

```makefile
target: prerequisites
	command
	...
```

+ **target**: 目标文件可以是object file，也可以是可执行文件
+ **prerequisites**:要生成的target文件所要依赖的文件
+ **command**： 生成该目标所需要使用的编译命令

基本语法规范按照，目标，依赖，命令式的结构；command是任何可以在shell下执行的命令，**每条命令必须以tab键间隔，使用\换行时后面不可以有空格** 

```makefile
edit : main.o kbd.o command.o display.o \
insert.o search.o files.o utils.o
	cc -o edit main.o kbd.o command.o display.o \
	insert.o search.o files.o utils.o
main.o : main.c defs.h
	cc -c main.c
kbd.o : kbd.c defs.h command.h
	cc -c kbd.c
command.o : command.c defs.h command.h
	cc -c command.c
display.o : display.c defs.h buffer.h
	cc -c display.c
insert.o : insert.c defs.h buffer.h
	cc -c insert.c
search.o : search.c defs.h buffer.h
	cc -c search.c
files.o : files.c defs.h buffer.h command.h
	cc -c files.c
utils.o : utils.c defs.h
	cc -c utils.c
clean :
	rm edit main.o kbd.o command.o display.o \
	insert.o search.o files.o utils.o
```

### 3. 使用变量

可以使用变量指明某些重复出现的字段使用${objs}获取变量值

### 4. 自动推导原则

​	在使用make编译.c源文件时,编译.c源文件规则的命令可以不用明确给出。这是因为make本身存在一个默认的规则,能够自动完成对.c文件的编译并生成对应的.o文件。它执行命令“cc -c”来编译.c源文件。在Makefile中我们只需要给出需要重建的目标文件名(一个.o文件),make会自动为这个.o文件寻找合适的依赖文件(对应的.c文件。

​	对应是指:文件名除后缀外,其余都相同的两个文件),而且使用正确的命令来重建这个目标文件。对于上边的例子,此默认规则就使用命令“cc -c main.c -o main.o”来创建文件“main.o”。对一个目标文件是“N.o”,倚赖文件是“N.c”的规则,完全可以省略其规则的命令行,而由make自身决定使用默认命令。此默认规则称为make的隐含规则

### 5. make 中的自动变量

>$@		规则的目标体对应的文件名
>
>$<		规则中第一个相关的文件名
>
>$^		规则中相关文件名列表
>
>$?		规则中所有日期新于目标文件
>
>$(@D)	目标文件的目录
>
>$(@F)	目标文件的文件名部分

### 6. make 的预定义变量

> AR		归档维护程序
>
> AS		汇编程序
>
> CC		c编译程序
>
> CCP	c++ 编译程序
>
> RM		删除程序
>
> ARFIAGS	归档选项  default=rv
>
> ASFLAGD	汇编选项  
>
> CFLAGS		c 编译选项
>
> CPPFLAGS	c++ 编译选项
>
> LDFLAGS	连接选项

```makefile
%.o: %.c
	$(cc) -c $^ -o $@
#  使用g++编译器编译依赖体中所有 .c 文件，生成 .o 目标文件，名称为目标体文件名
```

