// math.c
#include <stdint.h>

// The __attribute__((export_name("add"))) ensures the function 
// is visible to wazero with the name "add"
__attribute__((export_name("add")))
int32_t add(int32_t a, int32_t b) {
    return a + b;
}
