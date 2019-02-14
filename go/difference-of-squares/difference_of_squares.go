// Package diffsquares holds the implementation of SquareOfSums, SumOfSquares, Difference functions.
package diffsquares

// SquareOfSums return the square of the sum of all first 'n' natural number.
func SquareOfSums(n int) int {
	sum := 0

	for i := 1; i <= n; i++ {
		sum += i
	}

	return sum * sum
}

// SumOfSquares return the sum of the square of all first 'n' natural number.
func SumOfSquares(n int) int {
	sum := 0

	for i := 1; i <= n; i++ {
		sum += i * i
	}

	return sum
}

// Difference returns the difference of result of SquareOfSums and SumOfSquares.
func Difference(n int) int {
	return SquareOfSums(n) - SumOfSquares(n)
}