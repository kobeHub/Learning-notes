# C++ 中 malloc 与 new

[TOC]

## 1. C++ 内存分配机制

对于`malloc/free` ,`new/delete` 对,在c++中都可以用于动态分配和释放内存,对于静态的类型创建，会将其存放在栈中，但是动态创建的指针将会存放在栈中。c++的内存分配机制如下：

+ **栈区（stack）** 由编译器自动分配释放，存放函数局部变量值，参数值

+ **堆区（heap）** 人为分配释放，如果不人为释放将在程序结束后由操作系统回收，分配方式类似于链表

+ **全局静态区（static）** 全局变量以及静态变量，程序结束后有OS释放

  > + 初始化的全局变量
  > + 未初始化的全局变量

+ **文字常量区**   存放常量字符串

+ **代码区**      存放程序的二进制代码

```c++
//main.cc
int a = 0; //全局初始化区
char *p1; //全局未初始化区
main()
{
   int b; //栈
   char s[] = "abc"; //栈，注意此处是数组
   char *p2; //栈
   char *p3 = "123456"; // "123456\0" 放在常量区，p3在栈上
   static int c =0； //全局（静态）初始化区

   p1 = (char *)malloc(10);  //分配得来的10和20字节的区域就在堆区, 但是注意p1、p2本身是在栈中的
   p2 = (char*)malloc(20);  

   //"123456" 在代码区（不可寻址），"123456\0" 放在常量区
   //编译器可能会将它与p3所指向的"123456"优化成一个地方
   strcpy(p1,"123456");
}
```

### 对分区的归纳

**数据区：**堆，栈，静态存储区，常量区。 
**静态存储区：**全局区（全局变量区）和静态变量区（静态区）。 
**代码区：**存放程序编译后的二进制代码，不可寻址区。

### 关于const 可修改问题

```c++
#include <iostream>
using namespace std;

//const int j=5; //运行时报错

void ShowValue(const int &i) {
    cout<<i<<endl;
}

int main() 
{
    const int j=5;
    void *p=(void *)&j;
    int *ptr=(int *)p;
    (*ptr)++;
    //cout<<j<<endl; //还是会显示5，因为编译器优化的时候将j替换为文字常量5
    //但如果是int i=5; const int j=i; 则无法替换，直接输出j为6
    ShowValue(j); //显示6

    return 0; 
}
```

如上栈上申请const变量可修改，但是全局的const会运行时报错，全局变量const，让数字存放在常量区。字符串会放在常量区，有复用的可能。

字符串也是可以修改的，只要在栈上就可以：

```c++
#include <iostream>
using namespace std;

//const int j=5;

void ShowValue(const char* i) {
    cout<<i<<endl;
}

int main() 
{
    const char j[]="123"; //如果是 char *j 就会出现问题（虽然没有报错）
    void *p=(void *)&j;
    char *ptr=(char *)p;
    ptr[1]='d';
    ShowValue(j);

    return 0; 
}
```

const在C语言的意思并不是常量，而是只读变量。只读的全局变量会放在只读页面，同时编译时做类型检查。而只读的局部变量只会做类型检查，语法上不可修改而已，通过把地址强制转换一下还是可以修改的，比如 *(int* )&j = 12; 所以在汇编语言里还是有只读全局变量的概念，但是只读的局部变量却是实现不了了。

## 2. malloc 与 new 的区别

### 2.1 本质区别

`malloc/free` 是c/c++的标准库函数，而`new/delete` 是c++的操作符。对于用户自定义的对象而言，malloc无法满足动态管理对象的需求。由于malloc是库函数，而不是运算符，所以编译器无法对其进行控制，也就不可以在创建对象时执行构造函数，销毁时执行析构函数，所以其只执行了空间的分配。

因此在使用c++对于对象进行操作时需要使用`new/delete` ，如果使用`malloc/free` 将会是如下状况

```c++
class Obj
{
public:
	Obj( ) 
	{ cout  <<  "Initialization"  <<  endl; }
	~ Obj( )
	{ cout  <<  "Destroy" <<  endl; }
	void Initialize( )
	{ cout  <<  "Initialization"  <<  endl; }
	void  Destroy( )
	{ cout  <<  "Destroy"  <<  endl; }
}obj;

void  UseMallocFree( )
{
	Obj   * a  =  (Obj  *) malloc( sizeof ( obj ) );      //  allocate memory 
	a -> Initialize();                                    //  initialization
	// … 
	a -> Destroy();                                        // deconstruction 
	free(a);                                               // release memory
}

void  UseNewDelete( void )
{
	Obj   * a  =   new  Obj;                                           
	// … 
	delete a; 
}
```

new操作符在自由存储区动态分配空间，而malloc函数在堆上分配空间。自由存储区是C++基于new操作符的一个抽象概念，凡是通过new操作符进行内存申请，该内存即为自由存储区。而堆是操作系统中的术语，是操作系统所维护的一块特殊内存，用于程序的内存动态分配，C语言使用malloc从堆上分配内存，使用free释放已分配的对应内存。

### 2.2 其他区别

+ 是否需要指明内存大小

+ 返回指针的类型，是否需要进行类型转换

  malloc ： void*

  new:   对应对象的指针

  所以使用malloc函数存在返回类型安全性的问题

+ 内存分配失败的返回值

  new分配失败会抛出`bad_alloc` 异常，不会返回NULL，malloc失败返回NULL

  使用c时往往需要对分配后的结果进行判断

  ```c
  int *a  = (int *)malloc ( sizeof (int ));
  if(NULL == a)
  {
      ...
  }
  else 
  {
      ...
  }
  ```

  但是对于c++无意义，需要进行异常捕获

  ```c++
  try
  {
      int *a = new int();
  }
  catch (bad_alloc)
  {
      ...
  }
  ```

+ 是否可以重新分配内存大小

  malloc分配的内存可以根据需要进行扩展`realloc` ，但是new操作符不可以进行此操作

+ 数组的处理

  C++提供了new[]与delete[]来专门处理数组类型:

  ```c++
      A * ptr = new A[10];//分配10个A对象
  ```

  使用new[]分配的内存必须使用delete[]进行释放：

  ```c++
      delete [] ptr;
  ```

  new对数组的支持体现在它会分别调用构造函数函数初始化每一个数组元素，释放对象时为每个对象调用析构函数。注意delete[]要与new[]配套使用，不然会找出数组对象部分释放的现象，造成内存泄漏。

  至于malloc，它并知道你在这块内存上要放的数组还是啥别的东西，反正它就给你一块原始的内存，在给你个内存的地址就完事。所以如果要动态分配一个数组的内存，还需要我们手动自定数组的大小：

  ```c++
  int * ptr = (int *) malloc( sizeof(int)* 10 );//分配一个10个int元素的数组 
  ```

>  1、new自动计算需要分配的空间，而malloc需要手工计算字节数
>
> 2、new是类型安全的，而malloc不是，比如：
>                  int* p = new float[2]; // 编译时指出错误
>                  int* p = malloc(2*sizeof(float)); // 编译时无法指出错误
>           new operator 由两步构成，分别是 operator new 和 construct
>
>  3、operator new对应于malloc，但operator new可以重载，可以自定义内存分配策略，甚至不做        
>
> ​        内存分配，甚至分配到非内存设备上。而malloc无能为力
>
> 4、new将调用constructor，而malloc不能；delete将调用destructor，而free不能。
>
> 5、malloc/free要库文件支持，new/delete则不要。 

## 3. 对象的创建

c++可以动态，静态创建对象，分别对应与堆、栈分配空间。使用动态分配时得到的指针任然放在栈空间。

```c++
#include <iostream>  
using namespace std;  
class  Test {   
 5   private:  
  public:  
      add()
      {
         int x,y,sum;
        x=5;
         y=5;
         sum=x+y;
         cout<<sum<<endl;
     }
 };  
 void main()  
 {  
    Test test1;  //栈中分配  ，由操作系统进行内存的分配和管理
    Test test2 = Test;  //栈中分配  ，由操作系统进行内存的分配和管理
    Test *test3=new Test();  //堆中分配  ，由管理者进行内存的分配和管理，用完必须delete()，否则可能造成内存泄漏
    test1.add();
    test2.add();//"." 是结构体成员引用
    test3->add();//"->"是指针引用
    delete(test3);
    system("pause"); 
}
```

- new创建类对象需要指针接收，一处初始化，多处使用
- new创建类对象使用完需delete销毁
- new创建对象直接使用堆空间，而局部不用new定义类对象则使用栈空间
- new对象指针用途广泛，比如作为函数返回值、函数参数等
- 频繁调用场合并不适合new，就像new申请和释放内存一样

### new 加括号与不加括号

