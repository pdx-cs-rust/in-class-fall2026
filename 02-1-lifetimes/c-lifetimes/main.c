#include <stdio.h>
#include <inttypes.h>

uint32_t *g(uint32_t *p) {
    *p += 1;
    return p;
}

uint32_t *f() {
    uint32_t i = 17;
    return g(&i);
}

int main() {
    uint32_t *ip = f();
    printf("%u\n", *ip);
    *ip = 13;
    printf("%u\n", *ip);
    return 0;
}
