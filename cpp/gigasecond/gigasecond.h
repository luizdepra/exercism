#if !defined(GIGASECOND_H)
#define GIGASECOND_H

#include <boost/date_time/posix_time/posix_time.hpp>

using namespace boost::posix_time;

namespace gigasecond {

ptime advance(ptime from) {
    return from + seconds((long)std::pow(10, 9));
}

}

#endif
