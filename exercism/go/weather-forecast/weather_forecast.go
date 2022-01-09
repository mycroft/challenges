// Package weather provides an API to know forecast for a given location.
package weather

// CurrentCondition describes current weather condition.
var CurrentCondition string

// CurrentLocation describes current location.
var CurrentLocation string

// Forecast returns the forecast given the city & condition.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
