#include "space_age.h"

namespace space_age {

space_age::space_age(long int seconds)
    : _seconds(seconds) {}

int space_age::seconds() const {
    return _seconds;
}

double space_age::on_earth() const {
    return _seconds / EARTH_YEAR_IN_SECONDS;
}

double space_age::on_mercury() const {
    return _seconds / MERCURY_YEAR_IN_SECONDS;
}

double space_age::on_venus() const {
    return _seconds / VENUS_YEAR_IN_SECONDS;
}

double space_age::on_mars() const {
    return _seconds / MARS_YEAR_IN_SECONDS;
}

double space_age::on_jupiter() const {
    return _seconds / JUPITER_YEAR_IN_SECONDS;
}

double space_age::on_saturn() const {
    return _seconds / SATURN_YEAR_IN_SECONDS;
}

double space_age::on_uranus() const {
    return _seconds / URANUS_YEAR_IN_SECONDS;
}

double space_age::on_neptune() const {
    return _seconds / NEPTUNE_YEAR_IN_SECONDS;
}

}
