#include "vector.h"

point_index vector(point_index a, point_index b){
    point_index vec;
    vec.x = b.x - a.x;
    vec.y = b.y - a.y;
    vec.z = b.z - a.z;
    return vec;
}

void DEBUG(char* format, ...){
    va_list ap;
    va_start(ap, format);
    vfprintf(stdout, format, ap);
    printf("\n");
    va_end(ap);
    fflush(stdout);
}
