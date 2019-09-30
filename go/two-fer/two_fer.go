// Package twofer contains a function that solves the Two Fer problem from
// Exercism.
package twofer

import "fmt"

// ShareWith returns a Two Fer string containing the provided name. If the name
// is a empty string, this function will use "you" as name instead.
func ShareWith(name string) string {
	if name == "" {
		name = "you"
	}
	return fmt.Sprintf("One for %s, one for me.", name)
}
