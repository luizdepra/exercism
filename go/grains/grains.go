// Package grains contains the solution for the Grains exercise from Exercism.
package grains

import "errors"

// Square computes the quantity of grains of wheat contained in a certain
// chessboard square.
//
// We use the left shift operator to calculate the corresponding power of 2 for
// each chessboard square.
func Square(index int) (uint64, error) {
	if index < 1 || index > 64 {
		return 0, errors.New("square index must be between 1 and 64")
	}
	return 1 << (index - 1), nil
}

// Total computes the total quantity of grains of wheat contained in the whole
// chessboard.
// Here, the sum of nth power os 2 can be expressed by (n+1)^2 -1. As we are
// using left shift starting from 0 to 63 to compute each square value, we can
// use 64 to compute the total sum.
func Total() uint64 {
	return 1<<64 - 1
}
