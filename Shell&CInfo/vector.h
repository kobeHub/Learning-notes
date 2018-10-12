#include <stdio.h>
#include <stdarg.h>

/*
 * 定义三维空间中的点的坐标以及向量函数*/
#ifndef __VECTOR_H
#define __VECTOR_H
#if 1
typedef struct{
    int x;
    int y;
    int z;
} point_index;
#endif

//定义debug输出接口
void DEBUG(char* format, ...);
point_index local_point;
point_index vector(point_index a, point_index b);
#endif
