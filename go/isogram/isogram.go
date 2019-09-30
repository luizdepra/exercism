// Package isogram contains the solution for the Isogram exercise from Exercism.
package isogram

import "unicode"

// IsIsogram reports whether the text is an isogram.
func IsIsogram(text string) bool {
	encounteredRune := make(map[rune]bool)

	for _, r := range text {
		r = unicode.ToLower(r)

		if !unicode.IsLetter(r) {
			continue
		}

		if encounteredRune[r] {
			return false
		}

		encounteredRune[r] = true
	}

	return true
}
