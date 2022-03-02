// Package weather contains logics to handle weather conditions by location.
package weather

// CurrentCondition is the current set condition.
var CurrentCondition string

// CurrentLocation is the current set location.
var CurrentLocation string

// Forecast sets the current location and condition, and returns a formatted string.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
