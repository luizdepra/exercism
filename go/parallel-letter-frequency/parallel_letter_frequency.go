// Package letter contains the solution for the Parallel Letter Frequency
// exercise from Exercism.
package letter

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

// ConcurrentFrequency counts the frequency of each rune in a given text using
// concurrency, and returns a FreqMap.
func ConcurrentFrequency(ss []string) FreqMap {
	result := FreqMap{}
	ch := make(chan FreqMap, 10)

	for _, text := range ss {
		go func(t string) {
			ch <- Frequency(t)
		}(text)
	}

	for range ss {
		fm := <-ch
		for k, v := range fm {
			result[k] += v
		}
	}

	return result
}
