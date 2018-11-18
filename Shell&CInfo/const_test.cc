#include <iostream>

/*
 * c 中const关键字定义的变量为只读状态，分为两种只读状态：
 * 1. 全局只读状态，放在只读页面，编译器做类型检查，不可以修改
 * 2.局部只读状态，只做类型检查，语法上不可以修改，但是可以通过类型
 * 强制转换进行修改
 *
 * 注意这种修改只有在栈上操作才得以执行，即必须是静态分配的变量
 * 不可以使用动态分配类型*/
using namespace std;
void showValue(const int &j) {
  cout<<j<<endl;
}

void showValue(const char* i) {
  cout<<i<<endl;
}

int main() {
  const int i = 8;
  void *p = (void*) &i;
  int *ptr = (int*) p;
  (*ptr) ++;

  showValue(i);
  cout<<i<<endl;

  //const char* a = "test";   报错，只适用于静态分配的空间
  const char test[]  = "test";
  p = (void*) &test;
  char *c_ptr = (char*) p;
  c_ptr[1] = 'a';
  showValue(test);

  cout<<test<<endl;
}
