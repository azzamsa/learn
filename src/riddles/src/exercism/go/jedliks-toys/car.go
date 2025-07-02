package jedlik

import "fmt"

// Car implements a remote controlled car.
type Car struct {
	speed        int
	batteryDrain int

	battery  int
	distance int
}

// NewCar creates a new car with given specifications.
func NewCar(speed, batteryDrain int) *Car {
	return &Car{
		speed:        speed,
		batteryDrain: batteryDrain,
		battery:      100,
	}
}

func (c *Car) Drive() {
	// Can drive
	if c.batteryDrain <= c.battery {
		c.distance = c.distance + c.speed
		c.battery = c.battery - c.batteryDrain
	}

}

func (c Car) DisplayDistance() string {
	return fmt.Sprintf("Driven %d meters", c.distance)
}

func (c Car) DisplayBattery() string {
	return fmt.Sprintf("Battery at %d%%", c.battery)
}

func (c Car) CanFinish(trackDistance int) bool {
	maxDistance := c.speed * (c.battery / c.batteryDrain)
	return maxDistance >= trackDistance

}
