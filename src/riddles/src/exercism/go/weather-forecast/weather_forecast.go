// Package weather.
// An app to cheack the weather.
package weather

// CurrentCondition of the weather forecast.
var CurrentCondition string

// CurrentLocation of the weather forecast.
var CurrentLocation string

// Forecast the current weather condition.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
