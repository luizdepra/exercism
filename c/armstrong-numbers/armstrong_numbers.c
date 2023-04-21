#include <math.h>

#include "armstrong_numbers.h"

bool is_armstrong_number(int candidate) {
    if (candidate == 0) {
        return true;
    }

    int num_digits = log10(candidate) + 1;
    
    int sum = 0;
    int value = candidate;
    while (value > 0) {
        sum += pow(value % 10, num_digits);
        value /= 10;
    }

    return sum == candidate;
}