#include <stdio.h>
//#pragma pack(16)

typedef struct x1 {
    int i;
    char c1;
    char c2;
}x1;

typedef struct x2 {
    char c1;
    int i;
    char c2;
}x2;

typedef struct x3 {
    char c1;
    char c2;
    int i;
}x3;

typedef struct b1 {
    long long ll;
    int i;
    char c;
    short s;
}b1;

typedef struct xa {
    int i;
    void* ptr;
    char c;
}xa;


typedef struct stu {
    char sex;
    int length;
    char name[10];
}stu;


typedef struct all {
    double a;
    int b;
    short c;
}all;

// define alignment bound 
#pragma pack(4)
typedef struct s1 {
    short a;
    int b;
}s1;

typedef struct s2 {
    short c;
    s1 d;
    long e;
}s2;
#pragma pack()

int main() {
    printf("Basic size\n, int:%d, long:%d, ll:%d, char:%d, short:%d, void*: %d\n", sizeof(int), sizeof(long), 
            sizeof(long long), sizeof(char), sizeof(short), sizeof(void*));
    printf("x1: size %d\n", sizeof(x1));
    printf("x2: size %d\n", sizeof(x2));
    printf("x3: size %d\n", sizeof(x3));
    printf("b1: size %d\n", sizeof(b1));
    printf("xa: size %d\n", sizeof(xa));
    printf("stu: size %d\n", sizeof(stu));
    printf("all short: size %d\n", sizeof(all));
    printf("Aligned 8\n s1:%d, s2:%d\n", sizeof(s1), sizeof(s2));
}
