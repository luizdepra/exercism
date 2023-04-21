#include "hamming.h"

int compute(const char *lhs, const char *rhs) {
    int count = 0;
    int i = 0;
    
    while(lhs[i] && rhs[i]) {
        if (lhs[i] != rhs[i]) {
            count++;
        }

        i++;
    }

    if (lhs[i] || rhs[i]) {
        return -1;
    }
    
    return count;
}
