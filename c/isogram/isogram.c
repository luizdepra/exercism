#include <ctype.h>
#include <stdint.h>

#include "isogram.h"

bool is_isogram(const char phrase[]) {
    if (!phrase) {
        return false;
    }
    
    uint32_t letters = 0;
    for (int i = 0; phrase[i]; i++) {
        int ch = tolower(phrase[i]);
        if (ch < 'a' || ch > 'z') {
            continue;
        }
        
        if ((letters & 1 << (ch - 'a')) > 0) {
            return false;
        }

        letters |= 1 << (ch - 'a');
    }

    return true;
}