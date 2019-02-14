#include "raindrops.h"

namespace raindrops {

std::string convert(unsigned int num) {
    std::string s;

    if (num % 3 == 0) {
        s += "Pling";
    }
    if (num % 5 == 0) {
        s += "Plang";
    }
    if (num % 7 == 0) {
        s += "Plong";
    }

    if (s.empty()) {
        return std::to_string(num);
    }

    return s;
}

}
