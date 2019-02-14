/*
Package gigasecond resolves the following problem:

Write a program that will calculate the date that someone turned or will celebrate their 1 Gs anniversary.

A gigasecond is one billion (10**9) seconds.
*/
package gigasecond

import (
	"math"
	"time"
)

// TestVersion is used to check if this code is compatible with test cases.
const TestVersion = 2

// AddGigasecond adds a Gigasecond to time.
func AddGigasecond(t time.Time) time.Time {
	gigasecond := time.Duration(math.Pow(10, 9)) * time.Second
	return t.Add(gigasecond)
}

// Birthday is my birthday in a Time object.
var Birthday = time.Date(1987, time.December, 21, 0, 0, 0, 0, time.UTC)