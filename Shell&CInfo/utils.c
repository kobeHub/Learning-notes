#include "utils.h"

int print(char* point_name, point_index value){
    printf("%s:(%d, %d, %d)\n", point_name, value.x, value.y, value.z);
    return 0;
}
