#if !defined(SPACE_AGE_H)
#define SPACE_AGE_H

namespace space_age {

const double EARTH_YEAR_IN_SECONDS = 31557600.0;
const double MERCURY_YEAR_IN_SECONDS = EARTH_YEAR_IN_SECONDS * 0.2408467;
const double VENUS_YEAR_IN_SECONDS = EARTH_YEAR_IN_SECONDS * 0.61519726;
const double MARS_YEAR_IN_SECONDS = EARTH_YEAR_IN_SECONDS * 1.8808158;
const double JUPITER_YEAR_IN_SECONDS = EARTH_YEAR_IN_SECONDS * 11.862615;
const double SATURN_YEAR_IN_SECONDS = EARTH_YEAR_IN_SECONDS * 29.447498;
const double URANUS_YEAR_IN_SECONDS = EARTH_YEAR_IN_SECONDS * 84.016846;
const double NEPTUNE_YEAR_IN_SECONDS = EARTH_YEAR_IN_SECONDS * 164.79132;

class space_age {
        long int _seconds;
    public:
        explicit space_age(long int seconds);

        int seconds() const;
        double on_earth() const;
        double on_mercury() const;
        double on_venus() const;
        double on_mars() const;
        double on_jupiter() const;
        double on_saturn() const;
        double on_uranus() const;
        double on_neptune() const;
};

}

#endif
