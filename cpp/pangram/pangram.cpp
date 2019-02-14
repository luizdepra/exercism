#include "pangram.h"

#include <bitset>
#include <cctype>

namespace pangram {

bool is_pangram(std::string str) {
    std::bitset<26> alphabet_bitset;

    for(char& c : str) {
        if (isalpha(c)) {
            alphabet_bitset.set(std::tolower(c) - 'a');
        }
    }

    return alphabet_bitset.all();
}

}
