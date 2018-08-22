#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define LOG(format, ...) printf("[File:%s, Line:%d] "format"\n", __FILE__, __LINE__, ##__VA_ARGS__)
#define SWAP(a, b) \
    do{\
        a = a^b;\
        b = b^a;\
        a = a^b;\
    }while(0);

char *bin(int a){
    int len = (int)log2(a)+4;  //存数需要log+1, 多出3位,一位存符号位0,一位存'\0', 一位b
    char *res = (char*)malloc(sizeof(char)*len);

    int d, count=2;
    *res = '0';
    *(res+1) = 'b';
    for(int i =len-4;i>=0;i--){
        d = a >> i;
        if(d & 1)
            *(res+count) = 1 + '0';
        else
            *(res+count) = 0 + '0';
        count++;
    }
    *(res+count) = '\0';
    return res;
}

//char *binary(int a){
//    int len = (int)log2(a)+4;
//    char *res = (char*)malloc(sizeof(char)*len);
//
//    *res = '0';
//    *(res+1) = 'b';
//    itoa(a, res, 2);
//    return res;
//}

int main(){
    int a = 123;
    int b = 0;

    int c = 0b110101100101;
    int d = 127;
    int e = 0b000001111111;
    printf("return0: %d \n", a&b);
    char *test = bin(c &d);
    char *c_ptr = bin(c);
    char *d_ptr =  bin(d);
    LOG("bit operator test: ");
    LOG("c:%s  d:%s", c_ptr, d_ptr);
    LOG("c&d:get the last 7 bits: %s", test);
    LOG("128:%s", bin(128));


    LOG("\nTest a=%s, right move 0 bit: %d", bin(a), a>>0);
    LOG("Test ^ c:%s e%s-->c^e=%s", bin(c), bin(e), bin(c^e));
    long hex = 0xaff1093;
    short otc = 02703410;
    LOG("hex:%#lx otc:%#ho", hex, otc);


}
