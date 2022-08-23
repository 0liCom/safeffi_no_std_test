#include "cmath.h"

void lshift_32(uint32_t *dest, uint32_t a) {
    *dest = a << 1;
}

int32_t sum_32(int32_t *vec, size_t len) {
    int32_t sum = 0;

    for (int i = 0; i < len; i++) {
        sum += vec[i];
    }

    return sum;
}

int32_t sum_from_struct_32(struct sum_struct *s) {
    int32_t sum = 0;

    for (int i = 0; i < 4; i++) {
        sum += s->array[i];
    }

    return sum;
}