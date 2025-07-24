package meteorology

import "fmt"

type TemperatureUnit int

const (
	Celsius    TemperatureUnit = 0
	Fahrenheit TemperatureUnit = 1
)

// Add a String method to the TemperatureUnit type
func (unit TemperatureUnit) String() string {
	units := []string{"°C", "°F"}
	return units[unit]
}

type Temperature struct {
	degree int
	unit   TemperatureUnit
}

// Add a String method to the Temperature type
func (temp Temperature) String() string {
	return fmt.Sprintf("%v %v", temp.degree, temp.unit)
}

type SpeedUnit int

const (
	KmPerHour    SpeedUnit = 0
	MilesPerHour SpeedUnit = 1
)

// Add a String method to SpeedUnit
func (unit SpeedUnit) String() string {
	units := []string{"km/h", "mph"}
	return units[unit]
}

type Speed struct {
	magnitude int
	unit      SpeedUnit
}

// Add a String method to Speed
func (speed Speed) String() string {
	return fmt.Sprintf("%v %v", speed.magnitude, speed.unit)
}

type MeteorologyData struct {
	location      string
	temperature   Temperature
	windDirection string
	windSpeed     Speed
	humidity      int
}

// Add a String method to MeteorologyData
func (data MeteorologyData) String() string {
	return fmt.Sprintf("%s: %s, Wind %s at %s, %v%% Humidity", data.location, data.temperature, data.windDirection, data.windSpeed, data.humidity)
}
