#include <stdio.h>
#include <string.h>
#define LOG(module) fprintf(stdout, "[+]File:%s, LINE:%d "#module"\n", __FILE__, __LINE__)

#ifndef LOG
#warning "There is no log"
#endif

#define STR(A) #A
#define TEN_TIMES(a, b) (int)a##e##b
#define FUN(a, b) a##_##b
#define CONNECT(a, b) a " " b   //在c中被""包含的字符串在编译时会自动链接为字符串
#define VERSION 1.11

#define LOG_V2(...) \
    do { \
        printf("Filename:%s, Functions:%s, Line:%d>>", __FILE__, __FUNCTION__, __LINE__); \
        printf(__VA_ARGS__); \
        printf("\n"); \
    } while (0);

#define LOG_V3(format, args...) printf("[File:%s, Line:%d]"format"\n", __FILE__, __LINE__, ##args)

//得到指定地址上的字节或者字
#define MEM_B(x) (*(byte*)(x))
#define MEM_W(x) (*(word*)(x))
//得到一个field在struct中的偏移量
#define FPOS(type, field) ((dword)&((type *)0) -> field)
//结构体中field所用的字节数
#define FSIZ(type, field) sizeof(((type*)0)->field)

//upcase
#define UPCASE(c) (((c)>='a' && (c)<='z') ? ((c)-0x20):(c))




//#if VERSION == 1.11
//  printf("The first version\n");
//#elif VERSION == 1.12
//  printf("The second version\n");
//#else
//  printf("No version\n");
//#endif


/*关于#和##:
 * #用于将宏参数字符串化
 * ##将两个token连接为一个,进行连接作用,可用于规范化变量名,函数名*/


void foo(){
    printf("I\'m a func\n");
    LOG_V2("There gone be the log version v%d", 2);
}
int main(){
    LOG(Now it is test);
    const char a[20] = "I'M Kobe";
    const char b[20] = "Bryant";
    int FUN(test, size) = 20;
    LOG(CONNECT(a, b));
    char test[40] = CONNECT("Just a", "test\n");

    printf("1*10^2: %d", TEN_TIMES(1, 2));
    printf("\n%s  %d  %s", a, FUN(test, size), test);
    foo();
    LOG_V2("LOG_V2 allows all proper enter, %s", a);
    __LINE__;
    LOG_V3("It is a good use of v%d", 3);
    char ab = UPCASE('a');
    printf("after upcase:%c\n", ab);
}
