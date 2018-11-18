#include <iostream>
#include <stdlib.h>


using namespace std;
class Test {
  public:
    Test() {cout<<"Initializer"<<endl;}
    ~Test() {cout<<"Destroyer"<<endl;}
    void Initialize() {cout<<"Another initializer"<<endl;}
    void Destroy() {cout<<"Another Destroyer"<<endl;}
};

void UseMalloc() {
  Test *test = (Test*) malloc(sizeof(Test));
  test->Initialize();
  cout<<">>Using malloc..."<<endl;
  test->Destroy();
}

void UseNew(void) {
  Test *test = new Test;
  delete test;
}

int main() {
  UseMalloc();
  UseNew();
}

