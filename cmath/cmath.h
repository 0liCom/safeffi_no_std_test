#ifndef CMATH_h
#define CMATH_h

#include <stddef.h>
#include <stdint.h>

extern void lshift_32(uint32_t *dest, uint32_t a);

extern int32_t sum_32(int32_t *vec, size_t len);

struct sum_struct {
    int32_t array[4];
};

extern int32_t sum_from_struct_32(struct sum_struct *array);

#endif // CMATH_h