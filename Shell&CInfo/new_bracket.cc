#include <iostream>

using namespace std;

class Test {
  public:
    int a;
};

int main()
{
  Test *a = new Test();
  Test *b = new Test;
  Test c;

  int* test0 = new int;
  int* test1 = new int();

  cout<<"With bracket:"<<a->a<<*test1<<endl;
  cout<<"No bracket:"<<b->a<<*test0<<endl;
  cout<<"Static:"<<c.a<<endl;
}
