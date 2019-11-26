// Package luhn contains the solution for the Luhn exercise from Exercism.
package luhn

import (
	"strings"
	"unicode"
)

// Valid reports whether the code is a valid Luhn code.
func Valid(code string) bool {
	code = strings.ReplaceAll(code, " ", "")
	codeSize := len(code)

	if codeSize < 2 {
		return false
	}

	sum := 0
	mustDouble := codeSize%2 == 0
	for _, v := range code {
		if !unicode.IsNumber(v) {
			return false
		}

		value := int(v - '0')
		if mustDouble {
			value *= 2
			if value > 9 {
				value -= 9
			}
		}

		sum += value
		mustDouble = !mustDouble
	}

	return sum%10 == 0
}
