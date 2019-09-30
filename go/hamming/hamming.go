// Package hamming contains the solution for the Hamming exercise from Exercism.
package hamming

import "errors"

// Distance returns the Hamming Distance between two DNA strands. If the strands
// have different length, an error will be returned instead.
func Distance(a, b string) (int, error) {
	ar, br := []rune(a), []rune(b)

	if len(ar) != len(br) {
		return 0, errors.New("both sequences should have the same size")
	}

	count := 0
	for i, v := range ar {
		if br[i] != v {
			count++
		}
	}

	return count, nil
}
