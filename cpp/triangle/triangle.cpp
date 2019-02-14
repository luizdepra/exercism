#include <stdexcept>
#include "triangle.h"

namespace triangle {

triangle kind(double a, double b, double c) {
    if (a <= 0 || b <= 0 || c <= 0) {
        throw std::domain_error("invalid lenghts");
    }
    if (a + b < c || a + c < b || b + c < a) {
        throw std::domain_error("invalid lenght sum");
    }

    if (a == b && b == c) {
        return equilateral;
    } else if (a == b || a == c || b == c) {
        return isosceles;
    }
    return scalene;
}

}
