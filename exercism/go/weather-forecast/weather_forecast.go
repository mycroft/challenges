/*
Package weather provides handy functions to show weather forecast.
*/
package weather

// CurrentCondition is the current weather condition as a string.
var CurrentCondition string

// CurrentLocation is the current weather location as a string.
var CurrentLocation string

// Forecast ouputs the weather condition in given city.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
