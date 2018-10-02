#include "vector.h"

int max(int, int);

point_index a = {1, 2, 3};
point_index b = {5, 7 , 9};


int main(){
    int res;
    extern int X;
    extern int Y;
    res = max(X, Y);
    DEBUG("内部变量测试!");
    DEBUG("The max value:%d", res);

    DEBUG("多个文件间测试：");
    point_index ret = vector(a, b);
    print("first point", a);
    print("second point", b);
    print("vector:", ret);
}
int X = 10000;
int Y = 12;

int max(int x, int y){
    return (x > y ? x : y);
}



