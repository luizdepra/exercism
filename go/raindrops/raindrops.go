// Package raindrops holds the implementation a convert from numbers to raindrops language.
package raindrops

import "strconv"

// TestVersion is used to know if this code is compatible with the tests.
const TestVersion = 1

// Convert translate a number to its raindrops string representation.
func Convert(num int) string {
	str := ""
	if num%3 == 0 {
		str += "Pling"
	}
	if num%5 == 0 {
		str += "Plang"
	}
	if num%7 == 0 {
		str += "Plong"
	}
	if str == "" {
		str = strconv.Itoa(num)
	}
	return str
}