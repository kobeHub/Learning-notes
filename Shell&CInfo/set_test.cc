#include <iostream>
#include <set>

using namespace std;

int main() {
  set<set<int> > test;

  int a[] = {1, 2, 3, 5, 6};
  for(int i = 0; i < 5; i++) {
    set<int> te(a, a+5);
    test.insert(te);
  }

  set<int> te = *test.begin();
  set<int>::iterator it;
  for(it = te.begin(); it != te.end(); it++)
    cout<<*it<<" ";
}
