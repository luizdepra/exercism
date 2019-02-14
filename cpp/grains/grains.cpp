#include "grains.h"

namespace grains {

ull_t square(unsigned int num) {
    return 1ULL << (num - 1);
}

ull_t total() {
    return (1ULL << 64) - 1;
}

}
