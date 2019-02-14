package leap

// TestVersion is used to check if the code is compatible with test cases.
const TestVersion = 1

// IsLeapYear check if the year is a leap year.
func IsLeapYear(year int) bool {
	return year%4 == 0 && (year%100 != 0 || year%400 == 0)
}