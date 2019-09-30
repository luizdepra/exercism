// Package clock holds the implementation of a clock.
package clock

import "fmt"

// TestVersion is used to know if this code is compatible with the tests.
const TestVersion = 2

const (
	maxHour   = 24
	maxMinute = 60
)

// Clock is the clock structure that holds hour and minute data.
type Clock struct {
	hour   int
	minute int
}

func calcTime(hour, minute int) (int, int) {
	minuteOverflow := 0
	if minute < 0 {
		minuteOverflow = minute/maxMinute - 1
	} else {
		minuteOverflow = minute / maxMinute
	}
	correctMinute := minute % maxMinute
	if correctMinute < 0 {
		correctMinute += maxMinute
	}

	correctHour := (hour + minuteOverflow) % maxHour
	if correctHour < 0 {
		correctHour += maxHour
	}

	return correctHour, correctMinute
}

// Time returns a Clock object initialized with the hour and the minute values.
func Time(hour, minute int) Clock {
	hour, minute = calcTime(hour, minute)

	return Clock{
		hour:   hour,
		minute: minute,
	}
}

func (clock Clock) String() string {
	return fmt.Sprintf("%02d:%02d", clock.hour, clock.minute)
}

// Add return a new Clock object with minutes added to current Clock values.
func (clock Clock) Add(minutes int) Clock {
	hour, minute := calcTime(clock.hour, clock.minute+minutes)

	clock.hour = hour
	clock.minute = minute

	return clock
}