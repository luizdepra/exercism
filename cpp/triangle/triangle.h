#if !defined(TRIANGLE_H)
#define TRIANGLE_H

namespace triangle {

enum triangle {degenerate, equilateral, isosceles, scalene};

triangle kind(double a, double b, double c);

}

#endif
